use std::collections::BTreeMap;
use std::fs::File;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use super::models;

const GAME_DB_FILE_NAME: &'static str = "db/game.csv";
const MAP_DB_FILE_NAME: &'static str = "db/map.csv";
const TILES_DB_FILE_NAME: &'static str = "db/tiles.csv";
const CHARACTER_DB_FILE_NAME: &'static str = "db/characters.csv";

const ALL_DB_FILE_NAMES: &'static [&'static str] = &[
    GAME_DB_FILE_NAME,
    MAP_DB_FILE_NAME,
    TILES_DB_FILE_NAME,
    CHARACTER_DB_FILE_NAME,
];

#[derive(Debug)]
pub enum DBError {
    FindingTable(String),
    ParsingRecord(String),
    FindingRecord(String),
    Internal(std::io::Error),
}

#[derive(Serialize, Deserialize, Clone)]
struct DBGame {
    id: u32,
    map_id: u32,
    cursor_x: u32,
    cursor_y: u32,
}

#[derive(Serialize, Deserialize, Clone)]
struct DBMap {
    id: u32,
    default_terrain: models::Terrain,
    hint_max_x: u32,
    hint_max_y: u32,
}

#[derive(Serialize, Deserialize, Clone)]
struct DBTileLine {
    id: u32,
    map_id: u32,
    terrain: models::Terrain,
    x: u32,
    y: u32,
}

#[derive(Serialize, Deserialize, Clone)]
struct DBCharacter {
    id: u32,
    game_id: u32,
    character: models::Character,
    x: u32,
    y: u32,
}

pub struct DB {
    _nothing: (),
}

impl DB {
    pub fn new() -> Self {
        for file_name in ALL_DB_FILE_NAMES.iter() {
            match File::open(file_name) {
                Ok(_f) => {}
                Err(_e) => {
                    let mut writer = csv::Writer::from_path(file_name).unwrap();
                    writer.flush().unwrap();
                }
            }
        }
        DB { _nothing: () }
    }

    pub fn get_games(&self) -> Result<Vec<models::Game>, DBError> {
        Ok(self
            .read_db_games()?
            .into_iter()
            .map(|x| {
                let map = self.get_db_map(x.map_id)?;
                let tiles = self.read_db_tile_lines_for_map_id(map.id)?;
                let characters = self.read_db_characters_for_game_id(x.id)?;
                Ok(game_model_from_db(x, map, tiles, characters))
            })
            .collect::<Result<Vec<models::Game>, DBError>>()?)
    }

    pub fn get_game(&self, game_id: u32) -> Result<models::Game, DBError> {
        get_single_result(self.get_games()?, |game| game.id == game_id, "games")
    }

    fn read_db_games(&self) -> Result<Vec<DBGame>, DBError> {
        self.read_db_records(GAME_DB_FILE_NAME)
    }

    pub fn get_maps(&self) -> Result<Vec<models::Map>, DBError> {
        Ok(self
            .read_db_maps()?
            .into_iter()
            .map(|map| {
                let tiles = self.read_db_tile_lines_for_map_id(map.id)?;
                Ok(map_model_from_db(map, tiles))
            })
            .collect::<Result<Vec<models::Map>, DBError>>()?)
    }

    pub fn get_map(&self, map_id: u32) -> Result<models::Map, DBError> {
        get_single_result(self.get_maps()?, |record| record.id == map_id, "maps")
    }

    pub fn add_game(&self) -> Result<(), DBError> {
        let map = self.add_db_map()?;

        let mut records = self.read_db_games()?;
        let max_id = records
            .iter()
            .fold(0, |acc, game| std::cmp::max(acc, game.id));

        records.push(DBGame {
            id: max_id + 1,
            map_id: map.id,
            cursor_x: 0,
            cursor_y: 0,
        });

        self.write_replace_records(GAME_DB_FILE_NAME, records)
    }

    fn add_db_map(&self) -> Result<DBMap, DBError> {
        let mut records = self.read_db_maps()?;
        let max_id = records
            .iter()
            .fold(0, |acc, record| std::cmp::max(acc, record.id));

        let new_record = DBMap {
            id: max_id + 1,
            default_terrain: models::Terrain::Grass,
            hint_max_x: 15,
            hint_max_y: 12,
        };

        records.push(new_record.clone());

        self.write_replace_records(MAP_DB_FILE_NAME, records)?;

        Ok(new_record)
    }

    fn get_db_map(&self, map_id: u32) -> Result<DBMap, DBError> {
        get_single_result(self.read_db_maps()?, |record| record.id == map_id, "maps")
    }

    fn read_db_maps(&self) -> Result<Vec<DBMap>, DBError> {
        self.read_db_records(MAP_DB_FILE_NAME)
    }

    fn write_replace_records<S: Serialize>(
        &self,
        db_file_name: &'static str,
        records: Vec<S>,
    ) -> Result<(), DBError> {
        let mut writer = csv::Writer::from_path(db_file_name).unwrap();

        for record in records.into_iter() {
            writer.serialize(record).unwrap();
        }
        match writer.flush() {
            Ok(()) => (),
            Err(e) => return Err(DBError::Internal(e)),
        };

        Ok(())
    }

    fn read_db_records<S: DeserializeOwned>(
        &self,
        db_file_name: &'static str,
    ) -> Result<Vec<S>, DBError> {
        let mut rdr =
            csv::Reader::from_reader(File::open(db_file_name).map_err(|e| {
                DBError::FindingTable(format!("could not read from file: {:?}", e))
            })?);
        let records = rdr
            .deserialize()
            .into_iter()
            .map(|result| -> Result<S, DBError> {
                result.map_err(|e| DBError::ParsingRecord(format!("could not read row: {:?}", e)))
            })
            .collect::<Result<Vec<S>, DBError>>()?;
        Ok(records)
    }

    pub fn update_game_cursor(&self, id: u32, cursor: (u32, u32)) -> Result<(), DBError> {
        let records = self
            .read_db_games()?
            .into_iter()
            .map(|mut record| {
                if record.id == id {
                    record.cursor_x = cursor.0;
                    record.cursor_y = cursor.1;
                    record
                } else {
                    record
                }
            })
            .collect();
        self.write_replace_records(GAME_DB_FILE_NAME, records)
    }

    fn read_db_tile_lines_for_map_id(&self, map_id: u32) -> Result<Vec<DBTileLine>, DBError> {
        Ok(self
            .read_db_tile_lines()?
            .into_iter()
            .filter(|record| record.map_id == map_id)
            .collect())
    }

    fn read_db_tile_lines(&self) -> Result<Vec<DBTileLine>, DBError> {
        self.read_db_records(TILES_DB_FILE_NAME)
    }

    fn read_db_characters_for_game_id(&self, game_id: u32) -> Result<Vec<DBCharacter>, DBError> {
        Ok(self
            .read_db_characters()?
            .into_iter()
            .filter(|record| record.game_id == game_id)
            .collect())
    }

    fn read_db_characters(&self) -> Result<Vec<DBCharacter>, DBError> {
        self.read_db_records(CHARACTER_DB_FILE_NAME)
    }

    pub fn update_game_terrain(
        &self,
        game_id: u32,
        terrain: models::Terrain,
    ) -> Result<(), DBError> {
        let mut records = self.read_db_tile_lines()?;
        let max_id = records
            .iter()
            .fold(0, |acc, record| std::cmp::max(acc, record.id));

        let game = self.get_game(game_id)?;

        let new_record = DBTileLine {
            id: max_id + 1,
            map_id: game.map.id,
            terrain: terrain,
            x: game.current_selection.0,
            y: game.current_selection.1,
        };

        records.push(new_record.clone());

        records = records
            .into_iter()
            .map(|record| {
                (
                    (record.map_id, record.x, record.y),
                    (record.id, record.terrain),
                )
            })
            .collect::<BTreeMap<_, _>>()
            .into_iter()
            .map(|(key, value)| DBTileLine {
                id: value.0,
                map_id: key.0,
                terrain: value.1,
                x: key.1,
                y: key.2,
            })
            .collect::<Vec<DBTileLine>>();

        self.write_replace_records(TILES_DB_FILE_NAME, records)?;

        Ok(())
    }

    pub fn update_game_character(
        &self,
        game_id: u32,
        character: models::Character,
    ) -> Result<(), DBError> {
        let mut records = self.read_db_characters()?;
        let max_id = records
            .iter()
            .fold(0, |acc, record| std::cmp::max(acc, record.id));

        let game = self.get_game(game_id)?;

        let new_record = DBCharacter {
            id: max_id + 1,
            game_id: game.map.id,
            character: character,
            x: game.current_selection.0,
            y: game.current_selection.1,
        };

        records.push(new_record.clone());

        records = records
            .into_iter()
            .map(|record| {
                (
                    (record.game_id, record.x, record.y),
                    (record.id, record.character),
                )
            })
            .collect::<BTreeMap<_, _>>()
            .into_iter()
            .map(|(key, value)| DBCharacter {
                id: value.0,
                game_id: key.0,
                character: value.1,
                x: key.1,
                y: key.2,
            })
            .collect::<Vec<DBCharacter>>();

        self.write_replace_records(CHARACTER_DB_FILE_NAME, records)?;

        Ok(())
    }

    pub fn unset_game_terrain(&self, game_id: u32) -> Result<(), DBError> {
        let mut records = self.read_db_tile_lines()?;

        let game = self.get_game(game_id)?;

        records = records
            .into_iter()
            .filter(|record| {
                !((record.map_id == game.map.id)
                    && (record.x == game.current_selection.0)
                    && (record.y == game.current_selection.1))
            })
            .collect();

        self.write_replace_records(TILES_DB_FILE_NAME, records)?;

        Ok(())
    }

    pub fn unset_game_character(&self, game_id: u32) -> Result<(), DBError> {
        let mut records = self.read_db_characters()?;

        let game = self.get_game(game_id)?;

        records = records
            .into_iter()
            .filter(|record| {
                !((record.game_id == game.id)
                    && (record.x == game.current_selection.0)
                    && (record.y == game.current_selection.1))
            })
            .collect();

        self.write_replace_records(CHARACTER_DB_FILE_NAME, records)?;

        Ok(())
    }
}

fn game_model_from_db(
    g: DBGame,
    m: DBMap,
    tiles: Vec<DBTileLine>,
    characters: Vec<DBCharacter>,
) -> models::Game {
    models::Game {
        id: g.id,
        map: map_model_from_db(m, tiles),
        characters: characters
            .into_iter()
            .map(|character| ((character.x, character.y), character.character))
            .collect::<BTreeMap<_, _>>(),
        current_selection: (g.cursor_x, g.cursor_y),
    }
}

fn map_model_from_db(m: DBMap, tiles: Vec<DBTileLine>) -> models::Map {
    models::Map {
        id: m.id,
        default_terrain: m.default_terrain,
        specified_terrain: tiles
            .into_iter()
            .map(|tile| ((tile.x, tile.y), tile.terrain))
            .collect::<BTreeMap<_, _>>(),
        hint_max_x: m.hint_max_x,
        hint_max_y: m.hint_max_y,
    }
}

pub fn get_single_result<T, F: Fn(&T) -> bool>(
    results: Vec<T>,
    cmp: F,
    name: &'static str,
) -> Result<T, DBError> {
    for result in results.into_iter() {
        if cmp(&result) {
            return Ok(result);
        }
    }
    return Err(DBError::FindingRecord(name.into()));
}
