#![allow(dead_code)]
use std::mem::size_of;

pub type Integer = u32;

pub struct Bitvec {
    data: Vec<Integer>,
    bit_count: usize,
}

impl Bitvec {
    const VALUE_SIZE: usize = size_of::<Integer>() * 8;
    pub fn new(bit_count: Integer, init_as: bool) -> Self {
        let vec_size = (bit_count as f64 / Bitvec::VALUE_SIZE as f64).ceil() as usize;
        let init_value: Integer;
        if init_as {
            init_value = Integer::MAX;
        } else {
            init_value = 0;
        }
        Self {
            data: vec![init_value; vec_size],
            bit_count: bit_count as usize,
        }
    }

    pub fn get(&self, index: usize) -> bool {
        if index >= self.bit_count {
            return false;
        }
        let value_index = index / Bitvec::VALUE_SIZE;
        let bit_index = index % Bitvec::VALUE_SIZE;
        let value_mask = 1 << bit_index;
        (self.data[value_index] & value_mask) != 0
    }

    pub fn set(&mut self, index: usize) {
        if index >= self.bit_count {
            return;
        }
        let value_index = index / Bitvec::VALUE_SIZE;
        let bit_index = index % Bitvec::VALUE_SIZE;
        let value_mask = 1 << bit_index;
        self.data[value_index] |= value_mask;
    }

    pub fn clear(&mut self, index: usize) {
        if index >= self.bit_count {
            return;
        }
        let value_index = index / Bitvec::VALUE_SIZE;
        let bit_index = index % Bitvec::VALUE_SIZE;
        let value_mask = !(1 << bit_index);
        self.data[value_index] &= value_mask;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_write_read() {
        let bitcount = 129;
        let mut bv = Bitvec::new(bitcount as Integer, false);

        for n in 0..bitcount {
            let value = bv.get(n);
            assert!(!value);
            bv.set(n);
            let value = bv.get(n);
            assert!(value);
            bv.clear(n);
            let value = bv.get(n);
            assert!(!value);
        }
    }
}
