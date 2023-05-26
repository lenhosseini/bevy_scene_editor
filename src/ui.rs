use bevy::{
    gltf::Gltf,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_egui::{
    egui::{self, Widget},
    EguiContexts,
};

use itertools::Itertools;

use crate::{
    loading::{LoadingState, Scenes},
    SelectedScene,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SearchText>()
            .add_system(render_scene_collection.in_set(OnUpdate(LoadingState::Loaded)))
            .add_system(render_selected_on_cursor.in_set(OnUpdate(LoadingState::Loaded)));
    }
}

#[derive(Resource, Default, Debug)]
pub struct SearchText(String);

fn render_scene_collection(
    mut commands: Commands,
    mut contexts: EguiContexts,
    assets: Res<Scenes>,
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
                    let text = egui::RichText::new(name)
                        .color(egui::Color32::LIGHT_BLUE)
                        .size(16.);

                    let image_texture = asset
                        .image
                        .as_ref()
                        .map(|image| contexts.image_id(image).unwrap());

                    let button = match image_texture {
                        Some(image) => egui::Button::image_and_text(image, [100., 100.], text),
                        None => egui::Button::new(text),
                    };

                    if button.rounding(10.).ui(ui).clicked() {
                        commands.insert_resource(SelectedScene(Some(crate::Scene {
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

fn render_selected_on_cursor(
    selected_scene: Res<SelectedScene>,
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
            egui::Color32::WHITE,
            &scene.name,
        );

        primary_window.1.cursor.grab_mode = CursorGrabMode::Confined;
        primary_window.1.cursor.visible = false;
    }
}
