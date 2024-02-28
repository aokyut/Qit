use std::f64::consts::PI;

use super::core::{
    gates::{Applicable, Operator},
    Comp, Qubits,
};

#[test]
fn test_complex() {
    // abs
    assert_eq!(Comp::new(2.0, -1.0).abs_square(), 5.0);
    // add
    assert_eq!(
        Comp::new(2.0, -3.0) + Comp::new(1.2, 5.0),
        Comp::new(3.2, 2.0)
    );
    // mul
    assert_eq!(
        Comp::new(1.0, -1.0) * Comp::new(1.0, 1.0),
        Comp::new(2.0, 0.0)
    );
}

fn zero() -> Qubits {
    return Qubits::zeros(2);
}

#[test]
fn test_qubits() {}

#[test]
fn test_hadamard() {
    use super::core::gates::H;
    let h0 = H::new(0);
    let h1 = H::new(1);
    let q = h1.apply(h0.apply(zero()));
    assert!(isequal_probs(q.probs(), vec![0.25, 0.25, 0.25, 0.25]));
    let q = h1.apply(h0.apply(q));
    assert!(isequal_probs(q.probs(), vec![1.0, 0.0, 0.0, 0.0]));

    q.print_probs();
}

#[test]
fn test_r() {
    use super::core::gates::{R, Z};
    let r = R::new(0, PI);
    let z = Z::new(0);
    let q0 = r.apply(Qubits::from_num(2, 1));
    let q1 = z.apply(Qubits::from_num(2, 1));
    assert!(isequal_qubits(&q0, &q1));
}

#[test]
fn test_cx() {
    use super::core::gates::CX;
    let q = Qubits::from_num(5, 31);
    let cx = CX::new(0, 4);
    let q = cx.apply(q);
    assert!(isequal_qubits(&q, &Qubits::from_num(5, 15)));

    q.print_probs();
}

#[test]
fn test_cu() {
    use super::core::gates::{CU, CX, X};
    let cx = CX::new(0, 1);
    let x = X::new(1);
    let cu = CU::new(0, vec![Box::new(x)], String::from("test_cu"));
    let inputs = vec![
        Qubits::from_num(2, 0),
        Qubits::from_num(2, 1),
        Qubits::from_num(2, 2),
        Qubits::from_num(2, 3),
    ];
    for q in inputs {
        let q1 = q.clone();
        isequal_qubits(&cx.apply(q1), &cu.apply(q));
    }
}

#[test]
fn test_ccx() {
    use super::core::gates::CCX;
    let ccx = CCX::new(1, 2, 0);
    let inpts = vec![
        (Qubits::from_num(3, 0), Qubits::from_num(3, 0)),
        (Qubits::from_num(3, 1), Qubits::from_num(3, 1)),
        (Qubits::from_num(3, 2), Qubits::from_num(3, 2)),
        (Qubits::from_num(3, 3), Qubits::from_num(3, 3)),
        (Qubits::from_num(3, 4), Qubits::from_num(3, 4)),
        (Qubits::from_num(3, 5), Qubits::from_num(3, 5)),
        (Qubits::from_num(3, 6), Qubits::from_num(3, 7)),
        (Qubits::from_num(3, 7), Qubits::from_num(3, 6)),
    ];

    for (q, expected) in inpts {
        isequal_qubits(&ccx.apply(q), &expected);
    }
}

#[test]
fn test_half_adder() {
    use super::core::circuits::half_adder_bit;
    let u = half_adder_bit(0, 1, 2, 3);
    for num in 0..4 {
        let q_in = Qubits::from_num(4, num);
        let q_out = &u.apply(q_in);
        let add = ((num >> 1) & 1) + ((num >> 0) & 1);
        let num = num | (add << 2);
        let q_expected = Qubits::from_num(4, num);
        isequal_qubits(&q_out, &q_expected);
    }
}

#[test]
fn test_full_adder() {
    use super::core::circuits::full_adder_nbits;
    let u = full_adder_nbits(&vec![3, 4, 5], &vec![0, 1, 2], &vec![6, 7, 8]);
    for num in 0..64 {
        let q_in = Qubits::from_num(9, num);
        let q_out = u.apply(q_in);
        let add = (((num >> 3) & 7) + (num & 7)) & 7;
        let exp_num = (num & 0b111111000) | add;
        let q_expected = Qubits::from_num(9, exp_num);
        println!("input:{num:b}, expected:{exp_num:b}");
        isequal_qubits(&q_out, &q_expected);
    }
}

#[test]
fn test_full_adder10() {
    use super::core::circuits::full_adder_nbits;
    let u = full_adder_nbits(
        &vec![5, 6, 7, 8, 9],
        &vec![0, 1, 2, 3, 4],
        &vec![10, 11, 12, 13, 14],
    );
    for num in 0..1024 {
        let q_in = Qubits::from_num(15, num);
        let q_out = u.apply(q_in);
        let add = (((num >> 5) & 31) + (num & 31)) & 31;
        let expected_num = (num & 0b11111_1111100000) | add;
        let q_expected = Qubits::from_num(15, expected_num);
        let actual = q_out.pop_most_plausible();
        assert_eq!(actual, expected_num);
        isequal_qubits(&q_out, &q_expected);
    }
}

#[test]
fn test_full_sub() {
    use super::core::circuits::substract_nbits;
    let u = substract_nbits(&vec![3, 4, 5], &vec![0, 1, 2], &vec![6, 7, 8]);
    println!("{}", u.name());
    for num in 0..64 {
        let q_in = Qubits::from_num(9, num);
        let q_out = u.apply(q_in);
        let a_in = (num >> 3) & 7;
        let b_in = num & 7;
        let out = q_out.pop_most_plausible();
        let a_out = (out >> 3) & 7;
        let b_out = out & 7;
        assert_eq!(a_in, a_out);
        println!("{:>03b}-{:>03b}={:>03b}", b_in, a_in, b_out);
    }
}

#[test]
fn test_swap() {
    use super::core::circuits::swap;
    let u = swap(&vec![0, 1, 2], &vec![3, 4, 5]);
    println!("{}", u.name());
    for num in 0..64 {
        let q_in = Qubits::from_num(6, num);
        let a_in = (num >> 3) & 7;
        let b_in = num & 7;
        let q_expected = Qubits::from_num(6, (b_in << 3) + a_in);
        let q_out = u.apply(q_in);
        println!("input:{:>06b}, expected:{:>06b}", num, (b_in << 3) + a_in);
        println!("actual:{:>06b}", q_expected.pop_most_plausible());
        isequal_qubits(&q_out, &q_expected);
    }
}

#[test]
fn test_moduler_adder() {
    use super::core::circuits::mod_add;
    let u = mod_add(
        &vec![0, 1, 2, 3],
        &vec![4, 5, 6, 7],
        &vec![8, 9, 10, 11],
        &vec![12, 13, 14, 15],
        16,
        7,
    );
    for a in 0..7 {
        for b in 0..7 {
            let num_in = a | (b << 4) | (7 << 8);
            let q_in = Qubits::from_num(17, num_in);
            let b_out = (a + b) % 7;
            let num_out = a | (b_out << 4) | (7 << 8);
            println!("input: {:>017b}, expected: {:>017b}", num_in, num_out);
            println!("{} % {} = {}", a, b, b_out);
            // let expected = Qubits::from_num(17, num_out);
            let q_out = u.apply(q_in);
            assert_eq!(q_out.pop_most_plausible(), num_out);
        }
    }
}

#[test]
fn test_add_const() {
    use super::core::circuits::add_const;
    use super::core::circuits::{overflow_qadd_const, wrapping_qadd_const};
    // let u = add_const(vec![0, 1, 2, 3])
    for a in 0..8 {
        let u = add_const(&vec![0, 1, 2, 3], a);
        let u1 = wrapping_qadd_const(&vec![0, 1, 2, 3], a);
        let u2 = overflow_qadd_const(&vec![0, 1, 2], 3, a);
        println!("u1:{}", u1.name());
        println!("u2:{}", u2.name());
        for b in 0..8 {
            let q_in = Qubits::from_num(4, b);
            let b_out = a + b;
            println!("input: {:>07b}, expected: {:>07b}", b, b_out);
            println!("{:b} + {:b} = {:b}", a, b, b_out);
            let q_out = u.apply(q_in);
            assert_eq!(q_out.pop_most_plausible(), b_out);

            let q_in = Qubits::from_num(4, b);
            let q_out2 = u1.apply(q_in);
            let q_in = Qubits::from_num(4, b);
            let q_out3 = u2.apply(q_in);
            isequal_qubits(&q_out, &q_out2);
            isequal_qubits(&q_out, &q_out3);
        }
    }
}

#[test]
fn test_sub_const() {
    use super::core::circuits::sub_const;
    // let u = add_const(vec![0, 1, 2, 3])
    for a in 0..8 {
        let u = sub_const(&vec![0, 1, 2, 3], a);
        for b in 0..8 {
            let q_in = Qubits::from_num(4, b);
            // let b_out = (a + b);
            let q_out = u.apply(q_in);
            let b_out = (b + (!a & 15) + 1) & 15;
            println!(
                "{:>03b} - {:>03b} = {:>04b}[{}]",
                b,
                a,
                q_out.pop_most_plausible(),
                b < a
            );
            assert_eq!(q_out.pop_most_plausible(), b_out);
        }
    }
}

#[test]
fn test_mod_add_const() {
    use super::core::circuits::mod_add_const;
    // let u = add_const(vec![0, 1, 2, 3])
    let n = 7;
    for a in 0..8 {
        let u = mod_add_const(&vec![0, 1, 2], 3, a, n);
        for b in 0..6 {
            let q_in = Qubits::from_num(5, b);
            // let b_out = (a + b);
            let q_out = u.apply(q_in);
            let b_out = (a + b) % n;
            let actual = q_out.pop_most_plausible();
            println!(
                "[a={:>03b},b={:>03b}]{:>04b} % {:>03b} = {:>04b}={:>04b}[{}]",
                a,
                b,
                a + b,
                n,
                actual,
                b_out,
                actual == b_out
            );
            assert_eq!(q_out.pop_most_plausible(), b_out);
        }
    }
}

#[test]
fn test_add_const_2_power() {
    use super::core::circuits::add_const_2_power;

    for a in 0..4 {
        let u = add_const_2_power(&vec![0, 1, 2, 3, 4], a);
        println!("{}", u.name());
        for b in 0..16 {
            let q_in = Qubits::from_num(5, b);
            let b_out = b + (1 << a);
            let q_out = u.apply(q_in);
            let actual = q_out.pop_most_plausible();
            println!("{:>05b}+{:>05b}={:>05b}[{:>05b}]", 1 << a, b, b_out, actual);
            assert_eq!(actual, b_out);
        }
    }
}

#[test]
fn test_cmm_const() {
    use super::core::circuits::cmm_const;
    let n = 15;
    for a in 0..n {
        let u = cmm_const(&vec![0, 1, 2, 3], &vec![4, 5, 6, 7], 8, 9, a, n);
        for b in 0..n {
            let q_in = Qubits::from_num(10, b | 1 << 9);
            let q_out = u.apply(q_in);
            let actual = q_out.pop_most_plausible();
            // println!(
            //     "{:>04b}*{:>04b} == {:>05b} mod {}[{}]",
            //     a,
            //     b,
            //     (actual >> 4) & 0b11111,
            //     n,
            //     (a * b) % n == (actual >> 4) & 0b11111,
            // );
            assert_eq!((a * b) % n, (actual >> 4) & 0b11111)
        }
    }
}

#[test]
fn test_me_const() {
    use super::core::circuits::{cmm_const, me_const, swap};
    use super::core::gates::X;
    use super::core::mod_funcs::{is_coprime, mod_inv, mod_power};
    let n = 15;
    for a in 2..n {
        if !is_coprime(a, n) {
            continue;
        }
        let u = me_const(
            &vec![0, 1, 2, 3],
            &vec![4, 5, 6, 7],
            &vec![8, 9, 10, 11],
            12,
            a,
            n,
        );
        for x in 1..8 {
            let q_in = Qubits::from_num(13, x);
            let q_out3 = u.apply(q_in);
            let actual = q_out3.pop_most_plausible();
            let actual = (actual >> 4) & 0b1111;
            assert!(mod_power(a, x, n) == actual);
        }
    }
}

#[test]
fn test_qft() {
    use super::core::circuits::qft;
    use super::core::gates::{H, X};
    let u = qft(&vec![0, 1, 2, 3]);
    let mut q_in = Qubits::from_num(4, 0);
    let q_out = u.apply(q_in);
    let bits = vec![Comp::new(0.25, 0.0); (1 << 4)];
    let expected = Qubits::from_bits(4, bits);
    // q_out.print_cmps();
    isequal_qubits(&q_out, &expected);
}

#[test]
fn test_iqft() {
    use super::core::circuits::{inv_qft, qft};
    use super::core::gates::H;

    let u = qft(&vec![0, 1, 2, 3]);
    let u2 = inv_qft(&vec![0, 1, 2, 3]);
    let q_in = H::new(0).apply(Qubits::from_num(4, 2));
    let expected = H::new(0).apply(Qubits::from_num(4, 2));
    let q_out = u.apply(q_in);
    let q_out = u2.apply(q_out);

    println!("{}", u.name());
    println!("{}", u2.name());

    expected.print_cmps();
    q_out.print_cmps();

    isequal_qubits(&expected, &q_out);
}

#[test]
fn test_phase_estimation() {
    use super::core::circuits::{inv_qft, swap};
    use super::core::gates::H;
    use super::core::gates::{CU, R, U};

    let x = vec![0, 1, 2, 3];
    fn tar_u() -> U {
        let r = R::new(4, PI * 2.0 * 0.125);
        return U::new(vec![Box::new(r)], String::from("u"));
    }

    let mut pe_gates: Vec<Box<dyn Operator>> = Vec::new();
    for i in 0..x.len() {
        pe_gates.push(Box::new(H::new(x[i])));
    }

    for i in 0..x.len() {
        println!("x[{}]", i);
        let x_i = x[i];
        let mut u_i_gates: Vec<Box<dyn Operator>> = Vec::new();
        for _ in 0..(1 << i) {
            u_i_gates.push(Box::new(tar_u()));
        }
        pe_gates.push(Box::new(CU::new(x_i, u_i_gates, format!("u^2^{i}"))));
    }

    let iqft = inv_qft(&x);
    pe_gates.push(Box::new(iqft));
    let pe = U::new(pe_gates, String::from("phase_estimation"));

    let q_in = Qubits::from_num(5, 16);
    let q_out = pe.apply(q_in);

    q_out.print_cmps();

    assert_eq!(q_out.pop_most_plausible(), (1 << 4) | (1 << 1));
}

fn isequal_qubits(a: &Qubits, b: &Qubits) -> bool {
    assert_eq!(a.size, b.size);
    for i in 0..(1 << a.size) {
        assert!(
            isequal_comp(&a.bits[i], &b.bits[i]),
            "a[{}]={:#?} and b[{}]={:#?} is not equal",
            i,
            a.bits[i],
            i,
            b.bits[i]
        );
    }

    return true;
}

fn isequal_comp(a: &Comp, b: &Comp) -> bool {
    return isequal_f64(a.0, b.0) & isequal_f64(a.1, b.1);
}

fn isequal_probs(a: Vec<f64>, b: Vec<f64>) -> bool {
    assert_eq!(a.len(), b.len(), "size not match");
    for i in 0..a.len() {
        assert!(isequal_f64(a[i], b[i]), "a[{}] and b[{}] is not same", i, i);
    }

    return true;
}

fn isequal_f64(a: f64, b: f64) -> bool {
    return (a - b).abs() < 1e-9;
}
