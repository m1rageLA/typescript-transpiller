use proc_macro2::TokenStream;
use quote::quote;
use swc_ecma_ast::{Lit, Pat, VarDecl};

use crate::{dispatch_expr, helpers};


pub fn variable_declaration(var_decl: VarDecl) -> TokenStream {
    let declarations = var_decl.decls.iter().map(|decl| {
        let id = match &decl.name {
            Pat::Ident(ident) => helpers::rewerite_ident(ident.id.clone()),
            _ => panic!("Unsupported pattern in variable declaration"),
        };

        let init_expr = match &decl.init {
            Some(expr) => dispatch_expr(*expr.clone()),
            None => quote! { () },
        };

        quote! {
            let mut #id = #init_expr;
        }
    });

    quote! {
        #(#declarations)*
    }
}

pub fn literal_string(str_lit: swc_ecma_ast::Str) -> TokenStream {
    let val = str_lit.value.as_str().unwrap();
    let literal = proc_macro2::Literal::string(val);

    quote! {
        #literal
    }
}