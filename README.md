# UTF-16 literals for Rust
This crate provides a plugin that allows you to add UTF-16 string literals to your Rust code.

Example:

```rust
#![plugin(encoding_literals)]

pub fn something() {
    // The type of this variable will be [u8; N] where N is the length of the UTF-16 encoded sequence in bytes.
    let utf_literal = utf16!("This string will be a UTF-16 byte sequence");
}

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
