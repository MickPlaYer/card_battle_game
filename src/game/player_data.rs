use std::fmt::Display;

pub struct RawPlayerData {
    name: Option<String>,
    hit_point: Option<u32>,
    attack_point: Option<u32>,
    defence_point: Option<u32>,
}

impl RawPlayerData {
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn set_points(&mut self, hp: u32, atk: u32, def: u32) {
        self.hit_point = Some((hp + 1) * 5);
        self.attack_point = Some(atk);
        self.defence_point = Some(def);
    }

    pub fn to_player_data(&self) -> PlayerData {
        PlayerData {
            name: self.name.clone().unwrap_or(String::from("")),
            hit_point: self.hit_point.unwrap_or_default(),
            attack_point: self.attack_point.unwrap_or_default(),
            defence_point: self.defence_point.unwrap_or_default(),
        }
    }
}

impl Display for RawPlayerData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_player_data())
    }
}

pub struct PlayerData {
    name: String,
    hit_point: u32,
    attack_point: u32,
    defence_point: u32,
}

impl PlayerData {
    pub fn new() -> RawPlayerData {
        RawPlayerData {
            name: None,
            hit_point: None,
            attack_point: None,
            defence_point: None,
        }
    }
}

impl Display for PlayerData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}\nHP: {}\nATK: {}\nDEF: {}",
            self.name, self.hit_point, self.attack_point, self.defence_point
        )
    }
}
