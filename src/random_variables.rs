use crate::RandomVariable;

impl RandomVariable for bool {
    fn sample_space() -> impl Iterator<Item = Self> {
        [false, true].into_iter()
    }
}

macro_rules! int_random_variable_impl {
    ($t:ty) => {
        impl RandomVariable for $t {
            fn sample_space() -> impl Iterator<Item = Self> {
                Self::MIN..=Self::MAX
            }
        }
    };
}

int_random_variable_impl!(u8);
int_random_variable_impl!(u16);

int_random_variable_impl!(i8);
int_random_variable_impl!(i16);
