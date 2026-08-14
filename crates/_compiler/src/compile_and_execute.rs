use logger::Logger;
use lowering::lowering;
use parser::parse;

pub fn compile_and_execute(source: &str) {
    // Parse the source code into an AST module
    // string -> Module
    let normalized_ast = parse(source);

    // let ir = lower(normalized_ast);
    // Logger::success("ast-module to ir", "compiler");

    // let hir = hir::lower(normalized_ast);
    // Logger::success("ast-module to hir", "compiler");

    let ir = lowering(normalized_ast);

    Logger::success("finished compiling", "_compiler");
}
