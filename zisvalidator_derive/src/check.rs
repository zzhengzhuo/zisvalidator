use super::ast::{Container, Data, Style};
use super::error::Error;

pub(crate) fn check(error: &Error, cont: &Container) {
    check_custom(error, cont);
}

fn check_custom(error: &Error, cont: &Container) {
    if let Some(_) = &cont.attrs.custom {
        match cont.data {
            Data::Struct(style, _) if style == Style::Newtype || style == Style::Tuple => {}
            _ => error.push_span_error(
                cont.original,
                "#[validate(custom = \"...\")] can only used in tuple struct with 1 element",
            ),
        }
    }
}
