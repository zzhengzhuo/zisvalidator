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
// pub mod impls;
pub trait Validate{
    fn validate(&self,) -> Result<(),error::ValidatorError>;
}

pub trait ValidateLength{
    fn len(&self) -> usize;
}


