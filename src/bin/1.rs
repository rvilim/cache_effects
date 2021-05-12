use std::env;
use std::time::{Duration, Instant};

fn main() {
    let args: Vec<String> = env::args().collect();

    let size = args[1].parse::<usize>().unwrap();
    let skip = args[2].parse::<usize>().unwrap();
    let n_trials = args[3].parse::<u128>().unwrap();
    let burn_in = 1000;

    let mut total_nanos = 0;

    for i in 0..n_trials {
        let mut vec = vec![5; size];
        let now = Instant::now();

        for i in (0..vec.len()).step_by(skip) {
            vec[i] *= 2;
        }

        if i > burn_in {
            total_nanos += now.elapsed().as_nanos();
        }
    }

    println!("{}", total_nanos / n_trials);
}
