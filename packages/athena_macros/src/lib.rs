use proc_macro::TokenStream;
use quote::quote;
use syn::{
    braced, parenthesized, parse::{Parse, ParseStream, Result}, parse_macro_input, Attribute, Data, DeriveInput, Expr, Ident, Meta, Path, Token, Type
};

/// The input format is:
/// 
/// generate_display! {
///     #[display(users::Model)]
///     DisplayUser {
///         roles = Vec<DisplayRole>: Vec::new()
///     }
/// }
struct GenerateDisplayInput {
    display_attr: DisplayAttr,
    struct_name: Ident,
    fields: Vec<DisplayField>,
}

impl Parse for GenerateDisplayInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs: Vec<Attribute> = input.call(Attribute::parse_outer)?;

        if attrs.len() != 1 {
            return Err(input.error("expected exactly one #[display(...)] attribute"));
        }
        
        let attr = &attrs[0];

        if !attr.path.is_ident("display") {
            return Err(input.error("expected attribute #[display(...)]"));
        }

        let display_attr = syn::parse2::<DisplayAttr>(attr.tokens.clone())?;
        let struct_name: Ident = input.parse()?;
        let mut fields = Vec::new();
        let content;
        
        braced!(content in input);

        while !content.is_empty() {
            let field = content.parse::<DisplayField>()?;
            fields.push(field);

            // Handle trailing comma.
            if content.peek(Token![,]) {
                let _ : Token![,] = content.parse()?;
            }
        }
        
        Ok(GenerateDisplayInput { display_attr, struct_name, fields })
    }
}

struct DisplayAttr {
    model: Path,
}

impl Parse for DisplayAttr {
    fn parse(input: ParseStream) -> Result<Self> {
         let content;
         parenthesized!(content in input);
         let model: Path = content.parse()?;
         Ok(DisplayAttr { model })
    }
}

/// field_name = FieldType: initializer_expr
struct DisplayField {
    name: Ident,
    field_type: Type,
    initializer: Expr,
}

impl Parse for DisplayField {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let _: Token![=] = input.parse()?;
        let field_type: Type = input.parse()?;
        let _: Token![:] = input.parse()?;
        let initializer: Expr = input.parse()?;
        Ok(DisplayField { name, field_type, initializer })
    }
}

#[proc_macro]
pub fn generate_display(input: TokenStream) -> TokenStream {
    let GenerateDisplayInput {
        display_attr,
        struct_name,
        fields,
    } = parse_macro_input!(input as GenerateDisplayInput);
    
    let model = display_attr.model;
    
    let base_field = quote! {
        #[serde(skip)]
        pub base: #model
    };
    
    let fields_struct = fields.iter().map(|f| {
        let name = &f.name;
        let field_type = &f.field_type;
        quote! {
            pub #name: #field_type
        }
    });
    
    let fields_init = fields.iter().map(|f| {
        let name = &f.name;
        let initializer = &f.initializer;
        quote! {
            #name: #initializer
        }
    });
    
    let expanded = quote! {
        #[derive(serde::Serialize)]
        pub struct #struct_name {
            #base_field,
            #(#fields_struct,)*
        }
        
        impl #struct_name {
            pub fn new(base: #model) -> Self {
                Self {
                    base: base.clone(),
                    #(#fields_init,)*
                }
            }
        }
    };
    
    TokenStream::from(expanded)
}


#[proc_macro_attribute]
pub fn request_error(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = input.ident;

    let data_enum = match input.data {
        Data::Enum(data) => data,
        _ => {
            return syn::Error::new_spanned(
                enum_name,
                "RequestError can only be derived for enums",
            )
            .to_compile_error()
            .into()
        }
    };

    let mut new_variants = Vec::new();
    let mut status_code_arms = Vec::new();
    let mut display_arms = Vec::new();
    let mut error_code_arms = Vec::new();

    for variant in data_enum.variants.iter() {
        let variant_ident = &variant.ident;
        let variant_fields = &variant.fields;

        let preserved_attrs: Vec<_> = variant
            .attrs
            .iter()
            .filter(|attr| !attr.path.is_ident("error"))
            .collect();

        let error_attr = variant
            .attrs
            .iter()
            .find(|attr| attr.path.is_ident("error"));

        if let Some(attr) = error_attr {
            let meta = attr.parse_meta().expect("Unable to parse error attribute");
            if let Meta::List(meta_list) = meta {
                let nested: Vec<_> = meta_list.nested.into_iter().collect();

                if nested.len() != 3 {
                    return syn::Error::new_spanned(
                        attr,
                        "Expected three attribute arguments: status code, error code, display message",
                    )
                    .to_compile_error()
                    .into();
                }

                let status = &nested[0];
                let error_code = &nested[1];
                let display_msg = &nested[2];

                match variant_fields {
                    syn::Fields::Named(_) | syn::Fields::Unnamed(_) => {
                        // For variants with fields, match on the variant and bind its fields.
                        status_code_arms.push(quote! {
                            #enum_name::#variant_ident { .. } => #status,
                        });

                        error_code_arms.push(quote! {
                            #enum_name::#variant_ident { .. } => #error_code,
                        });

                        display_arms.push(quote! {
                            #enum_name::#variant_ident { .. } => write!(f, #display_msg),
                        });
                    }

                    syn::Fields::Unit => {
                        // For unit-like variants, match directly on the variant.
                        status_code_arms.push(quote! {
                            #enum_name::#variant_ident => #status,
                        });

                        error_code_arms.push(quote! {
                            #enum_name::#variant_ident => #error_code,
                        });

                        display_arms.push(quote! {
                            #enum_name::#variant_ident => write!(f, #display_msg),
                        });
                    }
                }
                
                new_variants.push(quote! {
                    #(#preserved_attrs)*
                    #[display(fmt = #display_msg)]
                    #variant_ident #variant_fields
                });
            } else {
                return syn::Error::new_spanned(
                    attr,
                    "The #[error] attribute must be in list form",
                )
                .to_compile_error()
                .into();
            }
        } else {
            new_variants.push(quote! {
                #variant_ident #variant_fields
            });
        }
    }

    let expanded = quote! {
        #[derive(Debug, derive_more::Display, serde::Serialize, Clone)]
        #[serde(untagged)]
        pub enum #enum_name {
            #(#new_variants),*
        }

        impl actix_web::ResponseError for #enum_name {
            fn status_code(&self) -> actix_web::http::StatusCode {
                match self {
                    #(#status_code_arms)*
                }
            }

            fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
                actix_web::HttpResponse::build(self.status_code()).json(serde_json::json!({
                    "status": self.status_code().as_u16(),
                    "error": self.get_error_code(),
                    "message": self.to_string(),
                    "data": self
                }))
            }
        }

        impl athena::RequestError for #enum_name {
            fn get_error_code(&self) -> &str {
                match self {
                    #(#error_code_arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}