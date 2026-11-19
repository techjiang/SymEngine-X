use super::integer;

/// 对有理数做符号归一化与约分；分母为 0 时原样保留以便调用方处理。
pub fn reduce(n: i64, d: i64) -> (i64, i64) {
    if d == 0 { return (n, d); }
    let g = integer::gcd(n, d).max(1);
    let mut n = n / g;
    let mut d = d / g;
    if d < 0 { n = -n; d = -d; }
    (n, d)
}

/// 有理数加法。
pub fn add(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    let (an, ad) = a;
    let (bn, bd) = b;
    reduce(
        (an as i128 * bd as i128 + bn as i128 * ad as i128) as i64,
        (ad as i128 * bd as i128) as i64,
    )
}

/// 有理数乘法。
pub fn mul(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    reduce(a.0.wrapping_mul(b.0), a.1.wrapping_mul(b.1))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normalizes_sign_and_reduces() {
        assert_eq!(reduce(6, -8), (-3, 4));
        assert_eq!(add((1, 2), (1, 3)), (5, 6));
        assert_eq!(mul((2, 3), (3, 4)), (1, 2));
    }
}
