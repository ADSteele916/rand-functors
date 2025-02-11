use core::ops::{Range, RangeInclusive};

use crate::RandomVariableRange;

// A generic implementation of RandomVariableRange<T> for Range<T> is impossible
// until std::iter::Step is stabilized.
macro_rules! impl_random_variable_range_for_range {
    ($t:ty) => {
        impl RandomVariableRange<$t> for Range<$t> {
            #[inline]
            fn sample_space(&self) -> impl Iterator<Item = $t> {
                self.clone()
            }
        }
    };
}

// A generic implementation of RandomVariableRange<T> for RangeInclusive<T> is
// impossible until std::iter::Step is stabilized.
macro_rules! impl_random_variable_range_for_range_inclusive {
    ($t:ty) => {
        impl RandomVariableRange<$t> for RangeInclusive<$t> {
            #[inline]
            fn sample_space(&self) -> impl Iterator<Item = $t> {
                self.clone()
            }
        }
    };
}

impl_random_variable_range_for_range!(u8);
impl_random_variable_range_for_range!(u16);
impl_random_variable_range_for_range!(u32);
impl_random_variable_range_for_range!(u64);
impl_random_variable_range_for_range!(u128);

impl_random_variable_range_for_range!(i8);
impl_random_variable_range_for_range!(i16);
impl_random_variable_range_for_range!(i32);
impl_random_variable_range_for_range!(i64);
impl_random_variable_range_for_range!(i128);

impl_random_variable_range_for_range_inclusive!(u8);
impl_random_variable_range_for_range_inclusive!(u16);
impl_random_variable_range_for_range_inclusive!(u32);
impl_random_variable_range_for_range_inclusive!(u64);
impl_random_variable_range_for_range_inclusive!(u128);

impl_random_variable_range_for_range_inclusive!(i8);
impl_random_variable_range_for_range_inclusive!(i16);
impl_random_variable_range_for_range_inclusive!(i32);
impl_random_variable_range_for_range_inclusive!(i64);
impl_random_variable_range_for_range_inclusive!(i128);
