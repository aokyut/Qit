/*!
Simple quantum computer simulator library without matrix operations.
# Example
```
use Qit::core::{gates::{X, Applicable}, Qubits};

let x_gate = X::new(0);

// make Qbit|000⟩ Qubits::from_num(size: 3, num: 0)
let q_in = Qubits::from_num(3, 0);

// apply gate to Qbit
let q_out = x_gate.apply(q_in);

q_out.print_probs();
// |000⟩ : +0.000 +0.000i
// |001⟩ : +1.000 +0.000i
// |010⟩ : +0.000 +0.000i
// |011⟩ : +0.000 +0.000i
// |100⟩ : +0.000 +0.000i
// |101⟩ : +0.000 +0.000i
// |110⟩ : +0.000 +0.000i
// |111⟩ : +0.000 +0.000i


```
*/

pub mod core;
#[cfg(test)]
mod tests;
