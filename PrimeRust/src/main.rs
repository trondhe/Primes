use std::time::{Duration, SystemTime};

use prime::Prime;

mod bitvec;
mod prime;

const N_MAX: usize = 1_000_000;

fn main() {
    let mut passes = 0;
    let t_start = SystemTime::now();
    loop {
        let mut sieve = Prime::new(N_MAX);
        sieve.run_sieve();
        passes += 1;

        let elapsed = t_start
            .elapsed()
            .expect("Error on calculating elapsed time");
        if elapsed >= Duration::from_secs(10) {
            sieve.print_results(elapsed, passes);
            break;
        }
    }
}
