mod prefix;
mod prefix_account;

use prefix::prefix_impl;
use prefix_account::prefix_account_impl;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro]
/// Generates a constant array of bytes from the input identifier,
/// making it lowercase and converting it to a byte array.
///
/// # Example
///
/// ```ignore
/// use anchor_lang::prelude::*;
/// use magic_macros::prefix;
///
/// prefix!(BUNDLE);
/// // Which is the equivalent of writing:
/// #[constant]
/// pub const BUNDLE: [u8; 6] = *b"bundle";
/// ```
pub fn prefix(input: TokenStream) -> TokenStream {
    TokenStream::from(prefix_impl(input.into()))
}

#[proc_macro_attribute]
/// Generates a constant array of bytes from the input identifier,
/// making it lowercase and converting it to a byte array.
/// 
/// # Example
/// 
/// ```ignore
/// use anchor_lang::prelude::*;
/// use magic_macros::prefix_account;
/// 
/// #[prefix_account]
/// pub struct Bundle {
///    pub name: String,
///    pub authority: Pubkey,
/// }
/// 
/// // Which is the equivalent of writing:
/// pub struct Bundle {
///   pub name: String,
///   pub authority: Pubkey,
/// }
/// #[const]
/// pub const BUNDLE: [u8; 6] = *b"bundle";
/// ```
pub fn prefix_account(attr: TokenStream, item: TokenStream) -> TokenStream {
    TokenStream::from(prefix_account_impl(
        attr.into(),
        parse_macro_input!(item as ItemStruct),
    ))
}
