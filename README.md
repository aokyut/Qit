[![Tests](https://github.com/aokyut/Qit/actions/workflows/rust.yml/badge.svg)](https://github.com/aokyut/Qit/actions/workflows/rust.yml)

# Qit

Simple quantum computer simulator library without matrix operations.


## Example
All gates can make changes to the qubit using the apply method.
### Usage basic gates
```rust
use Qit::core::{Applicable, Operator, Qubits};
use Qit::gates::{CX, H, U, X, Z, OperatorVec, PushOps};

// 1-Bit Gate
let h_0 = H::new(0);
// create |0⟩ Qubit
let q_in = Qubits::zeros(1);
let q_out = h_0.apply(q_in);
q_out.print_cmps();
// |0⟩ : +0.707 +0.000i
// |1⟩ : +0.707 +0.000i

// 2-Bit Gate
let cx01 = CX::new(0, 1);
// q_in = |01⟩ Qubit
let q_in = Qubits::from_num(2, 1);
let q_out = cx01.apply(q_in);
q_out.print_cmps();
// |00⟩ : +0.000 +0.000i
// |01⟩ : +0.000 +0.000i
// |10⟩ : +0.000 +0.000i
// |11⟩ : +1.000 +0.000i

// Combine gates into one unitary gate
let x = X::new(0);
let cx01 = CX::new(0, 1);
let z = Z::new(1);
let mut circ = OperatorVec::new();
circ.push_ops(x);
circ.push_ops(cx01);
circ.push_ops(z);
let u = U::new(circ, String::from("example_circ"));

let q_in = Qubits::from_num(2, 0);
let q_out = u.apply(q_in);
q_out.print_cmps();
// |00⟩ : +0.000 +0.000i
// |01⟩ : +0.000 +0.000i
// |10⟩ : -0.000 -0.000i
// |11⟩ : -1.000 -0.000i
```

### Usage prepared circuits
The circuits module implements a function that gives a circuit created using the structure of the gates module.

```rust
use Qit::circuits::wrapping_qsub_const;
use Qit::core::{Applicable, Qubits};
use Qit::gates::U;

let b = vec![0, 1, 2];
let sub_2 = wrapping_qsub_const(&b, 2);
let sub_3 = wrapping_qsub_const(&b, 3);

// combine sub_2 and sub_3
let sub_5 = U::new(
    vec![Box::new(sub_2), Box::new(sub_3)],
    String::from("sub_5"),
);
// q_in = |111⟩
let q_in = Qubits::from_num(3, 7);
let q_out = sub_5.apply(q_in);

q_out.print_cmps();
// |000⟩ : +0.000 +0.000i
// |001⟩ : +0.000 +0.000i
// |010⟩ : +1.000 +0.000i
// |011⟩ : +0.000 +0.000i
// |100⟩ : +0.000 +0.000i
// |101⟩ : +0.000 +0.000i
// |110⟩ : +0.000 +0.000i
// |111⟩ : +0.000 +0.000i
```

Current version: 0.1.3

Some additional info here

License: MIT
