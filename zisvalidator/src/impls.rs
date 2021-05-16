use super::*;
use std::iter::IntoIterator;

//impl ValdateRange
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

//impl ValidateSeqRange
impl<'a, F, R,Idx,T:'a,> ValidateSeqRange<'a,F, R, Idx> for T
where
    F: std::fmt::Display + ?Sized,
    R: std::ops::RangeBounds<Idx> + std::fmt::Debug,
    Idx:PartialOrd<Idx> + PartialOrd<<&'a T as IntoIterator>::Item>  + std::fmt::Debug + ?Sized + 'a,
    <&'a T as IntoIterator>::Item:PartialOrd<Idx>,
    &'a T: IntoIterator,
{
    fn validate_seq_range(&'a self, field: &F, range: &R) -> Result<(), error::ValidatorError> {
        if self.into_iter().find(|v| !range.contains(v)).is_some(){
            Err(error::ValidatorError {
                message: validator_error!(field, "range", range),
            })
        }else{
            Ok(())
        }
    }
}

//impl ValidateLength
// impl<'a, F, R,Idx,T:'a,> ValidateLength<F, R, Idx> for T
// where
//     F: std::fmt::Display + ?Sized,
//     R: std::ops::RangeBounds<Idx> + std::fmt::Debug,
//     Idx:PartialOrd<Idx> + PartialOrd<<&'a T as IntoIterator>::Item>  + std::fmt::Debug + ?Sized,
//     <&'a T as IntoIterator>::Item:PartialOrd<Idx>,
//     &'a T: IntoIterator,
// {
//     fn validate_length(&'a self, field: &F, range: &R) -> Result<(), error::ValidatorError> {
//         if self.into_iter().find(|v| !range.contains(v)).is_some(){
//             Err(error::ValidatorError {
//                 message: validator_error!(field, "range", range),
//             })
//         }else{
//             Ok(())
//         }
//     }
// }
