use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_simple_subsecond_system::*;

use crate::{
    assets::{materials::UIMaterials, visibility::MESH_VISIBILITY},
    cursor::Cursor,
    reload::{ReloadLevel, Reloadable},
};

use super::{
    line::{handle_dot_end_hover, handle_dot_hover},
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
    pub prev_transform: Transform,
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
            )
            .add_systems(PostUpdate, record_previous_transform);
    }
}

#[hot]
pub fn spawn_dot(
    mut commands: Commands,
    dot_mesh: Res<DotMeshHandle>,
    ui_materials: Res<UIMaterials>,
    cursor: Res<Cursor>,
) {
    let dot_entity = commands
        .spawn((
            Dot::default(),
            Mesh3d(dot_mesh.0.clone()),
            MeshMaterial3d(ui_materials.dot.clone()),
            Reloadable {
                level: ReloadLevel::Hard,
            },
            Transform::from_translation(cursor.position),
        ))
        .id();
    // setup_dot_observes(&mut commands, &ui_materials, dot_entity);
}

#[hot]
pub fn spawn_temporary_dot(commands: &mut Commands, position: Vec3) -> Entity {
    commands
        .spawn((
            Dot {
                mode: DotMode::Temporary,
                prev_transform: Transform::default(),
            },
            Reloadable {
                level: ReloadLevel::Hard,
            },
            Transform::from_translation(position),
        ))
        .id()
}

#[hot]
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
    return dot_entity;
}

#[hot]
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

// #[hot]
// pub fn setup_dot_observes(
//     commands: &mut Commands,
//     ui_materials: &Res<UIMaterials>,
//     dot_entity: Entity,
// ) {
//     commands
//         .entity(dot_entity)
//         .observe(update_material_on::<Pointer<Over>>(
//             ui_materials.hover.clone(),
//         ))
//         .observe(update_material_on::<Pointer<Out>>(ui_materials.dot.clone()))
//         .observe(update_material_on::<Pointer<Pressed>>(
//             ui_materials.pressed.clone(),
//         ))
//         .observe(update_material_on::<Pointer<Released>>(
//             ui_materials.hover.clone(),
//         ))
//         .observe(move_on_drag)
//         .observe(handle_dot_hover)
//         .observe(handle_dot_end_hover);
// }

#[hot]
pub fn record_previous_transform(mut dots: Query<(&mut Dot, &Transform)>) {
    for (mut dot, transform) in dots.iter_mut() {
        dot.prev_transform = *transform;
    }
}

#[hot]
pub fn move_on_drag(
    drag: Trigger<Pointer<Drag>>,
    cursor: Res<Cursor>,
    state: ResMut<State<SketchMode>>,
    mut transforms: Query<&mut Transform>,
) {
    // TODO: Consider a more reliably scheduled way to move dots
    if state.get().ne(&SketchMode::None) {
        return;
    }
    let mut transform = transforms.get_mut(drag.target()).unwrap();
    transform.translation = cursor.position;
}

// pub fn display_dots(dots: Query<&Transform, With<Dot>>, mut gizmos: Gizmos) {
//     for transform in dots.iter() {
//         gizmos.circle(
//             Isometry3d::new(
//                 transform.translation,
//                 Quat::from_rotation_arc(Vec3::Z, Dir3::Z.as_vec3()),
//             ),
//             DOT_RADIUS,
//             DOT,
//         );
//     }
// }
