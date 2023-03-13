use queue_simulation::entities::{
    Customer, CustomerGenerator, CustomerQueue, Served, ServicePoint,
};
use queue_simulation::ext::VecStatsExt;

fn main() {
    const ARRIVALS_PER_SECOND: f64 = 1.0 / 12.0;
    const SIMULATION_SECONDS: u32 = 360_000;
    const SERVICE_RATE: f64 = 1.0 / 60.0;

    let customer_generator = CustomerGenerator::new(ARRIVALS_PER_SECOND);
    let mut customer_queue = CustomerQueue::new();
    let mut service_point = ServicePoint::new(SERVICE_RATE);
    let mut served_customers: Vec<Customer<Served>> = vec![];

    for t in 0..SIMULATION_SECONDS {
        let incoming = customer_generator.generate(t);

        customer_queue.enqueue(incoming).unwrap();
        if service_point.is_available() {
            let customer = customer_queue.dequeue();
            if customer.is_some() {
                let customer = customer.unwrap().start_service(t);
                service_point.start_service(customer);
            }
        } else {
            if service_point.has_service_ended(t) {
                let customer = service_point.finish_service(t);
                if let Some(customer) = customer {
                    served_customers.push(customer)
                }
            }
        }
    }

    let average_service_time = served_customers
        .iter()
        .map(|c| c.waiting_time())
        .stats()
        .mean();
    println!("Average service time: {:?}", average_service_time);
}
