use std::cmp::Ordering;

/// 比较两个整数的大小。
pub fn compare(a: i64, b: i64) -> Ordering { a.cmp(&b) }

/// 计算最大公约数，结果始终非负。
pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.max(1)
}

/// 计算最小公倍数，使用 i128 降低中间结果溢出的概率。
pub fn lcm(a: i64, b: i64) -> i128 {
    if a == 0 || b == 0 { return 0; }
    let g = gcd(a, b) as i128;
    (a as i128).abs() / g * (b as i128).abs()
}

/// 大整数阶乘无法放进 i64 时返回 None，避免静默溢出。
pub fn checked_factorial(n: u32) -> Option<i64> {
    let mut acc = 1i64;
    for k in 2..=n { acc = acc.checked_mul(k as i64)?; }
    Some(acc)
}

/// 模幂计算，底层委托给 number_theory::mod_pow。
pub fn mod_pow(base: u64, exp: u64, modulus: u64) -> u64 {
    super::number_theory::mod_pow(base, exp, modulus)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gcd_and_lcm() {
        assert_eq!(gcd(12, 18), 6);
        assert_eq!(gcd(-12, 18), 6);
        assert_eq!(lcm(4, 6), 12);
    }
    #[test]
    fn factorial_overflow_is_reported() {
        assert_eq!(checked_factorial(5), Some(120));
        assert_eq!(checked_factorial(30), None);
    }
}
