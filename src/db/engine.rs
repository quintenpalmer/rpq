use std::fs::File;

use serde::de::DeserializeOwned;
use serde::Serialize;

use super::common::DBError;

pub struct Engine {
    _nothing: (),
}

impl Engine {
    pub fn new(table_names: &'static [&'static str]) -> Self {
        for file_name in table_names.iter() {
            match File::open(file_name) {
                Ok(_f) => {}
                Err(_e) => {
                    let mut writer = csv::Writer::from_path(file_name).unwrap();
                    writer.flush().unwrap();
                }
            }
        }
        Engine { _nothing: () }
    }

    pub fn write_replace_records<S: Serialize>(
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

    pub fn read_db_records<S: DeserializeOwned>(
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
}
