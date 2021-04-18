use zisvalidator::*;
use zisvalidator::{error::ValidatorError};
#[macro_use]
extern crate lazy_static;

#[derive(Validate)]
struct ValidateStruct{
    #[validate(range = "2..")]
    n:i32,
}

#[derive(Validate,Default)]
#[validate(range = "2..")]
struct ValidateStructTuple(i32);


#[derive(Validate)]
enum ValidateEnum{
    #[validate(range = "2..")]
    NewType(i32),
    #[validate(range = "2..")]
    Tuple(i32,i32),
    Fields{
        #[validate(range = "2..")]
        n:i32
    }
}
type ValidatorResult = Result<(),ValidatorError>;

lazy_static! {
    static ref VALIDATE_STRUCT_SUCC:ValidateStruct = ValidateStruct{
        n:3
    };
    static ref VALIDATE_STRUCT_FAIL:ValidateStruct = ValidateStruct{
        n:1
    };
    static ref VALIDATE_STRUCT_TUPLE_SUCC:ValidateStructTuple = ValidateStructTuple(3);
    static ref VALIDATE_STRUCT_TUPLE_FAIL:ValidateStructTuple = ValidateStructTuple(1);
    static ref VALIDATE_ENUM_NEWTYPE_FAIL:ValidateEnum = ValidateEnum::NewType(1);
    static ref VALIDATE_ENUM_NEWTYPE_SUCC:ValidateEnum = ValidateEnum::NewType(3);
    static ref VALIDATE_ENUM_TUPLE_FAIL:ValidateEnum = ValidateEnum::Tuple(3,1);
    static ref VALIDATE_ENUM_TUPLE_SUCC:ValidateEnum = ValidateEnum::Tuple(3,3);
    static ref VALIDATE_ENUM_FIELDS_FAIL:ValidateEnum = ValidateEnum::Fields{n:1};
    static ref VALIDATE_ENUM_FIELDS_SUCC:ValidateEnum = ValidateEnum::Fields{n:3};
    
}
fn validator_error<T:std::fmt::Debug>(field:&str,attr:&str,value:T) -> ValidatorError{
    ValidatorError{
        message:validator_error!(field,attr,value)
    }
}
#[cfg(test)]
mod test_range{
    use super::*;
    mod test_struct{
        use super::*;
        #[test]
        fn test_struct_field(){
            assert_eq!(VALIDATE_STRUCT_FAIL.validate(),ValidatorResult::Err(validator_error("n","range",2..)));
            assert_eq!(VALIDATE_STRUCT_SUCC.validate(),ValidatorResult::Ok(()));
        }
        #[test]
        fn test_struct_tuple(){
            assert_eq!(VALIDATE_STRUCT_TUPLE_FAIL.validate(),ValidatorResult::Err(validator_error("ValidateStructTuple","range",2..)));
            assert_eq!(VALIDATE_STRUCT_TUPLE_SUCC.validate(),ValidatorResult::Ok(()));
        }
    }
    pub mod test_enum{
        use super::*;
        #[test]
        fn test_enum(){
            assert_eq!(VALIDATE_ENUM_NEWTYPE_FAIL.validate(),ValidatorResult::Err(validator_error("NewType","range",2..)));
            assert_eq!(VALIDATE_ENUM_NEWTYPE_SUCC.validate(),ValidatorResult::Ok(()));
            assert_eq!(VALIDATE_ENUM_TUPLE_FAIL.validate(),ValidatorResult::Err(validator_error("Tuple","range",2..)));
            assert_eq!(VALIDATE_ENUM_TUPLE_SUCC.validate(),ValidatorResult::Ok(()));
            assert_eq!(VALIDATE_ENUM_FIELDS_FAIL.validate(),ValidatorResult::Err(validator_error("n","range",2..)));
            assert_eq!(VALIDATE_ENUM_FIELDS_SUCC.validate(),ValidatorResult::Ok(()));
        }
    }
        
}
