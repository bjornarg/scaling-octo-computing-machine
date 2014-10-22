use std::rand::{task_rng, Rng};

static MIN_RANGE: f64 = -10f64;
static MAX_RANGE: f64 = 10f64;
static MUTATION_RATE: f64 = 0.01f64;

pub struct DNA {
    dna: [f64, ..4],
    fitness: f64,
}

impl DNA {
    pub fn mutate(&self) -> DNA {
        let mut rng = task_rng();
        let mut new_dna = self.copy();
        if rng.gen_range(0f64, 1f64) < MUTATION_RATE {
            let i: uint = rng.gen_range(0, self.dna.len());
            new_dna.dna[i] = rng.gen_range(MIN_RANGE, MAX_RANGE);
        }
        new_dna
    }

    pub fn copy(&self) -> DNA {
        DNA {dna: self.dna, fitness: 0.0}
    }

    pub fn crossover(&self, other: DNA) -> DNA {
        let mut rng = task_rng();
        let mut new_dna = DNA::new();
        let weight = rng.gen_range(0f64, 1f64);
        for i in range(0, self.dna.len()) {
            new_dna.dna[i] = weight * self.dna[i] + (1.0-weight) * other.dna[i];
        }
        new_dna
    }

    pub fn new() -> DNA {
        let mut rng = task_rng();
        let random_values = [
            rng.gen_range(MIN_RANGE, MAX_RANGE),
            rng.gen_range(MIN_RANGE, MAX_RANGE),
            rng.gen_range(MIN_RANGE, MAX_RANGE),
            rng.gen_range(MIN_RANGE, MAX_RANGE)];
        DNA {dna: random_values, fitness: 0.0}
    }

    pub fn get_power(&self) -> f64 {
        self.dna[0]
    }

    pub fn get_constant(&self) -> f64 {
        self.dna[1]
    }

    pub fn get_factor(&self) -> f64 {
        self.dna[2]
    }

    pub fn get_ratio_factor(&self) -> f64 {
        self.dna[3]
    }
}
