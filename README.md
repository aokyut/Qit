[![Workflow Status](https://github.com/aokyut/Qit.git/workflows/Rust/badge.svg)](https://github.com/aokyut/Qit.git/actions?query=workflow%3A%22Rust%22)

# Qit

Quantum computer simulator library without matrix operations.
## Example
```rust
use Qit::core::{gates::{H}, Qubits};

let x_gate = X::new(0);

// make Qbit|000⟩
let q_in = Qubits::from_num(size: 3, number: 0);

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

Current version: 0.1.0

Some additional info here

License: MIT
