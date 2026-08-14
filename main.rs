use std::println;

use compiler::normalize_to_es5::normalize_to_es5;

fn main() {
    let rusult = normalize_to_es5("const foo = () => {return 5}");
    println!("{}", rusult);
}
