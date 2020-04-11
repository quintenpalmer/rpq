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
        self.characters.keys().fold(
            self.specified_terrain.keys().fold(
                (self.hint_max_x, self.hint_max_y),
                |(acc_x, acc_y), (x, y)| (std::cmp::max(acc_x, *x), std::cmp::max(acc_y, *y)),
            ),
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
        let (max_x, max_y) = self.map.maxes();
        match direction {
            Direction::Right => {
                if self.current_selection.0 <= max_x - 2 {
                    self.current_selection.0 += 1
                }
            }
            Direction::Up => {
                if self.current_selection.1 <= max_y - 2 {
                    self.current_selection.1 += 1
                }
            }
            Direction::Left => {
                if self.current_selection.0 > 0 {
                    self.current_selection.0 -= 1
                }
            }
            Direction::Down => {
                if self.current_selection.1 > 0 {
                    self.current_selection.1 -= 1
                }
            }
        }
    }
}
