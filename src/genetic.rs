use std::rand::{task_rng, Rng};

use dna;
use log;

static POPULATION_SIZE: uint = 20u;

/// Used to keep track of the fitness of a given DNA and sorting them by
/// fitness to aquire a new generation.
struct DNAPair {
    position: uint,
    fitness: f64,
}

impl Eq for DNAPair {
}

impl PartialEq for DNAPair {
    fn eq(&self, other: &DNAPair) -> bool {
        self.fitness == other.fitness
    }
}

impl Ord for DNAPair {
    fn cmp(&self, other: &DNAPair) -> Ordering {
        if self.fitness < other.fitness {
            return Less;
        } else if self.fitness > other.fitness {
            return Greater;
        }
        Equal
    }
}

impl PartialOrd for DNAPair {
    fn partial_cmp(&self, other: &DNAPair) -> Option<Ordering> {
        if self.fitness < other.fitness {
            return Some(Less);
        } else if self.fitness > other.fitness {
            return Some(Greater);
        }
        Some(Equal)
    }
}


/// Calculates the distance a given DNA would give for a given sample.
pub fn calculate_distance(individual: &dna::DNA, log: &log::Log) -> f64 {
    let ratio = individual.get_ratio_factor() * log.rssi as f64 / log.tx_power as f64;
    let ratio_powered = ratio.powf(individual.get_power());
    individual.get_factor() * ratio_powered + individual.get_constant()
}

/// Generates a new random population.
pub fn generate_population() -> Vec<dna::DNA> {
    Vec::from_fn(POPULATION_SIZE, |_| dna::DNA::new())
}


/// Progress a generation to a new generation.
pub fn new_generation(population: &Vec<dna::DNA>, logs: &Vec<log::Log>) -> Vec<dna::DNA> {
    let mut new_population = Vec::new();
    let mut fitness_evaluation = Vec::new();
    for i in range(0, population.len()) {
        fitness_evaluation.push(
            DNAPair{
                position: i,
                fitness: calculate_fitness(&population[i], logs),
            }
        );
    }
    fitness_evaluation.sort();
    normalize_generation(&mut fitness_evaluation);
    new_population.push(population[fitness_evaluation[0].position]);
    for _ in range(0, POPULATION_SIZE-1) {
        let first_parent = population[get_parent(&fitness_evaluation)];
        let second_parent = population[get_parent(&fitness_evaluation)];
        new_population.push(first_parent.crossover(second_parent).mutate());
    }
    new_population
}

/// Get a a parent for a new generation, with the probability of being
/// chosen based on each individuals fitness.
fn get_parent(fitness_evaluation: &Vec<DNAPair>) -> uint {
    let mut rng = task_rng();
    let mut parent_chance: f64 = rng.gen_range(0.0, 1.0);
    for pair in fitness_evaluation.iter() {
        if parent_chance <= pair.fitness {
            return pair.position;
        } else {
            parent_chance -= pair.fitness;
        }
    }
    POPULATION_SIZE - 1
}

/// Calculate the fitness of an individual based on the total difference
/// between calculated distance and actual distance.
/// The lower the fitness, the better.
pub fn calculate_fitness(individual: &dna::DNA, logs: &Vec<log::Log>) -> f64 {
    let mut fitness = 0f64;
    for log_item in logs.iter() {
        let distance = calculate_distance(individual, log_item);
        let difference = (distance - log_item.distance).abs();
        fitness += difference;
    }
    fitness
}

/// Normalize a generation's fitness and invert it, giving a higher
/// fitness to individuals with lower total difference in distance
/// calculations.
fn normalize_generation(fitness_evaluation: &mut Vec<DNAPair>) {
    if fitness_evaluation.len() == 0 {
        return;
    }
    let mut total_fitness = 0f64;
    for pair in fitness_evaluation.iter() {
        total_fitness += pair.fitness;
    }
    for pair in fitness_evaluation.iter_mut() {
        pair.fitness = (total_fitness-pair.fitness)/total_fitness/(POPULATION_SIZE as f64-1f64);
    }
}
