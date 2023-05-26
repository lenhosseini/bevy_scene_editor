use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct SettingsPlugin {
    settings: Settings,
}

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Settings {
            ..self.settings.clone()
        });
    }
}

#[derive(Debug, Clone)]
pub enum Path {
    Folder(String),
    Files(Vec<String>),
}

#[derive(Resource, Debug, Clone)]
pub struct Settings {
    pub models: Path,
    pub images: Option<Path>,
    pub grid_size: UVec3,
    pub tile_size: UVec3,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            models: Path::Folder("models".into()),
            images: Some(Path::Folder("preview".into())),
            grid_size: UVec3::splat(10),
            tile_size: UVec3::splat(1),
        }
    }
}
