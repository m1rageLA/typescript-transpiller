use logger::Logger;
use lowering::transform;
use parser::parse;

pub fn transform_to_ir(source: &str) {
    let ast = parse(source);
    transform(ast);
    Logger::success("finished parsing", "_compiler");
}
