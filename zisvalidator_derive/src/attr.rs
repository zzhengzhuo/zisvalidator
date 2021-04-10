use crate::error::Error;
use syn::{ExprPath, ExprRange, Meta::{List,NameValue},};
use syn::NestedMeta::{Lit, Meta};
use crate::symbol::*;
use proc_macro2::TokenStream;
use quote::ToTokens;

#[derive(Debug)]
struct Attr<T>{
    name:Symbol,
    // message:Option<String>,
    value:Option<T>,
    tokens:TokenStream
}

impl<T> Attr<T>{
    fn none(name:Symbol) -> Self{
        Attr{
            name,value:None,tokens:TokenStream::new(),
        }
    }
    fn set<A: ToTokens>(&mut self,error:&Error, obj: A, value: T) {
        let tokens = obj.into_token_stream();

        if self.value.is_some() {
            error
                .push_span_error(tokens, format!("duplicate validate attribute `{}`", self.name));
        } else {
            self.value = Some(value);
            self.tokens = tokens;
        }
        
    }
    // fn set_message<A: ToTokens>(&mut self,error:&Error, obj: A, msg: String) {
    //     let tokens = obj.into_token_stream();

    //     if self.message.is_some() {
    //         error
    //             .push_span_error(tokens, format!("duplicate message for validate attribute `{}`", self.name));
    //     } else {
    //         self.message = Some(msg);
    //         if self.tokens.is_empty(){
    //             self.tokens = tokens;
    //         }
    //     }
        
    // }
    fn get(self) -> Option<T> {
        self.value
    }
}

#[derive(Debug,Default)]
pub struct Container {
    pub schema:Option<ExprPath>,
    pub custom:Option<ExprPath>,
    pub range:Option<ExprRange>,
}

impl Container{
    pub(crate) fn from_ast(error:&Error,item:&syn::DeriveInput) -> Self{
        let mut schema = Attr::none(SCHEMA);
        let mut custom = Attr::none(CUSTOM);
        let mut range = Attr::none(RANGE);
        for meta_item in item.attrs.iter().flat_map(|attr|{
            get_validate_meta_items(&error, &attr)
        }).flatten(){
            match &meta_item{
                // Parse `#[validate(schema = "foo")]`
                Meta(NameValue(n)) if n.path == SCHEMA => {
                    let litstr =  if let syn::Lit::Str(litstr) = &n.lit{
                        litstr
                    }else{
                        error.push_span_error(&n.lit, format!(
                            "expected validate {} attribute to be a string: `{} = \"...\"`",
                            SCHEMA,SCHEMA
                        ));
                        continue;
                    };
                    if let Ok(expr_path) =  litstr.parse::<ExprPath>().map_err(|_|{
                        error.push_span_error(litstr, format!("failed to parse path: {:?}",litstr))
                    }){
                        schema.set(error, litstr, expr_path);
                    }

                },
                 // Parse `#[validate(match = "foo")]`
                Meta(NameValue(n)) if n.path == CUSTOM => {
                    let litstr =  if let syn::Lit::Str(litstr) = &n.lit{
                        litstr
                    }else{
                        error.push_span_error(&n.lit, format!(
                            "expected validate {} attribute to be a string: `{} = \"...\"`",
                            CUSTOM,CUSTOM
                        ));
                        continue;
                    };
                    if let Ok(expr_path) =  litstr.parse::<ExprPath>().map_err(|_|{
                        error.push_span_error(litstr, format!("failed to parse path: {:?}",litstr))
                    }){
                        custom.set(error, litstr, expr_path);
                    }

                },
                Meta(NameValue(n)) if n.path == RANGE => {
                    let litstr =  if let syn::Lit::Str(litstr) = &n.lit{
                        litstr
                    }else{
                        error.push_span_error(&n.lit, format!(
                            "expected validate {} attribute to be a string: `{} = \"...\"`",
                            RANGE,RANGE
                        ));
                        continue;
                    };
                    if let Ok(expr) =  litstr.parse::<ExprRange>().map_err(|_|{
                        error.push_span_error(litstr, format!("failed to parse path: {:?}",litstr))
                    }){
                        range.set(error, litstr, expr);
                    }

                },
                Meta(meta_item) => {
                    let path = meta_item
                        .path()
                        .into_token_stream()
                        .to_string()
                        .replace(' ', "");
                    error.push_span_error(
                        meta_item.path(),
                        format!("unknown validate container attribute `{}`", path),
                    );
                }

                Lit(lit) => {
                    error.push_span_error(lit, "unexpected literal in validate container attribute");
                }

            } 
        }

        Container{
            schema:schema.value,
            custom:custom.value,
            range:range.value,
        }
    }
}
fn get_validate_meta_items(error:&Error,attr:&syn::Attribute) -> Result<Vec<syn::NestedMeta>,()>{
    if attr.path != VALIDATE {
        return Ok(Vec::new());
    }

    match attr.parse_meta() {
        Ok(List(meta)) => Ok(meta.nested.into_iter().collect()),
        Ok(other) => {
            error.push_span_error(other, "expected #[validate(...)]");
            Err(())
        }
        Err(err) => {
            error.push_syn_error(err);
            Err(())
        }
    }
}
#[derive(Debug)]
pub struct Field{
    pub custom:Option<ExprPath>,
    pub range:Option<syn::ExprRange>,
}
impl Field{
    pub fn from_ast(error:&Error, field: &syn::Field,) -> Self{
        let mut custom =  Attr::none(CUSTOM);
        let mut range = Attr::none(RANGE);

        for meta_item in field.attrs.iter().flat_map(|attr| {
            get_validate_meta_items(&error, &attr)
        }).flatten()
        {
            match &meta_item{
                // Paese `#[validate(custom = "foo")]`
                Meta(NameValue(n)) if n.path == CUSTOM => {
                    let litstr =  if let syn::Lit::Str(litstr) = &n.lit{
                        litstr
                    }else{
                        error.push_span_error(&n.lit, format!(
                            "expected validate {} attribute to be a string: `{} = \"...\"`",
                            CUSTOM,CUSTOM
                        ));
                        continue;
                    };
                    if let Ok(expr_path) =  litstr.parse::<ExprPath>().map_err(|_|{
                        error.push_span_error(litstr, format!("failed to parse path: {:?}",litstr))
                    }){
                        custom.set(error, litstr, expr_path);
                    }

                },
                // Paese `#[validate(match_pat = "foo")]`
                Meta(NameValue(n)) if n.path == RANGE => {
                    let litstr =  if let syn::Lit::Str(litstr) = &n.lit{
                        litstr
                    }else{
                        error.push_span_error(&n.lit, format!(
                            "expected validate {} attribute to be a string: `{} = \"...\"`",
                            RANGE,RANGE
                        ));
                        continue;
                    };
                    if let Ok(exprrange) =  litstr.parse::<syn::ExprRange>().map_err(|_|{
                        error.push_span_error(litstr, format!("failed to parse path: {:?}",litstr))
                    }){
                        range.set(error, litstr, exprrange);
                    }

                },
                attr =>{
                    error.push_span_error(&attr, format!( "invalid validate attribute", ));
                }

            }
            
        } 
        Field{
            custom:custom.get(),range:range.get(),
        }
    }
}

#[derive(Debug)]
pub struct Variant {
    pub custom: Option<ExprPath>,
    pub range:Option<ExprRange>,
}

impl Variant{
    pub fn from_ast(error:&Error,variant:&syn::Variant) -> Self{
        let mut custom = Attr::none(CUSTOM);
        let mut range = Attr::none(RANGE);
        for meta_item in variant
        .attrs.iter()
        .flat_map(|attr| get_validate_meta_items(&error, &attr)).flatten(){
            match &meta_item{
                // Paese `#[validate(custom = "foo")]`
                Meta(NameValue(n)) if n.path == CUSTOM => {
                    let litstr =  if let syn::Lit::Str(litstr) = &n.lit{
                        litstr
                    }else{
                        error.push_span_error(&n.lit, format!(
                            "expected validate {} attribute to be a string: `{} = \"...\"`",
                            CUSTOM,CUSTOM
                        ));
                        continue;
                    };
                    if let Ok(expr_path) =  litstr.parse::<ExprPath>().map_err(|_|{
                        error.push_span_error(litstr, format!("failed to parse path: {:?}",litstr))
                    }){
                        custom.set(error, litstr, expr_path);
                    }

                },
                Meta(NameValue(n)) if n.path == RANGE => {
                    let litstr =  if let syn::Lit::Str(litstr) = &n.lit{
                        litstr
                    }else{
                        error.push_span_error(&n.lit, format!(
                            "expected validate {} attribute to be a string: `{} = \"...\"`",
                            RANGE,RANGE
                        ));
                        continue;
                    };
                    if let Ok(expr) =  litstr.parse::<ExprRange>().map_err(|_|{
                        error.push_span_error(litstr, format!("failed to parse path: {:?}",litstr))
                    }){
                        range.set(error, litstr, expr);
                    }

                },
                attr =>{
                    error.push_span_error(&attr, format!( "invalid validate attribute", ));
                }

            }
        }
        Variant{
            custom:custom.value,range:range.value,
        }
    }
}
