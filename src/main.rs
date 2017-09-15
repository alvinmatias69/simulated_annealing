extern crate rand;

use rand::Rng;

fn main() {
    let mut solution: [f64; 2] = [rand_between(-10.0, 10.0), rand_between(-10.0, 10.0)];
    let mut cost = value(solution[0], solution[1]);
    let mut min_cost = cost;
    let mut min_solution: [f64; 2] = solution;
    let temp_max = 90.0;
    let temp_min = 40.0;
    let mut temp = temp_max;
    let mut new_solution: [f64; 2] = [0.0, 0.0];
    let mut new_cost: f64;

    while temp > temp_min {
        for i in 0..100 {
            new_solution[0] = solution[0] + rand_between(-1.0, 1.0);
            new_solution[1] = solution[1] + rand_between(-1.0, 1.0);
            new_cost = value(new_solution[0], new_solution[1]);
            if new_cost < cost || check_acceptance(new_cost - cost, temp) {
                solution[0] = new_solution[0];
                solution[1] = new_solution[1];
                cost = new_cost;
            }
        }

        if min_cost > cost {
            min_cost = cost;
            min_solution[0] = solution[0];
            min_solution[1] = solution[1];
        }

        temp *= 0.9;
    }

    println!("{}", min_cost);
}

fn rand_between(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min, max)
}

fn value(x1: f64, x2: f64) -> f64 {
    (4.0 - 2.1 * x1.powi(2) + x1.powi(4) / 3.0) * x1.powi(2) + x1 * x2 + (-4.0 + 4.0 * x2.powi(2)) * x2.powi(2)
}

fn check_acceptance(delta: f64, temp: f64) -> bool {
    rand_between(0.0, 1.0) < (-delta / temp).exp()
}
