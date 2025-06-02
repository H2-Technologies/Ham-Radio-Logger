use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub name: String,
    pub callsign: String,
    pub maidenhead: String,
    pub latitude: u64,
    pub longitude: u64,
    pub altitude: u64,
    pub logbooks: Vec<Logbook>,
}

#[derive(Serialize, Deserialize)]
pub struct Logbook {
    pub callsign: String,
    pub log_name: String,
    pub entries: Vec<LogbookEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct LogbookEntry {
    pub datetime: String,
    pub station_callsign: String,
    pub operator_callsign: String,
    pub frequency: f64,
    pub mode: String,
    pub report_sent: String,
    pub report_received: String,
    pub comments: String,
}