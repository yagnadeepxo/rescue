
use num::bigint;
use num_bigint::BigUint;
use num_integer::binomial;
use lambdaworks_math::field::fields::u64_prime_field::U64PrimeField;
use lambdaworks_math::field::element::FieldElement;
use lambdaworks_math::field::traits::IsField;

use sha3::{Digest, Shake256};
use sha3::digest::{ExtendableOutput, Update, XofReader};


pub fn shake256(seed_bytes: &[u8], num_bytes: usize) -> Vec<u8> {
    let mut hasher = Shake256::default();
    hasher.update(seed_bytes);
    let mut reader = hasher.finalize_xof();
    let mut result = vec![0u8; num_bytes];
    reader.read(&mut result);
    result
}


pub struct Rescue {
    pub p: BigUint, // prime as a FieldElement
    pub width: usize,  // m 
    pub capacity: usize, // cp capacity
    pub rate: usize, // rate rp = m - cp
    pub alpha: u64, // alpha as a FieldElement
    pub sec_level: usize // 80 <= sec_level <= 512 
}

impl Rescue {
    pub fn new(p: BigUint, width: usize, capacity: usize, alpha: u64, sec_level: usize) -> Self {
        Self {
            p,
            width,
            capacity,
            rate: width - capacity,
            alpha,
            sec_level
        }
    }

    pub fn num_rounds(&self) -> usize {
        let rate = self.rate;

        let dcon = |n: usize| {
            ((0.5 * (self.alpha - 1) as f64 * self.width as f64 * (n - 1) as f64) + 2.0)
              .floor() as usize
          };
        let v = |n: usize| self.width * (n - 1) + rate;

        let target = BigUint::from(1u32) << self.sec_level;

        let mut l1 = 1;
        
        while l1 < 25 {
            let bin = binomial(BigUint::from(v(l1) + dcon(l1)), BigUint::from(v(l1)));
            if &bin * &bin > target {
                break;
            }  
            l1 += 1;
        }

        // Set a minimum value for sanity and add 50%
        let result = (1.5 * l1.max(5) as f64).ceil() as usize;

        result

    }


    pub fn get_round_constants<F: IsField>(&self, n: &usize) -> Vec<FieldElement<F>> {
        let p = self.p;
        let m = self.width;
        let cp = self.capacity;
        let sec_level = self.sec_level;
        let bytes_per_int = (p.bits() as f64 / 8.0).ceil() as usize + 1;
        let num_bytes = bytes_per_int * 2 * m * n;
        let params_str = format!("Rescue-XLIX({},{},{},{})", p, m, self.capacity, self.sec_level);
        let byte_string = shake256(params_str.as_bytes(), num_bytes);

        let mut round_constants = Vec::new();
        
        
    }



}