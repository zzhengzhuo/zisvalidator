use zisvalidator::Validate;
use zisvalidator::{error::ValidatorError};
#[macro_use]
extern crate lazy_static;

#[derive(Validate)]
struct ValidateStructSucc{
    #[validate(custom = "validate_succ")]
    s:String,
}

#[derive(Validate,Default)]
struct ValidateStructFail{
    #[validate(custom = "validate_fail")]
    s:String,
}
#[derive(Validate,Default)]
#[validate(custom = "validate_fail")]
struct ValidateStructTupleFail(String);

#[derive(Validate,Default)]
#[validate(custom = "validate_succ")]
struct ValidateStructTupleSucc(String);

#[derive(Validate)]
enum ValidateEnumSucc{
    #[validate(custom = "validate_succ")]
    NewType(String),
    #[validate(custom = "validate_succ")]
    Tuple(String,String),
    Fields{
        #[validate(custom = "validate_succ")]
        s:String
    }
}

#[derive(Validate)]
enum ValidateEnumFail{
    #[validate(custom = "validate_fail")]
    NewType(String),
    #[validate(custom = "validate_fail")]
    Tuple(String,String),
    Fields{
        #[validate(custom = "validate_fail")]
        s:String
    }
}

lazy_static! {
    static ref VALIDATE_FAILED:ValidatorError =  ValidatorError{
        message:"validate failed".to_owned()
    };
    static ref VALIDATE_STRUCT_SUCC:ValidateStructSucc = ValidateStructSucc{
        s:"succ".to_owned()
    };
    static ref VALIDATE_STRUCT_FAIL:ValidateStructFail = ValidateStructFail{
        s:"fail".to_owned()
    };
    static ref VALIDATE_STRUCT_TUPLE_SUCC:ValidateStructTupleSucc = ValidateStructTupleSucc("validate".to_owned());
    static ref VALIDATE_STRUCT_TUPLE_FAIL:ValidateStructTupleFail = ValidateStructTupleFail("validate".to_owned());
    static ref VALIDATE_ENUM_NEWTYPE_FAIL:ValidateEnumFail = ValidateEnumFail::NewType("validate".to_owned());
    static ref VALIDATE_ENUM_NEWTYPE_SUCC:ValidateEnumSucc = ValidateEnumSucc::NewType("validate".to_owned());
    static ref VALIDATE_ENUM_TUPLE_FAIL:ValidateEnumFail = ValidateEnumFail::Tuple("validate".to_owned(),"validate".to_owned());
    static ref VALIDATE_ENUM_TUPLE_SUCC:ValidateEnumSucc = ValidateEnumSucc::Tuple("validate".to_owned(),"validate".to_owned());
    static ref VALIDATE_ENUM_FIELDS_FAIL:ValidateEnumFail = ValidateEnumFail::Fields{s:"validate".to_owned()};
    static ref VALIDATE_ENUM_FIELDS_SUCC:ValidateEnumSucc = ValidateEnumSucc::Fields{s:"validate".to_owned()};

}
fn validate_fail(_:&str) -> ValidatorResult{
    Err(VALIDATE_FAILED.clone())
}

fn validate_succ(_:&str) -> ValidatorResult{
    Ok(())
}

type ValidatorResult = Result<(),ValidatorError>;

pub mod test_custom{
    use super::*;
    pub mod test_struct{
        use super::*;
        #[test]
        fn test_struct_field(){
            assert_eq!(VALIDATE_STRUCT_FAIL.validate(),ValidatorResult::Err(VALIDATE_FAILED.clone()));
            assert_eq!(VALIDATE_STRUCT_SUCC.validate(),ValidatorResult::Ok(()));
        }
        #[test]
        fn test_struct_tuple(){
            assert_eq!(VALIDATE_STRUCT_TUPLE_FAIL.validate(),ValidatorResult::Err(VALIDATE_FAILED.clone()));
            assert_eq!(VALIDATE_STRUCT_TUPLE_SUCC.validate(),ValidatorResult::Ok(()));
        }
        
    }
    pub mod test_enum{
        use super::*;
        #[test]
        fn test_enum(){
            assert_eq!(VALIDATE_ENUM_NEWTYPE_FAIL.validate(),ValidatorResult::Err(VALIDATE_FAILED.clone()));
            assert_eq!(VALIDATE_ENUM_NEWTYPE_SUCC.validate(),ValidatorResult::Ok(()));
            assert_eq!(VALIDATE_ENUM_TUPLE_FAIL.validate(),ValidatorResult::Err(VALIDATE_FAILED.clone()));
            assert_eq!(VALIDATE_ENUM_TUPLE_SUCC.validate(),ValidatorResult::Ok(()));
            assert_eq!(VALIDATE_ENUM_FIELDS_FAIL.validate(),ValidatorResult::Err(VALIDATE_FAILED.clone()));
            assert_eq!(VALIDATE_ENUM_FIELDS_SUCC.validate(),ValidatorResult::Ok(()));
        }
    }

}
