#![feature(plugin)]
#![plugin(encoding_literals)]

extern crate encoding;

use encoding::all;
use encoding::types::{Encoding,EncoderTrap,DecoderTrap};

#[test]
pub fn one_byte_escape_sequences_are_properly_encoded() {
    let runtime_encoded = all::UTF_16LE.encode("Te\r\nst", EncoderTrap::Strict).ok().unwrap();
    let literal = utf16!("Te\r\nst");
    assert_eq!("Te\r\nst", all::UTF_16LE.decode(&literal, DecoderTrap::Strict).ok().unwrap());
    assert_eq!(runtime_encoded, literal);
}

#[test]
pub fn three_byte_escape_sequences_are_properly_encoded() {
    let runtime_encoded = all::UTF_16LE.encode("Te\x43st", EncoderTrap::Strict).ok().unwrap();
    let literal = utf16!("Te\x43st");
    assert_eq!("Te\x43st", all::UTF_16LE.decode(&literal, DecoderTrap::Strict).ok().unwrap());
    assert_eq!(runtime_encoded, literal);
}

#[test]
pub fn white_space_escape_sequences_are_properly_encoded() {
    let runtime_encoded = all::UTF_16LE.encode("Te\u{1234}st", EncoderTrap::Strict).ok().unwrap();
    let literal = utf16!("Te\u{1234}st");
    assert_eq!("Te\u{1234}st", all::UTF_16LE.decode(&literal, DecoderTrap::Strict).ok().unwrap());
    assert_eq!(runtime_encoded, literal);
}

#[test]
pub fn utf16_encodes_as_expected() {
    let runtime_encoded = all::UTF_16LE.encode("Test", EncoderTrap::Strict).ok().unwrap();
    let literal = utf16!("Test");
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

#[test]
pub fn c_utf16_encodes_as_expected() {
    let literal = c_utf16!("Test");
    assert_eq!(['T' as u8, 0, 'e' as u8, 0, 's' as u8, 0, 't' as u8, 0, 0, 0], literal);
}

#[test]
pub fn c_utf16be_encodes_as_expected() {
    let literal = c_utf16be!("Test");
    assert_eq!([0, 'T' as u8, 0, 'e' as u8, 0, 's' as u8, 0, 't' as u8, 0, 0], literal);
}

#[test]
pub fn c_utf8_encodes_as_expected() {
    let literal = c_utf8!("Test");
    assert_eq!(['T' as u8, 'e' as u8, 's' as u8, 't' as u8, 0], literal);
}
