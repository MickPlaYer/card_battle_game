use crate::game::*;
use std::fmt::Display;

#[derive(Clone)]
pub struct EntityData {
    name: String,
    hit_point: u32,
    current_hit_point: u32,
    attack_point: u32,
    defence_point: u32,
}

impl EntityData {
    pub fn slime() -> Self {
        Self {
            name: String::from("Slime"),
            hit_point: 10,
            current_hit_point: 10,
            attack_point: 2,
            defence_point: 2,
        }
    }
}

impl Display for EntityData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}\nHP: {}/{}\nATK: {}\nDEF: {}",
            self.name,
            self.current_hit_point,
            self.hit_point,
            self.attack_point,
            self.defence_point
        )
    }
}

pub struct IntoEntityData<'a, T: Stats>(pub &'a T);
impl<T: Stats> Into<EntityData> for IntoEntityData<'_, T> {
    fn into(self) -> EntityData {
        let stats = self.0;
        let hit_point = stats.get_hit_point();
        EntityData {
            name: stats.get_name().clone(),
            hit_point,
            current_hit_point: hit_point,
            attack_point: stats.get_attack_point(),
            defence_point: stats.get_defence_point(),
        }
    }
}

impl Stats for EntityData {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_hit_point(&self) -> u32 {
        self.hit_point
    }

    fn get_current_hit_point(&self) -> u32 {
        self.current_hit_point
    }

    fn set_current_hit_point(&mut self, value: u32) {
        self.current_hit_point = value
    }

    fn get_attack_point(&self) -> u32 {
        self.attack_point
    }

    fn get_defence_point(&self) -> u32 {
        self.defence_point
    }
}
