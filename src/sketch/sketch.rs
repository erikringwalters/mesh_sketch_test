use crate::cursor::Cursor;

use bevy::prelude::*;
use bevy_simple_subsecond_system::*;

use super::{
    dot::{DotMeshHandle, DotPlugin, handle_sketch_dot},
    line::{LineMeshHandle, LinePlugin, handle_sketch_line},
};

// use super::arc::{ArcPlugin, handle_sketch_arc};
// use super::circle::{CirclePlugin, handle_sketch_circle};
// use super::rectangle::{RectanglePlugin, handle_sketch_rectangle};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States, Default, Reflect)]
pub enum SketchMode {
    #[default]
    None,
    Dot,
    Line,
    Rectangle,
    Circle,
    Arc,
}

// pub const DEFAULT_RESOLUTION: u32 = 64;
pub const DEFAULT_POS: Vec3 = Vec3::splat(f32::MIN);

#[derive(Resource, Debug, PartialEq)]
pub struct CurrentSketch {
    pub position: [Vec3; 3],
    pub lines: Vec<Entity>,
}

impl Default for CurrentSketch {
    fn default() -> Self {
        CurrentSketch {
            position: [DEFAULT_POS, DEFAULT_POS, DEFAULT_POS],
            lines: Vec::new(),
        }
    }
}
#[derive(Resource, Default, Debug, PartialEq, PartialOrd)]
pub struct LineChain {
    pub count: u32,
}

pub struct SketchPlugin;

impl Plugin for SketchPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SketchMode>()
            .insert_resource(CurrentSketch::default())
            .insert_resource(LineChain::default())
            .add_plugins(DotPlugin)
            .add_plugins(LinePlugin)
            .add_systems(Update, (change_sketch_mode, handle_sketch).chain());
    }
}

#[hot]
fn change_sketch_mode(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<NextState<SketchMode>>,
    current_sketch: ResMut<CurrentSketch>,
    line_chain: ResMut<LineChain>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        reset_sketch(current_sketch, line_chain);
        state.set(SketchMode::None);
    } else if keyboard.just_pressed(KeyCode::KeyD) {
        reset_sketch(current_sketch, line_chain);
        state.set(SketchMode::Dot);
    } else if keyboard.just_pressed(KeyCode::KeyS) {
        reset_sketch(current_sketch, line_chain);
        state.set(SketchMode::Line);
    }
}

#[hot]
fn handle_sketch(
    mut commands: Commands,
    dot_mesh: Res<DotMeshHandle>,
    line_mesh: Res<LineMeshHandle>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    state: Res<State<SketchMode>>,
    cursor: Res<Cursor>,
    current_sketch: ResMut<CurrentSketch>,
    line_chain: ResMut<LineChain>,
) {
    match state.get() {
        SketchMode::Dot => {
            handle_sketch_dot(commands, &dot_mesh, materials, mouse_input, cursor);
        }
        SketchMode::Line => {
            handle_sketch_line(
                &mut commands,
                &dot_mesh,
                &line_mesh,
                &mut materials,
                mouse_input,
                cursor,
                current_sketch,
                line_chain,
            );
        }
        _ => {
            return;
        }
    }
}

#[hot]
pub fn reset_sketch(current_sketch: ResMut<CurrentSketch>, mut line_chain: ResMut<LineChain>) {
    reset_current_sketch(current_sketch);
    line_chain.count = 0
}

#[hot]
pub fn reset_current_sketch(mut current_sketch: ResMut<CurrentSketch>) {
    *current_sketch = CurrentSketch::default();
}
