use super::*;
use std::iter::IntoIterator;

impl< F, R, Idx,T> ValidateRange<F, R, Idx> for T
where
    F: std::fmt::Display + ?Sized,
    R: std::ops::RangeBounds<Idx> + std::fmt::Debug,
    Idx: PartialOrd<Idx> + PartialOrd<Self> + std::fmt::Debug + ?Sized,
    Self: PartialOrd<Idx>,
{
    fn validate_range(&self, field: &F, range: &R) -> Result<(), error::ValidatorError> {
        if range.contains(self) {
            Ok(())
        } else {
            Err(error::ValidatorError {
                message: validator_error!(field, "range", range),
            })
        }
    }
}

impl<'a, F, R,Idx,T:'a,> ValidateSeqRange<F, R, Idx> for &'a T
where
    F: std::fmt::Display + ?Sized,
    R: std::ops::RangeBounds<Idx> + std::fmt::Debug,
    Idx:PartialOrd<Idx> + PartialOrd<<&'a T as IntoIterator>::Item>  + std::fmt::Debug + ?Sized + 'a,
    <&'a T as IntoIterator>::Item:PartialOrd<Idx>,
    &'a T: IntoIterator,
{
    fn validate_seq_range(self, field: &F, range: &R) -> Result<(), error::ValidatorError> {
        if self.into_iter().find(|v| !range.contains(v)).is_some(){
            Err(error::ValidatorError {
                message: validator_error!(field, "range", range),
            })
        }else{
            Ok(())
        }
    }
}
