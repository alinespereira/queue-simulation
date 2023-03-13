use std::fmt;

use uuid::Uuid;

pub type SimulationTime = u32;

#[derive(Debug, Clone)]
pub struct Enqueued;

#[derive(Debug, Clone)]
pub struct Declined;

#[derive(Debug, Clone)]
pub struct Reneged {
    reneged_at: SimulationTime,
}

#[derive(Debug, Clone)]
pub struct Served {
    pub started_at: SimulationTime,
    pub ended_at: Option<SimulationTime>,
}

#[derive(Debug, Clone)]
pub struct Customer<Status: Sized> {
    id: Uuid,
    pub arrived_at: SimulationTime,
    pub service_status: Status,
}

impl fmt::Display for Customer<Enqueued> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut customer_str = String::from("Customer:\n");
        customer_str.push_str(format!("        id: {}\n", self.id).as_str());
        customer_str.push_str(format!("arrived at: {}\n", self.arrived_at).as_str());
        customer_str.push_str(format!("    status: {}", "enqueued").as_str());

        writeln!(f, "{customer_str}")
    }
}

impl fmt::Display for Customer<Reneged> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut customer_str = String::from("Customer:\n");
        customer_str.push_str(format!("        id: {}\n", self.id).as_str());
        customer_str.push_str(format!("arrived at: {}\n", self.arrived_at).as_str());
        customer_str.push_str(
            format!("    status: reneged after waiting {}", self.reneging_time()).as_str(),
        );

        writeln!(f, "{customer_str}")
    }
}

impl fmt::Display for Customer<Served> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut customer_str = String::from("Customer:\n");
        customer_str.push_str(format!("        id: {}\n", self.id).as_str());
        customer_str.push_str(format!("arrived at: {}\n", self.arrived_at).as_str());
        customer_str.push_str(
            format!(
                "    status: served in {} after waiting {}",
                self.service_time().unwrap(),
                self.waiting_time()
            )
            .as_str(),
        );

        writeln!(f, "{customer_str}")
    }
}

impl<Status> Customer<Status> {
    pub fn elapsed_time_at(&self, current_time: SimulationTime) -> SimulationTime {
        current_time - self.arrived_at
    }
}

impl Customer<Enqueued> {
    pub fn new(arrived_at: SimulationTime) -> Self {
        Self {
            id: Uuid::new_v4(),
            arrived_at,
            service_status: Enqueued,
        }
    }

    pub fn renege(self, reneged_at: SimulationTime) -> Customer<Reneged> {
        let Customer { id, arrived_at, .. } = self;
        let service_status = Reneged { reneged_at };

        Customer {
            id,
            arrived_at,
            service_status,
        }
    }

    pub fn decline(self) -> Customer<Declined> {
        let Customer { id, arrived_at, .. } = self;

        Customer {
            id,
            arrived_at,
            service_status: Declined,
        }
    }

    pub fn start_service(self, started_at: SimulationTime) -> Customer<Served> {
        let Customer { id, arrived_at, .. } = self;
        let service_status = Served {
            started_at,
            ended_at: None,
        };

        Customer {
            id,
            arrived_at,
            service_status,
        }
    }
}

impl Customer<Served> {
    pub fn finish_service(&mut self, ended_at: SimulationTime) {
        self.service_status.ended_at = Some(ended_at)
    }

    pub fn service_time(&self) -> Option<SimulationTime> {
        let Customer {
            service_status:
                Served {
                    started_at,
                    ended_at,
                },
            ..
        } = self;
        ended_at.map(|ended_at| ended_at - started_at)
    }

    pub fn waiting_time(&self) -> SimulationTime {
        let Customer {
            arrived_at,
            service_status: Served { started_at, .. },
            ..
        } = self;
        started_at - arrived_at
    }
}

impl Customer<Reneged> {
    pub fn reneging_time(&self) -> SimulationTime {
        let Customer {
            arrived_at,
            service_status: Reneged { reneged_at },
            ..
        } = self;
        reneged_at - arrived_at
    }
}
