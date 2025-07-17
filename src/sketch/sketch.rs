use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_simple_subsecond_system::*;

use super::{
    dot::DotPlugin,
    line::{Line, LinePlugin},
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
            .add_systems(
                Update,
                (
                    change_sketch_mode,
                    reset_current_sketch.run_if(input_just_pressed(MouseButton::Right)),
                )
                    .chain(),
            );
    }
}

#[hot]
fn change_sketch_mode(
    commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<NextState<SketchMode>>,
    current_sketch: ResMut<CurrentSketch>,
    line_chain: ResMut<LineChain>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        reset_current_sketch(commands, current_sketch, line_chain);
        state.set(SketchMode::None);
    } else if keyboard.just_pressed(KeyCode::KeyD) {
        reset_current_sketch(commands, current_sketch, line_chain);
        state.set(SketchMode::Dot);
    } else if keyboard.just_pressed(KeyCode::KeyS) {
        reset_current_sketch(commands, current_sketch, line_chain);
        state.set(SketchMode::Line);
    }
}

#[hot]
pub fn reset_current_sketch(
    mut commands: Commands,
    mut current_sketch: ResMut<CurrentSketch>,
    mut line_chain: ResMut<LineChain>,
) {
    for entity in &current_sketch.lines {
        commands.entity(*entity).despawn();
    }
    *current_sketch = CurrentSketch::default();
    line_chain.count = 0
}

#[hot]
pub fn is_defined(value: Vec3) -> bool {
    value != DEFAULT_POS
}
