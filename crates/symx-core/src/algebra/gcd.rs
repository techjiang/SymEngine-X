pub fn gcd_i64(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 { let t = a % b; a = b; b = t; }
    a
}

pub fn lcm_i64(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 { 0 } else { (a.abs() / gcd_i64(a, b)) * b.abs() }
}
