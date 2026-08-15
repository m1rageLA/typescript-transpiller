use schema::declarations::{VariableDeclaration};
use swc_ecma_ast::VarDecl;

use crate::transform;

pub(crate) fn transform_var_decl(node: &VarDecl) -> VariableDeclaration {

    for decl in &node.decls {
        transform(decl);
    }

    VariableDeclaration {
        declarations: transformed_declarations,
        position: Position {
            line: node.span.lo().line,
            column: node.span.lo().col,
        }
    }
}

