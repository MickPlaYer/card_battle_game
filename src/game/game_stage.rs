use crate::game::*;

mod end_stage;
mod main_stage;
mod new_stage;

pub use self::end_stage::EndStage;
pub use self::main_stage::MainStage;
pub use self::new_stage::NewStage;

pub trait GameStage {
    fn update(&mut self) -> Option<GameCall> {
        None
    }

    fn input(&mut self, _input: String) -> Option<GameCall> {
        None
    }
}
