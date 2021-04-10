//! # Zisvalidator Derive
//!
//! Provide Zisvalidator derive macro
//! 
//! ```rust
//! use zisvalidator_derive::Validate;
//! #[derive(Validate)]
//! struct S;
//! fn main{}
//! ```
//! 
//! # Validate for Struct and Enum
//! ```rust
//! #[derive(Validate)]
//! struct S(String);               //validate tuple struct with 1 element
//!
//! #[derive(Validate)]
//! struct S(String,u64,);          //validate tuple struct with mulitple elements
//!
//! #[derive(Validate)]
//! struct S{                       //validate struct with fields
//!     str:String 
//! }
//! 
//! #[derive(Validate)]
//! enum E{                         //validate enum
//!     S(String),                  //validate tuple varient with 1 element
//!     tuple(String,String,),      //validate tuple varient with multiple element
//!     Nested{                     //validate varient with fields
//!         field:String,
//!     }
//! }
//! ```
//! 
//! # Usage
//! 
//! ## Validate Attribute Categories 
//! ```
//! #[derive(Validate)]
//! #[validate(attr = "foo")]          //<-- Container Attribute
//! struct S{
//!     #[validate(attr = "foo")]      //<--  Field Attribute
//!     field:String
//! }
//!
//! #[derive(Validate)]
//! enum E{
//!     #[validate(attr = "foo")]       //<-- Variant Attribute
//!     A(String)
//! }
//! ```
//!
//! ## Container Attribute
//! 
//! ### Schema
//! 
//! - #[validate(schema = "path")]
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


#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input,DeriveInput};

mod ast;
mod attr;
mod error;
mod symbol;
mod validate;
mod check;


#[proc_macro_derive(Validate, attributes(validate))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    validate::expand_derive_validate(&input).unwrap_or_else(to_compile_errors)
    .into()
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
