/*!
Qubit struct used for simulation and complex number struct that composes it

# Example usage
```
use Qit::core::{Comp, Qubits}
let zero = Comp::zero()
```
*/

use std::fmt;
use std::ops;

/**
 Complex numbers implemented with functions required for quantum simulation
 It is implemented with the only purpose of expressing quantum bits.

 # Example usage
 ```
let zero = Comp::zero();
println!("{}", zero);
// +0.000 +0.000i
let re: f64 = 1.0;
let im: f64 = -30.0;
let c = Comp::new(re, im);
println!("{}", c);
// +0.200 -30.000i
let c1 = Comp::new(2.0, 1.0);
let c2 = Comp::new(1.0, 2.0);
let add_c1c2 = c1 + c2;
assert_eq!(add_c1c2, Comp::new(3.0, 3.0));

let sub_c1c2 = c1 - c2;
assert_eq!(sub_c1c2, Comp::new(1.0, -1.0));

let mul_c1c2 = c1 * c2;
assert_eq!(mul_c1c2, Comp::new(0.0, 5.0));

let f1: f64 = 2.0;
let add_c1f1 = c1 + f1;
assert_eq!(add_c1f1, Comp::new(4.0, 1.0));

let sub_c1f1 = c1 - f1;
assert_eq!(sub_c1f1, Comp::new(0.0, 1.0));

let mul_c1f1 = c1 * f1;
assert_eq!(mul_c1f1, Comp::new(4.0, 2.0));
 ```
 */
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Comp(pub f64, pub f64);

pub mod gates;
pub mod mod_funcs;
pub mod modules;

impl Comp {
    pub fn new(re: f64, im: f64) -> Self {
        return Comp(re, im);
    }

    pub fn abs_square(&self) -> f64 {
        return self.0 * self.0 + self.1 * self.1;
    }

    pub fn zero() -> Self {
        return Comp(0.0, 0.0);
    }
}

impl fmt::Display for Comp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:+.3} {:+.3}i",
            (self.0 * 1000.0).round() / 1000.0,
            (self.1 * 1000.0).round() / 1000.0
        )
    }
}

impl ops::Add<Comp> for Comp {
    type Output = Comp;
    fn add(self, _rhs: Comp) -> Comp {
        return Comp(self.0 + _rhs.0, self.1 + _rhs.1);
    }
}

impl ops::Add<f64> for Comp {
    type Output = Comp;
    fn add(self, rhs: f64) -> Comp {
        return Comp(self.0 + rhs, self.1);
    }
}

impl ops::Sub<Comp> for Comp {
    type Output = Comp;
    fn sub(self, _rhs: Comp) -> Comp {
        return Comp(self.0 - _rhs.0, self.1 - _rhs.1);
    }
}

impl ops::Sub<f64> for Comp {
    type Output = Comp;
    fn sub(self, rhs: f64) -> Comp {
        return Comp(self.0 - rhs, self.1);
    }
}

impl ops::Mul<Comp> for Comp {
    type Output = Comp;
    fn mul(self, _rhs: Comp) -> Comp {
        return Comp(
            self.0 * _rhs.0 - self.1 * _rhs.1,
            self.0 * _rhs.1 + self.1 * _rhs.0,
        );
    }
}

impl ops::Mul<f64> for Comp {
    type Output = Comp;
    fn mul(self, rhs: f64) -> Comp {
        return Comp(self.0 * rhs, self.1 * rhs);
    }
}

#[derive(Clone)]
pub struct Qubits {
    pub size: usize,
    pub bits: Vec<Comp>,
}

impl Qubits {
    pub fn from_num(size: usize, number: usize) -> Self {
        assert!((1 << size) > number);
        let mut bits = vec![Comp::zero(); 1 << size];
        bits[number] = Comp(1.0, 0.0);
        return Qubits {
            size: size,
            bits: bits,
        };
    }

    pub fn from_comp(size: usize, number: usize, comp: Comp) -> Self {
        assert!((1 << size) > number);
        assert!(comp.abs_square() == 1.0);
        let mut bits = vec![Comp::zero(); 1 << size];
        bits[number] = comp;
        return Qubits {
            size: size,
            bits: bits,
        };
    }

    pub fn from_bits(size: usize, bits: Vec<Comp>) -> Self {
        assert_eq!(1 << size, bits.len());
        return Qubits {
            size: size,
            bits: bits,
        };
    }

    pub fn zeros(size: usize) -> Self {
        let mut bits = vec![Comp::zero(); 1 << size];
        bits[0] = Comp(1.0, 0.0);
        return Qubits {
            size: size,
            bits: bits,
        };
    }

    pub fn print_probs(&self) {
        for index in 0..(1 << self.size) {
            println!(
                "|{index:0>size$b}⟩ : {prob:>3}%",
                index = index,
                size = self.size,
                prob = (self.bits[index].abs_square() * 100.0).round()
            );
        }
    }

    pub fn print_cmps(&self) {
        for index in 0..(1 << self.size) {
            println!(
                "|{index:0>size$b}⟩ : {cmp}",
                index = index,
                size = self.size,
                cmp = self.bits[index]
            );
        }
    }

    pub fn probs(&self) -> Vec<f64> {
        let mut prob = vec![0.0; (1 << self.size)];
        for index in 0..(1 << self.size) {
            prob[index] = self.bits[index].abs_square();
        }
        return prob;
    }

    pub fn pop_most_plausible(&self) -> usize {
        let mut max_prob = 0.0;
        let mut max_idx = 0;
        for i in 0..(1 << self.size) {
            let prob = self.bits[i].abs_square();
            if max_prob < prob {
                max_prob = prob;
                max_idx = i;
            }
        }
        return max_idx;
    }

    pub fn _measure(&self, tar: &[usize]) -> Vec<f64> {
        let mut probs: Vec<f64> = Vec::new();
        for _ in 0..(1 << tar.len()) {
            probs.push(0.0);
        }
        for i in 0..(1 << self.size) {
            let mut tar_idx = 0;
            for j in 0..tar.len() {
                tar_idx |= (1 & (i >> tar[j])) << j;
            }
            probs[tar_idx] += self.bits[i].abs_square();
        }

        return probs;
    }

    pub fn _print_measure(&self, tar: &[usize]) {
        let mut probs: Vec<f64> = Vec::new();
        for _ in 0..(1 << tar.len()) {
            probs.push(0.0);
        }
        for i in 0..(1 << self.size) {
            let mut tar_idx = 0;
            for j in 0..tar.len() {
                tar_idx |= (1 & (i >> tar[j])) << j;
            }
            probs[tar_idx] += self.bits[i].abs_square();
        }

        for index in 0..(1 << tar.len()) {
            println!(
                "|{index:0>size$b}⟩ : {prob:>3.2}%",
                index = index,
                size = tar.len(),
                prob = (probs[index] * 10000.0).round() / 100.0
            );
        }
    }
}

pub fn pop_from_probs(probs: &[f64], size: usize) -> usize {
    use rand::prelude::*;

    loop {
        let mut r: f64 = rand::thread_rng().gen();
        for i in 0..(1 << size) {
            r -= probs[i];
            if r < 0.0 {
                return i;
            }
        }
    }
}
