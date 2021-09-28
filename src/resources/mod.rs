#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameState {
    Play,
    Win
}

pub struct CurrentStageData {
    pub state: GameState,
    pub stage: usize
}

impl Default for CurrentStageData {
    fn default() -> Self {
        Self {
            state: GameState::Play,
            stage: 1
        }
    }
}

impl CurrentStageData {
    pub fn next_stage(&mut self) {
        self.state = GameState::Play;
        self.stage += 1;
    }
}