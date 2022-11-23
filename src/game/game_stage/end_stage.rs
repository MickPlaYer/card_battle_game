use crate::game::*;

pub struct EndStage {}

impl GameStage for EndStage {
    fn update(&mut self) -> Option<GameCall> {
        Some(GameCall::System(SystemCall::Stop))
    }
}
