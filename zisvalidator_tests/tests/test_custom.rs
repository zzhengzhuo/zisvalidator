use zisvalidator_derive::*;
use zisvalidator::*;
use zisvalidator::{error::ValidatorError};
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref VALIDATE_STR_FAILED:ValidatorError =  ValidatorError{
        message:"validate for str fail".to_owned()
    };
}
fn validate_for_str_fail(_:&str) -> Result<(),ValidatorError>{
    Err(VALIDATE_STR_FAILED.clone())
}

fn validate_for_str_succ(_:&str) -> Result<(),ValidatorError>{
    Ok(())
}

pub mod test_custom{
    use super::*;
    pub mod test_struct{
        use super::*;
        pub mod struct_field{
            use super::*;
            #[test]
            fn test_struct_field_fail(){
                #[derive(Validate)]
                struct TestStructFieldFail{
                    #[validate(custom = "validate_for_str_fail")]
                    field:String,
                }
        
                let test_struct_field_fail = TestStructFieldFail{
                    field:"test_struct_field_fail".to_owned()
                };
                
                assert_eq!(test_struct_field_fail.validate(),Result::<(),_>::Err(VALIDATE_STR_FAILED.clone()));
            }
        
            #[test]
            fn test_struct_field_succ(){
                #[derive(Validate)]
                struct TestStructFieldSucc{
                    #[validate(custom = "validate_for_str_succ")]
                    field:String,
                }
                let test_struct_field_succ = TestStructFieldSucc{
                    field:"test_struct_field_succ".to_owned()
                };
                
                assert_eq!(test_struct_field_succ.validate(),Result::<(),_>::Ok(()));
            }
        }
        pub mod struct_new_type{
            use super::*;
            #[test]
            fn test_struct_new_type_fail(){
                #[derive(Validate)]
                #[validate(custom = "validate_for_str_fail")]
                struct TestStructNewTypeFail(String);
        
                let test_struct_new_type_fail = TestStructNewTypeFail(
                    "test_struct_new_type_fail".to_owned()
                );
                assert_eq!(test_struct_new_type_fail.validate(),Result::<(),_>::Err(VALIDATE_STR_FAILED.clone()));
            }
        
            #[test]
            fn test_struct_new_type_succ(){
                #[derive(Validate)]
                #[validate(custom = "validate_for_str_succ")]
                struct TestStructNewTypeSucc(
                    String,
                );
                let test_struct_new_type_succ = TestStructNewTypeSucc(
                    "test_struct_field_succ".to_owned()
                );
    
                assert_eq!(test_struct_new_type_succ.validate(),Result::<(),_>::Ok(()));
            }
        }
        
    }
    
    pub mod test_enum{
        use super::*;
    
        #[test]
        fn test_enum_fail(){
            #[derive(Validate)]
            enum TestEnum{
                #[validate(custom = "validate_for_str_fail")]
                NewType(String),
                Field{
                    #[validate(custom = "validate_for_str_fail")]
                    field:String,
                }
            }
    
            let test_enum_new_type = TestEnum::NewType("new type".into());
            assert_eq!(test_enum_new_type.validate(),Err(VALIDATE_STR_FAILED.clone()));
    
            let test_enum_field = TestEnum::Field{field:"field".into()};
            assert_eq!(test_enum_field.validate(),Err(VALIDATE_STR_FAILED.clone()));
        }

        #[test]
        fn test_enum_succ(){
            #[derive(Validate)]
            enum TestEnum{
                #[validate(custom = "validate_for_str_succ")]
                NewType(String),
                Field{
                    #[validate(custom = "validate_for_str_succ")]
                    field:String,
                }
            }
    
            let test_enum_new_type = TestEnum::NewType("new type".into());
            assert_eq!(test_enum_new_type.validate(),Ok(()));
    
            let test_enum_field = TestEnum::Field{field:"field".into()};
            assert_eq!(test_enum_field.validate(),Ok(()));
        }
        
    }
}
