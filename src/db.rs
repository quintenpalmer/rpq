use std::collections::BTreeMap;
use std::fs::File;

use serde::{Deserialize, Serialize};

use super::models;

const DISPLAY_DB_FILE_NAME: &'static str = "db/display.csv";
const MAP_DB_FILE_NAME: &'static str = "db/map.csv";

const ALL_DB_FILE_NAMES: &'static [&'static str] = &[
    DISPLAY_DB_FILE_NAME,
    MAP_DB_FILE_NAME,
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

    pub fn get_displays(&self) -> Result<Vec<models::Display>, String> {
        Ok(self
            .read_db_displays()?
            .into_iter()
            .map(|x| {
                let map = self.get_db_map(x.map_id)?;
                Ok(display_model_from_db(x, map))
            })
            .collect::<Result<Vec<models::Display>, String>>()?)
    }

    pub fn get_display(&self, display_id: u32) -> Result<models::Display, String> {
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
            .fold(1, |acc, display| std::cmp::max(acc, display.id));

        records.push(DBDisplay {
            id: max_id + 1,
            map_id: map.id,
            cursor_x: 0,
            cursor_y: 0,
        });

        self.write_db_displays(records)
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

        self.write_db_maps(records)?;

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

    fn write_db_maps(&self, records: Vec<DBMap>) -> Result<(), String> {
        let mut writer = csv::Writer::from_path(MAP_DB_FILE_NAME).unwrap();

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
        self.write_db_displays(records)
    }

    fn write_db_displays(&self, records: Vec<DBDisplay>) -> Result<(), String> {
        let mut writer = csv::Writer::from_path(DISPLAY_DB_FILE_NAME).unwrap();

        for record in records.into_iter() {
            writer.serialize(record).unwrap();
        }
        match writer.flush() {
            Ok(()) => (),
            Err(e) => return Err(format!("error flushing: {:?}", e)),
        };

        Ok(())
    }
}

fn display_model_from_db(d: DBDisplay, m: DBMap) -> models::Display {
    models::Display {
        id: d.id,
        map: models::Map {
            default_terrain: m.default_terrain,
            specified_terrain: (0..12)
                .into_iter()
                .map(|i| ((9, i), models::Terrain::Dirt))
                .chain(
                    (0..12)
                        .into_iter()
                        .map(|i| ((10, i), models::Terrain::Dirt)),
                )
                .chain(
                    (0..12)
                        .into_iter()
                        .map(|i| ((i, 10), models::Terrain::Dirt)),
                )
                .chain(
                    (0..12)
                        .into_iter()
                        .map(|i| ((11, i), models::Terrain::Rock)),
                )
                .chain(
                    (0..12)
                        .into_iter()
                        .map(|i| ((i, 11), models::Terrain::Rock)),
                )
                .chain(
                    (0..12)
                        .into_iter()
                        .map(|i| ((i, 12), models::Terrain::Dirt)),
                )
                .collect::<BTreeMap<_, _>>(),
            characters: vec![
                ((4, 3), models::Character::Knight),
                ((5, 5), models::Character::Mage),
                ((1, 8), models::Character::Thief),
            ]
            .into_iter()
            .collect::<BTreeMap<_, _>>(),
            hint_max_x: m.hint_max_x,
            hint_max_y: m.hint_max_y,
        },
        current_selection: (d.cursor_x, d.cursor_y),
    }
}
