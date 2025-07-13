use bevy::prelude::*;
use bevy_simple_subsecond_system::*;

use crate::{
    cursor::Cursor,
    reload::{ReloadLevel, Reloadable},
};

use super::{
    dot::{Dot, spawn_dot},
    sketch::{CurrentSketch, DEFAULT_POS, LineChain, reset_sketch},
};

#[derive(Component, Debug, Default)]
pub struct Line {
    pub start: Vec3,
    pub end: Vec3,
}

#[hot]
pub fn handle_sketch_line(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    cursor: Res<Cursor>,
    mut current_sketch: ResMut<CurrentSketch>,
    mut line_chain: ResMut<LineChain>,
) {
    if mouse_input.just_pressed(MouseButton::Right) {
        reset_sketch(current_sketch, line_chain);
        return;
    }
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    // Define start of line
    if current_sketch.position[0] == DEFAULT_POS {
        current_sketch.position[0] = cursor.position;
    }
    // Define end of line
    else if current_sketch.position[1] == DEFAULT_POS {
        current_sketch.position[1] = cursor.position
    }

    let start = current_sketch.position[0];
    let end = current_sketch.position[1];

    // Create line and dots entities if both start and end are defined
    if start != DEFAULT_POS && end != DEFAULT_POS {
        if line_chain.count == 0 {
            spawn_dot(commands, meshes, materials, start);
        }
        spawn_dot(commands, meshes, materials, end);

        commands.spawn((
            Line {
                start: start,
                end: end,
            },
            Reloadable {
                level: ReloadLevel::Hard,
            },
        ));
        current_sketch.position[0] = end;
        current_sketch.position[1] = DEFAULT_POS;
        line_chain.count += 1;
    }
}
