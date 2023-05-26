use bevy::{
    prelude::*,
    window::{PresentMode, WindowLevel, WindowResolution},
};
use bevy_egui::EguiPlugin;
use bevy_mod_picking::prelude::*;
use bevy_scene_editor::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync,
                title: "Bevy Scene Editor".into(),
                resolution: WindowResolution::new(800., 600.),
                window_level: WindowLevel::AlwaysOnTop,
                position: WindowPosition::Centered(MonitorSelection::Index(1)),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(SceneEditorPlugin)
        .run();
}
