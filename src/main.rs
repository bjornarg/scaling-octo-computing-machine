#![feature(globs)]

extern crate sqlite3;

mod dna;
mod log;

fn main() {
    let logs = match log::get_logs("database.sqlite") {
        Ok(n) => n,
        Err(id) => return,
    };
    for item in logs.iter() {
        println!("Log ({}, {}, {}, {})", item.rssi, item.txPower, item.distance, item.macAddress);
    }
}
