use std::{collections::HashMap, time::Duration};

use crate::bitvec::{Bitvec, Integer};

pub struct Prime {
    sieve_size: usize,
    bits: Bitvec,
}

impl Prime {
    pub fn new(n: usize) -> Self {
        Prime {
            sieve_size: n,
            bits: Bitvec::new(n as Integer, true),
        }
    }

    pub fn run_sieve(&mut self) {
        let mut factor = 3;
        let q = (self.sieve_size as f32).sqrt() as usize;

        while factor < q {
            for num in factor..self.sieve_size {
                if self.get_bit(num) {
                    factor = num;
                    break;
                }
            }
            for num in ((factor * 3)..self.sieve_size).step_by(factor * 2) {
                self.clear_bit(num);
            }
            factor += 2;
        }
    }

    pub fn print_results(&self, duration: Duration, passes: usize) {
        let mut count = 1;
        for num in 3..self.sieve_size {
            if self.get_bit(num) {
                count += 1;
            }
        }

        println!(
            "Passes: {}, Time: {}, Avg: {}, Limit: {}, Count: {}, Valid: {}\n",
            passes,
            duration.as_micros() as f64 / 1e6,
            duration.as_secs() as f64 / passes as f64,
            self.sieve_size,
            count,
            self.validate_results()
        );
    }

    fn get_bit(&self, index: usize) -> bool {
        if index % 2 == 0 {
            return false;
        }
        self.bits.get(index)
    }

    fn clear_bit(&mut self, index: usize) {
        if index % 2 == 0 {
            return;
        }
        self.bits.clear(index);
    }

    fn count_primes(&self) -> usize {
        let mut count = 0;
        for index in 0..self.sieve_size {
            if self.get_bit(index) {
                count += 1;
            }
        }
        count
    }

    fn validate_results(&self) -> bool {
        let mut validation_map: HashMap<usize, usize> = HashMap::new();
        validation_map.insert(10, 1);
        validation_map.insert(100, 25);
        validation_map.insert(1000, 168);
        validation_map.insert(10000, 1229);
        validation_map.insert(100000, 9592);
        validation_map.insert(1000000, 78498);
        validation_map.insert(10000000, 664579);
        validation_map.insert(100000000, 5761455);
        if !validation_map.contains_key(&self.sieve_size) {
            return false;
        }
        validation_map[&self.sieve_size] == self.count_primes()
    }
}
