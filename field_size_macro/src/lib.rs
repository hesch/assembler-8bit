extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(FieldSize)]
pub fn derive_field_size(input: TokenStream) -> TokenStream {
    let syn_item: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &syn_item.ident;
    let len = match syn_item.data {
        syn::Data::Enum(enum_item) => enum_item.variants.len(),
        _ => panic!("FieldSize only works on Enums"),
    };
    let expanded = quote! {
        impl FieldSize for #name {
            fn field_size() -> usize {
                #len
            }
        }
    };
    expanded.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn does_not_compile() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/failing/non_enum.rs");
    }
}
