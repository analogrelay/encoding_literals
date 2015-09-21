#![feature(plugin)]
#![plugin(encoding_literals)]

extern crate encoding;

use encoding::all;
use encoding::types::{Encoding,EncoderTrap,DecoderTrap};

#[test]
pub fn utf16_encodes_as_expected() {
    let runtime_encoded = all::UTF_16LE.encode("Test", EncoderTrap::Strict).ok().unwrap();
    let literal = utf16!("Test");
    assert_eq!(runtime_encoded, literal);
    assert_eq!(['T' as u8, 0, 'e' as u8, 0, 's' as u8, 0, 't' as u8, 0], literal);
    assert_eq!("Test", all::UTF_16LE.decode(&literal, DecoderTrap::Strict).ok().unwrap());
}

#[test]
pub fn utf16le_encodes_as_expected() {
    let runtime_encoded = all::UTF_16LE.encode("Test", EncoderTrap::Strict).ok().unwrap();
    let literal = utf16le!("Test");
    assert_eq!(runtime_encoded, literal);
    assert_eq!(['T' as u8, 0, 'e' as u8, 0, 's' as u8, 0, 't' as u8, 0], literal);
    assert_eq!("Test", all::UTF_16LE.decode(&literal, DecoderTrap::Strict).ok().unwrap());
}

#[test]
pub fn utf16be_encodes_as_expected() {
    let runtime_encoded = all::UTF_16BE.encode("Test", EncoderTrap::Strict).ok().unwrap();
    let literal = utf16be!("Test");
    assert_eq!(runtime_encoded, literal);
    assert_eq!([0, 'T' as u8, 0, 'e' as u8, 0, 's' as u8, 0, 't' as u8], literal);
    assert_eq!("Test", all::UTF_16BE.decode(&literal, DecoderTrap::Strict).ok().unwrap());
}
