use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, ReadExpect, WriteStorage},
    input::{InputHandler, StringBindings, VirtualKeyCode},
};

use crate::{
    components::{
        basics::{Player, Obstacle, Movable},
        grid2d::{Grid2D, Grid2DDelta},
    },
    config::FREEZE_TIME,
    resources::{CurrentStageData, GameState},
};

#[derive(Default, SystemDesc)]
pub struct PlayerMoveSystem {
    move_timer: u32
}

impl<'s> System<'s> for PlayerMoveSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Obstacle>,
        ReadStorage<'s, Movable>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Grid2D>,
        ReadExpect<'s, CurrentStageData>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(
        &mut self,
        (
            entities, player, obstacles, movables,
            mut transforms, mut grid2ds, stage_data, input
        ): Self::SystemData
    ) {
        if stage_data.state != GameState::Play {
            return
        }

        let movement = if input.key_is_down(VirtualKeyCode::Up) {
            Grid2DDelta::new(0, 1)
        } else if input.key_is_down(VirtualKeyCode::Down) {
            Grid2DDelta::new(0, -1)
        } else if input.key_is_down(VirtualKeyCode::Left) {
            Grid2DDelta::new(-1, 0)
        } else if input.key_is_down(VirtualKeyCode::Right) {
            Grid2DDelta::new(1, 0)
        } else {
            self.move_timer = 0;
            return
        };

        if self.move_timer > 0 {
            self.move_timer -= 1;
            return;
        }

        let obstacles_place = (&obstacles, &grid2ds, &entities).join()
            .map(|(_, grid, ent)| (grid.clone(), ent.id()))
            .collect::<std::collections::HashMap<_, _>>();
        
        let movables_place = (&movables, &grid2ds, &entities).join()
            .map(|(_, grid, ent)| (grid.clone(), ent.id()))
            .collect::<std::collections::HashMap<_, _>>();

        let mut to_move = Vec::new();

        let (player_ent, _, player_grid2d) = (&entities, &player, &grid2ds).join().next().unwrap();
        let player_next_grid = player_grid2d.clone() + movement.clone();
        // if there is a obstacle, player can't move
        if obstacles_place.contains_key(&player_next_grid) {
            return;
        }
        // if there is a movable item, need to check the next block
        if let Some(&mov_ent_id) = movables_place.get(&player_next_grid) {
            let mov_next_grid = player_next_grid.clone() + movement;
            // if there is something on next block, it can't be moved
            if obstacles_place.contains_key(&mov_next_grid) || movables_place.contains_key(&mov_next_grid) {
                return
            } else {
                to_move.push((player_ent.id(), player_next_grid));
                to_move.push((mov_ent_id, mov_next_grid));
            }
        } else {
            to_move.push((player_ent.id(), player_next_grid));
        }

        for (ent_id, next_grid) in to_move {
            let ent = entities.entity(ent_id);
            let grid2d = grid2ds.get_mut(ent).unwrap();
            let transform = transforms.get_mut(ent).unwrap();
            *grid2d = next_grid;
            transform.set_translation(*grid2d.to_transform(2.).translation());
        }

        self.move_timer += FREEZE_TIME;
    }
}