use crate::game::*;

pub enum SystemCall {
    Next,
    WaitForInput(String),
    Stop,
}

pub enum GameCall {
    ChangeStage(Box<dyn GameStage>),
    System(SystemCall),
}

pub fn wait_for_input(messaage: &str) -> Option<GameCall> {
    Some(GameCall::System(SystemCall::WaitForInput(String::from(
        messaage,
    ))))
}

pub fn change_stage(stage: Box<dyn GameStage>) -> Option<GameCall> {
    Some(GameCall::ChangeStage(stage))
}
