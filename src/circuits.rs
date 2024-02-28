/*!
 Commonly used quantum circuits.

 You can easily use basic operations such as addition and subtraction and
 major quantum circuits such as quantum Fourier transform.
*/
use std::collections::HashSet;

use super::{
    core::{
        mod_funcs::{is_coprime, mod_inv, mod_power},
        Operator, Reversible,
    },
    gates::*,
};

use std::f64::consts::PI;

/**
Circuit that performs half addition on qubit

Required number of qubits: 1(a_in) + 1(b_in) + 1(s_out) + 1(c_out) = **4**
*/
pub fn half_adder_bit(a_in: usize, b_in: usize, s_out: usize, c_out: usize) -> U {
    let cx_a = CX::new(a_in, s_out);
    let cx_b = CX::new(b_in, s_out);
    let ccx = CCX::new(a_in, b_in, c_out);
    return U::new(
        vec![Box::new(cx_a), Box::new(cx_b), Box::new(ccx)],
        String::from("half_addr"),
    );
}

pub fn full_adder_bit(a_in: usize, b_in: usize, c_in: usize, c_out: usize) -> U {
    //! |a⟩|b⟩|C⟩|0⟩ → |a⟩|a+b+c⟩|C⟩|C_out⟩
    let ccx1 = CCX::new(a_in, b_in, c_out);
    let cx1 = CX::new(a_in, b_in);
    let ccx2 = CCX::new(b_in, c_in, c_out);
    let cx2 = CX::new(c_in, b_in);
    return U::new(
        vec![Box::new(ccx1), Box::new(cx1), Box::new(ccx2), Box::new(cx2)],
        String::from("full_adder_bit"),
    );
}

pub fn full_adder_nbits(a_in: &[usize], b_in: &[usize], c_inout: &[usize]) -> U {
    //! |a⟩|b⟩|0⟩ → |a⟩|a+b⟩|0⟩
    assert_eq!(a_in.len(), b_in.len());
    assert!(a_in.len() > 0);
    check_unique(vec![&a_in, &b_in, &c_inout]);

    let mut gate_vec: Vec<Box<dyn Operator>> = Vec::new();
    // add half adder for most right bit
    let mut a = a_in[0];
    let mut b = b_in[0];
    let mut c_in = c_inout[0];
    gate_vec.push(Box::new(CCX::new(a, b, c_in)));
    let mut c_out;
    for i in 1..a_in.len() {
        a = a_in[i];
        b = b_in[i];
        c_out = c_inout[i];
        if i == a_in.len() - 1 {
            gate_vec.push(Box::new(CX::new(a, b)));
            gate_vec.push(Box::new(CX::new(c_in, b)));
        } else {
            gate_vec.push(Box::new(CCX::new(a, b, c_out)));
            gate_vec.push(Box::new(CX::new(a, b)));
            gate_vec.push(Box::new(CCX::new(c_in, b, c_out)));
            c_in = c_out;
        }
    }

    for i in 2..a_in.len() {
        let i = a_in.len() - i;
        a = a_in[i];
        b = b_in[i];
        c_out = c_inout[i];
        c_in = c_inout[i - 1];
        let block: Vec<Box<dyn Operator>> = vec![
            Box::new(CCX::new(c_in, b, c_out)),
            Box::new(CX::new(a, b)),
            Box::new(CCX::new(a, b, c_out)),
            Box::new(CX::new(a, b)),
            Box::new(CX::new(c_in, b)),
        ];
        gate_vec.extend(block);
    }
    a = a_in[0];
    b = b_in[0];
    c_out = c_inout[0];
    gate_vec.push(Box::new(CCX::new(a, b, c_out)));
    gate_vec.push(Box::new(CX::new(a, b)));

    return U::new(gate_vec, String::from("full_adder"));
}

pub fn substract_nbits(a_in: &[usize], b_in: &[usize], c_inout: &[usize]) -> U {
    let mut sub = full_adder_nbits(a_in, b_in, c_inout);
    sub.reverse();
    return sub;
}

pub fn add_const_2_power(b: &[usize], m: usize) -> U {
    //! |0⟩|b⟩ → |overflow⟩|b + 2^m⟩
    assert!(b.len() > 0);
    assert!(b.len() - 1 > m);
    let mut u_gates: Vec<Box<dyn Operator>> = Vec::new();
    for i in (m + 1)..b.len() {
        let i = b.len() - i + m;
        let controlls: Vec<usize> = (m..i).map(|x| b[x]).collect();
        if controlls.len() == 1 {
            u_gates.push(Box::new(CX::new(controlls[0], b[i])));
        } else if controlls.len() == 2 {
            u_gates.push(Box::new(CCX::new(controlls[0], controlls[1], b[i])));
        } else {
            u_gates.push(Box::new(CNX::new(controlls, b[i])));
        }
    }
    u_gates.push(Box::new(X::new(b[m])));
    return U::new(u_gates, String::from("add_const_2^n"));
}

/**
Add the input 2 to the power of m to b and store the result in b. Store overflow results

|0⟩|b⟩ → |overflow⟩|b + 2^m⟩

* b: Index of input qubits.
* overflow: Qubit index to set bit in case of overflow
* m: exponent of power of 2 to add

Required number of qubits: n(b) + 1(overflow) = **n + 1**
*/
pub fn overflow_qadd_const_2_power(b: &[usize], overflow: usize, m: usize) -> U {
    assert!(b.len() > 0);
    assert!(b.len() > m);
    check_unique(vec![&b, &vec![overflow]]);
    let b = &[b, &vec![overflow]].concat();
    let mut u_gates: Vec<Box<dyn Operator>> = Vec::new();
    for i in (m + 1)..b.len() {
        let i = b.len() - i + m;
        let controlls: Vec<usize> = (m..i).map(|x| b[x]).collect();
        if controlls.len() == 1 {
            u_gates.push(Box::new(CX::new(controlls[0], b[i])));
        } else if controlls.len() == 2 {
            u_gates.push(Box::new(CCX::new(controlls[0], controlls[1], b[i])));
        } else {
            u_gates.push(Box::new(CNX::new(controlls, b[i])));
        }
    }
    u_gates.push(Box::new(X::new(b[m])));
    return U::new(u_gates, String::from("o_qadd_const_2^n"));
}

// wrapping_qadd_const_2_power
/**
Wrapping add the input 2 to the power of m to b and store the result in b.
|b⟩ → |b + 2^m⟩

* b: Index of input qubits.
* m: exponent of power of 2 to add

Required number of qubits: n(b) = **n**
*/
pub fn wrapping_qadd_const_2_power(b: &[usize], m: usize) -> U {
    assert!(b.len() > 0);
    assert!(b.len() > m);
    check_unique(vec![&b]);
    let mut u_gates: Vec<Box<dyn Operator>> = Vec::new();
    for i in (m + 1)..b.len() {
        let i = b.len() - i + m;
        let controlls: Vec<usize> = (m..i).map(|x| b[x]).collect();
        if controlls.len() == 1 {
            u_gates.push(Box::new(CX::new(controlls[0], b[i])));
        } else if controlls.len() == 2 {
            u_gates.push(Box::new(CCX::new(controlls[0], controlls[1], b[i])));
        } else {
            u_gates.push(Box::new(CNX::new(controlls, b[i])));
        }
    }
    u_gates.push(Box::new(X::new(b[m])));
    return U::new(u_gates, String::from("w_qadd_const_2^n"));
}

/**
A quantum circuit that directly overwrites the input qubit with the result of adding the value represented by the qubit and a constant.

|0⟩|b⟩ → |overflow⟩|b + a⟩

*/
pub fn add_const(b: &[usize], a_const: usize) -> U {
    assert!(b.len() > 1);
    assert!((a_const >> (b.len() - 1)) == 0);
    check_unique(vec![b]);

    let mut u_gates: Vec<Box<dyn Operator>> = Vec::new();
    for i in 0..(b.len() - 1) {
        if (a_const >> i) & 1 == 1 {
            // let adder = add_const_2_power(b, i);
            // println!("*******************start");
            // println!("{}", adder.name());
            // println!("*******************end");
            u_gates.extend(add_const_2_power(b, i).gates);
        }
    }

    return U::new(u_gates, String::from("add_const"));
}

// overflow_qadd_const
/**
Add const_a to b and store the result in b. Store overflow results

|0⟩|b⟩ → |overflow⟩|b + a_const⟩

* b: Index of input qubits.
* overflow: Qubit index to set bit in case of overflow
* a_const: constant number to add

Required number of qubits: n(b) + 1(overflow) = **n + 1**
 */
pub fn overflow_qadd_const(b: &[usize], overflow: usize, a_const: usize) -> U {
    assert!(b.len() > 0);
    assert!((a_const >> (b.len())) == 0);

    check_unique(vec![&b, &vec![overflow]]);

    let mut u_gates: Vec<Box<dyn Operator>> = Vec::new();
    for i in 0..(b.len()) {
        if (a_const >> i) & 1 == 1 {
            u_gates.extend(overflow_qadd_const_2_power(b, overflow, i).gates);
        }
    }

    return U::new(u_gates, String::from("o_qadd_const"));
}

// wrapping_qadd_const
/**
Wrapping add const_a to b and store the result in b.

|b⟩ → |b + a_const⟩

* b: Index of input qubits.
* a_const: constant number to add

Required number of qubits: n(b) = **n**
 */
pub fn wrapping_qadd_const(b: &[usize], a_const: usize) -> U {
    assert!(b.len() > 0);
    assert!((a_const >> (b.len())) == 0);

    check_unique(vec![&b]);

    let mut u_gates: Vec<Box<dyn Operator>> = Vec::new();
    for i in 0..(b.len()) {
        if (a_const >> i) & 1 == 1 {
            u_gates.extend(wrapping_qadd_const_2_power(b, i).gates);
        }
    }

    return U::new(u_gates, String::from("w_qadd_const"));
}

pub fn sub_const(b: &[usize], a_const: usize) -> U {
    //! |0⟩|b⟩|0⟩ → |sign⟩|b + a⟩|0⟩
    let mut sub = add_const(b, a_const);
    sub.reverse();
    return U::new(sub.gates, String::from("sub_const"));
}

/**
Substract const_a to b and store the result in b. Store overflow results

|0⟩|b⟩ → |overflow⟩|b - a_const⟩

* b: Index of input qubits.
* overflow: Qubit index to set bit in case of overflow
* a_const: constant number to sub

Required number of qubits: n(b) + 1(overflow) = **n + 1**
 */
pub fn overflow_qsub_const(b: &[usize], overflow: usize, a_const: usize) -> U {
    let mut sub = overflow_qadd_const(b, overflow, a_const);
    sub.reverse();
    sub.rename(String::from("o_qsub_const"));
    return sub;
}

/**

Wrapping substract const_a to b and store the result in b.

|b⟩ → |b - a_const⟩

* b: Index of input qubits.
* a_const: constant number to sub

Required number of qubits: n(b) = **n**
 */
pub fn wrapping_qsub_const(b: &[usize], a_const: usize) -> U {
    let mut sub = wrapping_qadd_const(b, a_const);
    sub.reverse();
    sub.rename(String::from("w_qsub_const"));
    return sub;
}

/**
Swap Q-bit between index of a_in and b_in

Required number of qubits: n(a_in) + n(b_in) = **2**
 */
pub fn swap(a_in: &[usize], b_in: &[usize]) -> U {
    assert_eq!(a_in.len(), b_in.len());
    check_unique(vec![&a_in, &b_in]);

    let mut u_gates: Vec<Box<dyn Operator>> = Vec::new();
    for i in 0..a_in.len() {
        let a = a_in[i];
        let b = b_in[i];
        u_gates.push(Box::new(CX::new(a, b)));
        u_gates.push(Box::new(CX::new(b, a)));
        u_gates.push(Box::new(CX::new(a, b)));
    }

    return U::new(u_gates, String::from("swap"));
}

/**
A quantum circuit that stores the result of addition and modular operation in one input bit.

|a⟩|b⟩|N⟩|0⟩ → |a⟩|a+b mod N⟩|N⟩|0⟩
*/
pub fn mod_add(
    a: &[usize],
    b: &[usize],
    n_in: &[usize],
    zero: &[usize],
    t: usize,
    num: usize,
) -> U {
    assert_eq!(a.len(), b.len());
    assert_eq!(n_in.len(), b.len());
    assert_eq!(zero.len(), b.len());
    assert!(num >> (n_in.len() - 1) == 0);
    check_unique(vec![&a, &b, &n_in, &zero, &vec![t]]);

    let mut u_gates: Vec<Box<dyn Operator>> = Vec::new();

    // (1)[add] |a⟩|b⟩ -> |a⟩|a+b⟩
    u_gates.extend(full_adder_nbits(a, b, zero).gates);
    // (2)[sub] |a+b⟩|N⟩ -> |a+b-N⟩|N⟩
    u_gates.extend(substract_nbits(n_in, b, zero).gates);

    // (3)[flag] |0⟩ ->  |0⟩ (if a + b < N), |1⟩ (if a + b >= N)
    let b_max = &b[b.len() - 1];
    u_gates.push(Box::new(X::new(*b_max)));
    u_gates.push(Box::new(CX::new(*b_max, t)));
    u_gates.push(Box::new(X::new(*b_max)));

    // (4)[arrow] |N⟩ -> |0⟩ (if a + b < N), |N⟩ (if a + b >= N)
    for idx in 0..n_in.len() {
        if (num >> idx) & 1 == 1 {
            u_gates.push(Box::new(CX::new(t, n_in[idx])));
        }
    }

    // (5)[add] |a+b-N⟩|0 or N⟩ -> |a+b or a+b-N⟩|0 or N⟩
    u_gates.extend(full_adder_nbits(n_in, b, zero).gates);

    // (6)[arrow] |0 or N⟩ -> |N⟩
    for idx in 0..n_in.len() {
        if (num >> idx) & 1 == 1 {
            u_gates.push(Box::new(CX::new(t, n_in[idx])));
        }
    }

    // (7)[sub] |a⟩|a+b or a+b-N⟩ -> |a⟩|b or b-N⟩
    u_gates.extend(substract_nbits(a, b, zero).gates);

    // (8)[unflag] t|0 or 1⟩ -> |0⟩
    u_gates.push(Box::new(CX::new(*b_max, t)));

    // (9)[add] |a⟩|b or b-N⟩ -> |a⟩|a+b or a+b-N⟩
    u_gates.extend(full_adder_nbits(a, b, zero).gates);

    return U::new(u_gates, String::from("moduler_adder"));
}

/**
A quantum circuit that adds constants and multiplies modular operations using constants, and stores the result in one input bit.

|b⟩|overflow:0⟩ → |a+b mod N⟩|0⟩

* overflow = |0⟩.
* a, N: const.
* N < 2^(n+1).
* a, b < N.
* b.len() == n.

Required number of qubits: n(b) + 1(overflow) = **n + 1**
*/
pub fn mod_add_const(b: &[usize], overflow: usize, a_const: usize, n_const: usize) -> U {
    assert!(b.len() > 0);
    check_unique(vec![&b, &vec![overflow]]);
    let mut u_gates: Vec<Box<dyn Operator>> = Vec::new();

    // (1)[add_a] |b⟩ -> |a+b⟩
    u_gates.extend(overflow_qadd_const(b, overflow, a_const).gates);

    // (2)[sub_n] |a+b⟩ -> |a+b-N⟩
    u_gates.extend(overflow_qsub_const(b, overflow, n_const).gates);

    // [ovrflow] |0⟩ -> |0⟩ (a+b >= N), (0<= a+b - N < N)
    //               -> |1⟩ (a+b <  N), (a+b - N < 0, overflowed)

    // (4)[cont_add_N] |a+b-N⟩ -> |a+b-N⟩ (a+b >= N)
    //                         -> |a+b⟩   (a+b <  N)
    let add_n = wrapping_qadd_const(b, n_const);
    let const_add_n = CU::new(overflow, add_n.gates, String::from("cu-add_N"));
    u_gates.push(Box::new(const_add_n));

    // (5)[sub_a] |a+b-N or a+b⟩ -> |b-N or b⟩
    u_gates.extend(overflow_qsub_const(b, overflow, a_const).gates);

    // (6)[unflag] |0 or 1⟩ -> |0⟩
    u_gates.push(Box::new(X::new(overflow)));

    // (7)[add_a] |b-N or b⟩ -> |a+b-N or a+b⟩
    u_gates.extend(wrapping_qadd_const(b, a_const).gates);

    return U::new(u_gates, String::from("mod_add_const"));
}

/**
A circuit that multiplies input qubits by a constant and stores the result of modular operation using the constant in the input qubit.

* |x⟩|0⟩|overflow: 0⟩|cont⟩ → |x⟩|ax mod N, or x⟩|0⟩|cont⟩
* overflow = |0⟩
* a, N: const. N < 2^(n+1). a, x < N
* tar_reg.len() == n
* x.len() == n

Required number of qubits: n(x) + n(tar_reg) + 1(overflow) + 1(cont) = **2n + 2**
*/
pub fn cmm_const(
    x: &[usize],
    tar_reg: &[usize],
    overflow: usize,
    cont: usize,
    a_const: usize,
    n_const: usize,
) -> U {
    assert!(tar_reg.len() == x.len());
    assert!(a_const < (1 << x.len()));
    assert!(n_const < (1 << x.len()));
    check_unique(vec![x, tar_reg, &vec![cont, overflow]]);

    let mut u_gates: Vec<Box<dyn Operator>> = Vec::new();

    let mut mul: Vec<Box<dyn Operator>> = Vec::new();

    for i in 0..x.len() {
        let adder = mod_add_const(tar_reg, overflow, (a_const << i) % n_const, n_const);
        mul.push(Box::new(CU::from_u(x[i], adder)));
    }

    u_gates.push(Box::new(CU::new(cont, mul, String::from("cu-mmul"))));

    u_gates.push(Box::new(X::new(cont)));

    for i in 0..x.len() {
        u_gates.push(Box::new(CCX::new(cont, x[i], tar_reg[i])));
    }

    u_gates.push(Box::new(X::new(cont)));

    return U::new(u_gates, String::from("cmm_const"));
}

/**
A circuit that outputs the result of modular operation with n_const after raising a_const to the power of the input qubit.

|x⟩|1⟩|0⟩ → |x⟩|a^x mod N⟩|0⟩

* a_x: n-bit
* zero: n-bit
* x: m-bit

Required number of qubits: n(a_x) + n(zero) + m(x) + 1(overflow) = **2n + m + 1**.
 */
pub fn me_const(
    x: &[usize],
    a_x: &[usize],
    zero: &[usize],
    overflow: usize,
    a_const: usize,
    n_const: usize,
) -> U {
    assert!(zero.len() == a_x.len());
    assert!(a_x.len() >= 1);
    assert!(is_coprime(a_const, n_const));
    check_unique(vec![&x, &a_x, &vec![overflow]]);
    let mut u_gates: Vec<Box<dyn Operator>> = Vec::new();

    // a^x |0⟩ -> |1⟩
    u_gates.push(Box::new(X::new(a_x[0])));

    for i in 0..x.len() {
        let x_i = x[i];
        let const_a_xi = mod_power(a_const, 1 << i, n_const);
        let _const_a_xi = mod_inv(const_a_xi, n_const);
        //[cmm] |x⟩|0⟩ -> |x⟩|0 + x * a^2^x_n mod N⟩
        u_gates.extend(cmm_const(a_x, zero, overflow, x_i, const_a_xi, n_const).gates);
        u_gates.extend(swap(a_x, zero).gates);
        //[icmm] |x⟩|x * a^2^x_n mod N⟩ -> |x - x * a^2^x_n * a^(-2^x_n)⟩|x * a^2^x_n mod N⟩
        //                              -> |0⟩|x * a^2^x_n mod N⟩
        let mut icmm = cmm_const(a_x, zero, overflow, x_i, _const_a_xi, n_const);
        icmm.reverse();
        u_gates.extend(icmm.gates);
    }

    return U::new(u_gates, String::from("me_const"));
}

/**
Circuit that performs quantum Fourier transform

|j⟩ → exp(i2πkj / 2^n)|k⟩
*/
pub fn qft(x: &[usize]) -> U {
    let n = x.len();
    let mut u_gates: Vec<Box<dyn Operator>> = Vec::new();

    let (a, b): (Vec<usize>, Vec<usize>) = (
        (0..(n / 2)).map(|i| x[i]).collect::<Vec<usize>>(),
        (0..(n / 2)).map(|i| x[n - i - 1]).collect::<Vec<usize>>(),
    );

    let sw = swap(&a, &b);
    u_gates.extend(sw.gates);

    for i in 0..n {
        // hadamard
        u_gates.push(Box::new(H::new(x[i])));
        for j in (i + 1)..n {
            let angle = (-((j + 1 - i) as f64)).exp2();
            let r = R::new(x[i], 2.0 * PI * angle);
            u_gates.push(Box::new(CU::new(
                x[j],
                vec![Box::new(r)],
                format!("r_+2^-{}", j + 1 - i),
            )));
        }
    }

    return U::new(u_gates, String::from("qft"));
}

/**
Circuit that performs inverse quantum Fourier transform

Σexp(i2πkj / 2^n)|k⟩ → |j⟩
*/
pub fn inv_qft(x: &[usize]) -> U {
    let n = x.len();
    let mut u_gates: Vec<Box<dyn Operator>> = Vec::new();
    let (a, b): (Vec<usize>, Vec<usize>) = (
        (0..(n / 2)).map(|i| x[i]).collect::<Vec<usize>>(),
        (0..(n / 2)).map(|i| x[n - i - 1]).collect::<Vec<usize>>(),
    );

    let sw = swap(&a, &b);
    u_gates.extend(sw.gates);

    for i in 0..n {
        // hadamard
        u_gates.push(Box::new(H::new(x[i])));
        for j in (i + 1)..n {
            let angle = 1.0 - (-((j + 1 - i) as f64)).exp2();
            let r = R::new(x[i], 2.0 * PI * angle);
            u_gates.push(Box::new(CU::new(
                x[j],
                vec![Box::new(r)],
                format!("r_-2^-{}", j + 1 - i),
            )));
        }
    }

    let mut u = U::new(u_gates, String::from("iqft"));
    u.reverse();

    return u;
}

fn check_unique(vecs: Vec<&[usize]>) {
    let mut set: HashSet<usize> = HashSet::new();
    for v in vecs.iter() {
        for idx in v.iter() {
            assert!(!set.contains(idx));
            set.insert(*idx);
        }
    }
    return;
}
