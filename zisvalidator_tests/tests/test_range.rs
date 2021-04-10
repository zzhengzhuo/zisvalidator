use zisvalidator_derive::*;
use zisvalidator::*;

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
                    #[validate(range = r#"2.."#)]
                    field1:i32,
                }
        
                let test_struct_field_fail = TestStructFieldFail{
                    field1:1,
                };
                
                assert!(test_struct_field_fail.validate().is_err());
            }
        
            #[test]
            fn test_struct_field_succ(){
                #[derive(Validate)]
                struct TestStructFieldSucc{
                    #[validate(range = r#"2.."#)]
                    field:i32,
                }
                let test_struct_field_succ = TestStructFieldSucc{
                    field:3
                };
                
                assert!(test_struct_field_succ.validate().is_ok());
            }
        }
        pub mod struct_new_type{
            use super::*;
            #[test]
            fn test_struct_new_type_fail(){
                #[derive(Validate)]
                #[validate(range = "2..")]
                struct TestStructNewTypeFail(i32,u8);
        
                let test_struct_new_type_fail = TestStructNewTypeFail(
                    1,2
                );
                assert!(test_struct_new_type_fail.validate().is_err());
            }
        
            #[test]
            fn test_struct_new_type_succ(){
                #[derive(Validate)]
                #[validate(range = "2..")]
                struct TestStructNewTypeSucc(i32,u8);
                let test_struct_new_type_succ = TestStructNewTypeSucc(
                    3,4
                );
                assert!(test_struct_new_type_succ.validate().is_ok());
            }
        }
        
    }
    
    pub mod test_enum{
        use super::*;
    
        #[test]
        fn test_enum_fail(){
            #[derive(Validate)]
            enum TestEnum{
                #[validate(range = "2..")]
                NewType(i32),
                Field{
                    #[validate(range = "2..")]
                    field:i64,
                }
            }
    
            let test_enum_new_type = TestEnum::NewType(1);
            assert!(test_enum_new_type.validate().is_err());
    
            let test_enum_field = TestEnum::Field{field:1};
            assert!(test_enum_field.validate().is_err());
        }

        #[test]
        fn test_enum_succ(){
            #[derive(Validate)]
            enum TestEnum{
                #[validate(range = "2..")]
                NewType(i32),
                Field{
                    #[validate(range = "2..")]
                    field:i64,
                }
            }
    
            let test_enum_new_type = TestEnum::NewType(3);
            assert!(test_enum_new_type.validate().is_ok());
    
            let test_enum_field = TestEnum::Field{field:3};
            assert!(test_enum_field.validate().is_ok());
        }
        
    }
}
