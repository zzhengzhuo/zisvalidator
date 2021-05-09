
use std::borrow::Cow;

use zisvalidator::error::ValidatorError;
use zisvalidator::*;

#[allow(dead_code)]
#[derive(Validate)]
enum ValidateType<'a>{
    #[validate(range = "true..=true")]
    Bool(bool),
    #[validate(range = "2..")]
    Isize(isize),
    #[validate(range = "2..")]
    I8(i8),
    #[validate(range = "2..")]
    I32(i32),
    #[validate(range = "2..")]
    I64(i64),
    #[validate(range = "2..")]
    Usize(usize),
    #[validate(range = "2..")]
    U8(u8),
    #[validate(range = "2..")]
    U32(u32),
    #[validate(range = "2..")]
    U64(u64),
    #[validate(range = "2.0..")]
    F32(f32),
    #[validate(range = "2.0..")]
    F64(f64),
    #[validate(range = "'2'..")]
    Ch(char),
    #[validate(range = "\"2\"..")]
    Str(&'a str),
    #[validate(range = "\"2\".to_owned()..")]
    String(String),
    #[validate(range = "Some(\"2\".to_owned())..")]
    Opt(Option<String>),
    #[validate(range = "Cow::Owned(\"2\".to_owned())..")]
    Cow(Cow<'a,String>),
}

#[derive(Validate)]
struct ValidateStruct<'a> {
    #[validate(range = "LOW_STR..")]
    s: &'a str,
}

#[derive(Validate, Default)]
#[validate(range = "LOW_STR..")]
struct ValidateStructTuple<'a>(&'a str);

#[derive(Validate)]
enum ValidateEnum<'a> {
    #[validate(range = "LOW_STR..")]
    NewType(&'a str),
    #[validate(range = "LOW_STR..")]
    Tuple(&'a str, &'a str),
    Fields {
        #[validate(range = "LOW_STR..")]
        s: &'a str,
    },
}


type ValidatorResult = Result<(), ValidatorError>;
const LOW_STR: &str = "2";

const VALIDATE_STRUCT_SUCC: ValidateStruct<'static> = ValidateStruct {s: "3" };
const VALIDATE_STRUCT_FAIL: ValidateStruct = ValidateStruct {s: "1" };
const VALIDATE_STRUCT_TUPLE_SUCC: ValidateStructTuple = ValidateStructTuple("3");
const VALIDATE_STRUCT_TUPLE_FAIL: ValidateStructTuple = ValidateStructTuple("1");
const VALIDATE_ENUM_NEWTYPE_FAIL: ValidateEnum = ValidateEnum::NewType("1");
const VALIDATE_ENUM_NEWTYPE_SUCC: ValidateEnum = ValidateEnum::NewType("3");
const VALIDATE_ENUM_TUPLE_FAIL: ValidateEnum = ValidateEnum::Tuple("3", "1");
const VALIDATE_ENUM_TUPLE_SUCC: ValidateEnum = ValidateEnum::Tuple("3", "3");
const VALIDATE_ENUM_FIELDS_FAIL: ValidateEnum = ValidateEnum::Fields { s: "1", };
const VALIDATE_ENUM_FIELDS_SUCC: ValidateEnum = ValidateEnum::Fields { s: "3", };
fn validator_error<T: std::fmt::Debug>(field: &str, attr: &str, value: T) -> ValidatorError {
    ValidatorError {
        message: validator_error!(field, attr, value),
    }
}
#[cfg(test)]
mod test_range {
    use super::*;
    mod test_type{
        use super::*;
        #[test]
        fn test_type(){
            assert!(ValidateType::Bool(true).validate().is_ok());
            assert!(ValidateType::Bool(false).validate().is_err());
            assert!(ValidateType::Isize(2).validate().is_ok());
            assert!(ValidateType::Isize(1).validate().is_err());
            assert!(ValidateType::I8(2).validate().is_ok());
            assert!(ValidateType::I8(1).validate().is_err());
            assert!(ValidateType::I32(2).validate().is_ok());
            assert!(ValidateType::I32(1).validate().is_err());
            assert!(ValidateType::I64(2).validate().is_ok());
            assert!(ValidateType::I64(1).validate().is_err());
            assert!(ValidateType::Usize(2).validate().is_ok());
            assert!(ValidateType::Usize(1).validate().is_err());
            assert!(ValidateType::U8(2).validate().is_ok());
            assert!(ValidateType::U8(1).validate().is_err());
            assert!(ValidateType::U32(2).validate().is_ok());
            assert!(ValidateType::U32(1).validate().is_err());
            assert!(ValidateType::U64(2).validate().is_ok());
            assert!(ValidateType::U64(1).validate().is_err());
            assert!(ValidateType::F32(2.0).validate().is_ok());
            assert!(ValidateType::F32(1.0).validate().is_err());
            assert!(ValidateType::F64(2.0).validate().is_ok());
            assert!(ValidateType::F64(1.0).validate().is_err());
            assert!(ValidateType::Ch('2').validate().is_ok());
            assert!(ValidateType::Ch('1').validate().is_err());
            assert!(ValidateType::Str("2").validate().is_ok());
            assert!(ValidateType::Str("1").validate().is_err());
            assert!(ValidateType::String("2".to_owned()).validate().is_ok());
            assert!(ValidateType::String("1".to_owned()).validate().is_err());
            assert!(ValidateType::Opt(Some("2".to_owned())).validate().is_ok());
            assert!(ValidateType::Opt(Some("1".to_owned())).validate().is_err());
        }
    }
    mod test_struct {
        use super::*;
        #[test]
        fn test_struct_field() {
            assert_eq!(
                VALIDATE_STRUCT_FAIL.validate(),
                ValidatorResult::Err(validator_error("s", "range", "2"..))
            );
            assert_eq!(VALIDATE_STRUCT_SUCC.validate(), ValidatorResult::Ok(()));
        }
        #[test]
        fn test_struct_tuple() {
            assert_eq!(
                VALIDATE_STRUCT_TUPLE_FAIL.validate(),
                ValidatorResult::Err(validator_error("ValidateStructTuple", "range", "2"..))
            );
            assert_eq!(
                VALIDATE_STRUCT_TUPLE_SUCC.validate(),
                ValidatorResult::Ok(())
            );
        }
    }
    pub mod test_enum {
        use super::*;
        #[test]
        fn test_enum() {
            assert_eq!(
                VALIDATE_ENUM_NEWTYPE_FAIL.validate(),
                ValidatorResult::Err(validator_error("NewType", "range", "2"..))
            );
            assert_eq!(
                VALIDATE_ENUM_NEWTYPE_SUCC.validate(),
                ValidatorResult::Ok(())
            );
            assert_eq!(
                VALIDATE_ENUM_TUPLE_FAIL.validate(),
                ValidatorResult::Err(validator_error("Tuple", "range", "2"..))
            );
            assert_eq!(VALIDATE_ENUM_TUPLE_SUCC.validate(), ValidatorResult::Ok(()));
            assert_eq!(
                VALIDATE_ENUM_FIELDS_FAIL.validate(),
                ValidatorResult::Err(validator_error("s", "range", "2"..))
            );
            assert_eq!(
                VALIDATE_ENUM_FIELDS_SUCC.validate(),
                ValidatorResult::Ok(())
            );
        }
    }

}
