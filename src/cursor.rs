use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

use crate::assets::materials::ChangingMaterial;
use crate::schedule::ScheduleSet;
use crate::sketching::selection::{deselect_other_entities, select_entity};
use crate::sketching::sketch::SketchMode;

#[derive(Resource, Default)]
pub struct Cursor {
    pub position: Vec3,
    pub prev_position: Vec3,
}

#[derive(Resource)]
pub struct Picking {
    pub ray: Ray3d,
    pub hovered: Entity,
    pub prev_hovered: Entity,
}

impl Default for Picking {
    fn default() -> Self {
        Picking {
            ray: Ray3d::new(Vec3::Z, Dir3::Z),
            hovered: Entity::PLACEHOLDER,
            prev_hovered: Entity::PLACEHOLDER,
        }
    }
}

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Cursor::default())
            .insert_resource(Picking::default())
            .add_systems(
                Update,
                (
                    update_cursor,
                    hover_entity,
                    mark_hovered_changing_material,
                    select_entity.run_if(
                        in_state(SketchMode::None).and(input_just_pressed(MouseButton::Left)),
                    ),
                    deselect_other_entities.run_if(input_just_pressed(MouseButton::Left)),
                )
                    .chain()
                    .in_set(ScheduleSet::UserInput),
            );
    }
}

pub fn is_cursor_moving(cursor: Res<Cursor>) -> bool {
    cursor.position - cursor.prev_position != Vec3::ZERO
}

fn update_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut cursor: ResMut<Cursor>,
    mut picking: ResMut<Picking>,
) {
    cursor.prev_position = cursor.position;

    let Ok(windows) = windows.single() else {
        return;
    };

    let (camera, camera_transform) = *camera_query;

    let Some(cursor_position) = windows.cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the floor plane.
    let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Dir3::Z)) else {
        return;
    };

    picking.ray = ray;
    cursor.position = ray.get_point(distance);
}

pub fn hover_entity(mut ray_cast: MeshRayCast, mut picking: ResMut<Picking>) {
    // Cast the ray and get the first hit
    let Some((entity, _)) = ray_cast
        .cast_ray(picking.ray, &MeshRayCastSettings::default())
        .first()
    else {
        picking.prev_hovered = picking.hovered;
        picking.hovered = Entity::PLACEHOLDER;
        return;
    };
    picking.prev_hovered = picking.hovered;
    picking.hovered = *entity;
}

pub fn mark_hovered_changing_material(mut commands: Commands, picking: Res<Picking>) {
    if picking.hovered != picking.prev_hovered && picking.hovered != Entity::PLACEHOLDER {
        commands.entity(picking.hovered).insert(ChangingMaterial);
    }
    if picking.prev_hovered != picking.hovered && picking.prev_hovered != Entity::PLACEHOLDER {
        commands
            .entity(picking.prev_hovered)
            .insert(ChangingMaterial);
    }
}

pub fn reset_picking(mut picking: ResMut<Picking>) {
    *picking = Picking::default();
}
