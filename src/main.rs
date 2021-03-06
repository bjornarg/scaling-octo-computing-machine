#![feature(globs)]

extern crate sqlite3;

mod dna;
mod log;
mod genetic;

/// Reads logs from an SQLite database and performs a genetic algorithm to
/// attempt to find sane values for calculating the connection between
/// measured signal strength and distance to an Estimote iBeacon.
fn main() {
    let logs = match log::get_logs("database.sqlite") {
        Some(n) => n,
        None => return,
    };
    let mut population = genetic::generate_population();
    let mut fittest: dna::DNA = population[0];
    for i in range(0i, 1000000i) {
        population = genetic::new_generation(&population, &logs);
        if fittest != population[0] {
            fittest = population[0];
            println!("{{\"power\": {}, \"constant\": {}, \"factor\": {}, \"ratio_factor\": {}, \"generation\": {}}}", fittest.get_power(), fittest.get_constant(), fittest.get_factor(), fittest.get_ratio_factor(), i)
        }
    }
}
