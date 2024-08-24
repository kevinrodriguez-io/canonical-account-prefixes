# Canonical account prefixes

![Logo](./images/MIN.jpg)

[![Crates.io](https://img.shields.io/crates/v/canonical_account_prefixes)](https://crates.io/crates/canonical_account_prefixes)
[![Docs.rs](https://docs.rs/canonical_account_prefixes/badge.svg)](https://docs.rs/canonical_account_prefixes)

Generates a const prefix for a given account name. This is useful for generating
a prefix for a given account xname that can be used in a Solana Program.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
canonical_account_prefixes = "0.1.0"
```

If you're using `anchor`, you can enable the `anchor` feature:

```toml
[dependencies]
canonical_account_prefixes = { version = "0.1.0", features = ["anchor"] }
```

## Usage

### The `prefix!` macro

```rust
use canonical_account_prefixes::prefix;

prefix!(OTHER)
```

This generates the following const:

```rust
#[const] // If feature=anchor is enabled
pub const OTHER: [u8; 5] = [111, 116, 104, 101, 114];
```

Which is the equivalent of writing:

```rust
#[const] // If feature=anchor is enabled
pub const OTHER: [u8; 5] = *b"other";
```

That way you don't have to specify the string length.

### The `prefix_account` attribute macro

```rust
use canonical_account_prefixes::prefix_account;

#[prefix_account]
pub struct Other {
    pub data: [u8; 32],
}
```

Which is the equivalent of writing:

```rust
pub struct Other {
    pub data: [u8; 32],
}
#[const] // If feature=anchor is enabled
pub const OTHER: [u8; 5] = *b"other";
```

For account names in `UpperCamelCase`, the macro will do the following:

```rust
#[prefix_account]
pub struct UpperCamelCase {
    pub data: [u8; 32],
}
```

Will generate:

```rust
pub struct UpperCamelCase {
    pub data: [u8; 32],
}
#[const] // If feature=anchor is enabled
pub const UPPER_CAMEL_CASE: [u8; 16] = *b"upper_camel_case";
```

## License

You can use this code under the MIT license. See [LICENSE](./LICENSE.md) for more details.
