use std::collections::BTreeMap;

#[derive(Clone)]
pub enum Terrain {
    Grass,
    Dirt,
    Rock,
}

#[derive(Clone)]
pub enum Character {
    Knight,
    Mage,
    Thief,
}

pub struct Map {
    pub default_terrain: Terrain,
    pub specified_terrain: BTreeMap<(u32, u32), Terrain>,
    pub characters: BTreeMap<(u32, u32), Character>,
    pub hint_max_x: u32,
    pub hint_max_y: u32,
}

impl Map {
    pub fn maxes(&self) -> (u32, u32) {
        self.specified_terrain.keys().fold(
            (self.hint_max_x, self.hint_max_y),
            |(acc_x, acc_y), (x, y)| (std::cmp::max(acc_x, *x), std::cmp::max(acc_y, *y)),
        )
    }
}
