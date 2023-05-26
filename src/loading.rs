use bevy::{
    asset::{HandleId, LoadState},
    gltf::Gltf,
    prelude::*,
    utils::HashMap,
};
use bevy_egui::EguiUserTextures;

use crate::settings::{Path, Settings};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Scenes>()
            .add_state::<LoadingState>()
            .add_system(load_scene_assets.in_schedule(OnEnter(LoadingState::Loading)))
            .add_system(check_if_loaded.in_set(OnUpdate(LoadingState::Loading)));
    }
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum LoadingState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Debug, Default)]
pub struct SceneAsset {
    pub model: Handle<Gltf>,
    pub image: Option<Handle<Image>>,
}

#[derive(Resource, Debug, Default)]
pub struct Scenes(pub HashMap<String, SceneAsset>);

impl Scenes {
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

pub fn load_scene_assets(
    mut commands: Commands,
    server: Res<AssetServer>,
    settings: Res<Settings>,
    mut egui_user_textures: ResMut<EguiUserTextures>,
) {
    let mut scenes = Scenes::default();

    let model_scenes: Vec<(String, Handle<Gltf>)> = match &settings.models {
        Path::Folder(folder) => server
            .load_folder(folder)
            .unwrap()
            .iter()
            .map(|untyped_handle| untyped_handle.clone().typed::<Gltf>())
            .map(|handle| {
                let name = get_asset_name(&handle, &server);
                (name, handle)
            })
            .collect(),
        Path::Files(files) => files
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
            SceneAsset {
                model: model.clone(),
                ..default()
            },
        )
    }));

    if let Some(image_assets) = &settings.images {
        let image_scenes: Vec<(String, Handle<Image>)> = match image_assets {
            Path::Folder(folder) => server
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
            Path::Files(files) => files
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

pub fn check_if_loaded(
    scenes: Res<Scenes>,
    server: Res<AssetServer>,
    mut loading_state: ResMut<NextState<LoadingState>>,
) {
    if scenes.is_loaded(server) {
        loading_state.set(LoadingState::Loaded);
    }
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
