# UTF-16 literals for Rust
This crate provides a plugin that allows you to add UTF-16 string literals to your Rust code.

Example:

```rust
#![plugin(encoding_literals)]

pub fn something() {
    // The type of this variable will be [u8; N] where N is the length of the UTF-16 encoded sequence in bytes.
    let utf_literal = utf16!("This string will be a UTF-16 byte sequence");
}
