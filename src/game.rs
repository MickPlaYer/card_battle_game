mod game_call;
mod game_stage;
mod player_data;

use self::game_call::*;
use self::game_stage::*;
use self::player_data::*;

pub use self::game_call::SystemCall;

pub struct Game {
    stage: Box<dyn GameStage>,
}

impl Game {
    pub fn new() -> Game {
        let stage = Box::new(NewStage::new());
        Game { stage }
    }

    pub fn update(&mut self) -> SystemCall {
        match self.stage.update() {
            Some(game_call) => match game_call {
                GameCall::System(system_call) => system_call,
                _ => {
                    self.handle_game_call(game_call);
                    SystemCall::Next
                }
            },
            None => SystemCall::Next,
        }
    }

    pub fn input(&mut self, input: String) {
        let result = self.stage.input(input);
        if let Some(game_call) = result {
            self.handle_game_call(game_call);
        }
    }

    fn handle_game_call(&mut self, game_call: GameCall) {
        if let GameCall::ChangeStage(stage) = game_call {
            self.stage = stage
        }
    }
}
