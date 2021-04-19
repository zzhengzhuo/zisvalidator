use super::*;

macro_rules! impl_validate_range {
    ($ty:ident) => {
        impl ValidateRange for $ty {
            fn validate_range<T, R, Idx>(
                &self,
                field: T,
                range: &R,
            ) -> Result<(), error::ValidatorError>
            where
                T: std::fmt::Display,
                R: std::ops::RangeBounds<Idx> + std::fmt::Debug,
                Idx: PartialOrd<Idx> + PartialOrd<Self> + std::fmt::Debug,
                Self: PartialOrd<Idx>,
            {
                if range.contains(self) {
                    Ok(())
                } else {
                    Err(error::ValidatorError {
                        message: validator_error!(field, "range", range),
                    })
                }
            }
        }
    };
}

impl_validate_range!(bool);
impl_validate_range!(isize);
impl_validate_range!(i8);
impl_validate_range!(i32);
impl_validate_range!(i64);
impl_validate_range!(usize);
impl_validate_range!(u8);
impl_validate_range!(u32);
impl_validate_range!(u64);
impl_validate_range!(f32);
impl_validate_range!(f64);
impl_validate_range!(char);
impl_validate_range!(String);
