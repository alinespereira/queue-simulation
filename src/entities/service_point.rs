use rand::prelude::Distribution;
use statrs::distribution::{Bernoulli, ContinuousCDF, Exp};
// use uuid::Uuid;

use super::{Customer, Served, SimulationTime};

#[derive(Debug, PartialEq)]
pub enum ServicePointStatus {
    Idle,
    Serving,
}

#[derive(Debug)]
pub struct ServicePoint {
    // id: Uuid,
    status: ServicePointStatus,
    service_time_dist: Exp,
    served_customer: Option<Customer<Served>>,
}

impl ServicePoint {
    pub fn new(service_rate: f64) -> Self {
        let service_time_dist = Exp::new(service_rate).unwrap();

        Self {
            // id: Uuid::new_v4(),
            status: ServicePointStatus::Idle,
            service_time_dist,
            served_customer: None,
        }
    }

    pub fn is_available(&self) -> bool {
        self.status == ServicePointStatus::Idle
    }

    pub fn start_service(&mut self, customer: Customer<Served>) {
        self.served_customer = Some(customer);
        self.status = ServicePointStatus::Serving;
    }

    fn should_service_end(&self, p: f64) -> bool {
        let bernoulli = Bernoulli::new(p).unwrap();
        bernoulli.sample(&mut rand::thread_rng()) == 1.0
    }

    fn service_end_probability(&self, elapsed_time: f64) -> f64 {
        self.service_time_dist.cdf(elapsed_time)
    }

    pub fn has_service_ended(&self, current_time: SimulationTime) -> bool {
        if let Some(customer) = &self.served_customer {
            let elapsed_time = customer.elapsed_time_at(current_time);
            let p = self.service_end_probability(elapsed_time as f64);
            self.should_service_end(p)
        } else {
            true
        }
    }

    pub fn finish_service(&mut self, ended_at: SimulationTime) -> Option<Customer<Served>> {
        match self.served_customer.clone() {
            Some(mut customer) => {
                customer.finish_service(ended_at);
                self.served_customer = None;
                self.status = ServicePointStatus::Idle;
                Some(customer)
            }
            None => None,
        }
    }
}
