use quote::quote;
use syn::{parse_str, ItemStruct};

use crate::prefix::prefix_impl;

// Converts UpperCamelCase to snail_case
fn upper_camel_case_to_snake_case(upper_camel_case: &str) -> String {
    let mut snake_case = String::new();
    for c in upper_camel_case.chars() {
        if c.is_uppercase() {
            if !snake_case.is_empty() {
                snake_case.push('_');
            }
            snake_case.push(c.to_ascii_lowercase());
        } else {
            snake_case.push(c);
        }
    }
    snake_case
}

pub(crate) fn prefix_account_impl(
    _attr: proc_macro2::TokenStream,
    item: ItemStruct,
) -> proc_macro2::TokenStream {
    let snake_case = upper_camel_case_to_snake_case(&item.ident.to_string()).to_uppercase();
    let prefix = prefix_impl(parse_str(snake_case.as_str()).unwrap());
    quote! {
        #item
        #prefix
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::*;

    #[test]
    fn test_prefix_account() {
        let input: proc_macro2::TokenStream = quote! {
            pub struct MyAccount {
                pub data: u64,
            }
        };
        let attr = quote! {""};
        let result = prefix_account_impl(attr, syn::parse2(input).unwrap());
        let expected_output = quote! {
            pub struct MyAccount {
                pub data : u64,
            }
            pub const MY_ACCOUNT: [u8; 10usize] = [109u8, 121u8, 95u8, 97u8, 99u8, 99u8, 111u8, 117u8, 110u8, 116u8];
        };
        assert_eq!(result.to_string(), expected_output.to_string());
    }
}
