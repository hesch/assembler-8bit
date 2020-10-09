
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

    let variants_ident = variants.iter().map(|v| &v.ident);

    let variant_index = 0u8..(variants.len() as u8);

    let name = &ast.ident;
    let variants_fields = variants.iter().map(|v| {
        let len;
        if let syn::Fields::Unnamed(unnamed_fields) = &v.fields {
            len = unnamed_fields.unnamed.len();
        } else {
            len = 0 
        }
        (0..len)
    });

    let variant_field_code = variants_fields.map(|f_i| {
        if f_i.is_empty() {
            quote! {}
        } else {
            let test = "test";
            quote! {
                (#(field#f_i),*)
            }
        }
    });
    
    let generated_code = quote! {
        impl GenMicrocode for #name {
            fn test() {
            }
        }

        impl From<#name> for u8 {
            fn from(keywords: #name) -> Self {
                match keywords {
                    #( #name::#variants_ident#variant_field_code => #variant_index, )* 
                }
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
