use crate::game::*;
use std::fmt::Display;

pub struct PlayerData {
    name: Option<String>,
    hit_point: Option<u32>,
    attack_point: Option<u32>,
    defence_point: Option<u32>,
}

impl PlayerData {
    pub fn new() -> PlayerData {
        PlayerData {
            name: None,
            hit_point: None,
            attack_point: None,
            defence_point: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn set_points(&mut self, hp: u32, atk: u32, def: u32) {
        self.hit_point = Some((hp + 1) * 5);
        self.attack_point = Some(atk);
        self.defence_point = Some(def);
    }

    pub fn to_data(&self) -> EntityData {
        IntoEntityData(self).into()
    }
}

impl Display for PlayerData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_data())
    }
}

pub trait Stats {
    fn get_name(&self) -> &String;

    fn get_hit_point(&self) -> u32;

    fn get_current_hit_point(&self) -> u32;

    fn set_current_hit_point(&mut self, value: u32);

    fn get_attack_point(&self) -> u32;

    fn get_defence_point(&self) -> u32;
}

impl Stats for PlayerData {
    fn get_name(&self) -> &String {
        self.name.as_ref().unwrap()
    }

    fn get_hit_point(&self) -> u32 {
        self.hit_point.unwrap_or_default()
    }

    fn get_current_hit_point(&self) -> u32 {
        self.get_hit_point()
    }

    fn set_current_hit_point(&mut self, _value: u32) {}

    fn get_attack_point(&self) -> u32 {
        self.attack_point.unwrap_or_default()
    }

    fn get_defence_point(&self) -> u32 {
        self.defence_point.unwrap_or_default()
    }
}
