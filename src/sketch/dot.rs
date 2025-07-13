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

#[derive(Resource, Debug)]
pub struct DotMeshHandle(pub Handle<Mesh>);

pub struct DotPlugin;

impl Plugin for DotPlugin {
    fn build(&self, app: &mut App) {
        let mesh_handle = app
            .world_mut()
            .resource_mut::<Assets<Mesh>>()
            .add(Sphere::new(DOT_RADIUS));
        app.insert_resource(DotMeshHandle(mesh_handle));
    }
}

#[hot]
pub fn handle_sketch_dot(
    mut commands: Commands,
    dot_mesh: &Res<DotMeshHandle>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    cursor: Res<Cursor>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }
    spawn_dot(&mut commands, dot_mesh, &mut materials, cursor.position);
}

#[hot]
pub fn spawn_dot(
    commands: &mut Commands,
    dot_mesh: &Res<DotMeshHandle>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    commands.spawn((
        Dot { position: position },
        Mesh3d(dot_mesh.0.clone()),
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
}
