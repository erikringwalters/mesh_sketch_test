use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct Constraint {
    pub horizontal: bool,
    pub vertical: bool,
}
