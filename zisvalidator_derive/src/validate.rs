use ast::{Data, Style};
use proc_macro2::{ Span, TokenStream};
use syn::Member;
use crate::error::Error;
use crate::ast;
use crate::attr;
use crate::check;

struct Parameters{
    local:syn::Ident,
    
}

impl Parameters{
    fn new(cont:&ast::Container) -> Self{
        let local = cont.ident.clone();

        Parameters{
            local
        }
    }
}

pub fn expand_derive_validate(input:&syn::DeriveInput) -> Result<TokenStream,Vec<syn::Error>>{
    let error = Error::new();
    let cont = match ast::Container::from_ast(&error, &input){
        Some(cont) => cont,
        None => return Err(error.check().unwrap_err()),
    };
    check::check(&error, &cont);

    error.check()?;

    let (_, ty_generics, where_clause) = cont.generics.split_for_impl();
    let ident = cont.ident.clone();
    let params = Parameters::new(&cont);

    let body = validate_body(&cont,&params);
    let block = quote!{
        impl #ty_generics ::zisvalidator::Validate for #ident #ty_generics #where_clause{
            fn validate(&self) -> ::std::result::Result<(),::zisvalidator::error::ValidatorError>{
                #body
                ::std::result::Result::Ok(())
            }
        } 
    };
    Ok(block.into())
}
fn validate_body(cont:&ast::Container,params:&Parameters) -> TokenStream{
    let container_block = validate_container(&cont,);

    let data_block =  match &cont.data{
        ast::Data::Enum(variants) => {
            validate_enum(variants, params)
        },
        ast::Data::Struct(_,fields) => {
            validate_fields(fields,true)
        }
    };

    let block = quote!{
        #container_block
        #data_block
    };
    block.into()
}

fn validate_container(cont:&ast::Container) -> TokenStream{
    let schema = &cont.attrs.schema;
    let schema_block = match schema{
        Some(schema) => {quote!{
            #schema(&self)?;
        }},
        None =>{TokenStream::new()}
    };

    let custom = &cont.attrs.custom;

    let custom_block = match custom{
        Some(custom) => {
            match &cont.data{
                Data::Struct(style,fields) if *style == Style::Newtype || *style == Style::Tuple =>{
                    let mut tokenstream = TokenStream::new();
                    for field in fields{
                        let member = &field.member;
                        tokenstream.extend(quote! {
                            #custom(&self.#member)?;
                        });
                    }
                    tokenstream
                    
                },
                _ =>{TokenStream::new()}
            }
        },
        None => TokenStream::new()
    };
    let range = match &cont.attrs.range{
        Some(range) => {
            match &cont.data{
                Data::Struct(style,fields) if *style == Style::Newtype || *style == Style::Tuple =>{
                    let mut tokenstream = TokenStream::new();
                    let ident = &cont.ident.to_string();
                    for field in fields{
                        let member = &field.member;
                        tokenstream.extend(quote! {
                            if !(#range).contains(&self.#member){
                                let message = format!("invalid {:?}: expect range `{:?}`",#ident,#range);
                                return Err(::zisvalidator::error::ValidatorError{message});
                            }
                        });
                    }
                    tokenstream
                    
                },
                _ =>{TokenStream::new()}
            }
            
        },
        None => TokenStream::new()
    };

    let block = quote! {
        #schema_block
        #custom_block
        #range
    };
    
    block.into()
}

fn validate_enum(variants:&[ast::Variant],params:&Parameters) -> TokenStream{
    let arms = variants
        .iter()
        .map(|variant| {
            validate_variant(
                variant,params
            )
        }).collect::<Vec<_>>();
        // eprintln!("{:#?}",arms);
    quote! {
        match &self{
            #(#arms)*
        }
    }
}
/// #[derive(Validate)]
/// #[validate(custom(function = "",message = ""))]
/// enum TestEnum{
///     #[validate(name)]
///     A(String),
///     B{
///         #[validate(name)]
///         val:String
///     },
///     #[validate(name)]
///     C,
/// }
/// ```
fn validate_variant(variant:&ast::Variant,params:&Parameters) -> TokenStream {
    let ident = &params.local;
    let inner_ident = &variant.ident;

    let case = match variant.style {
        ast::Style::Unit => {
            quote! {
                #ident::#inner_ident
            }
        }
        ast::Style::Newtype => {
            quote! {
                #ident::#inner_ident (ref __field0)
            }
        }
        ast::Style::Tuple => {
            let field_names = (0..variant.fields.len())
                .map(|i| syn::Ident::new(&format!("__field{}", i), Span::call_site()));
            quote! {
                #ident::#inner_ident (#(ref #field_names),*)
            }
        }
        ast::Style::Struct => {
            let members = variant.fields.iter().map(|f| &f.member);
            quote! {
                #ident::#inner_ident { #(ref #members),* }
            }
        }
    };
    //TODO 校验variant body
    let body = match variant.style{
        ast::Style::Unit => TokenStream::new(),
        ast::Style::Newtype => {
            validate_new_type(&variant.ident,&variant.attrs)
        }
        ast::Style::Struct => {
            validate_fields(&variant.fields,false)
        }
        ast::Style::Tuple => {
            let ident = &variant.ident.to_string();
            let field = variant.fields.iter().enumerate()
            .map(|(i,field)| {
                let i = syn::Ident::new(&format!("__field{}", i), Span::call_site());
                let custom = match &field.attrs.custom{
                    Some(custom) => quote! {#custom(&#i)?;},
                    None => TokenStream::new(),
                };
                let range = match &field.attrs.range{
                    Some(range) => quote! {
                        quote!{
                            if !(#range).contains(#i){
                                let message = format!("invalid {:?}: expect range {}",#ident,range);
                                return Err(::zisvalidator::error::ValidatorError{message});
                            }
                        }
                    },
                    None => TokenStream::new(),
                }; 
                quote!{
                    #custom
                    #range
                }
            });
            quote! {
                #(
                    #field
                )*
            }
        }
    };
    quote! {
        #case => {#body}
    }
}

fn validate_new_type(ident:&syn::Ident,attr:&attr::Variant) -> TokenStream{
    let ident = &ident.to_string();
    let custom = match &attr.custom{
        Some(custom) =>{
            quote! {
                #custom(&__field0)?;
            }
        },
        None =>{
            TokenStream::new()
        }
    };
    let range = match &attr.range{
        Some(range) => {
            quote!{
                if !(#range).contains(__field0){
                    let message = format!("invalid {:?}: expect range {:?}",#ident,#range);
                    return Err(::zisvalidator::error::ValidatorError{message});
                }
            }
        },
        None => TokenStream::new(),
    };
    quote! {
        #custom
        #range
    }
}

fn validate_fields(fields:&[ast::Field],is_self:bool) -> TokenStream{
    let field = fields.iter().map(|field|{
        let member = get_member(&field.member, is_self);
        let custom = match &field.attrs.custom{
            Some(custom) =>{
                quote! {
                    #custom(&#member)?;
                }
            },
            None =>{
                TokenStream::new()
            }
        };
        let range = match &field.attrs.range{
            Some(range) => {
                quote!{
                    if !(#range).contains(#member){
                        let message = format!("invalid {:?}: expect range `{:?}`",#member,#range);
                        return Err(::zisvalidator::error::ValidatorError{message});
                    }
                }
            },
            None => TokenStream::new(),
        };
        quote! {
            #custom
            #range
        }
    }).collect::<Vec<_>>();
    quote! {
        #(
            #field
        )*
    }
}

fn get_member(member:&Member,is_self:bool) -> TokenStream{
    if is_self{
        quote! {
            &self.#member
        }
    }else{
        quote!{
            #member
        }
    }
}


