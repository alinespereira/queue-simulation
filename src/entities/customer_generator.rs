use rand::distributions::Distribution;
use statrs::distribution::Poisson;

use super::{Customer, Enqueued, SimulationTime};

pub struct CustomerGenerator {
    dist: Poisson,
}

impl CustomerGenerator {
    pub fn new(lambda: f64) -> Self {
        Self {
            dist: Poisson::new(lambda).unwrap(),
        }
    }

    pub fn generate(&self, arrived_at: SimulationTime) -> Vec<Customer<Enqueued>> {
        let ref mut rng = rand::thread_rng();
        let n: usize = self.dist.sample(rng) as usize;

        (0..n).map(|_| Customer::new(arrived_at)).collect()
    }
}
