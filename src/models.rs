use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

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

    pub fn form_strings(&self) -> (&str, &str) {
        match self {
            Direction::Right => ("right", ">"),
            Direction::Up => ("up", "^"),
            Direction::Left => ("left", "<"),
            Direction::Down => ("down", "v"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Terrain {
    Grass,
    Dirt,
    Rock,
}

impl Terrain {
    pub fn all_values() -> Vec<Self> {
        vec![Terrain::Grass, Terrain::Dirt, Terrain::Rock]
    }
}

impl Terrain {
    pub fn url_frag_string(&self) -> String {
        match self {
            Terrain::Grass => "grass",
            Terrain::Dirt => "dirt",
            Terrain::Rock => "rock",
        }
        .into()
    }

    pub fn display_string(&self) -> String {
        match self {
            Terrain::Grass => "Grass",
            Terrain::Dirt => "Dirt",
            Terrain::Rock => "Rock",
        }
        .into()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Character {
    Knight,
    Mage,
    Thief,
}

impl Character {
    pub fn all_values() -> Vec<Self> {
        vec![Character::Knight, Character::Mage, Character::Thief]
    }
}

impl Character {
    pub fn url_frag_string(&self) -> String {
        match self {
            Character::Knight => "knight",
            Character::Mage => "mage",
            Character::Thief => "thief",
        }
        .into()
    }

    pub fn display_string(&self) -> String {
        match self {
            Character::Knight => "Knight",
            Character::Mage => "Mage",
            Character::Thief => "Thief",
        }
        .into()
    }
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

    pub fn at(&self, cursor: &(u32, u32)) -> (Terrain, Option<Character>) {
        (
            self.specified_terrain
                .get(cursor)
                .map(|x| x.clone())
                .unwrap_or(self.default_terrain.clone()),
            self.characters.get(cursor).map(|x| x.clone()),
        )
    }
}

pub struct Display {
    pub id: u32,
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
