use crate::game::*;

mod battle_stage;
mod end_stage;
mod main_stage;
mod new_stage;

pub use self::battle_stage::*;
pub use self::end_stage::*;
pub use self::main_stage::*;
pub use self::new_stage::*;

pub trait GameStage {
    fn on_enter(&mut self) {}

    fn update(&mut self) -> Option<GameCall> {
        None
    }

    fn input(&mut self, _input: String) -> Option<GameCall> {
        None
    }
}
