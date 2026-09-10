use std::{println, todo};
use swc_ecma_ast::{
    Decl, Expr, Lit, Module,
    ModuleItem::{self},
    Stmt,
};
use proc_macro2::TokenStream;
mod helpers;
mod transform;

pub fn codegen(ast: Module) -> String {
    let mut output = TokenStream::new();

    for node in ast.body {
        println!("1) for - node: {:#?}", node);
        let generated = match node {
            ModuleItem::Stmt(stmt) => dispatch_stmt(stmt),
            _ => TokenStream::new(), // Handle other module items as needed
        };
        output.extend(generated);
    }
    output.to_string()
}

fn dispatch_stmt(stmt: Stmt) -> TokenStream {
    match stmt {
        Stmt::Decl(decl) => dispatch_decl(decl),
        _ => TokenStream::new(), // Handle other statements as needed
    }
}

fn dispatch_decl(decl: Decl) -> TokenStream {
    match decl {
        Decl::Var(var_decl) => transform::variable_declaration(*var_decl),
        _ => TokenStream::new(), // Handle other declarations as needed
    }
}

fn dispatch_expr(expr: Expr) -> TokenStream {
    match expr {
        Expr::Lit(lit) => dispatch_literal(lit),
        _ => todo!("Handle other expressions as needed"),
    }
}

pub fn dispatch_literal(lit: Lit) -> TokenStream {
    match lit {
        Lit::Str(str_lit) => transform::literal_string(str_lit),
        Lit::Num(num_lit) => transform::literal_number(num_lit),
        _ => todo!("Handle other literal types as needed"),
    }
}