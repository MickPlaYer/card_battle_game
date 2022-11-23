use crate::game::*;

pub struct MainStage {
    player_data: PlayerData,
}

impl MainStage {
    pub fn new(player_data: PlayerData) -> MainStage {
        MainStage { player_data }
    }
}

impl GameStage for MainStage {
    fn update(&mut self) -> Option<GameCall> {
        println!("{}", self.player_data);
        wait_for_input("Please Input Something or Input 'exit' to stop:")
    }

    fn input(&mut self, input: String) -> Option<GameCall> {
        if input == "exit" {
            change_stage(Box::new(EndStage {}))
        } else {
            None
        }
    }
}
