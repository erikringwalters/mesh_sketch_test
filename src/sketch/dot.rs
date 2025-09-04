use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    assets::{
        materials::{UIMaterialProvider, UIMaterials},
        visibility::MESH_VISIBILITY,
    },
    cursor::Cursor,
    reload::{ReloadLevel, Reloadable},
};

use super::{
    size::DOT_MESH_RADIUS,
    sketch::{Current, SketchMode},
};

#[derive(Debug, Default, PartialEq)]
enum DotMode {
    #[default]
    Final,
    Temporary,
}

#[derive(Component, Debug, Default)]
pub struct Dot {
    mode: DotMode,
}

impl UIMaterialProvider for Dot {
    fn get_material(ui_materials: &UIMaterials) -> Handle<StandardMaterial> {
        ui_materials.dot.clone()
    }
}

#[derive(Resource, Debug)]
pub struct DotMeshHandle(pub Handle<Mesh>);

pub struct DotPlugin;

impl Plugin for DotPlugin {
    fn build(&self, app: &mut App) {
        let dot_mesh_handle = app
            .world_mut()
            .resource_mut::<Assets<Mesh>>()
            .add(Sphere::new(DOT_MESH_RADIUS));
        app.insert_resource(DotMeshHandle(dot_mesh_handle))
            .add_systems(
                Update,
                (spawn_dot
                    .run_if(in_state(SketchMode::Dot).and(input_just_pressed(MouseButton::Left))),)
                    .chain(),
            );
    }
}

pub fn spawn_dot(
    mut commands: Commands,
    dot_mesh: Res<DotMeshHandle>,
    ui_materials: Res<UIMaterials>,
    cursor: Res<Cursor>,
) {
    // let dot_entity =
    commands.spawn((
        Dot::default(),
        Mesh3d(dot_mesh.0.clone()),
        MeshMaterial3d(ui_materials.dot.clone()),
        Reloadable {
            level: ReloadLevel::Hard,
        },
        Transform::from_translation(cursor.position),
    ));
    // .id();
    // setup_dot_observes(&mut commands, &ui_materials, dot_entity);
}

pub fn spawn_temporary_dot(commands: &mut Commands, position: Vec3) -> Entity {
    commands
        .spawn((
            Dot {
                mode: DotMode::Temporary,
            },
            Reloadable {
                level: ReloadLevel::Hard,
            },
            Transform::from_translation(position),
        ))
        .id()
}

pub fn finalize_dot(
    commands: &mut Commands,
    dot_mesh: &Res<DotMeshHandle>,
    ui_materials: &Res<UIMaterials>,
    dot_entity: Entity,
    dots: &mut Query<&mut Dot>,
) -> Entity {
    if let Ok(mut dot) = dots.get_mut(dot_entity) {
        if dot.mode == DotMode::Final {
            return dot_entity;
        } else {
            dot.mode = DotMode::Final;
        }
    };
    commands
        .entity(dot_entity)
        .insert(Mesh3d(dot_mesh.0.clone()))
        .insert(MeshMaterial3d(ui_materials.dot.clone()))
        .insert(MESH_VISIBILITY);

    // setup_dot_observes(commands, ui_materials, dot_entity);
    dot_entity
}

pub fn finalize_dots(
    mut commands: Commands,
    current: ResMut<Current>,
    dot_mesh: Res<DotMeshHandle>,
    ui_materials: Res<UIMaterials>,
    mut dots: Query<&mut Dot>,
) {
    for dot in &current.dots {
        finalize_dot(&mut commands, &dot_mesh, &ui_materials, *dot, &mut dots);
    }
}
