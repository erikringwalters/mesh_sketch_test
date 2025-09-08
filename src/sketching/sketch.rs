use bevy::input::common_conditions::input_pressed;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::cursor::{Cursor, is_cursor_moving};
use crate::schedule::ScheduleSet;

use super::dot::mark_moving_dots;
use super::line::{display_lines, mark_moving_lines, update_line_mesh_transforms};
use super::{dot::DotPlugin, line::LinePlugin, size::LINE_WIDTH};

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

#[derive(Component, Default)]
#[component(storage = "SparseSet")]
pub struct Moving;

// pub const DEFAULT_RESOLUTION: u32 = 64;
pub const DEFAULT_POS: Vec3 = Vec3::splat(f32::MIN);

#[derive(Resource, Debug, PartialEq)]
pub struct Current {
    pub position: [Vec3; 3],
    pub dots: Vec<Entity>,
    pub lines: Vec<Entity>,
}

#[derive(Resource, Debug, Default, PartialEq)]
pub struct Checked {
    pub lines: Vec<Entity>,
}

impl Default for Current {
    fn default() -> Self {
        Current {
            position: [DEFAULT_POS, DEFAULT_POS, DEFAULT_POS],
            dots: Vec::new(),
            lines: Vec::new(),
        }
    }
}

pub struct SketchPlugin;

impl Plugin for SketchPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SketchMode>()
            .insert_resource(Current::default())
            .insert_resource(Checked::default())
            .add_plugins(DotPlugin)
            .add_plugins(LinePlugin)
            .add_systems(Startup, sketch_setup)
            .add_systems(
                Update,
                (
                    change_sketch_mode,
                    reset_current.run_if(input_just_pressed(MouseButton::Right)),
                    (
                        mark_moving_dots,
                        mark_moving_lines,
                        update_moving_transforms,
                    )
                        .run_if(is_dragging())
                        .chain(),
                    update_line_mesh_transforms.run_if(is_cursor_moving),
                    remove_moving.run_if(not(is_cursor_moving)),
                    display_lines,
                )
                    .chain()
                    .in_set(ScheduleSet::EntityUpdates),
            );
    }
}

fn sketch_setup(mut gizmo_store: ResMut<GizmoConfigStore>) {
    let config = gizmo_store.config_mut::<DefaultGizmoConfigGroup>().0;
    config.line.width = LINE_WIDTH;
}

fn change_sketch_mode(
    commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<NextState<SketchMode>>,
    current: ResMut<Current>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        reset_current(commands, current);
        state.set(SketchMode::None);
    } else if keyboard.just_pressed(KeyCode::KeyD) {
        reset_current(commands, current);
        state.set(SketchMode::Dot);
    } else if keyboard.just_pressed(KeyCode::KeyS) {
        reset_current(commands, current);
        state.set(SketchMode::Line);
    }
}

pub fn reset_current(mut commands: Commands, mut current: ResMut<Current>) {
    for entity in &current.lines {
        commands.entity(*entity).despawn();
    }
    for entity in &current.dots {
        commands.entity(*entity).despawn();
    }
    *current = Current::default();
}

pub fn remove_moving(mut commands: Commands, query: Query<Entity, With<Moving>>) {
    for entity in query.iter() {
        commands.entity(entity).remove::<Moving>();
    }
}

pub fn update_moving_transforms(
    cursor: Res<Cursor>,
    mut query: Query<&mut Transform, With<Moving>>,
) {
    let delta = cursor.position - cursor.prev_position;
    for mut transform in query.iter_mut() {
        transform.translation += delta;
    }
}

pub fn is_dragging() -> impl Condition<()> {
    input_pressed(MouseButton::Left).and(is_cursor_moving)
}
