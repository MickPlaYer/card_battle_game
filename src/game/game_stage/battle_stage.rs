use crate::game::*;
use std::{thread::sleep, time::Duration};

enum Turn {
    Player,
    Enemy,
}

pub struct BattleStage {
    turn: Turn,
    player: EntityData,
    enemy: EntityData,
}

enum AttackResult {
    DealDamage {
        attacker: String,
        receiver: String,
        damage: u32,
    },
    Eliminate {
        attacker: String,
        receiver: String,
    },
}

trait Battle {
    fn attack(&self, other: &mut impl Battle) -> AttackResult {
        let current_hit_point = other.get_current_hit_point();
        let damage = self.get_attack_point();
        if current_hit_point <= damage {
            return AttackResult::Eliminate {
                attacker: self.get_name().clone(),
                receiver: other.get_name().clone(),
            };
        }
        other.set_current_hit_point(current_hit_point - damage);
        AttackResult::DealDamage {
            damage,
            attacker: self.get_name().clone(),
            receiver: other.get_name().clone(),
        }
    }

    fn get_name(&self) -> &String;

    fn get_attack_point(&self) -> u32;

    fn get_hit_point(&self) -> u32;

    fn get_current_hit_point(&self) -> u32;

    fn set_current_hit_point(&mut self, value: u32);

    fn get_defence_point(&self) -> u32;
}

impl<T: Stats> Battle for T {
    fn get_name(&self) -> &String {
        self.get_name()
    }

    fn get_attack_point(&self) -> u32 {
        self.get_attack_point()
    }

    fn get_hit_point(&self) -> u32 {
        self.get_hit_point()
    }

    fn get_current_hit_point(&self) -> u32 {
        self.get_current_hit_point()
    }

    fn set_current_hit_point(&mut self, value: u32) {
        self.set_current_hit_point(value)
    }

    fn get_defence_point(&self) -> u32 {
        self.get_defence_point()
    }
}

impl BattleStage {
    pub fn new(player: EntityData, enemy: EntityData) -> BattleStage {
        BattleStage {
            turn: Turn::Player,
            player,
            enemy,
        }
    }
}

impl GameStage for BattleStage {
    fn on_enter(&mut self) {
        println!("Enemy {} show up!", Battle::get_name(&self.enemy))
    }

    fn update(&mut self) -> Option<GameCall> {
        let attack_result = match self.turn {
            Turn::Player => self.player.attack(&mut self.enemy),
            Turn::Enemy => self.enemy.attack(&mut self.player),
        };
        match attack_result {
            AttackResult::DealDamage {
                damage,
                attacker,
                receiver,
            } => {
                sleep(Duration::from_secs(1));
                println!("{} deal {} damage to {}!", attacker, damage, receiver);
                self.turn = match self.turn {
                    Turn::Player => Turn::Enemy,
                    Turn::Enemy => Turn::Player,
                }
            }
            AttackResult::Eliminate { attacker, receiver } => {
                println!("{} eliminate {}!", attacker, receiver);
                return change_stage(Box::new(MainStage::new(self.player.clone())));
            }
        }
        None
    }
}
