use crate::RandomVariable;

impl RandomVariable for bool {
    #[inline]
    fn sample_space() -> impl Iterator<Item = Self> {
        [false, true].into_iter()
    }
}

macro_rules! impl_random_variable_for_int {
    ($t:ty) => {
        impl RandomVariable for $t {
            #[inline]
            fn sample_space() -> impl Iterator<Item = Self> {
                Self::MIN..=Self::MAX
            }
        }
    };
}

impl_random_variable_for_int!(u8);
impl_random_variable_for_int!(u16);
impl_random_variable_for_int!(u32);
impl_random_variable_for_int!(u64);
impl_random_variable_for_int!(u128);
impl_random_variable_for_int!(usize);

impl_random_variable_for_int!(i8);
impl_random_variable_for_int!(i16);
impl_random_variable_for_int!(i32);
impl_random_variable_for_int!(i64);
impl_random_variable_for_int!(i128);
impl_random_variable_for_int!(isize);
