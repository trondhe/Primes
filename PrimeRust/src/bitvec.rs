#![allow(dead_code)]
use std::mem::size_of;

pub struct Bitvec {
    data: Vec<usize>,
    bit_count: usize,
    value_size: usize,
    vec_size: usize,
}

impl Bitvec {
    pub fn new(bit_count: usize) -> Self {
        let value_size = size_of::<usize>() * 8;
        let vec_size = (bit_count as f64 / value_size as f64).ceil() as usize;
        Self {
            data: vec![0; vec_size],
            bit_count,
            value_size,
            vec_size,
        }
    }

    pub fn get(&self, index: usize) -> bool {
        if index >= self.bit_count {
            return false;
        }
        let value_index = index / self.value_size;
        let bit_index = index % self.value_size;
        let value_mask: usize = 1 << bit_index;
        (self.data[value_index] & value_mask) != 0
    }

    pub fn set(&mut self, index: usize) {
        if index >= self.bit_count {
            return;
        }
        let value_index = index / self.value_size;
        let bit_index = index % self.value_size;
        let value_mask: usize = 1 << bit_index;
        self.data[value_index] |= value_mask;
    }

    pub fn clear(&mut self, index: usize) {
        if index >= self.bit_count {
            return;
        }
        let value_index = index / self.value_size;
        let bit_index = index % self.value_size;
        let value_mask: usize = !(1 << bit_index);
        self.data[value_index] &= value_mask;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_write_read() {
        let bitcount = 129;
        let mut bv = Bitvec::new(bitcount);

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
