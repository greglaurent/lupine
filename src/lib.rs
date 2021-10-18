use fxhash::FxHasher;
use bit_vec::BitVec;
use std::{hash::{Hash, Hasher}, marker::PhantomData};

const FALSE_POS_PROB: f64 = -1.0;
const LN_2: f64 = core::f64::consts::LN_2;
const LN_2_SQR: f64 = LN_2 * LN_2;

/// A standard Bloom Filter, using Kirsch and Mitzenmacher
/// optimization and two hash functions.
/// Hash: [FX Hash](https://github.com/cbreeden/fxhash)
pub struct BloomFilter<T: ?Sized> {
    k: u64, 
    m: usize,
    hashers: [FxHasher; 2],
    bitmap: BitVec,
    _phantom: PhantomData<T>,
}

impl<T: ?Sized> BloomFilter<T> { 
    /// Returns a BloomFilter with an optimized k and m 
    /// 
    /// # Arguments
    ///
    /// * 'size' - A usize that sets the size of the filter
    /// * 'false_pos_rate' - The acceptable false positive rate
    ///
    /// # Examples
    ///
    /// ```
    /// use lupine::BloomFilter;
    /// let filter: BloomFilter<usize> = BloomFilter::new(1_000, 0.001);
    /// ```
    pub fn new(size: usize, false_pos_rate: f64) -> Self { 
        let k = Self::optimal_k(false_pos_rate);
        let m = Self::optimal_m(false_pos_rate, size);
        let bitmap = BitVec::from_elem(m, false);
        let hashers = [ 
            FxHasher::default(),
            FxHasher::default(),
        ];

        BloomFilter { 
            k,
            m,
            hashers,
            bitmap,
            _phantom: PhantomData
        }
    }
 
    /// Calculate optimal m value for the filter 
    /// where m is the optimal number of bits in the bit array
    /// while preventing overfill
    ///
    /// where P is the probability of false positives
    /// and n is the acceptable false postive rate
    /// k = ( -( n * lnP ) / (ln2)^2 )
    fn optimal_m(false_pos_rate: f64, size: usize) -> usize { 
        ((size as f64 * FALSE_POS_PROB * false_pos_rate.ln()) / LN_2_SQR).ceil() as usize
    }

    /// Calculate optimal k value for the filter 
    /// where k is the number of functions to hash input T
    /// yielding k indices into the bit array
    ///
    /// where P is the probability of false positives
    /// k = ( - lnP / ln2 )
    fn optimal_k(false_pos_rate: f64) -> u64 {
        (false_pos_rate.ln() * FALSE_POS_PROB / LN_2).ceil() as u64
    }

    /// Hash values T for Bloomfilter
    fn hash(&self, t: &T) -> (u64, u64) where T: Hash{ 
        let hash1 = &mut self.hashers[0].clone();
        let hash2 = &mut self.hashers[1].clone();

        t.hash(hash1);
        t.hash(hash2);

        (hash1.finish(), hash2.finish())
    }

    /// Retrieve the index of indexes by simulating 
    /// more than 2 hashers
    ///
    /// Prevent Overflow:
    /// wrapping_add: wrapping add around at the boundary type
    /// wrapping_mul: wrapping mult around at the boundary type
    fn find_index(&self, i: u64, hash1: u64, hash2: u64) -> usize { 
        hash1.wrapping_add((i).wrapping_mul(hash2)) as usize % self.m
    }

    /// Insert T into the BloomFilter index
    ///
    /// # Examples: 
    /// ```
    /// use lupine::BloomFilter;
    /// let mut bf = BloomFilter::new(1000, 0.0001);
    /// bf.insert(&10);
    /// assert!(bf.contains(&10));
    /// ```
    pub fn insert(&mut self, t: &T) where T: Hash {
        let (hash1, hash2) = self.hash(t);

        for i in 0..self.k { 
            let index = self.find_index(i, hash1, hash2);
            self.bitmap.set(index, true);
        }
    }

    /// Check if t of type T into the BloomFilter index
    ///
    /// # Examples: 
    /// ```
    /// use lupine::BloomFilter;
    /// let mut bf = BloomFilter::new(1000, 0.0001);
    /// assert!(!bf.contains(&10));
    /// bf.insert(&10);
    /// assert!(bf.contains(&10));
    /// ```
    pub fn contains(&mut self, t: &T) -> bool where T: Hash {
        let (hash1, hash2) = self.hash(t);

        for i in 0..self.k { 
            let index = self.find_index(i, hash1, hash2);
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
    fn test_optimal_m() { 
        let o = BloomFilter::<u16>::optimal_m(0.001, 1000);
        assert_eq!(o, 14378);
    }

    #[test]
    fn test_optimal_k() { 
        let o = BloomFilter::<u16>::optimal_k(0.001);
        assert_eq!(o, 10);
    }

    #[test]
    fn test_hash() { 
        let bf = BloomFilter::new(1000, 0.0001);
        let o = bf.hash("test");
        assert_eq!(o, (14825577416135075223, 14825577416135075223));
    }
}
