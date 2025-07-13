use bevy::prelude::*;
use bevy_simple_subsecond_system::*;

use crate::{
    cursor::Cursor,
    reload::{ReloadLevel, Reloadable},
};

use super::size::DOT_RADIUS;

#[derive(Component, Debug, Default)]
pub struct Dot {
    pub position: Vec3,
}

#[hot]
pub fn handle_sketch_dot(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    cursor: Res<Cursor>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }
    spawn_dot(commands, meshes, materials, cursor.position);
}

#[hot]
pub fn spawn_dot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    commands.spawn((
        Dot { position: position },
        Mesh3d(meshes.add(Sphere::new(DOT_RADIUS))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(1., 0.6, 0.6, 1.),
            unlit: true,
            ..default()
        })),
        Reloadable {
            level: ReloadLevel::Hard,
        },
        Transform::from_translation(position),
    ));
    println!("spawned dot at {:?}", position);
}
