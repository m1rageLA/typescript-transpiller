use logger::Logger;
use swc_ecma_ast::{Module, ModuleItem, Stmt};
// ModuleItem;

// entry point for lowering
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

fn handle_stmt(stmt: &Stmt) {
    match stmt {
        // Core
        Stmt::Decl(_) => {}
        Stmt::Expr(_) => {}
        // Other statements
        Stmt::Block(_) => {}
        Stmt::Empty(_) => {}
        Stmt::Debugger(_) => {}
        Stmt::With(_) => {}
        // Control flow statements
        Stmt::Return(_) => {}
        Stmt::Labeled(_) => {}
        Stmt::Break(_) => {}
        Stmt::Continue(_) => {}
        // Choice statements
        Stmt::If(_) => {}
        Stmt::Switch(_) => {}
        // Loops
        Stmt::While(_) => {}
        Stmt::DoWhile(_) => {}
        Stmt::For(_) => {}
        Stmt::ForIn(_) => {}
        //
        _ => Logger::not_supported(
            &format!("Statement: {:?} is not part of the ES5 standard", stmt),
            "lowering",
        ),
    }
}
