pub fn factorize(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    while n % 2 == 0 { factors.push(2); n /= 2; }
    let mut d = 3;
    while d * d <= n {
        while n % d == 0 { factors.push(d); n /= d; }
        d += 2;
    }
    if n > 1 { factors.push(n); }
    factors
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    for p in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % p == 0 { return n == p; }
    }
    let mut d = n - 1;
    let mut s = 0u32;
    while d % 2 == 0 { d /= 2; s += 1; }
    for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 { continue; }
        let mut witness = true;
        for _ in 1..s {
            x = x.saturating_mul(x) % n;
            if x == n - 1 { witness = false; break; }
        }
        if witness { return false; }
    }
    true
}

pub fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 0 { return 0; }
    let mut result = 1 % modulus;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 { result = result.saturating_mul(base) % modulus; }
        base = base.saturating_mul(base) % modulus;
        exp /= 2;
    }
    result
}

pub fn mod_inverse(a: u64, modulus: u64) -> Option<u64> {
    if modulus <= 1 { return None; }
    let (mut old_r, mut r) = (a as i128 % modulus as i128, modulus as i128);
    let (mut old_s, mut s) = (1i128, 0i128);
    while r != 0 {
        let q = old_r / r;
        let next_r = old_r - q * r; old_r = r; r = next_r;
        let next_s = old_s - q * s; old_s = s; s = next_s;
    }
    if old_r != 1 { return None; }
    Some((old_s.rem_euclid(modulus as i128)) as u64)
}
