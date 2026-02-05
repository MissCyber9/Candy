use candy_ast::{Function, Program};

pub fn parse_program(_src: &str) -> Result<Program, String> {
    // MVP stub: on fait comme si on avait trouvé "main"
    Ok(Program {
        functions: vec![Function { name: "main".into() }],
    })
}
