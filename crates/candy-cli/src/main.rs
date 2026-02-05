use candy_parser::parse_program;
use candy_typecheck::typecheck;

fn main() {
    let src = r#"
        fn main() -> i32 { 42 }
    "#;

    let ast = parse_program(src).expect("parse failed");
    typecheck(&ast).expect("typecheck failed");
    println!("Candy🍭: parse+typecheck OK");
}
