# Zisvalidator

Zisvalidator is designed for validating input struct or enum in web or other scence.

Reference to [Serde] and [Validator].

[Serde]: https://docs.serde.rs/serde/index.html
[Validator]: https://github.com/Keats/validator

## EXAMPLE

### Validate for Struct and Enum

```rust
#[derive(Validate)]
struct S(String);               //validate tuple struct with 1 element
#[derive(Validate)]
struct S(String,u64,);          //validate tuple struct with mulitple elements
#[derive(Validate)]
struct S{                       //validate struct with fields
    str:String 
}

#[derive(Validate)]
enum E{                         //validate enum
    S(String),                  //validate tuple varient with 1 element
    Tuple(String,String,),      //validate tuple varient with multiple element
    Nested{                     //validate varient with fields
        field:String,
    }
}
```

### Validate for sequence

Validate for sequence like std::Vec which implement trait IntoIter.

### Validate Arrribute

#### Container Attribute
  
* #[validate(schema = "foo")]

    Validate type T by customer by function `foo` as `Fn(t:T) -> Result<(),ValidatorError)`

* #[validate(custom = "foo")]

    Valid for tuple struct.For example,for type `struct S(String,String)`,validate by function `foo` as `Fn(t:&String) -> Result<(),ValidatorError>`

* #[validate(range = "start..end")]

    Valid for tuple struct.Validate for elements' range.

#### Field Attribute

* #[validate(range = "start..end")]

    Validate for fields' range.

* #[validate(custom = "foo")]

    Validate by function `foo` as `Fn(t:&T) -> Result<(),ValidatorError>`.

#### Variant Attribute

* #[validate(range = "start..end")]

    Validate for variants' range.

* #[validate(custom = "foo")]

    Validate by function `foo` as `Fn(t:&T) -> Result<(),ValidatorError>`.
