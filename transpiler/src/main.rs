/*

BSD 3-Clause License

Copyright (c) 2025, glomdom

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

*/

#![feature(rustc_private)]

use rustc_session::parse::ParseSess;
use rustc_span::FileName;

extern crate rustc_ast;
extern crate rustc_driver;
extern crate rustc_parse;
extern crate rustc_session;
extern crate rustc_span;

struct ASTVisitor;

impl<'ast> rustc_ast::visit::Visitor<'ast> for ASTVisitor {
    fn visit_crate(&mut self, krate: &'ast rustc_ast::Crate) -> Self::Result {
        for item in &krate.items {
            match &item.kind {
                rustc_ast::ItemKind::ExternCrate(symbol) => todo!(),
                rustc_ast::ItemKind::Use(use_tree) => todo!(),
                rustc_ast::ItemKind::Static(static_item) => todo!(),
                rustc_ast::ItemKind::Const(const_item) => todo!(),
                rustc_ast::ItemKind::Fn(r#fn) => todo!(),
                rustc_ast::ItemKind::Mod(safety, mod_kind) => todo!(),
                rustc_ast::ItemKind::ForeignMod(foreign_mod) => todo!(),
                rustc_ast::ItemKind::GlobalAsm(inline_asm) => todo!(),
                rustc_ast::ItemKind::TyAlias(ty_alias) => todo!(),
                rustc_ast::ItemKind::Enum(enum_def, generics) => todo!(),
                rustc_ast::ItemKind::Struct(variant_data, generics) => todo!(),
                rustc_ast::ItemKind::Union(variant_data, generics) => todo!(),
                rustc_ast::ItemKind::Trait(_) => todo!(),
                rustc_ast::ItemKind::TraitAlias(generics, vec) => todo!(),
                rustc_ast::ItemKind::Impl(_) => todo!(),
                rustc_ast::ItemKind::MacCall(p) => todo!(),
                rustc_ast::ItemKind::MacroDef(macro_def) => todo!(),
                rustc_ast::ItemKind::Delegation(delegation) => todo!(),
                rustc_ast::ItemKind::DelegationMac(delegation_mac) => todo!(),
            }
        }
    }

    fn visit_fn(
        &mut self,
        fk: rustc_ast::visit::FnKind<'ast>,
        _: rustc_span::Span,
        _: rustc_ast::NodeId,
    ) -> Self::Result {
        println!("sack");
    }
}

fn main() {
    rustc_span::create_default_session_globals_then(|| {
        let source = r#"
            fn main() {
                // test comment
            }"#;
        let parser_session = ParseSess::new(vec![]);

        let parser_result = rustc_parse::new_parser_from_source_str(
            &parser_session,
            FileName::Custom("anonymous".into()),
            source.into(),
        );
        let mut parser = rustc_parse::unwrap_or_emit_fatal(parser_result);
        let krate = parser.parse_crate_mod().unwrap();

        dbg!(&krate);

        let mut visitor = ASTVisitor {};
        rustc_ast::visit::Visitor::visit_crate(&mut visitor, &krate);
    });
}
