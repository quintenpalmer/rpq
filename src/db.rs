use std::collections::BTreeMap;
use std::fs::File;

use serde::{Deserialize, Serialize};

use super::models;

const DISPLAY_DB_FILE_NAME: &'static str = "db/display.csv";
const MAP_DB_FILE_NAME: &'static str = "db/map.csv";
const TILES_DB_FILE_NAME: &'static str = "db/tiles.csv";
const CHARACTER_DB_FILE_NAME: &'static str = "db/characters.csv";

const ALL_DB_FILE_NAMES: &'static [&'static str] = &[
    DISPLAY_DB_FILE_NAME,
    MAP_DB_FILE_NAME,
    TILES_DB_FILE_NAME,
    CHARACTER_DB_FILE_NAME,
];

#[derive(Serialize, Deserialize, Clone)]
struct DBDisplay {
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
    map_id: u32,
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

    pub fn get_displays(&self) -> Result<Vec<models::Game>, String> {
        Ok(self
            .read_db_displays()?
            .into_iter()
            .map(|x| {
                let map = self.get_db_map(x.map_id)?;
                let tiles = self.read_db_tile_lines_for_map_id(map.id)?;
                let characters = self.read_db_characters_for_map_id(map.id)?;
                Ok(display_model_from_db(x, map, tiles, characters))
            })
            .collect::<Result<Vec<models::Game>, String>>()?)
    }

    pub fn get_display(&self, display_id: u32) -> Result<models::Game, String> {
        for display in self.get_displays()?.into_iter() {
            if display.id == display_id {
                return Ok(display);
            }
        }
        return Err("could not find display with supplied id".into());
    }

    fn read_db_displays(&self) -> Result<Vec<DBDisplay>, String> {
        let mut rdr = csv::Reader::from_reader(
            File::open(DISPLAY_DB_FILE_NAME)
                .map_err(|e| format!("could not read from file: {:?}", e))?,
        );
        let records = rdr
            .deserialize()
            .into_iter()
            .map(|result| -> Result<DBDisplay, String> {
                result.map_err(|e| format!("could not read row for display: {:?}", e))
            })
            .collect::<Result<Vec<DBDisplay>, String>>()?;
        Ok(records)
    }

    pub fn add_display(&self) -> Result<(), String> {
        let map = self.add_db_map()?;

        let mut records = self.read_db_displays()?;
        let max_id = records
            .iter()
            .fold(0, |acc, display| std::cmp::max(acc, display.id));

        records.push(DBDisplay {
            id: max_id + 1,
            map_id: map.id,
            cursor_x: 0,
            cursor_y: 0,
        });

        self.write_replace_records(DISPLAY_DB_FILE_NAME, records)
    }

    fn add_db_map(&self) -> Result<DBMap, String> {
        let mut records = self.read_db_maps()?;
        let max_id = records
            .iter()
            .fold(0, |acc, record| std::cmp::max(acc, record.id));

        let new_record = DBMap {
            id: max_id + 1,
            default_terrain: models::Terrain::Grass,
            hint_max_x: 25,
            hint_max_y: 15,
        };

        records.push(new_record.clone());

        self.write_replace_records(MAP_DB_FILE_NAME, records)?;

        Ok(new_record)
    }

    fn get_db_map(&self, map_id: u32) -> Result<DBMap, String> {
        for record in self.read_db_maps()?.into_iter() {
            if record.id == map_id {
                return Ok(record);
            }
        }
        return Err("could not find record with supplied id".into());
    }

    fn read_db_maps(&self) -> Result<Vec<DBMap>, String> {
        let mut rdr = csv::Reader::from_reader(
            File::open(MAP_DB_FILE_NAME)
                .map_err(|e| format!("could not read from file: {:?}", e))?,
        );
        let records = rdr
            .deserialize()
            .into_iter()
            .map(|result| -> Result<DBMap, String> {
                result.map_err(|e| format!("could not read row for display: {:?}", e))
            })
            .collect::<Result<Vec<DBMap>, String>>()?;
        Ok(records)
    }

    fn write_replace_records<S: Serialize>(
        &self,
        db_file_name: &'static str,
        records: Vec<S>,
    ) -> Result<(), String> {
        let mut writer = csv::Writer::from_path(db_file_name).unwrap();

        for record in records.into_iter() {
            writer.serialize(record).unwrap();
        }
        match writer.flush() {
            Ok(()) => (),
            Err(e) => return Err(format!("error flushing: {:?}", e)),
        };

        Ok(())
    }

    pub fn update_display_cursor(&self, id: u32, cursor: (u32, u32)) -> Result<(), String> {
        let records = self
            .read_db_displays()?
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
        self.write_replace_records(DISPLAY_DB_FILE_NAME, records)
    }

    fn read_db_tile_lines_for_map_id(&self, map_id: u32) -> Result<Vec<DBTileLine>, String> {
        Ok(self
            .read_db_tile_lines()?
            .into_iter()
            .filter(|record| record.map_id == map_id)
            .collect())
    }

    fn read_db_tile_lines(&self) -> Result<Vec<DBTileLine>, String> {
        let mut rdr = csv::Reader::from_reader(
            File::open(TILES_DB_FILE_NAME)
                .map_err(|e| format!("could not read from file: {:?}", e))?,
        );
        let records = rdr
            .deserialize()
            .into_iter()
            .map(|result| -> Result<DBTileLine, String> {
                result.map_err(|e| format!("could not read row for record: {:?}", e))
            })
            .collect::<Result<Vec<DBTileLine>, String>>()?;
        Ok(records)
    }

    fn read_db_characters_for_map_id(&self, map_id: u32) -> Result<Vec<DBCharacter>, String> {
        Ok(self
            .read_db_characters()?
            .into_iter()
            .filter(|record| record.map_id == map_id)
            .collect())
    }

    fn read_db_characters(&self) -> Result<Vec<DBCharacter>, String> {
        let mut rdr = csv::Reader::from_reader(
            File::open(CHARACTER_DB_FILE_NAME)
                .map_err(|e| format!("could not read from file: {:?}", e))?,
        );
        let records = rdr
            .deserialize()
            .into_iter()
            .map(|result| -> Result<DBCharacter, String> {
                result.map_err(|e| format!("could not read row for record: {:?}", e))
            })
            .collect::<Result<Vec<DBCharacter>, String>>()?;
        Ok(records)
    }

    pub fn update_display_terrain(
        &self,
        display_id: u32,
        terrain: models::Terrain,
    ) -> Result<(), String> {
        let mut records = self.read_db_tile_lines()?;
        let max_id = records
            .iter()
            .fold(0, |acc, record| std::cmp::max(acc, record.id));

        let display = self.get_display(display_id)?;

        let new_record = DBTileLine {
            id: max_id + 1,
            map_id: display.map.id,
            terrain: terrain,
            x: display.current_selection.0,
            y: display.current_selection.1,
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

    pub fn update_display_character(
        &self,
        display_id: u32,
        character: models::Character,
    ) -> Result<(), String> {
        let mut records = self.read_db_characters()?;
        let max_id = records
            .iter()
            .fold(0, |acc, record| std::cmp::max(acc, record.id));

        let display = self.get_display(display_id)?;

        let new_record = DBCharacter {
            id: max_id + 1,
            map_id: display.map.id,
            character: character,
            x: display.current_selection.0,
            y: display.current_selection.1,
        };

        records.push(new_record.clone());

        records = records
            .into_iter()
            .map(|record| {
                (
                    (record.map_id, record.x, record.y),
                    (record.id, record.character),
                )
            })
            .collect::<BTreeMap<_, _>>()
            .into_iter()
            .map(|(key, value)| DBCharacter {
                id: value.0,
                map_id: key.0,
                character: value.1,
                x: key.1,
                y: key.2,
            })
            .collect::<Vec<DBCharacter>>();

        self.write_replace_records(CHARACTER_DB_FILE_NAME, records)?;

        Ok(())
    }

    pub fn unset_display_terrain(&self, display_id: u32) -> Result<(), String> {
        let mut records = self.read_db_tile_lines()?;

        let display = self.get_display(display_id)?;

        records = records
            .into_iter()
            .filter(|record| {
                !((record.map_id == display.map.id)
                    && (record.x == display.current_selection.0)
                    && (record.y == display.current_selection.1))
            })
            .collect();

        self.write_replace_records(TILES_DB_FILE_NAME, records)?;

        Ok(())
    }

    pub fn unset_display_character(&self, display_id: u32) -> Result<(), String> {
        let mut records = self.read_db_characters()?;

        let display = self.get_display(display_id)?;

        records = records
            .into_iter()
            .filter(|record| {
                !((record.map_id == display.map.id)
                    && (record.x == display.current_selection.0)
                    && (record.y == display.current_selection.1))
            })
            .collect();

        self.write_replace_records(CHARACTER_DB_FILE_NAME, records)?;

        Ok(())
    }
}

fn display_model_from_db(
    d: DBDisplay,
    m: DBMap,
    tiles: Vec<DBTileLine>,
    characters: Vec<DBCharacter>,
) -> models::Game {
    models::Game {
        id: d.id,
        map: models::Map {
            id: m.id,
            default_terrain: m.default_terrain,
            specified_terrain: tiles
                .into_iter()
                .map(|tile| ((tile.x, tile.y), tile.terrain))
                .collect::<BTreeMap<_, _>>(),
            characters: characters
                .into_iter()
                .map(|character| ((character.x, character.y), character.character))
                .collect::<BTreeMap<_, _>>(),
            hint_max_x: m.hint_max_x,
            hint_max_y: m.hint_max_y,
        },
        current_selection: (d.cursor_x, d.cursor_y),
    }
}
