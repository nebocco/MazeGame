//! Basic Components
use amethyst::{
    ecs::{Component, NullStorage},
};

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct Player;

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct Goal;

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct Obstacle;

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct Wall;

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct WallInvisible;