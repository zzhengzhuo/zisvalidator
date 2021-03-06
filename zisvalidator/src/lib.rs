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

pub trait ValidateRange<F:?Sized, R,Idx:?Sized>
{
    fn validate_range(&self, field: &F, range: &R) -> Result<(), error::ValidatorError>;
}

pub trait ValidateSeqRange<'a,F:?Sized, R,Idx:?Sized>
{
    fn validate_seq_range(&'a self, field: &F, range: &R) -> Result<(), error::ValidatorError>;
}

pub trait ValidateLength<F:?Sized, R,Idx:?Sized>
{
    fn validate_length(&self, field: &F, range: &R) -> Result<(), error::ValidatorError>;
}
