use crate::game::*;
use std::{thread::sleep, time::Duration};

enum Turn {
    Player,
    Enemy,
}

enum AttackResult {
    DealDamage {
        damage: u32,
        attacker: String,
        receiver: String,
    },
    Eliminate {
        damage: u32,
        attacker: String,
        receiver: String,
    },
}

trait Battle {
    fn attack(&self, other: &mut impl Battle, extra_damage: u32, block: u32) -> AttackResult {
        let current_hit_point = other.get_current_hit_point();
        let total_defence = other.get_defence_point() + block;
        let damage = (self.get_attack_point() + extra_damage).checked_sub(total_defence);
        let damage = damage.unwrap_or(0);
        println!("Extra Damage: {}, Block: {}", extra_damage, block);
        println!("Total Damage: {}, Total Defence: {}", damage, total_defence);
        if current_hit_point <= damage {
            other.set_current_hit_point(0);
            return AttackResult::Eliminate {
                damage: current_hit_point,
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

pub struct BattleStage {
    turn: Turn,
    deck: Deck,
    player: EntityData,
    enemy: EntityData,
}

impl BattleStage {
    pub fn new(player: EntityData, enemy: EntityData) -> BattleStage {
        BattleStage {
            turn: Turn::Player,
            deck: Deck::new(),
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
        print!("\n");
        let attack_result = match self.turn {
            Turn::Player => process_attack(&mut self.deck, &self.player, &mut self.enemy),
            Turn::Enemy => process_attack(&mut self.deck, &self.enemy, &mut self.player),
        };
        match attack_result {
            AttackResult::DealDamage {
                damage,
                attacker,
                receiver,
            } => {
                wait();
                println!("{} deal {} damage to {}!", attacker, damage, receiver);
                self.turn = match self.turn {
                    Turn::Player => Turn::Enemy,
                    Turn::Enemy => Turn::Player,
                }
            }
            AttackResult::Eliminate {
                damage,
                attacker,
                receiver,
            } => {
                wait();
                println!("{} deal {} damage to {}!", attacker, damage, receiver);
                wait();
                println!("{} eliminate {}!", attacker, receiver);
                return match self.turn {
                    Turn::Player => change_stage(Box::new(MainStage::new(self.player.clone()))),
                    Turn::Enemy => {
                        println!("You die! The journey is end.");
                        change_stage(Box::new(EndStage {}))
                    }
                };
            }
        }
        None
    }
}

fn wait() {
    sleep(Duration::from_secs(1));
}

fn process_attack(
    deck: &mut Deck,
    attacker: &impl Battle,
    other: &mut impl Battle,
) -> AttackResult {
    println!("{} try to attack {}", attacker.get_name(), other.get_name());
    println!("Drawing cards:");
    let extra_damage = calculate_extra_value(deck);
    println!("{} try to block", other.get_name());
    println!("Drawing cards:");
    let block = calculate_extra_value(deck);
    attacker.attack(other, extra_damage, block)
}

fn calculate_extra_value(deck: &mut Deck) -> u32 {
    let mut extra_value: u32 = 0;
    let mut modify_stack = Vec::new();
    loop {
        wait();
        match deck.draw() {
            Some(card) => {
                println!("{}", card);
                match card {
                    Card::Number(number) => {
                        let mut number = *number;
                        apply_modify_to_number(&mut modify_stack, &mut number, deck);
                        break extra_value += number;
                    }
                    Card::Double | Card::Half | Card::Add => modify_stack.push(card.clone()),
                }
            }
            None => break deck.full_shuffle(),
        }
    }
    extra_value
}

fn apply_modify_to_number(modify_stack: &mut Vec<Card>, number: &mut u32, deck: &mut Deck) {
    loop {
        let modify = modify_stack.pop();
        match modify {
            Some(card) => match card {
                Card::Double => *number *= 2,
                Card::Half => *number /= 2,
                Card::Add => {
                    let added_value = calculate_extra_value(deck);
                    *number += added_value
                }
                _ => {}
            },
            None => break,
        }
    }
}
