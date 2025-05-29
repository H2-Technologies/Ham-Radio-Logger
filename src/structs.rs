pub struct Configuration {
    pub name: String,
    pub callsign: String,
    pub maidenhead: String,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub logbooks: Vec<Logbook>,
}

pub struct Logbook {
    pub callsign: String,
    pub log_name: String,
    pub entries: Vec<LogbookEntry>,
}

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