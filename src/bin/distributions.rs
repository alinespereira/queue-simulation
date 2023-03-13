fn _run_exp() {
    use statrs::distribution::{ContinuousCDF, Exp};
    let rate = 1.0 / 30.0;
    let service_time_dist = Exp::new(rate).unwrap();

    for i in 0..100 {
        println!("{: >2}: {:.6}", i, service_time_dist.cdf(i as f64));
    }
}

fn run_poisson() {
    use rand::distributions::Distribution;
    use statrs::distribution::Poisson;

    let lambda = 1.0 / 60.0;
    let dist = Poisson::new(lambda).unwrap();
    let ref mut rng = rand::thread_rng();

    for _ in 0..50 {
        println!("{}", dist.sample(rng) as usize);
    }
}

fn main() {
    run_poisson();
}
