#![crate_type="dylib"]
#![feature(plugin_registrar, box_syntax, rustc_private, plugin)]

#![allow(unused)]


extern crate syntax;

#[macro_use]
extern crate rustc;

use syntax::ast;
use syntax::parse::token;
use rustc::plugin::Registry;

use syntax::ext::base;
use syntax::codemap;
use syntax::ptr::P;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(token::intern("dangerous"), 
                                  base::Decorator(Box::new(self::expand)));
}

pub fn expand(ecx: &mut base::ExtCtxt, span: codemap::Span, 
              meta_item: &ast::MetaItem, item: &ast::Item, 
              push: Box<FnMut(P<ast::Item>)>) {
    ecx.span_warn(span, &format!("Item '{}' is dangerous!", 
                                 token::get_ident(item.ident).as_slice()));
}

