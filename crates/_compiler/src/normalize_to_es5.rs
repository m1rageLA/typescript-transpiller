use logger::Logger;
use parser::{normalize_source_to_es5, parse};

pub fn normalize_to_es5(source: &str) -> String {
    let normalized_ast = parse(source);

    let result = normalize_source_to_es5(normalized_ast);

    Logger::success("finished parsing", "_compiler");

    result
}
