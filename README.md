# Zisvalidator

Zisvalidator is designed for validating input struct or enum in web or other scence.

Reference to [Serde] and [Validator].

[Serde]: https://docs.serde.rs/serde/index.html
[Validator]: https://github.com/Keats/validator

## Install

```rust
zisvalidator = {version = "0.1.6",features = ["derive"]}
```

## EXAMPLE

### Short Example

```rust
use zisvalidator::*;

const LOW_STR: &str = "2";

#[derive(Validate)]
struct ValidateStruct<'a> {
    #[validate(range = "LOW_STR..")]
    s: &'a str,
    #[validate(seq_range = "&LOW_STR..")]
    seq: Vec<&'a str>
}

```

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

### Validate Arrribute

#### Container Attribute
  
* #[validate(schema = "foo")]

    Validate type T by customer by function `foo` as `Fn(t:T) -> Result<(),ValidatorError)`

* #[validate(custom = "foo")]

    Valid for tuple struct.For example,for type `struct S(String,String)`,validate by function `foo` as `Fn(t:&String) -> Result<(),ValidatorError>`

* #[validate(range = "start..end")]

    Valid for tuple struct.Validate range for elements.

* #[validate(seq_range = "start..end")]

    Valid for tuple struct.Validate range for elements which implement `IntoIterator` like `std::vec::Vec`.

#### Field Attribute

* #[validate(range = "start..end")]

    Validate for fields' range.

* #[validate(seq_range = "start..end")]

    Validate range for fields which implement `IntoIterator` like `std::vec::Vec`.

* #[validate(custom = "foo")]

    Validate by function `foo` as `Fn(t:&T) -> Result<(),ValidatorError>`.

#### Variant Attribute

* #[validate(range = "start..end")]

    Validate for variants' range.

* #[validate(seq_range = "start..end")]

    Validate range for variants which implement `IntoIterator` like `std::vec::Vec`.

* #[validate(custom = "foo")]

    Validate by function `foo` as `Fn(t:&T) -> Result<(),ValidatorError>`.
