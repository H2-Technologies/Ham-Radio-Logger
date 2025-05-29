use crate::structs::*;

fn generate_lat_long(maidenhead: &str) -> (f64, f64) {
    // Maidenhead locator to latitude/longitude conversion
    // Supports 4 or 6 character locators
    let maidenhead = maidenhead.to_uppercase();
    let chars: Vec<char> = maidenhead.chars().collect();

    if chars.len() < 4 {
        // Invalid locator, return (0,0)
        return (0.0, 0.0);
    }

    // First pair: Field (A-R), 20x10 degrees
    let lon = (((chars[0] as u8) - b'A') as f64) * 20.0 - 180.0;
    let lat = (((chars[1] as u8) - b'A') as f64) * 10.0 - 90.0;

    // Second pair: Square (0-9), 2x1 degrees
    let lon = lon + (((chars[2] as u8) - b'0') as f64) * 2.0;
    let lat = lat + (((chars[3] as u8) - b'0') as f64) * 1.0;

    // Third pair: Subsquare (A-X), 5x2.5 minutes
    let (lon, lat) = if chars.len() >= 6 {
        let lon = lon + (((chars[4] as u8) - b'A') as f64) * (2.0 / 24.0);
        let lat = lat + (((chars[5] as u8) - b'A') as f64) * (1.0 / 24.0);
        (lon, lat)
    } else {
        (lon, lat)
    };

    // Center of the square
    let lon = lon + (if chars.len() >= 6 { 2.0 / 48.0 } else { 1.0 });
    let lat = lat + (if chars.len() >= 6 { 1.0 / 48.0 } else { 0.5 });

    (lat, lon)
}

impl Config {
    pub fn new(name: String, callsign: String, maidenhead: String, altitude: f64) -> Self {
        let (latitude, longitude) = generate_lat_long(&maidenhead);

        Config {
            name,
            callsign,
            maidenhead,
            latitude,
            longitude,
            altitude,
            logbooks: Vec::new(),
        }
    }
}

impl Logbook {
    pub fn new(callsign: String, log_name: String) -> Self {
        Logbook {
            callsign,
            log_name,
            entries: Vec::new(),
        }
    }
}

impl LogbookEntry {
    pub fn new(
        datetime: String,
        station_callsign: String,
        operator_callsign: String,
        frequency: f64,
        mode: String,
        report_sent: String,
        report_received: String,
        comments: String
    ) -> Self {
        LogbookEntry {
            datetime,
            station_callsign,
            operator_callsign,
            frequency,
            mode,
            report_sent,
            report_received,
            comments,
        }
    }
}

