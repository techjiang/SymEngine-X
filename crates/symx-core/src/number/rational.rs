#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rational { pub num: i64, pub den: i64 }

impl Rational {
    pub fn new(num: i64, den: i64) -> Self {
        let g = crate::algebra::gcd::gcd_i64(num, den).max(1);
        Self { num: num / g, den: den / g }
    }
}
