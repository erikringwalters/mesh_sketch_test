use std::f32::consts::PI;

use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::input::common_conditions::input_just_pressed;
use bevy::{math::ops::atan, prelude::*};
use bevy_simple_subsecond_system::*;

use crate::assets::materials::UIMaterials;
use crate::{
    cursor::Cursor,
    reload::{ReloadLevel, Reloadable},
};

use super::dot::{Dot, SketchDotMeshHandle, spawn_sketch_dot};
use super::sketch::{is_defined, update_material_on};
use super::{
    dot::{DotMeshHandle, spawn_dot},
    size::LINE_WIDTH,
    sketch::{CurrentSketch, DEFAULT_POS, LineChain, SketchMode},
};

#[derive(Component, Debug)]
pub struct Line {
    pub start: Entity,
    pub end: Entity,
}

#[derive(Resource, Debug)]
pub struct LineMeshHandle(pub Handle<Mesh>);

pub struct LinePlugin;

impl Plugin for LinePlugin {
    fn build(&self, app: &mut App) {
        let mesh_handle = app
            .world_mut()
            .resource_mut::<Assets<Mesh>>()
            .add(Cylinder::new(LINE_WIDTH, 1.));
        app.insert_resource(LineMeshHandle(mesh_handle))
            .add_systems(
                Update,
                (
                    handle_spawn_current_line.run_if(
                        in_state(SketchMode::Line).and(input_just_pressed(MouseButton::Left)),
                    ),
                    handle_sketch_current_line.run_if(in_state(SketchMode::Line)),
                    display_lines,
                    //         handle_sketch_line_start.run_if(
                    //             in_state(SketchMode::Line).and(input_just_pressed(MouseButton::Left)),
                    //         ),
                    //         handle_transform_current_line.run_if(in_state(SketchMode::Line)),
                    //         handle_sketch_dot_start.run_if(
                    //             in_state(SketchMode::Line).and(input_just_pressed(MouseButton::Left)),
                    //         ),
                    //         handle_sketch_dot_end.run_if(
                    //             in_state(SketchMode::Line).and(input_just_pressed(MouseButton::Left)),
                    //         ),
                    //         handle_finalize_sketch_line.run_if(
                    //             in_state(SketchMode::Line).and(input_just_pressed(MouseButton::Left)),
                    //         ),
                )
                    .chain(),
            );
    }
}

#[hot]
pub fn handle_spawn_current_line(
    mut commands: Commands,
    mut sketch_dot_mesh: Res<SketchDotMeshHandle>,
    mut ui_materials: Res<UIMaterials>,
    cursor: Res<Cursor>,
    mut current_sketch: ResMut<CurrentSketch>,
    line_chain: ResMut<LineChain>,
) {
    if line_chain.count == 0 {
        let dot = spawn_sketch_dot(
            &mut commands,
            &mut sketch_dot_mesh,
            &mut ui_materials,
            cursor.position,
        );
        current_sketch.dots.push(dot);
    }
    let dot = spawn_sketch_dot(
        &mut commands,
        &mut sketch_dot_mesh,
        &mut ui_materials,
        cursor.position,
    );
    current_sketch.dots.push(dot);

    let line = spawn_line(&mut commands, &mut current_sketch);
    current_sketch.lines.push(line);
}

#[hot]
pub fn handle_sketch_current_line(
    cursor: Res<Cursor>,
    current_sketch: ResMut<CurrentSketch>,
    mut dots: Query<(&Dot, &mut Transform)>,
) {
    if current_sketch.dots.is_empty() {
        return;
    }
    if let Ok((_, mut transform)) = dots.get_mut(current_sketch.dots[1]) {
        transform.translation = cursor.position;
    }
}

// #[hot]
// pub fn handle_sketch_dot_start(
//     commands: Commands,
//     dot_mesh: Res<DotMeshHandle>,
//     ui_materials: Res<UIMaterials>,
//     current_sketch: ResMut<CurrentSketch>,
//     line_chain: ResMut<LineChain>,
// ) {
//     if line_chain.count == 0
//         && is_defined(current_sketch.position[0])
//         && is_defined(current_sketch.position[1])
//     {
//         spawn_dot(commands, dot_mesh, ui_materials, current_sketch.position[0]);
//     }
// }

// #[hot]
// pub fn handle_sketch_dot_end(
//     commands: Commands,
//     dot_mesh: Res<DotMeshHandle>,
//     ui_materials: Res<UIMaterials>,
//     current_sketch: ResMut<CurrentSketch>,
// ) {
//     if is_defined(current_sketch.position[0]) && is_defined(current_sketch.position[1]) {
//         spawn_dot(commands, dot_mesh, ui_materials, current_sketch.position[1]);
//     }
// }

// #[hot]
// pub fn handle_sketch_line_start(
//     commands: Commands,
//     line_mesh: Res<LineMeshHandle>,
//     ui_materials: Res<UIMaterials>,
//     cursor: Res<Cursor>,
//     mut current_sketch: ResMut<CurrentSketch>,
// ) {
//     if is_defined(current_sketch.position[0]) {
//         return;
//     }
//     current_sketch.position[0] = cursor.position;
//     let start = current_sketch.position[0];

//     current_sketch.lines.push(spawn_line(
//         commands,
//         line_mesh,
//         ui_materials,
//         start,
//         cursor.position,
//     ));
// }

// #[hot]
// pub fn handle_sketch_line_end(cursor: Res<Cursor>, mut current_sketch: ResMut<CurrentSketch>) {
//     if !is_defined(current_sketch.position[0]) {
//         return;
//     }
//     // TODO: Implement picking system that checks which entity we're hovering over
//     // If we're hovering over the drawing plane, use cursor position
//     // If we're hovering over a dot, use the dot's position
//     // TODO: Make line mesh non-pickable while it's being drawn
//     current_sketch.position[1] = cursor.position;
// }

#[hot]
fn spawn_line(commands: &mut Commands, current_sketch: &mut CurrentSketch) -> Entity {
    commands
        .spawn(Line {
            start: current_sketch.dots[0],
            end: current_sketch.dots[1],
        })
        .id()
}

// #[hot]
// fn spawn_line(
//     mut commands: Commands,
//     line_mesh: Res<LineMeshHandle>,
//     ui_materials: Res<UIMaterials>,
//     start: Entity,
//     end: Entity,
// ) -> Entity {
//     commands
//         .spawn((
//             Line {
//                 start: start.transform.translation,
//                 end: end,
//             },
//             Mesh3d(line_mesh.0.clone()),
//             MeshMaterial3d(ui_materials.line.clone()),
//             Transform::from_translation(start),
//             Reloadable {
//                 level: ReloadLevel::Hard,
//             },
//             Visibility::Hidden,
//         ))
//         .observe(update_material_on::<Pointer<Over>>(
//             ui_materials.hover.clone(),
//         ))
//         .observe(update_material_on::<Pointer<Out>>(
//             ui_materials.line.clone(),
//         ))
//         .observe(update_material_on::<Pointer<Pressed>>(
//             ui_materials.pressed.clone(),
//         ))
//         .observe(update_material_on::<Pointer<Released>>(
//             ui_materials.hover.clone(),
//         ))
//         .id()
// }

// #[hot]
// pub fn handle_finalize_sketch_line(
//     commands: Commands,
//     line_mesh: Res<LineMeshHandle>,
//     ui_materials: Res<UIMaterials>,
//     cursor: Res<Cursor>,
//     mut current_sketch: ResMut<CurrentSketch>,
//     mut line_chain: ResMut<LineChain>,
//     mut lines: Query<&Line>,
// ) {
//     if !is_defined(current_sketch.position[1]) {
//         return;
//     }
//     if let Ok(line) = lines.get_mut(current_sketch.lines[0]) {
//         let end = line.end;
//         current_sketch.lines.clear();
//         current_sketch.lines.push(spawn_line(
//             commands,
//             line_mesh,
//             ui_materials,
//             end,
//             cursor.position,
//         ));
//         current_sketch.position[0] = end;
//         current_sketch.position[1] = DEFAULT_POS;
//         line_chain.count += 1;
//     }
// }

// #[hot]
// fn handle_transform_current_line(
//     current_sketch: ResMut<CurrentSketch>,
//     cursor: Res<Cursor>,
//     mut lines: Query<(&mut Line, &mut Transform, &mut Visibility)>,
// ) {
//     if current_sketch.lines.is_empty() {
//         return;
//     }

//     if let Ok((mut line, mut transform, mut visibility)) = lines.get_mut(current_sketch.lines[0]) {
//         line.start = current_sketch.position[0];
//         line.end = cursor.position;

//         let a = line.start;
//         let b = line.end;
//         let n = b.y - a.y;
//         let d = b.x - a.x;
//         let angle = if d != 0. { atan(n / d) } else { 0. };
//         let rot = Vec3::Z * angle + vec3(0., 0., PI / 2.);

//         transform.translation = (a + b) / 2.;
//         transform.scale.y = (b - a).length();
//         transform.rotation = Quat::from_euler(EulerRot::YXZ, rot.x, rot.y, rot.z);

//         *visibility = Visibility::Visible;
//     }
// }

#[hot]
fn display_lines(
    mut gizmos: Gizmos,
    lines: Query<&Line>,
    dots: Query<&Transform>,
    cursor: Res<Cursor>,
    state: Res<State<SketchMode>>,
    current_drawing: ResMut<CurrentSketch>,
) {
    // Display existing lines
    for line in lines.iter() {
        let Ok(start_position) = dots.get(line.start) else {
            continue;
        };
        let Ok(end_position) = dots.get(line.end) else {
            continue;
        };
        gizmos.line(
            start_position.translation,
            end_position.translation,
            Color::WHITE,
        );
    }
    // Display currently drawn line
    if state.get() == &SketchMode::Line && current_drawing.position[0] != DEFAULT_POS {
        gizmos.line(current_drawing.position[0], cursor.position, Color::WHITE);
    }
}
