use proc_macro::TokenStream;
use quote::quote;
use syn::{
    braced, parenthesized,
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Attribute, Expr, Ident, Path, Token, Type,
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
    
    // Generate the base field with #[serde(skip)].
    let base_field = quote! {
        #[serde(skip)]
        pub base: #model
    };
    
    // Generate the additional fields.
    let fields_struct = fields.iter().map(|f| {
        let name = &f.name;
        let field_type = &f.field_type;
        quote! {
            pub #name: #field_type
        }
    });
    
    // Generate the field initializations in the new() function.
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
