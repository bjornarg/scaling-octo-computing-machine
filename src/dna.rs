use std::rand::{task_rng, Rng};
use std::fmt;

static MIN_RANGE: f64 = -5f64;
static MAX_RANGE: f64 = 5f64;
static MUTATION_RATE: f64 = 0.1f64;

pub struct DNA {
    dna: [f64, ..4],
}

impl DNA {
    /// Copies itself and potentially performs a mutation.
    /// A mutation will insert a new random value into any part of the DNA.
    pub fn mutate(&self) -> DNA {
        let mut rng = task_rng();
        let mut new_dna = self.copy();
        for i in range(0, self.dna.len()) {
            if rng.gen_range(0f64, 1f64) < MUTATION_RATE {
                if i == 3 {
                    new_dna.dna[i] = rng.gen_range(0f64, MAX_RANGE);
                } else {
                    new_dna.dna[i] = rng.gen_range(MIN_RANGE, MAX_RANGE);
                }
            }
        }
        new_dna
    }

    pub fn copy(&self) -> DNA {
        DNA {dna: self.dna}
    }

    /// Crosses this DNA with `other` DNA, using a random weight to determine
    /// the importance of each parent.
    pub fn crossover(&self, other: DNA) -> DNA {
        let mut rng = task_rng();
        let mut new_dna = DNA::new();
        let weight = rng.gen_range(0f64, 1f64);
        for i in range(0, self.dna.len()) {
            new_dna.dna[i] = weight * self.dna[i] + (1.0-weight) * other.dna[i];
        }
        new_dna
    }

    /// Generates a new random DNA.
    pub fn new() -> DNA {
        let mut rng = task_rng();
        let random_values = [
            rng.gen_range(MIN_RANGE, MAX_RANGE),
            rng.gen_range(MIN_RANGE, MAX_RANGE),
            rng.gen_range(MIN_RANGE, MAX_RANGE),
            rng.gen_range(0f64, MAX_RANGE)];
        DNA {dna: random_values}
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

impl fmt::Show for DNA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DNA (Power: {} - Constant: {} - Factor: {} - Ratio factor: {})", self.dna[0], self.dna[1], self.dna[2], self.dna[3])
    }
}

impl Eq for DNA {
}

impl PartialEq for DNA {
    fn eq(&self, other: &DNA) -> bool {
        self.dna == other.dna
    }
}
