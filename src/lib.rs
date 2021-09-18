use bit_vec::BitVec;
use std::{
    collections::hash_map::{DefaultHasher, RandomState}, 
    hash::{BuildHasher, Hash, Hasher}, 
    marker::PhantomData};

const FALSE_POS_PROB: f64 = -1.0f64;
const LN_2_SQR: f64 = core::f64::consts::LN_2 * core::f64::consts::LN_2;

pub struct BloomFilter<T: ?Sized + Send + Sync> {
    k: u64,
    m: usize,
    hashers: [DefaultHasher; 2],
    bitmap: BitVec,
    _phantom: PhantomData<T>,
}

impl<T: ?Sized + Send + Sync> BloomFilter<T> { 
    pub fn new(size: usize, false_pos_rate: f64) -> Self { 
        let k = Self::optimal_k(false_pos_rate);
        let m = Self::calc_bitmap(size, false_pos_rate);
        let bitmap = BitVec::from_elem(m as usize, false);
        let hashers = [ 
            RandomState::new().build_hasher(),
            RandomState::new().build_hasher(),
        ];

        BloomFilter { 
            k,
            m,
            hashers,
            bitmap,
            _phantom: PhantomData
        }
    }

    fn calc_bitmap(size: usize, false_pos_rate: f64) -> usize { 
        ((size as f64 * FALSE_POS_PROB * false_pos_rate.ln()) / LN_2_SQR).ceil() as usize
    }
    
    fn optimal_k(false_pos_rate: f64) -> u64 {
        (false_pos_rate.ln() * FALSE_POS_PROB / LN_2_SQR).ceil() as u64
    }

    fn hash(&self, item: &T) -> (u64, u64) where T: Hash{ 
        let hash1 = &mut self.hashers[0].clone();
        let hash2 = &mut self.hashers[1].clone();

        item.hash(hash1);
        item.hash(hash2);

        (hash1.finish(), hash2.finish())
    }

    fn get_index(&self, hash1: u64, hash2: u64, i: u64) -> usize { 
        hash1.wrapping_add((i).wrapping_mul(hash2)) as usize % self.m
    }

    pub fn insert(&mut self, item: &T) where T: Hash {
        let (h1, h2) = self.hash(item);

        for i in 0..self.k { 
            let index = self.get_index(h1, h2, i as u64);

            self.bitmap.set(index, true);
        }
    }

    pub fn contains(&mut self, item: &T) -> bool where T: Hash {
        let (h1, h2) = self.hash(item);

        for i in 0..self.k { 
            let index = self.get_index(h1, h2, i as u64);
            if !self.bitmap.get(index).unwrap() { 
                return false
            }
       }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut bf = BloomFilter::new(1000, 0.0000001);
        bf.insert(&10);
        assert!(bf.contains(&10));
    }

    #[test]
    fn test_check_and_insert() {
        let mut bf = BloomFilter::new(1000, 0.01);

        assert!(!bf.contains("test2"));
        bf.insert("test2");
        assert!(bf.contains("test2"));
    }
}
