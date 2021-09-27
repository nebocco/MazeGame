use amethyst::{
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, SystemData, WriteExpect},
};

use crate::{
    components::{
        basics::{Baggage, Goal},
        grid2d::{Grid2D},
    },
    resources::{CurrentStageData, GameState},
};

#[derive(Default, SystemDesc)]
pub struct WinSystem;

impl<'s> System<'s> for WinSystem {
    type SystemData = (
        ReadStorage<'s, Baggage>,
        ReadStorage<'s, Goal>,
        ReadStorage<'s, Grid2D>,
        WriteExpect<'s, CurrentStageData>
    );

    fn run(&mut self, (baggages, goals, grid2ds, mut stage_data): Self::SystemData) {
        if stage_data.state != GameState::Play {
            return;
        }

        let mut baggages_position: Vec<(i32, i32)> = (&baggages, &grid2ds).join()
            .map(|(_, grid)| (grid.x, grid.y))
            .collect();

        let mut goals_position: Vec<(i32, i32)> = (&goals, &grid2ds).join()
            .map(|(_, grid)| (grid.x, grid.y))
            .collect();
        
        baggages_position.sort();
        goals_position.sort();

        if baggages_position == goals_position {
            stage_data.state = GameState::Win;
        }
    }
}