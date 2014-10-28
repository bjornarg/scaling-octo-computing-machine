extern crate sqlite3;

use sqlite3::types::*;

use std::collections::HashMap;

pub struct Log {
    pub rssi: int,
    pub tx_power: int,
    pub distance: f64,
    pub mac_address: String,
}

/// Retrieves logs from and SQLite database.
pub fn get_logs(db_file: &str) -> Option<Vec<Log>> {
    let mut logs = Vec::new();
    let db: sqlite3::Database = match sqlite3::open(db_file) {
        Ok(db) => db,
        Err(id) => {
            println!("Could not open database error: {}", id);
            return None;
        }
    };

    let option = None;

    let mut cursor: sqlite3::Cursor = match db.prepare("SELECT rssi, txPower, distance, macAddress FROM logs", &option) {
        Ok(cursor) => cursor,
        Err(id) => {
            println!("Could not create cursor, error: {}", id);
            println!("Error: {}", db.get_errmsg());
            return None;
        }
    };

    loop {
        let step_result = cursor.step();
        match step_result {
            SQLITE_ROW => (),
            _ => break,
        }
        logs.push(Log{
            rssi: cursor.get_int(0),
            tx_power: cursor.get_int(1),
            distance: cursor.get_f64(2),
            mac_address: match cursor.get_text(3) {
                Some(n) => String::from_str(n),
                None => String::new(),
            }
        });
    }

    Some(logs)
}

impl Clone for Log {
    fn clone(&self) -> Log {
        Log{rssi: self.rssi, tx_power: self.tx_power, distance: self.distance, mac_address: self.mac_address.clone()}
    }
}

pub fn split_based_on_distance(logs: &Vec<Log>) -> HashMap<i64, Vec<Log>> {
    let mut distances: HashMap<i64, Vec<Log>> = HashMap::new();
    let mut distance: i64;
    for log in logs.iter() {
        distance = log.distance as i64;
        if distances.contains_key(&distance) {
            distances.get_mut(&distance).push(log.clone());
        } else {
            let mut new_list = Vec::new();
            new_list.push(log.clone());
            distances.insert(distance, new_list);
        }
    }
    distances
}
