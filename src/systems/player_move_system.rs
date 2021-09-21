use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings, VirtualKeyCode},
};

use crate::{
    components::{
        basics::{Player, Obstacle},
        grid2d::{Grid2D, Grid2DDelta},
    },
    config::FREEZE_TIME,
};

#[derive(Default, SystemDesc)]
pub struct PlayerMoveSystem {
    move_timer: u32
}

impl<'s> System<'s> for PlayerMoveSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, Obstacle>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Grid2D>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (player, obstacles, mut transforms, mut grid2ds, input): Self::SystemData) {
        if self.move_timer > 0 {
            self.move_timer -= 1;
            return;
        }
        let obstacles_place = (&obstacles, &grid2ds).join()
            .map(|(_, grid)| grid.clone())
            .collect::<Vec<Grid2D>>();

        for (_, transform, grid2d) in (&player, &mut transforms, &mut grid2ds).join() {
            let movement = if input.key_is_down(VirtualKeyCode::Up) {
                Some(Grid2DDelta::new(0, 1))
            } else if input.key_is_down(VirtualKeyCode::Down) {
                Some(Grid2DDelta::new(0, -1))
            } else if input.key_is_down(VirtualKeyCode::Left) {
                Some(Grid2DDelta::new(-1, 0))
            } else if input.key_is_down(VirtualKeyCode::Right) {
                Some(Grid2DDelta::new(1, 0))
            } else {
                None
            };
            if let Some(delta) = movement {
                let next_grid = grid2d.clone() + delta;
                if !obstacles_place.contains(&next_grid) {
                    *grid2d = next_grid;
                    transform.set_translation(
                        *Transform::from(grid2d.clone()).translation()
                    );
                }

                self.move_timer += FREEZE_TIME;
            }
        }
    }
}