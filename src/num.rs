use crate::monoid::{Monoid, Semigroup};

pub enum Add {}

macro_rules! impl_zero {
    ($($t:ty)*) => ($(
        impl Semigroup<$t> for Add
        {
            fn bin_op(left: &$t, right: &$t) -> $t {
                left + right
            }
        }
        impl Monoid< $t > for Add
        where
            $t: Clone,
        {
            fn id() -> $t {
                0
            }
        }

    )*)
}
macro_rules! impl_zero_f{
    ($($t:ty)*) => ($(
        impl Semigroup<$t> for Add
        {
            fn bin_op(left: &$t, right: &$t) -> $t {
                left + right
            }
        }
        impl Monoid<$t> for Add
        where
            $t: Clone + std::ops::Add<Output = $t>,
        {
            fn id() -> $t {
                0.0
            }
        }
    )*)
}

impl_zero!(usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128);
impl_zero_f!(f32 f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_bop() {
        let result = Add::bin_op(&2, &2);
        assert_eq!(result, 4);
        let result = Add::bin_op(&2, &0);
        assert_eq!(result, 2);
    }
    #[test]
    fn test_float_bop() {
        let result = Add::bin_op(&2.0, &2.0);
        assert_eq!(result, 4.0);
        let result = Add::bin_op(&2.0, &0.0);
        assert_eq!(result, 2.0);
    }
    #[test]
    fn test_integer_id() {
        let result: u8 = Add::id();
        assert_eq!(result, 0);
    }
    #[test]
    fn test_float_id() {
        let result: f32 = Add::id();
        const DELTA: f32 = 0.00001;
        assert!(result < DELTA && result > -DELTA);
    }
}
