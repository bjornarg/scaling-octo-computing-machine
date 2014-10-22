extern crate sqlite3;

use sqlite3::types::*;

pub struct Log {
    pub rssi: int,
    pub txPower: int,
    pub distance: f64,
    pub macAddress: String,
}

pub fn get_logs(db_file: &str) -> SqliteResult<Vec<Log>> {
    let mut logs = Vec::new();
    let db: sqlite3::Database = match sqlite3::open(db_file) {
        Ok(db) => db,
        Err(id) => {
            println!("Could not open database error: {}", id);
            return Err(id);
        }
    };
    println!("Opened database '{}'", db_file);

    let option = None;

    let mut cursor: sqlite3::Cursor = match db.prepare("SELECT rssi, txPower, distance, macAddress FROM logs", &option) {
        Ok(cursor) => cursor,
        Err(id) => {
            println!("Could not create cursor, error: {}", id);
            println!("Error: {}", db.get_errmsg());
            return Err(id);
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
            txPower: cursor.get_int(1),
            distance: cursor.get_f64(2),
            macAddress: match cursor.get_text(3) {
                Some(n) => String::from_str(n),
                None => String::new()
            }
        });
    }

    Ok(logs)
}
