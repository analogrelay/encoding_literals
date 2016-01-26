#![feature(quote, plugin_registrar, rustc_private, slice_patterns)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;
extern crate encoding;

use syntax::ast;
use syntax::codemap;
use syntax::ptr::P;
use syntax::ext::build::AstBuilder;
use syntax::ext::base::{ExtCtxt, MacResult, MacEager, DummyResult, get_single_str_from_tts};

use encoding::types::{EncoderTrap, Encoding};

use rustc_plugin::Registry;

fn expand<T>(name: &str,
             encoding: &T,
             ct: &mut ExtCtxt,
             sp: codemap::Span,
             args: &[ast::TokenTree],
             c_str: bool,
             size_in_bytes: isize) -> Box<MacResult> where T: Encoding {

    let text = match get_single_str_from_tts(ct, sp, args, name) {
        Some(text) => text,
        None => return DummyResult::expr(sp)
    };

    // Encode the string
    let encoded = match encoding.encode(&text, EncoderTrap::Strict) {
        Ok(vec) => vec,
        Err(_) => {
            ct.span_err(sp, &format!("{}: literal could not be encoded to {}", name, encoding.name()));
            return DummyResult::expr(sp);
        }
    };

    // Generate the expression
    let mut bytes : Vec<P<ast::Expr>> = encoded
        .iter()
        .map(|v: &u8| -> P<ast::Expr> { ct.expr_u8(sp, *v) })
        .collect();
    if c_str {
        for _ in 0..size_in_bytes {
            bytes.push(ct.expr_u8(sp, 0));
        }
    }

    let expr = ct.expr(sp, ast::Expr_::ExprVec(bytes));
    MacEager::expr(expr)
}

fn expand_c_utf8(ct: &mut ExtCtxt, sp: codemap::Span, args: &[ast::TokenTree]) -> Box<MacResult> {
    expand("c_utf8", encoding::all::UTF_8, ct, sp, args, true, 1)
}

fn expand_c_utf16(ct: &mut ExtCtxt, sp: codemap::Span, args: &[ast::TokenTree]) -> Box<MacResult> {
    expand("c_utf16", encoding::all::UTF_16LE, ct, sp, args, true, 2)
}

fn expand_c_utf16be(ct: &mut ExtCtxt, sp: codemap::Span, args: &[ast::TokenTree]) -> Box<MacResult> {
    expand("c_utf16be", encoding::all::UTF_16BE, ct, sp, args, true, 2)
}

fn expand_utf16(ct: &mut ExtCtxt, sp: codemap::Span, args: &[ast::TokenTree]) -> Box<MacResult> {
    expand("utf16", encoding::all::UTF_16LE, ct, sp, args, false, 2)
}

fn expand_utf16be(ct: &mut ExtCtxt, sp: codemap::Span, args: &[ast::TokenTree]) -> Box<MacResult> {
    expand("utf16be", encoding::all::UTF_16BE, ct, sp, args, false, 2)
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("utf16", expand_utf16);
    reg.register_macro("utf16be", expand_utf16be);
    reg.register_macro("c_utf8", expand_c_utf8);
    reg.register_macro("c_utf16", expand_c_utf16);
    reg.register_macro("c_utf16be", expand_c_utf16be);
}
