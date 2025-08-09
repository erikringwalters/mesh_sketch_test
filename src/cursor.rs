use bevy::prelude::*;
use bevy_simple_subsecond_system::hot;

#[derive(Resource, Default)]
pub struct Cursor {
    pub position: Vec3,
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
            .add_systems(PreUpdate, (update_cursor, pick_mesh)); //, draw_cursor;
    }
}

#[hot]
fn update_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut cursor: ResMut<Cursor>,
    mut picking: ResMut<Picking>,
) {
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

#[hot]
fn draw_cursor(mut gizmos: Gizmos, cursor: Res<Cursor>) {
    gizmos.circle(
        Isometry3d::new(
            cursor.position,
            Quat::from_rotation_arc(Vec3::Z, Dir3::Z.as_vec3()),
        ),
        0.05,
        Color::WHITE,
    );
}

#[hot]
pub fn pick_mesh(mut ray_cast: MeshRayCast, mut picking: ResMut<Picking>) {
    // Cast the ray and get the first hit
    // println!("picking.selection: {:?}", picking.selection);
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
