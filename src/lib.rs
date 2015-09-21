#![feature(quote, plugin_registrar, rustc_private, slice_patterns)]

extern crate syntax;
extern crate rustc;
extern crate encoding;

use syntax::ast;
use syntax::codemap;
use syntax::ptr::P;
use syntax::parse::token;
use syntax::ext::build::AstBuilder;
use syntax::ext::base::{ExtCtxt,MacResult,MacEager,DummyResult};

use encoding::types::{EncoderTrap,Encoding};

use rustc::plugin;

fn expand<T>(encoding: &T, ct: &mut ExtCtxt, sp: codemap::Span, args: &[ast::TokenTree]) -> Box<MacResult> where T: Encoding {
    let text = match args {
        [ast::TtToken(_, token::Literal(token::Lit::Str_(s), _))] => s.to_string(),
        _ => {
            ct.span_err(sp, "argument should be a single string literal");
            return DummyResult::any(sp);
        }
    };

    // Encode the string
    let encoded = match encoding.encode(&text, EncoderTrap::Strict) {
        Ok(vec) => vec,
        Err(_) => {
            ct.span_err(sp, &format!("literal could not be encoded to {}", encoding.name()));
            return DummyResult::any(sp);
        }
    };

    // Generate the expression
    let expr = ct.expr(sp, ast::Expr_::ExprVec(encoded
        .iter()
        .map(|v: &u8| -> P<ast::Expr> { ct.expr_u8(sp, *v) })
        .collect()));
    MacEager::expr(expr)
}

fn expand_utf16(ct: &mut ExtCtxt, sp: codemap::Span, args: &[ast::TokenTree]) -> Box<MacResult> {
    expand(encoding::all::UTF_16LE, ct, sp, args)
}

fn expand_utf16be(ct: &mut ExtCtxt, sp: codemap::Span, args: &[ast::TokenTree]) -> Box<MacResult> {
    expand(encoding::all::UTF_16BE, ct, sp, args)
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut plugin::Registry) {
    reg.register_macro("utf16", expand_utf16);
    reg.register_macro("utf16le", expand_utf16);
    reg.register_macro("utf16be", expand_utf16be);
}
