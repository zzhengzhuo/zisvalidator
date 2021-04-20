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

#[cfg(feature = "derive")]
pub use zisvalidator_derive::Validate;

#[macro_export]
macro_rules! validator_error {
    ($field:expr,$attr:expr,$value:expr) => {{
        ::std::format!("invalid `{}`: expected {} `{:?}`", $field, $attr, $value)
    }};
}

pub trait ValidateRange<T, R, Idx> 
where
    T: std::fmt::Display,
    R: std::ops::RangeBounds<Idx> + std::fmt::Debug,
    Idx: PartialOrd<Idx> + std::fmt::Debug,
    {
    fn validate_range(&self, field: T, range: &R) -> Result<(), error::ValidatorError>;
    
}
