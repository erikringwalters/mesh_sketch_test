use bevy::prelude::*;

// #[derive(Component, Default, Debug)]
// pub struct Constraint {
//     pub horizontal: bool,
//     pub vertical: bool,
// }

#[derive(Component, Debug)]
pub struct Horizontal {
    pub to: Entity,
}

#[derive(Component, Debug)]
pub struct Vertical {
    pub to: Entity,
}
