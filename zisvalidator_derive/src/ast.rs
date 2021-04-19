use crate::attr;
// use proc_macro::TokenStream;
use crate::error::Error;
// use syn::Meta::{List,NameValue, Path};
// use syn::NestedMeta::{Lit, Meta};
use syn::punctuated::Punctuated;
// use crate::symbol::*;
// use crate::quote::ToTokens;

#[derive(Debug)]
pub struct Container<'a> {
    /// The struct or enum name (without generics).
    pub ident: syn::Ident,
    /// Attributes on the structure, parsed for Validate.
    pub attrs: attr::Container,
    /// The contents of the struct or enum.
    pub data: Data<'a>,
    // Any generics on the struct or enum.
    pub generics: &'a syn::Generics,
    // Original input.
    pub original: &'a syn::DeriveInput,
}

#[derive(Debug)]
pub enum Data<'a> {
    // Enum(Vec<Variant<'a>>),
    // Struct(Style, Vec<Field<'a>>),
    Enum(Vec<Variant<'a>>),
    Struct(Style, Vec<Field<'a>>),
}

/// A variant of an enum.
#[derive(Debug)]
pub struct Variant<'a> {
    pub ident: syn::Ident,
    pub attrs: attr::Variant,
    pub style: Style,
    pub fields: Vec<Field<'a>>,
    pub original: &'a syn::Variant,
}

/// A field of a struct.
#[derive(Debug)]
pub struct Field<'a> {
    pub member: syn::Member,
    pub attrs: attr::Field,
    pub ty: &'a syn::Type,
    // pub original: &'a syn::Field,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Style {
    /// Named fields.
    Struct,
    /// Many unnamed fields.
    Tuple,
    /// One unnamed field.
    Newtype,
    /// No fields.
    Unit,
}

impl<'a> Container<'a> {
    pub fn from_ast(error: &Error, item: &'a syn::DeriveInput) -> Option<Self> {
        let ident = item.ident.clone();
        let attrs = attr::Container::from_ast(error, &item);
        let data = match &item.data {
            syn::Data::Enum(data) => Data::Enum(enum_from_ast(error, &data.variants)),
            syn::Data::Struct(data) => {
                // struct_from_ast(error, &data.fields);
                // Data::Struct(data)
                let (style, fields) = struct_from_ast(error, &data.fields);
                Data::Struct(style, fields)
            }
            syn::Data::Union(_) => {
                error.push_span_error(item, "Validate does not support for unions");
                return None;
            }
        };
        let generics = &item.generics;
        if error.errors.borrow().is_empty() {
            Some(Container {
                ident,
                data,
                generics,
                attrs,
                original: item,
            })
        } else {
            None
        }
    }
}
fn enum_from_ast<'a>(
    error: &Error,
    variants: &'a Punctuated<syn::Variant, Token![,]>,
) -> Vec<Variant<'a>> {
    variants
        .iter()
        .map(|variant| {
            let attrs = attr::Variant::from_ast(error, variant);
            let (style, fields) = struct_from_ast(error, &variant.fields);
            Variant {
                ident: variant.ident.clone(),
                attrs,
                style,
                fields,
                original: variant,
            }
        })
        .collect()
}

fn struct_from_ast<'a>(errors: &Error, fields: &'a syn::Fields) -> (Style, Vec<Field<'a>>) {
    match fields {
        syn::Fields::Named(fields) => (Style::Struct, fields_from_ast(errors, &fields.named)),
        syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            (Style::Newtype, fields_from_ast(errors, &fields.unnamed))
        }
        syn::Fields::Unnamed(fields) => (Style::Tuple, fields_from_ast(errors, &fields.unnamed)),
        syn::Fields::Unit => (Style::Unit, Vec::new()),
    }
}

fn fields_from_ast<'a>(
    error: &Error,
    fields: &'a Punctuated<syn::Field, Token![,]>,
) -> Vec<Field<'a>> {
    fields
        .iter()
        .enumerate()
        .map(|(i, field)| Field {
            member: match &field.ident {
                Some(ident) => syn::Member::Named(ident.clone()),
                None => syn::Member::Unnamed(i.into()),
            },
            attrs: attr::Field::from_ast(error, field),
            ty: &field.ty,
            // original: field,
        })
        .collect()
}
