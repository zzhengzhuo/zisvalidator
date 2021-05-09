use zisvalidator::*;
use zisvalidator::error::ValidatorError;

#[allow(dead_code)]
#[derive(Validate)]
enum ValidateType{
    #[validate(seq_range = "&1..")]
   Vec(Vec<i32>),
}


#[derive(Validate)]
struct ValidateStruct<'a> {
    #[validate(seq_range = "&LOW_STR..")]
    s: Vec<&'a str>,
}

#[derive(Validate, Default)]
#[validate(seq_range = "&LOW_STR..")]
struct ValidateStructTuple<'a>(Vec<&'a str>);

#[derive(Validate)]
enum ValidateEnum<'a> {
    #[validate(seq_range = "&LOW_STR..")]
    NewType(Vec<&'a str>),
    #[validate(seq_range = "&LOW_STR..")]
    Tuple(Vec<&'a str>, Vec<&'a str>),
    Fields {
        #[validate(seq_range = "&LOW_STR..")]
        s: Vec<&'a str>,
    },
}


type ValidatorResult = Result<(), ValidatorError>;
const LOW_STR: &str = "2";

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
            assert!(ValidateType::Vec(vec![1,2]).validate().is_ok());
            assert!(ValidateType::Vec(vec![0,1]).validate().is_err());
        }
    }
    mod test_struct {
        use super::*;
        #[test]
        fn test_struct_field() {
            let validate_struct_fail = ValidateStruct{s:vec!["1"]};
            assert_eq!(
                validate_struct_fail.validate(),
                ValidatorResult::Err(validator_error("s", "range", "2"..))
            );

            let validate_struct_succ = ValidateStruct{s:vec!["2"]};
            assert_eq!(validate_struct_succ.validate(), ValidatorResult::Ok(()));
        }
        #[test]
        fn test_struct_tuple() {
            let validate_fail = ValidateStructTuple(vec!["1"]);
            assert_eq!(
                validate_fail.validate(),
                ValidatorResult::Err(validator_error("ValidateStructTuple", "range", "2"..))
            );
            let validate_succ = ValidateStructTuple(vec!["3"]);
            assert_eq!(
                validate_succ.validate(),
                ValidatorResult::Ok(())
            );
        }
    }
    pub mod test_enum {
        use super::*;
        #[test]
        fn test_enum() {
            let validate_fail = ValidateEnum::NewType(vec!["1"]);
            assert_eq!(
                validate_fail.validate(),
                ValidatorResult::Err(validator_error("NewType", "range", "2"..))
            );
            let validate_succ = ValidateEnum::NewType(vec!["3"]);
            assert_eq!(
                validate_succ.validate(),
                ValidatorResult::Ok(())
            );
            let validate_fail = ValidateEnum::Tuple(vec!["3"], vec!["1"]);
            assert_eq!(
                validate_fail.validate(),
                ValidatorResult::Err(validator_error("Tuple", "range", "2"..))
            );
            let validate_succ = ValidateEnum::Tuple(vec!["3"], vec!["3"]);
            assert_eq!(validate_succ.validate(), ValidatorResult::Ok(()));
            let validate_fail = ValidateEnum::Fields { s: vec!["1"], };
            assert_eq!(
                validate_fail.validate(),
                ValidatorResult::Err(validator_error("s", "range", "2"..))
            );
            let validate_succ = ValidateEnum::Fields { s: vec!["3"], };
            assert_eq!(
                validate_succ.validate(),
                ValidatorResult::Ok(())
            );
        }
    }
}