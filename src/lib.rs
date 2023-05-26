use bevy::prelude::*;
use bevy_egui::egui::TextureId;
use bevy_mod_picking::prelude::*;

mod controls;
mod grid;
mod loading;
mod settings;
mod ui;

pub use crate::prelude::*;

#[derive(Debug)]
pub struct SceneEditorPlugin;

impl Plugin for SceneEditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedScene>()
            .add_plugin(GridPlugin)
            .add_plugin(SettingsPlugin::default())
            .add_plugin(LoadingPlugin)
            .add_plugin(UiPlugin)
            .add_startup_system(setup);
    }
}

#[derive(Resource, Debug, Clone, Default)]
pub struct SelectedScene(Option<Scene>);

#[derive(Debug, Default, Clone)]
pub struct Scene {
    name: String,
    model: Handle<bevy::scene::Scene>,
    image: Option<TextureId>,
}

fn setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        RaycastPickCamera::default(),
    ));
}

pub mod prelude {
    pub use crate::grid::GridPlugin;
    pub use crate::loading::LoadingPlugin;
    pub use crate::settings::SettingsPlugin;
    pub use crate::ui::UiPlugin;
    pub use crate::SceneEditorPlugin;
}
