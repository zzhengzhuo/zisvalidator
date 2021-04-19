//! # Zisvalidator
//!
//! Zisvalidator is designed for validating input struct or enum in web or other scence.
//!
//! Reference to [Serde] and [Validator].
//!
//! [Serde]: https://docs.serde.rs/serde/index.html
//! [Validator]: https://github.com/Keats/validator
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!

pub mod error;
pub mod impls;
pub trait Validate {
    fn validate(&self) -> Result<(), error::ValidatorError>;
}

use std::ops::Range;

#[cfg(feature = "derive")]
pub use zisvalidator_derive::Validate;

#[macro_export]
macro_rules! validator_error {
    ($field:expr,$attr:expr,$value:expr) => {{
        ::std::format!("invalid `{}`: expected {} `{:?}`", $field, $attr, $value)
    }};
}

pub trait ValidateRange {
    fn validate_range<T, R, Idx>(&self, field: T, range: &R) -> Result<(), error::ValidatorError>
    where
        T: std::fmt::Display,
        R: std::ops::RangeBounds<Idx> + std::fmt::Debug,
        Idx: PartialOrd<Idx> + PartialOrd<Self> + std::fmt::Debug,
        Self: PartialOrd<Idx>;
}
