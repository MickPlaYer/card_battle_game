use crate::game::*;

enum Step {
    InputName,
    AssignPoints,
    ConfirmStatus,
}

pub struct NewStage {
    step: Step,
    player_data: RawPlayerData,
    total_points: u8,
}

impl NewStage {
    pub fn new() -> NewStage {
        NewStage {
            step: Step::InputName,
            player_data: PlayerData::new(),
            total_points: 10,
        }
    }
}

impl GameStage for NewStage {
    fn update(&mut self) -> Option<GameCall> {
        match self.step {
            Step::InputName => wait_for_input("Please Input Your Name:"),
            Step::AssignPoints => wait_for_input(
                format!(
                    "Please Assign Your Points: (hp,atk,def ex: 3,3,4 and sum must be {})",
                    self.total_points
                )
                .as_str(),
            ),
            Step::ConfirmStatus => wait_for_input("Confirm your status?: (yes/no)"),
        }
    }

    fn input(&mut self, input: String) -> Option<GameCall> {
        match self.step {
            Step::InputName => {
                println!("Your Name: {}", input);
                self.player_data.set_name(input);
                self.step = Step::AssignPoints
            }
            Step::AssignPoints => {
                let mut split = input.split(',');
                let hp: u32 = split.next().unwrap_or_default().parse().unwrap_or(0);
                let atk: u32 = split.next().unwrap_or_default().parse().unwrap_or(0);
                let def: u32 = split.next().unwrap_or_default().parse().unwrap_or(0);
                if hp + atk + def != self.total_points.into() {
                    println!("Sum must be {}!", self.total_points);
                    return None;
                }
                self.player_data.set_points(hp, atk, def);
                println!("{}", self.player_data);
                self.step = Step::ConfirmStatus
            }
            Step::ConfirmStatus => {
                if input == "yes" {
                    let player_data = self.player_data.to_player_data();
                    return change_stage(Box::new(MainStage::new(player_data)));
                } else if input == "no" {
                    self.step = Step::InputName
                }
            }
        }
        None
    }
}
