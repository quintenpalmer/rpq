#[derive(Debug)]
pub enum DBError {
    FindingTable(String),
    ParsingRecord(String),
    FindingRecord(String),
    Internal(std::io::Error),
}
