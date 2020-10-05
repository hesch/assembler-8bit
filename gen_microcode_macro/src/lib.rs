extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::Data;

#[proc_macro_derive(gen_microcode)]
pub fn gen_microcode_macro(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let variants;

    if let Data::Enum(data) = ast.data {
        variants = data.variants;
    } else {
        panic!("this derive only works on Enums");
    };

    let name = &ast.ident;
    let variants = variants.iter().map(|v| {
        let x;
        if let syn::Fields::Unnamed(fields) = &v.fields {
            x = &fields.unnamed.first().unwrap().ty;
        } else {
            panic!();
        }
        x
    });
    
    let generated_code = quote! {
        impl GenMicrocode for #name {
            fn test() {
            }
        }

        impl From<#name> for u8 {
            fn from(keywords: #name) -> Self {
               0 
            }
        }
    };

    generated_code.into()
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
