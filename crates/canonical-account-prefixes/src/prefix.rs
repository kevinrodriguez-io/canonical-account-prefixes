extern crate proc_macro;
use quote::quote;

pub(crate) fn prefix_impl(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let name_lower = input.to_string().to_lowercase();
    let len: usize = name_lower.len();
    let bytes = name_lower.as_bytes().iter().map(|b| *b as u8);
    if cfg!(feature = "anchor") {
        quote! {
            #[constant]
            pub const #input: [u8; #len] = [#(#bytes),*];
        }
    } else {
        quote! {
            pub const #input: [u8; #len] = [#(#bytes),*];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix() {
        let expanded = prefix_impl(quote! {OTHER});
        assert_eq!(
            expanded.to_string(),
            quote! {
                pub const OTHER: [u8; 5usize] = [111u8, 116u8, 104u8, 101u8, 114u8];
            }
            .to_string()
        );
    }
}
