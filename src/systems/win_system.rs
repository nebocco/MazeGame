use amethyst::{
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, SystemData, WriteExpect},
};

use crate::{
    components::{
        basics::{Player, Goal},
        grid2d::{Grid2D},
    },
    resources::{CurrentStageData, GameState},
};

#[derive(Default, SystemDesc)]
pub struct WinSystem;

impl<'s> System<'s> for WinSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, Goal>,
        ReadStorage<'s, Grid2D>,
        WriteExpect<'s, CurrentStageData>
    );

    fn run(&mut self, (player, goal, grid2ds, mut stage_data): Self::SystemData) {

        let (_, player_position) = (&player, &grid2ds).join().next().unwrap();
        let (_, goal_position) = (&goal, &grid2ds).join().next().unwrap();

        if player_position == goal_position {
            stage_data.state = GameState::Win;
        }
    }
}