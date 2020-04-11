use std::collections::BTreeMap;
use std::fs::File;

use serde::{Deserialize, Serialize};

use super::models;

const DISPLAY_DB_FILE_NAME: &'static str = "db/display.csv";

const ALL_DB_FILE_NAMES: &'static [&'static str] = &[
    DISPLAY_DB_FILE_NAME,
];

#[derive(Serialize, Deserialize)]
struct DBDisplay {
    id: u32,
    cursor_x: u32,
    cursor_y: u32,
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

    pub fn read_display(&self) -> Result<models::Display, String> {
        let mut rdr = csv::Reader::from_reader(
            File::open(DISPLAY_DB_FILE_NAME)
                .map_err(|e| format!("could not read from file: {:?}", e))?,
        );
        let mut o_cursor = None;
        for result in rdr.deserialize() {
            if o_cursor.is_none() {
                let record: DBDisplay =
                    result.map_err(|e| format!("could not read row for display: {:?}", e))?;
                o_cursor = Some(record);
            }
        }
        let cursor = match o_cursor {
            Some(v) => v,
            None => return Err("could not find a display db entry".to_string()),
        };
        Ok(display_model_from_db(cursor))
    }

    pub fn get_display(&self, display_id: u32) -> Result<models::Display, String> {
        let mut rdr = csv::Reader::from_reader(
            File::open(DISPLAY_DB_FILE_NAME)
                .map_err(|e| format!("could not read from file: {:?}", e))?,
        );
        for result in rdr.deserialize() {
            let record: DBDisplay = result.unwrap();
            if record.id == display_id {
                return Ok(display_model_from_db(record));
            }
        }
        return Err("could not find character with supplied id".into());
    }

    pub fn write_display(&self, display: &models::Display) -> Result<(), String> {
        let records = vec![DBDisplay {
            id: display.id,
            cursor_x: display.current_selection.0,
            cursor_y: display.current_selection.1,
        }];

        let mut writer = csv::Writer::from_path(DISPLAY_DB_FILE_NAME).unwrap();

        for record in records.into_iter() {
            writer.serialize(record).unwrap();
        }
        writer.flush().unwrap();

        Ok(())
    }
}

fn display_model_from_db(d: DBDisplay) -> models::Display {
    models::Display {
        id: 1,
        map: models::Map {
            default_terrain: models::Terrain::Grass,
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
            hint_max_x: 12,
            hint_max_y: 15,
        },
        current_selection: (d.cursor_x, d.cursor_y),
    }
}
