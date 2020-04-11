use std::collections::BTreeMap;

#[derive(Debug)]
pub enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "right" => Some(Direction::Right),
            "up" => Some(Direction::Up),
            "left" => Some(Direction::Left),
            "down" => Some(Direction::Down),
            _ => None,
        }
    }
}

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

pub struct Display {
    pub map: Map,
    pub current_selection: (u32, u32),
}

impl Display {
    pub fn move_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::Right => self.current_selection.0 += 1,
            Direction::Up => self.current_selection.1 += 1,
            Direction::Left => self.current_selection.0 -= 1,
            Direction::Down => self.current_selection.1 -= 1,
        }
    }
}
