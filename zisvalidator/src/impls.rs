use std::collections::{BTreeSet, BinaryHeap, LinkedList, VecDeque};

use super::*;

macro_rules! impl_validate_range {
    ($ty:ident) => {
        impl<T, R, Idx> ValidateRange<T, R, Idx> for $ty
        where
                T: std::fmt::Display,
                R: std::ops::RangeBounds<Idx> + std::fmt::Debug,
                Idx: PartialOrd<Idx> + PartialOrd<Self> + std::fmt::Debug,
                Self: PartialOrd<Idx>, {
            fn validate_range(
                &self,
                field: T,
                range: &R,
            ) -> Result<(), error::ValidatorError>
            
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

macro_rules! impl_validate_range_seq {
    ($ty:ident<$item:ident>) => {
        impl<F, R, Idx,$item> ValidateRange<F, R, Idx,> for $ty<$item> 
        where  
            Self: std::iter::IntoIterator<Item = $item>,
            F: std::fmt::Display,
            R: std::ops::RangeBounds<Idx> + std::fmt::Debug,
            Idx: PartialOrd<Idx> + PartialOrd<$item>  + std::fmt::Debug,
            $item:PartialOrd<Idx>,
        {
            fn validate_range<>(
                &self,
                field: F,
                range: &R,
            ) -> Result<(), error::ValidatorError>
            {
                if self.iter().find(|item| !range.contains(*item)).is_some(){
                    Err(error::ValidatorError {
                        message: validator_error!(field, "range", range),
                    })
                }else{
                    Ok(())
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

impl_validate_range_seq!(Vec<T>);
impl_validate_range_seq!(BTreeSet<T>);
impl_validate_range_seq!(BinaryHeap<T>);
impl_validate_range_seq!(LinkedList<T>);
impl_validate_range_seq!(VecDeque<T>);

