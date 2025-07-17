use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_simple_subsecond_system::*;

use crate::{
    cursor::Cursor,
    reload::{ReloadLevel, Reloadable},
};

use super::{size::DOT_RADIUS, sketch::SketchMode};

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
        app.insert_resource(DotMeshHandle(mesh_handle)).add_systems(
            Update,
            handle_sketch_dot
                .run_if(in_state(SketchMode::Dot).and(input_just_pressed(MouseButton::Left))),
        );
    }
}

#[hot]
pub fn handle_sketch_dot(
    commands: Commands,
    dot_mesh: Res<DotMeshHandle>,
    materials: ResMut<Assets<StandardMaterial>>,
    cursor: Res<Cursor>,
) {
    spawn_dot(commands, dot_mesh, materials, cursor.position);
}

#[hot]
pub fn spawn_dot(
    mut commands: Commands,
    dot_mesh: Res<DotMeshHandle>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    commands.spawn((
        Dot { position: position },
        Mesh3d(dot_mesh.0.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(1., 0.4, 0., 1.),
            unlit: true,
            ..default()
        })),
        Reloadable {
            level: ReloadLevel::Hard,
        },
        Transform::from_translation(position),
    ));
}
