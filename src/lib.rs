/*!
Simple quantum computer simulator library without matrix operations.


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

pub mod circuits;
pub mod core;
pub mod gates;
#[cfg(test)]
mod tests;
