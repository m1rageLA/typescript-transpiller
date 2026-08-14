use logger::Logger;
use swc_ecma_ast::{Decl, Module, ModuleItem, Stmt};

mod transformers;

use transformers::*;
// -------------------------------
// LEVEL 1
// ------------------------------
pub fn lowering(ast: Module) -> Result<Module, String> {
    for item in ast.body.iter() {
        match item {
            ModuleItem::Stmt(stmt) => handle_stmt(stmt),

            _ => {
                Logger::not_supported(
                    &format!("Module item: {:?} is not supported", item),
                    "lowering",
                );
                return Err("[lowering] Module item is not supported!".to_string());
            }
        }
    }
    Ok(ast)
}

// -------------------------------
// LEVEL 2
// ------------------------------
fn handle_stmt(node: &Stmt) {
    match node {
        Stmt::Decl(decl) => handle_decl(decl),

        _ => Logger::not_supported(
            &format!("Statement: {:?} is not part of the ES5 standard", node),
            "lowering",
        ),
    }
}

// -------------------------------
// LEVEL 3
// ------------------------------
fn handle_decl(decl: &Decl) {
    match decl {
        Decl::Var(var) => var_decl::transform_var_decl(var),

        _ => Logger::not_supported(
            &format!("Declaration: {:?} is not part of the ES5 standard", decl),
            "lowering",
        ),
    }
}