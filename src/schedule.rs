use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum ScheduleSet {
    UserInput,
    EntityUpdates,
    DespawnEntities,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                // Flush commands (i.e. 'apply_deferred' runs)
                ScheduleSet::UserInput,
                ScheduleSet::EntityUpdates,
                ScheduleSet::DespawnEntities,
            )
                .chain(),
        );
    }
}
