use logger::Logger;
use swc_common::{FileName, SourceMap, comments::SingleThreadedComments, sync::Lrc};
use swc_ecma_ast::Module;
use swc_ecma_codegen::{Emitter, text_writer::JsWriter};
use swc_ecma_parser::{self, Parser};

mod lexer;
mod normalizer;

pub fn parse(source: &str) -> Module {
    let comments: SingleThreadedComments = SingleThreadedComments::default();
    // SourceMap manages source files and resolves byte positions to source locations
    // It can inform us about exact position of Error, element, code etc.
    // In the next generation we can add linter using this SourceMap, because we will be able to locate exact 'heavy' function or part of code
    // OR we will be able to show users what is not suppoertd for now
    let source_map: Lrc<SourceMap> = Default::default();
    let program = source_map.new_source_file(
        FileName::Custom("input.ts".into()).into(),
        source.to_owned(),
    );

    // Lexer is a just list of tokens (parts of code like 'function', '(', ')', '{'...})
    let lexer = lexer::lexer(program.as_ref(), &comments);
    let mut parser = Parser::new_from(lexer);
    let ast = parser.parse_module().unwrap(); // TODO: handle error

    let normalized_ast = normalizer::normalizer(ast);

    Logger::step("convert source to ast-module", "parser");

    normalized_ast
}

pub fn normalize_source_to_es5(normalized_ast: Module) -> String {
    let source_map: Lrc<SourceMap> = Default::default();
    let mut buf = Vec::new();
    let comments: SingleThreadedComments = SingleThreadedComments::default();
    let writer = JsWriter::new(source_map.clone(), "\n", &mut buf, None);

    let mut emitter = Emitter {
        cfg: Default::default(),
        cm: source_map.clone(),
        comments: Some(&comments),
        wr: Box::new(writer),
    };

    emitter.emit_module(&normalized_ast).unwrap();

    let normalized_source = String::from_utf8(buf).unwrap();

    Logger::step("convert source to ast-module", "parser");

    normalized_source
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        swc_common::GLOBALS.set(&Default::default(), || {
            // parse();
        });
    }
}
