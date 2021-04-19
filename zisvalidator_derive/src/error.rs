use quote::ToTokens;
use std::cell::RefCell;
use std::fmt::Display;

// We have to use RefCell here for avoid the scense like borrow mut in iterator and closures
#[derive(Debug)]
pub struct Error {
    pub errors: RefCell<Vec<syn::Error>>,
}
impl Error {
    pub fn new() -> Self {
        Error {
            errors: RefCell::new(Vec::new()),
        }
    }
    pub fn push_syn_error(&self, err: syn::Error) {
        self.errors.borrow_mut().push(err);
    }

    pub fn push_span_error<A: ToTokens, T: Display>(&self, obj: A, msg: T) {
        self.errors
            .borrow_mut()
            .push(syn::Error::new_spanned(obj, msg));
    }

    pub fn check(self) -> Result<(), Vec<syn::Error>> {
        let errors = self.errors.into_inner();
        match errors.len() {
            0 => Ok(()),
            _ => Err(errors),
        }
    }
}
