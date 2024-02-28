/*!
 Utility functions used within circuits
*/

pub fn mod_power(a: usize, exp: usize, m: usize) -> usize {
    /*!
     get (a^e mod m)
    */
    if exp == 0 {
        return 1;
    } else if exp == 1 {
        return a;
    }

    if exp % 2 == 1 {
        return (a * mod_power((a * a) % m, exp / 2, m)) % m;
    } else {
        return mod_power((a * a) % m, exp / 2, m);
    }
}

// aX + bY = c を満たす(X, Y)を求める
fn ext_gcd(a: isize, b: isize, c: isize) -> (isize, isize) {
    /*!
    A function to find an integer (X,Y) that satisfies aX + bY = c
    */
    if b == 0 {
        return (a / c, 0);
    } else {
        let r = a % b;
        let q = a / b;
        let (s, t) = ext_gcd(b, r, c);
        return (t, s - q * t);
    }
}

pub fn is_coprime(a: usize, b: usize) -> bool {
    /*!
    Returns gcd(a, b) == 1
    */
    if b == 1 {
        return true;
    } else {
        if a % b == 0 {
            return false;
        }
        return is_coprime(b, a % b);
    }
}

pub fn mod_inv(a: usize, m: usize) -> usize {
    /*!
    Returns b that satisfies a * b = 0 (mod m)
    */
    assert!(is_coprime(a, m));
    let (a, m) = (a as isize, m as isize);
    let (mut x, _) = ext_gcd(a, -m, 1);
    loop {
        if x < 1 {
            x += m;
        } else {
            break;
        }
    }
    return x as usize;
}
