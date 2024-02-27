use std::fmt;
use std::ops;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Comp(pub f64, pub f64);

pub mod gates;
pub mod mod_funcs;
pub mod modules;

impl Comp {
    pub fn new(re: f64, im: f64) -> Self {
        return Comp(re, im);
    }

    pub fn abs(&self) -> f64 {
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
        assert!(comp.abs() == 1.0);
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
                prob = (self.bits[index].abs() * 100.0).round()
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
            prob[index] = self.bits[index].abs();
        }
        return prob;
    }

    pub fn pop_most_plausible(&self) -> usize {
        let mut max_prob = 0.0;
        let mut max_idx = 0;
        for i in 0..(1 << self.size) {
            let prob = self.bits[i].abs();
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
            probs[tar_idx] += self.bits[i].abs();
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
            probs[tar_idx] += self.bits[i].abs();
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
