use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::settings::Settings;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_grid);
    }
}

#[derive(Component, Debug, Reflect)]
pub struct Grid;

#[derive(Bundle, Debug)]
pub struct GridBundle {
    grid: Grid,
    spatial: SpatialBundle,
}

const GRID_TILE_HEIGHT: f32 = 0.01;

impl Default for GridBundle {
    fn default() -> Self {
        Self {
            grid: Grid,
            spatial: SpatialBundle {
                transform: Transform::from_xyz(0., -GRID_TILE_HEIGHT, 0.),
                ..default()
            },
        }
    }
}

#[derive(Component, Debug, Reflect, Default)]
pub struct GridTile(Option<Entity>);

#[derive(Bundle)]
pub struct GridTileBundle {
    tile: GridTile,
    pbr: PbrBundle,
    pickable: PickableBundle,
    pick_target: RaycastPickTarget,
    on_pointer_over: OnPointer<Over>,
    on_pointer_out: OnPointer<Out>,
}

impl GridTileBundle {
    fn new(mesh: Handle<Mesh>, material: Handle<StandardMaterial>, position: Vec2) -> Self {
        GridTileBundle {
            tile: GridTile::default(),
            pbr: PbrBundle {
                mesh,
                material,
                transform: Transform::from_xyz(position.x, 0., position.y),
                ..default()
            },
            pickable: PickableBundle::default(),
            pick_target: RaycastPickTarget::default(),
            on_pointer_over: OnPointer::<Over>::run_callback(on_pointer_over),
            on_pointer_out: OnPointer::<Out>::run_callback(on_pointer_out),
        }
    }
}

fn setup_grid(
    mut commands: Commands,
    settings: Res<Settings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let grid_id = commands.spawn(GridBundle::default()).id();

    let mesh = meshes.add(
        shape::Box::new(
            settings.tile_size.x as f32,
            GRID_TILE_HEIGHT,
            settings.tile_size.z as f32,
        )
        .into(),
    );

    let material = materials.add(Color::rgba(0.8, 0.7, 0.6, 0.4).into());

    let mut children: Vec<Entity> = Vec::new();

    for x_pos in 0..=settings.grid_size.x {
        for z_pos in 0..=settings.grid_size.z {
            let x_pos_offset = x_pos as f32 - (settings.grid_size.x as f32 / 2.);
            let z_pos_offset = z_pos as f32 - (settings.grid_size.z as f32 / 2.);

            let child = commands
                .spawn(GridTileBundle::new(
                    mesh.clone(),
                    material.clone(),
                    Vec2::new(x_pos_offset, z_pos_offset),
                ))
                .id();

            children.push(child);
        }
    }

    commands.entity(grid_id).insert_children(0, &children);
}

fn on_pointer_over(In(event): In<ListenedEvent<Over>>) -> Bubble {
    info!("Hovering over Tile");
    Bubble::Burst
}

fn on_pointer_out(In(event): In<ListenedEvent<Out>>) -> Bubble {
    info!("Hovering out of Tile");
    Bubble::Burst
}
