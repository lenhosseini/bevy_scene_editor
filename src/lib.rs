use bevy::{
    asset::{HandleId, LoadState},
    gltf::Gltf,
    prelude::*,
    utils::HashMap,
    window::PrimaryWindow,
};
use bevy_egui::{
    egui::{self, Color32, RichText, TextureId, Widget},
    EguiContexts, EguiUserTextures,
};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SceneEditorAsset {
    Folder(String),
    Files(Vec<String>),
}

#[derive(Default)]
pub struct SceneEditorPlugin {
    pub settings: SceneEditorSettings,
}

#[derive(Resource, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SceneEditorSettings {
    pub models: SceneEditorAsset,
    pub images: Option<SceneEditorAsset>,
}

impl SceneEditorSettings {
    pub fn new(models: SceneEditorAsset, images: Option<SceneEditorAsset>) -> Self {
        Self { models, images }
    }
}

impl Default for SceneEditorSettings {
    fn default() -> Self {
        Self {
            models: SceneEditorAsset::Folder("models".into()),
            images: default(),
        }
    }
}

impl Plugin for SceneEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SceneEditorState>()
            .init_resource::<SceneEditorScenes>()
            .init_resource::<SearchText>()
            .init_resource::<SceneEditorScene>()
            .insert_resource(SceneEditorSettings {
                ..self.settings.clone()
            })
            .add_system(load_scene_assets.in_schedule(OnEnter(SceneEditorState::Loading)))
            .add_system(transition_state.in_set(OnUpdate(SceneEditorState::Loading)))
            .add_system(setup_scene.in_schedule(OnEnter(SceneEditorState::Editor)))
            .add_system(render_scene_editor.in_set(OnUpdate(SceneEditorState::Editor)))
            .add_system(highlight_selected_scene.in_set(OnUpdate(SceneEditorState::Editor)));
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Asset {
    model: Handle<Gltf>,
    image: Option<Handle<Image>>,
}

#[derive(Resource, Debug, Clone, PartialEq, Eq, Default)]
pub struct SceneEditorScenes(HashMap<String, Asset>);

#[derive(Resource, Debug, Clone, PartialEq, Eq, Default)]
pub struct SceneEditorScene(Option<Scene>);

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Scene {
    name: String,
    model: Handle<bevy::scene::Scene>,
    image: Option<TextureId>,
}

impl SceneEditorScenes {
    fn is_loaded(&self, server: Res<AssetServer>) -> bool {
        let models: Vec<HandleId> = self.0.values().map(|scene| scene.model.id()).collect();
        let images: Vec<HandleId> = self
            .0
            .values()
            .map(|scene| scene.image.clone())
            .filter(|image| image.is_some())
            .map(|image| image.unwrap().id())
            .collect();

        let models_loaded = server.get_group_load_state(models) == LoadState::Loaded;
        let images_loaded = server.get_group_load_state(images) == LoadState::Loaded;

        models_loaded && images_loaded
    }
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum SceneEditorState {
    #[default]
    Loading,
    Editor,
}

#[derive(Resource, Default, Clone, PartialEq, Eq, Debug)]
pub struct SearchText(String);

fn load_scene_assets(
    mut commands: Commands,
    server: Res<AssetServer>,
    settings: Res<SceneEditorSettings>,
    mut egui_user_textures: ResMut<EguiUserTextures>,
) {
    let mut scenes = SceneEditorScenes::default();

    let model_scenes: Vec<(String, Handle<Gltf>)> = match &settings.models {
        SceneEditorAsset::Folder(folder) => server
            .load_folder(folder)
            .unwrap()
            .iter()
            .map(|untyped_handle| untyped_handle.clone().typed::<Gltf>())
            .map(|handle| {
                let name = get_asset_name(&handle, &server);
                (name, handle)
            })
            .collect(),
        SceneEditorAsset::Files(files) => files
            .iter()
            .map(|file| server.load_untyped(file).typed::<Gltf>())
            .map(|handle| {
                let name = get_asset_name(&handle, &server);
                (name, handle)
            })
            .collect(),
    };

    scenes.0.extend(model_scenes.iter().map(|(name, model)| {
        (
            name.clone(),
            Asset {
                model: model.clone(),
                ..default()
            },
        )
    }));

    if let Some(image_assets) = &settings.images {
        let image_scenes: Vec<(String, Handle<Image>)> = match image_assets {
            SceneEditorAsset::Folder(folder) => server
                .load_folder(folder)
                .unwrap()
                .iter()
                .map(|untyped_handle| untyped_handle.clone().typed::<Image>())
                .map(|handle| {
                    egui_user_textures.add_image(handle.clone());
                    let name = get_asset_name(&handle, &server);
                    (name, handle)
                })
                .collect(),
            SceneEditorAsset::Files(files) => files
                .iter()
                .map(|file| server.load_untyped(file).typed::<Image>())
                .map(|handle| {
                    egui_user_textures.add_image(handle.clone());
                    let name = get_asset_name(&handle, &server);
                    (name, handle)
                })
                .collect(),
        };

        for (name, image) in image_scenes {
            scenes
                .0
                .entry(name)
                .and_modify(|scene| scene.image = Some(image.clone()));
        }
    }

    commands.insert_resource(scenes);
}

fn transition_state(
    server: Res<AssetServer>,
    assets: Res<SceneEditorScenes>,
    mut state: ResMut<NextState<SceneEditorState>>,
) {
    if assets.is_loaded(server) {
        state.set(SceneEditorState::Editor);
    };
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn get_asset_name<T: bevy::asset::Asset>(handle: &Handle<T>, server: &Res<AssetServer>) -> String {
    server
        .get_handle_path(handle)
        .unwrap()
        .path()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .split('.')
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .to_owned()
        .into()
}

fn render_scene_editor(
    mut commands: Commands,
    mut contexts: EguiContexts,
    assets: Res<SceneEditorScenes>,
    mut search_text: ResMut<SearchText>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("Scenes")
        .default_width(200.)
        .show(&ctx.clone(), |ui| {
            ui.text_edit_singleline(&mut search_text.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                for (name, asset) in assets
                    .0
                    .iter()
                    .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
                    .filter(|scene| scene.0.contains(&search_text.0))
                {
                    let text = RichText::new(name).color(Color32::LIGHT_BLUE).size(16.);

                    let image_texture = asset
                        .image
                        .as_ref()
                        .map(|image| contexts.image_id(image).unwrap());

                    let button = match image_texture {
                        Some(image) => egui::Button::image_and_text(image, [100., 100.], text),
                        None => egui::Button::new(text),
                    };

                    if button.rounding(10.).ui(ui).clicked() {
                        commands.insert_resource(SceneEditorScene(Some(Scene {
                            name: name.clone(),
                            model: assets_gltf.get(&asset.model).unwrap().scenes[0].clone(),
                            image: image_texture,
                        })));
                    }
                }

                ui.allocate_space(ui.available_size());
            })
        });
}

fn highlight_selected_scene(
    selected_scene: Res<SceneEditorScene>,
    mut contexts: EguiContexts,
    mut windows: Query<(Entity, &mut Window), With<PrimaryWindow>>,
) {
    if let Some(scene) = &selected_scene.0 {
        let mut primary_window = windows.single_mut();

        let cursor_position = match primary_window.1.cursor_position() {
            Some(pos) => pos,
            None => return,
        };

        let ctx = contexts.ctx_for_window_mut(primary_window.0);

        let to_egui_pos = |v: Vec2| egui::pos2(v.x, primary_window.1.height() - v.y);

        let painter = ctx.debug_painter();

        if let Some(image) = scene.image {
            painter.image(
                image,
                egui::Rect::from_center_size(
                    to_egui_pos(cursor_position),
                    egui::Vec2::new(100., 100.),
                ),
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );
        }

        let text_pos = match scene.image.is_some() {
            true => (to_egui_pos(cursor_position) - egui::Pos2::new(0., -50.)).to_pos2(),
            false => to_egui_pos(cursor_position),
        };

        painter.debug_text(
            text_pos,
            egui::Align2::CENTER_CENTER,
            Color32::WHITE,
            &scene.name,
        );

        primary_window.1.cursor.visible = false;
    }
}
