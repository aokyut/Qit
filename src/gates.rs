/*!
 Basic gates that make up quantum circuits and traits related to gate applications

 The following gates can act directly on qubits.

* 1-Bit Gates
    * X(X-Gate. Not Gate)
    * Y(Y-Gate)
    * Z(Z-Gate)
    * H(Hadamard-Gate)
    * R(R_z-Gate. Gate that rotates at any angle around the z-axis)
* 2-Bit Gate
    * CX(Controlled Not Gate)
* 3-Bit Gate
    * CCX(Controlled Controlled Not Gate)
* N-Bit Gate
    * CNX(Controlled Controlled ...(n) Not Gate)

CNX is an X gate with an arbitrary number of control bits prepared for convenience in circuit creation.

In addition to the above, there is a U structure that combines multiple gates into one circuit,
and a CU structure that allows you to control multiple gates with one bit.

# Example
All gates can make changes to the qubit using the apply method.
## 1-Bit Gates
```
use Qit::{gates::{X, Y, Z, H, R}, core::{Applicable, Qubits}};
use std::f64::consts::PI;

// create gate struct
let x_0 = X::new(0);
// make Qbit|000⟩ Qubits::from_num(size: 3, num: 0)
let q_in = Qubits::from_num(2, 0);
// apply X-gate to Qbit
let q_out = x_0.apply(q_in);

q_out.print_probs();
// |00⟩ : +0.000 +0.000i
// |01⟩ : +1.000 +0.000i
// |10⟩ : +0.000 +0.000i
// |11⟩ : +0.000 +0.000i

// Y-Gate
let y_0 = Y::new(0);
let q_in = Qubits::zeros(2);
let q_out = y_0.apply(q_in);
q_out.print_cmps();
// |00⟩ : +0.000 +0.000i
// |01⟩ : +0.000 -1.000i
// |10⟩ : +0.000 +0.000i
// |11⟩ : +0.000 +0.000i

// Z-Gate
let z_0 = Z::new(0);
let q_in = Qubits::from_num(2, 1);
let q_out = z_0.apply(q_in);
q_out.print_cmps();
// |00⟩ : +0.000 +0.000i
// |01⟩ : -1.000 -0.000i
// |10⟩ : +0.000 +0.000i
// |11⟩ : -0.000 -0.000i

// Hadamard-Gate
let h_0 = H::new(0);
let q_in = Qubits::zeros(2);
let q_out = h_0.apply(q_in);
q_out.print_cmps();
// |00⟩ : +0.707 +0.000i
// |01⟩ : +0.707 +0.000i
// |10⟩ : +0.000 +0.000i
// |11⟩ : +0.000 +0.000i

// R gate requires angle of f64.
let angle = 0.5 * PI;
let r_0 = R::new(0, angle);
let q_in = q_out;
let q_out = r_0.apply(q_in);
q_out.print_cmps();
// |00⟩ : +0.707 +0.000i
// |01⟩ : +0.000 +0.707i
// |10⟩ : +0.000 +0.000i
// |11⟩ : +0.000 +0.000i

```

## 2-Bit Gate
```
use Qit::core::{Applicable, Qubits, Comp};
use Qit::gates::CX;

// create CX(0 → 1)
let cx01 = CX::new(0, 1);
let q_in = Qubits::from_num(2, 1);
let q_out = cx01.apply(q_in);
q_out.print_cmps();
// |00⟩ : +0.000 +0.000i
// |01⟩ : +0.000 +0.000i
// |10⟩ : +0.000 +0.000i
// |11⟩ : +1.000 +0.000i
assert_eq!(q_out.bits[0b11], Comp::new(1.0, 0.0));
```

## 3-Bit Gate
```
use Qit::core::{Applicable, Comp, Qubits};
use Qit::gates::CCX;

let ccx = CCX::new(0, 1, 2);
// q_in = |011⟩
let q_in = Qubits::from_num(3, 3);
let q_out = ccx.apply(q_in);
q_out.print_cmps();
// |000⟩ : +0.000 +0.000i
// |001⟩ : +0.000 +0.000i
// |010⟩ : +0.000 +0.000i
// |011⟩ : +0.000 +0.000i
// |100⟩ : +0.000 +0.000i
// |101⟩ : +0.000 +0.000i
// |110⟩ : +0.000 +0.000i
// |111⟩ : +1.000 +0.000i
assert_eq!(q_out.bits[0b111], Comp::new(1.0, 0.0));
```

## n-Bit Gate
```
use Qit::core::{Applicable, Comp, Qubits};
use Qit::gates::CNX;

let cnx = CNX::new(vec![0, 1, 2], 3);
// q_in = |0111⟩
let q_in = Qubits::from_num(4, 7);
let q_out = cnx.apply(q_in);
q_out.print_cmps();
// |0000⟩ : +0.000 +0.000i
// |0001⟩ : +0.000 +0.000i
//        .
//        .
//        .
// |1110⟩ : +0.000 +0.000i
// |1111⟩ : +1.000 +0.000i
assert_eq!(q_out.bits[0b1111], Comp::new(1.0, 0.0));
```
*/

use std::f64::consts::SQRT_2;

use super::core::{Applicable, BitSlideIndex, Comp, Operator, Qubits, Reversible};

const SQRT2_INV: f64 = 1.0 / SQRT_2;

/**
 * Hadamard Gate. 1√2(|0⟩⟨0| + |1⟩⟨0| + |0⟩⟨1| - |1⟩⟨1|)

 # Usage
 ```
use Qit::{gates::H, core::{Applicable, Qubits}};

// Hadamard-Gate
let h_0 = H::new(0);
let q_in = Qubits::zeros(2);
let q_out = h_0.apply(q_in);
q_out.print_cmps();
// |00⟩ : +0.707 +0.000i
// |01⟩ : +0.707 +0.000i
// |10⟩ : +0.000 +0.000i
// |11⟩ : +0.000 +0.000i
 ```

*/
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

/**
Not Gate(pauli-X). (|1⟩⟨0| + |0⟩⟨1|)

# Usage
```
use Qit::{gates::X, core::{Applicable, Qubits}};

// X-Gate
let x_0 = X::new(0);
let q_in = Qubits::from_num(2, 0);
let q_out = x_0.apply(q_in);
q_out.print_probs();
// |00⟩ : +0.000 +0.000i
// |01⟩ : +1.000 +0.000i
// |10⟩ : +0.000 +0.000i
// |11⟩ : +0.000 +0.000i
```
*/
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

/**
 pauli-Y Gate. i(|1⟩⟨0| - |0⟩⟨1|)

# Usage
```
use Qit::{gates::Y, core::{Applicable, Qubits}};

// Y-Gate
let y_0 = Y::new(0);
let q_in = Qubits::zeros(2);
let q_out = y_0.apply(q_in);
q_out.print_cmps();
// |00⟩ : +0.000 +0.000i
// |01⟩ : +0.000 -1.000i
// |10⟩ : +0.000 +0.000i
// |11⟩ : +0.000 +0.000i
```
*/
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

/**
 pauli-Z Gate. (|0⟩⟨0| - |1⟩⟨1|)

 # Usage

 ```
use Qit::{gates::Z, core::{Applicable, Qubits}};

// Z-Gate
let z_0 = Z::new(0);
let q_in = Qubits::from_num(2, 1);
let q_out = z_0.apply(q_in);
q_out.print_cmps();
// |00⟩ : +0.000 +0.000i
// |01⟩ : -1.000 -0.000i
// |10⟩ : +0.000 +0.000i
// |11⟩ : -0.000 -0.000i
 ```
*/
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

/**
Phase shift Gate. (|0⟩⟨0| + exp(ir)|1⟩⟨1|)

(|0⟩ + |1⟩)  →R(π)→  (|0⟩ - |1⟩)

(|0⟩ + |1⟩)  →R(π/2)→  (|0⟩ + i|1⟩)

# Usage
```
use Qit::{gates::R, core::{Applicable, Qubits, Comp}};
use std::f64::consts::PI;

// R gate requires angle of f64.
let angle = 0.5 * PI;
let r_0 = R::new(0, angle);
let q_in = Qubits::from_num(2, 1);
let q_out = r_0.apply(q_in);
q_out.print_cmps();
// |00⟩ : +0.000 +0.000i
// |01⟩ : +0.000 +1.000i
// |10⟩ : +0.000 +0.000i
// |11⟩ : +0.000 +0.000i
```
 */
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

/**
Controlled-Not Gate.

# Usage

```
use Qit::core::{Applicable, Qubits, Comp};
use Qit::gates::CX;

// create CX(0 → 1)
let cx01 = CX::new(0, 1);
let q_in = Qubits::from_num(2, 1);
let q_out = cx01.apply(q_in);
q_out.print_cmps();
// |00⟩ : +0.000 +0.000i
// |01⟩ : +0.000 +0.000i
// |10⟩ : +0.000 +0.000i
// |11⟩ : +1.000 +0.000i
assert_eq!(q_out.bits[0b11], Comp::new(1.0, 0.0));
```
 */
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

/**
Controlled-Controlled-Not(CXX) Gate.

# Usage

```
use Qit::core::{Applicable, Comp, Qubits};
use Qit::gates::CCX;

let ccx = CCX::new(0, 1, 2);
// q_in = |011⟩
let q_in = Qubits::from_num(3, 3);
let q_out = ccx.apply(q_in);
q_out.print_cmps();
// |000⟩ : +0.000 +0.000i
// |001⟩ : +0.000 +0.000i
// |010⟩ : +0.000 +0.000i
// |011⟩ : +0.000 +0.000i
// |100⟩ : +0.000 +0.000i
// |101⟩ : +0.000 +0.000i
// |110⟩ : +0.000 +0.000i
// |111⟩ : +1.000 +0.000i
assert_eq!(q_out.bits[0b111], Comp::new(1.0, 0.0));
```
 */
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

/**
Controlled-Controlled-Controlled-...(N)-Not Gate

# Usage
```
use Qit::core::{Applicable, Comp, Qubits};
use Qit::gates::CNX;

let cnx = CNX::new(vec![0, 1, 2], 3);
// q_in = |0111⟩
let q_in = Qubits::from_num(4, 7);
let q_out = cnx.apply(q_in);
q_out.print_cmps();
// |0000⟩ : +0.000 +0.000i
// |0001⟩ : +0.000 +0.000i
//        .
//        .
//        .
// |1110⟩ : +0.000 +0.000i
// |1111⟩ : +1.000 +0.000i
assert_eq!(q_out.bits[0b1111], Comp::new(1.0, 0.0));
```
*/
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

/**
Controlled-Unitary Gate.
Control a group of arbitrary gates using a specific qubit.

```
use Qit::circuits::wrapping_qadd_const;
use Qit::core::{Applicable, Comp, Operator, Qubits};
use Qit::gates::CU;

// make controlled-add_3
let b_in = vec![0, 1, 2];
let controll_bit = 3;

let add_3 = wrapping_qadd_const(&b_in, 3);
let controlled_add_3 = CU::from_u(controll_bit, add_3);

// or
let add_3 = wrapping_qadd_const(&b_in, 3);
let add_3_gates: Vec<Box<dyn Operator>> = add_3.gates;
let controlled_add_3 = CU::new(
    controll_bit,
    add_3_gates,
    String::from("something_name_you_like"),
);

// q_in = |1001⟩
let q_in = Qubits::from_num(4, 0b1001);
let q_out = controlled_add_3.apply(q_in);
q_out.print_cmps();
// |0000⟩ : +0.000 +0.000i
//        .
//        .
//        .
// |1011⟩ : +0.000 +0.000i
// |1100⟩ : +1.000 +0.000i
// |1101⟩ : +0.000 +0.000i
// |1110⟩ : +0.000 +0.000i
// |1111⟩ : +0.000 +0.000i
```
*/
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

/**
Unitary struct. struct for applying a vector of arbitrary gates together to a qubit

# Example usage
```
use Qit::{gates::*};
// full adder
let a_in = 0;
let b_in = 1;
let c_in = 2;
let c_out = 3;
let ccx1 = CCX::new(a_in, b_in, c_out);
let cx1 = CX::new(a_in, b_in);
let ccx2 = CCX::new(b_in, c_in, c_out);
let cx2 = CX::new(c_in, b_in);
let adder = U::new(vec![Box::new(ccx1), Box::new(cx1), Box::new(ccx2), Box::new(cx2)],
        String::from("full_adder_bit"));
```
 */
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

    pub fn rename(&mut self, name: String) {
        self.label = name;
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
