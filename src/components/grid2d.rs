use std::ops::{Add, AddAssign};
use amethyst::{
    core::transform::Transform,
    ecs::{Component, VecStorage},
};
use crate::config::CELL_SIZE;

/// Transform expressed with integers.
/// since its size <= 16 bytes and it is to be attached to most of entities,
/// VecStorage is preferable over DenseVecStorage.
/// ```
/// assert_eq!(std::mem::size_of::<Grid2D>(), 8);
/// ```
#[derive(Clone, Debug, Default, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Grid2D {
    x: i32,
    y: i32,
}

impl Grid2D {
    pub fn new(x: i32, y: i32) -> Self {
        Self{x, y}
    }

    pub fn to_transform(&self, z: f32) -> Transform {
        let mut transform = Transform::default();
        transform.set_translation_xyz(
            (self.x as f32 + 0.5) * CELL_SIZE,
            (self.y as f32 + 0.5) * CELL_SIZE,
            z,
        );
        transform
    }
}

impl From<Grid2D> for Transform {
    fn from(grid: Grid2D) -> Self {
        let mut transform = Transform::default();
        transform.set_translation_xyz(
            (grid.x as f32 + 0.5) * CELL_SIZE,
            (grid.y as f32 + 0.5) * CELL_SIZE,
            0.,
        );
        transform
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Grid2DDelta{
    x: i32,
    y: i32
}

impl Grid2DDelta {
    pub fn new(x: i32, y: i32) -> Self {
        Self{x, y}
    }
}

impl Add<Grid2DDelta> for Grid2D {
    type Output = Grid2D;
    fn add(self, rhs: Grid2DDelta) -> Self::Output {
        Grid2D::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign<Grid2DDelta> for Grid2D {
    fn add_assign(&mut self, rhs: Grid2DDelta) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add for Grid2DDelta {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Grid2DDelta::new(self.x + rhs.x, self.y + rhs.y)
    }
}