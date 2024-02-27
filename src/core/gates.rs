use std::f64::consts::SQRT_2;

use super::{Comp, Qubits};

const SQRT2_INV: f64 = 1.0 / SQRT_2;

pub trait Applicable {
    fn apply(&self, qubits: Qubits) -> Qubits {
        let it = BitSlideIndex::new(1 << qubits.size, 0);
        return self.apply_iter(qubits, &it);
    }
    fn name(&self) -> String;
    fn apply_iter(&self, qubits: Qubits, iter: &BitSlideIndex) -> Qubits;
}

pub trait Reversible {
    fn reverse(&mut self) {}
}

pub trait Operator: Applicable + Reversible {}

pub struct BitSlideIndex {
    idx: usize,
    pub mask: usize,
    to: usize,
}

impl BitSlideIndex {
    pub fn new(to: usize, mask: usize) -> Self {
        return BitSlideIndex {
            idx: 0,
            mask: mask,
            to: to,
        };
    }

    pub fn merge(&self, other: usize) -> Self {
        if self.mask & other > 0 {
            println!("self.mask:{:b}, other_mask:{:b}", self.mask, other);
            panic!("invalid mask was input.");
        }
        return BitSlideIndex {
            idx: 0,
            mask: self.mask | other,
            to: self.to,
        };
    }

    pub fn init(&mut self) {
        self.idx = 0;
    }
}

impl Iterator for BitSlideIndex {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        if (idx & self.mask) != self.mask {
            let idx = idx | self.mask;
            self.idx = idx + 1;
            if idx < self.to {
                return Some(idx);
            }
        } else {
            self.idx = idx + 1;
            if idx < self.to {
                return Some(idx);
            }
        }
        return None;
    }
}

#[derive(Clone, Copy)]
pub struct H {
    target_bit: usize,
}

impl H {
    pub fn new(target_bit: usize) -> Self {
        return H {
            target_bit: target_bit,
        };
    }
}

impl Applicable for H {
    fn name(&self) -> String {
        return format!("H({})", self.target_bit);
    }

    fn apply_iter(&self, mut qubits: Qubits, iter: &BitSlideIndex) -> Qubits {
        let step = 1 << self.target_bit;
        let iter = iter.merge(step);

        for idx1 in iter {
            let idx0 = idx1 - step;
            let temp = qubits.bits[idx0];
            qubits.bits[idx0] = (qubits.bits[idx1] + temp) * SQRT2_INV;
            qubits.bits[idx1] = (temp - qubits.bits[idx1]) * SQRT2_INV;
        }

        return qubits;
    }
}

impl Reversible for H {}
impl Operator for H {}

#[derive(Clone, Copy)]
pub struct X {
    target_bit: usize,
}

impl X {
    pub fn new(target_bit: usize) -> Self {
        return X {
            target_bit: target_bit,
        };
    }
}

impl Applicable for X {
    fn name(&self) -> String {
        return format!("X({})", self.target_bit);
    }

    fn apply_iter(&self, mut qubits: Qubits, iter: &BitSlideIndex) -> Qubits {
        let step = 1 << self.target_bit;
        let iter = iter.merge(step);

        for idx1 in iter {
            let idx0 = idx1 - step;
            let temp = qubits.bits[idx0];
            qubits.bits[idx0] = qubits.bits[idx1];
            qubits.bits[idx1] = temp;
        }

        return qubits;
    }
}

impl Reversible for X {}
impl Operator for X {}

#[derive(Clone, Copy)]
pub struct Y {
    target_bit: usize,
}

impl Y {
    pub fn new(target_bit: usize) -> Self {
        return Y {
            target_bit: target_bit,
        };
    }
}

impl Applicable for Y {
    fn name(&self) -> String {
        return format!("Y({})", self.target_bit);
    }

    fn apply_iter(&self, mut qubits: Qubits, iter: &BitSlideIndex) -> Qubits {
        let step = 1 << self.target_bit;
        let iter = iter.merge(step);

        for idx1 in iter {
            let idx0 = idx1 - step;
            let temp = qubits.bits[idx0];
            qubits.bits[idx0] = Comp::new(0.0, 1.0) * qubits.bits[idx1];
            qubits.bits[idx1] = Comp::new(0.0, -1.0) * temp;
        }

        return qubits;
    }
}

impl Reversible for Y {}
impl Operator for Y {}

#[derive(Clone, Copy)]
pub struct Z {
    target_bit: usize,
}

impl Z {
    pub fn new(target_bit: usize) -> Self {
        return Z {
            target_bit: target_bit,
        };
    }
}

impl Applicable for Z {
    fn name(&self) -> String {
        return format!("Z({})", self.target_bit);
    }

    fn apply_iter(&self, mut qubits: Qubits, iter: &BitSlideIndex) -> Qubits {
        let step = 1 << self.target_bit;
        let iter = iter.merge(step);

        for idx1 in iter {
            qubits.bits[idx1] = qubits.bits[idx1] * -1.0;
        }

        return qubits;
    }
}

impl Reversible for Z {}
impl Operator for Z {}

#[derive(Clone, Copy)]
pub struct R {
    target_bit: usize,
    angle: f64,
    phase: Comp,
}

impl R {
    pub fn new(target_bit: usize, angle: f64) -> Self {
        let phase = Comp(angle.cos(), angle.sin());
        return R {
            target_bit: target_bit,
            angle: angle,
            phase: phase,
        };
    }
}

impl Applicable for R {
    fn name(&self) -> String {
        return format!("R_{}({})", self.angle, self.target_bit);
    }

    fn apply_iter(&self, mut qubits: Qubits, iter: &BitSlideIndex) -> Qubits {
        let step = 1 << self.target_bit;
        let iter = iter.merge(step);
        for idx1 in iter {
            qubits.bits[idx1] = qubits.bits[idx1] * self.phase;
        }

        return qubits;
    }
}

impl Reversible for R {}
impl Operator for R {}

#[derive(Clone, Copy)]
pub struct CX {
    controll_bit: usize,
    target_bit: usize,
}

impl CX {
    pub fn new(controll_bit: usize, target_bit: usize) -> Self {
        return CX {
            controll_bit: controll_bit,
            target_bit: target_bit,
        };
    }
}

impl Applicable for CX {
    fn name(&self) -> String {
        return format!("CX({}->{})", self.controll_bit, self.target_bit);
    }
    fn apply_iter(&self, mut qubits: Qubits, iter: &BitSlideIndex) -> Qubits {
        let step = 1 << self.target_bit;
        let iter = iter.merge((1 << self.controll_bit) | step);

        for idx1 in iter {
            let idx0 = idx1 - step;
            let temp = qubits.bits[idx0];
            qubits.bits[idx0] = qubits.bits[idx1];
            qubits.bits[idx1] = temp;
        }

        return qubits;
    }
}

impl Reversible for CX {}
impl Operator for CX {}

#[derive(Clone, Copy)]
pub struct CCX {
    controll_bit1: usize,
    controll_bit2: usize,
    target_bit: usize,
}

impl CCX {
    pub fn new(controll_bit1: usize, controll_bit2: usize, target_bit: usize) -> Self {
        return CCX {
            controll_bit1: controll_bit1,
            controll_bit2: controll_bit2,
            target_bit: target_bit,
        };
    }
}

impl Applicable for CCX {
    fn name(&self) -> String {
        return format!(
            "CCX([{},{}]->{})",
            self.controll_bit1, self.controll_bit2, self.target_bit
        );
    }
    fn apply_iter(&self, mut qubits: Qubits, iter: &BitSlideIndex) -> Qubits {
        let step = 1 << self.target_bit;
        let iter = iter.merge((1 << self.controll_bit1) | (1 << self.controll_bit2) | step);

        for idx1 in iter {
            let idx0 = idx1 - step;
            let temp = qubits.bits[idx0];
            qubits.bits[idx0] = qubits.bits[idx1];
            qubits.bits[idx1] = temp;
        }

        return qubits;
    }
}

impl Reversible for CCX {}
impl Operator for CCX {}

#[derive(Clone)]
pub struct CNX {
    controll_bits: Vec<usize>,
    target_bit: usize,
}

impl CNX {
    pub fn new(controll_bits: Vec<usize>, target_bit: usize) -> Self {
        return CNX {
            controll_bits: controll_bits,
            target_bit: target_bit,
        };
    }

    fn cbit_mask(&self) -> usize {
        let mut mask = 0;
        for cbit in self.controll_bits.iter() {
            mask |= 1 << (*cbit);
        }
        return mask;
    }
}

impl Applicable for CNX {
    fn name(&self) -> String {
        let mut s = String::from("CNX[");
        for i in self.controll_bits.iter() {
            s += &format!("{},", i);
        }
        s += &format!("]->{}", self.target_bit);
        return s;
    }
    fn apply_iter(&self, mut qubits: Qubits, iter: &BitSlideIndex) -> Qubits {
        let step = 1 << self.target_bit;
        let iter = iter.merge(self.cbit_mask() | step);

        for idx1 in iter {
            let idx0 = idx1 - step;
            let temp = qubits.bits[idx0];
            qubits.bits[idx0] = qubits.bits[idx1];
            qubits.bits[idx1] = temp;
        }

        return qubits;
    }
}

impl Reversible for CNX {}
impl Operator for CNX {}

pub struct CU {
    controll_bit: usize,
    gates: Vec<Box<dyn Operator>>,
    label: String,
}

impl CU {
    pub fn new(controll_bit: usize, gates: Vec<Box<dyn Operator>>, label: String) -> Self {
        return CU {
            controll_bit: controll_bit,
            gates: gates,
            label: label,
        };
    }

    pub fn from_u(controll_bit: usize, u: U) -> Self {
        return CU {
            controll_bit: controll_bit,
            gates: u.gates,
            label: u.label,
        };
    }
}

impl Applicable for CU {
    fn name(&self) -> String {
        let mut s = format!("CU({}->", self.controll_bit);
        for gate in &self.gates {
            s.push_str(&format!("\n{}", gate.name()));
        }
        return format!("{})", s);
    }

    fn apply_iter(&self, mut qubits: Qubits, iter: &BitSlideIndex) -> Qubits {
        let iter = iter.merge(1 << self.controll_bit);
        for gate in &self.gates {
            qubits = gate.apply_iter(qubits, &iter);
        }

        return qubits;
    }
}

impl Reversible for CU {
    fn reverse(&mut self) {
        for g in self.gates.iter_mut() {
            g.reverse();
        }
        self.gates.reverse();
    }
}

impl Operator for CU {}

pub struct U {
    pub gates: Vec<Box<dyn Operator>>,
    label: String,
}

impl U {
    pub fn new(gates: Vec<Box<dyn Operator>>, name: String) -> Self {
        return U {
            gates: gates,
            label: name,
        };
    }
}

impl Applicable for U {
    fn name(&self) -> String {
        let mut s = format!("U[{}](", self.label);
        for gate in &self.gates {
            s.push_str(&format!("\n{}", gate.name()));
        }
        return format!("{})", s);
    }

    fn apply_iter(&self, mut qubits: Qubits, iter: &BitSlideIndex) -> Qubits {
        for gate in &self.gates {
            qubits = gate.apply_iter(qubits, &iter);
        }

        return qubits;
    }
}

impl Reversible for U {
    fn reverse(&mut self) {
        for g in self.gates.iter_mut() {
            g.reverse();
        }
        self.gates.reverse();
    }
}

impl Operator for U {}
