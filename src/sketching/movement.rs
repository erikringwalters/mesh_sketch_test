use bevy::input::common_conditions::input_pressed;
use bevy::prelude::*;

use crate::constraints::constraint::{Horizontal, Vertical};
use crate::cursor::{Cursor, is_cursor_moving};

use super::dot::Dot;
use super::line::{Line, get_line_mesh_transform};
#[derive(Component, Default)]
#[component(storage = "SparseSet")]
pub struct Moving;

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

pub fn enforce_horizontal_constraint(
    mut lines: Query<&Line, With<Horizontal>>,
    mut dots: Query<&mut Transform, With<Dot>>,
) {
    for line in lines.iter_mut() {
        if let Ok([mut start_tf, mut end_tf]) = dots.get_many_mut([line.start, line.end]) {
            let avg_y = (start_tf.translation.y + end_tf.translation.y) / 2.0;
            start_tf.translation.y = avg_y;
            end_tf.translation.y = avg_y;
        } else {
            continue;
        };
    }
}
pub fn move_horizontally(
    cursor: Res<Cursor>,
    mut query: Query<&mut Transform, (With<Moving>, Without<Vertical>)>,
) {
    let delta = cursor.position - cursor.prev_position;
    for mut transform in query.iter_mut() {
        transform.translation.x += delta.x;
    }
}

pub fn move_vertically(
    cursor: Res<Cursor>,
    mut query: Query<&mut Transform, (With<Moving>, Without<Horizontal>)>,
) {
    let delta = cursor.position - cursor.prev_position;
    for mut transform in query.iter_mut() {
        transform.translation.y += delta.y;
    }
}

pub fn is_dragging() -> impl SystemCondition<()> {
    input_pressed(MouseButton::Left).and(is_cursor_moving)
}
