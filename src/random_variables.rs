use crate::RandomVariable;

impl RandomVariable for bool {
    fn sample_space() -> impl ExactSizeIterator<Item = Self> {
        [false, true].into_iter()
    }
}

macro_rules! random_variable_impl {
    ($t:ty) => {
        impl RandomVariable for $t {
            fn sample_space() -> impl ExactSizeIterator<Item = Self> {
                Self::MIN..=Self::MAX
            }
        }
    };
}

random_variable_impl!(u8);
random_variable_impl!(u16);

random_variable_impl!(i8);
random_variable_impl!(i16);
