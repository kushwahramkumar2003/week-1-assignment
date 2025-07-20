use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

fn snake_to_pascal(s: &str) -> String {
    s.split('_').map(|word| {
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
            first.to_uppercase().collect::<String>() + chars.collect::<String>().as_str()
        } else {
            String::new()
        }
    }).collect()
}

#[proc_macro_attribute]
pub fn todo_app(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as DeriveInput);

    if let syn::Data::Struct(ref mut data_struct) = input.data {
        if let syn::Fields::Named(ref mut fields) = data_struct.fields {
            for field in fields.named.iter_mut() {
                let field_name = field.ident.as_ref().unwrap().to_string();
                let pascal_name = snake_to_pascal(&field_name);
                let new_name = format!("TodoApp{}", pascal_name);
                let rename_attr: syn::Attribute = syn::parse_quote! { #[serde(rename = #new_name)] };
                field.attrs.push(rename_attr);
            }
        }
    }

    quote!( #input ).into()
}