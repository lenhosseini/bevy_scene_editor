use bevy::{
    prelude::*,
    window::{WindowLevel, WindowResolution},
};

use bevy_egui::EguiPlugin;
use bevy_scene_editor::{SceneEditorAsset, SceneEditorPlugin, SceneEditorSettings};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Scene Editor".into(),
                resolution: WindowResolution::new(800., 600.),
                window_level: WindowLevel::AlwaysOnTop,
                position: WindowPosition::Centered(MonitorSelection::Index(1)),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .add_plugin(SceneEditorPlugin {
            settings: SceneEditorSettings {
                images: Some(SceneEditorAsset::Folder("textures".into())),
                ..default()
            },
        })
        .add_system(bevy::window::close_on_esc)
        .run();
}
