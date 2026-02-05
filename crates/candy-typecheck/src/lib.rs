use candy_ast::Program;

pub fn typecheck(p: &Program) -> Result<(), String> {
    let has_main = p.functions.iter().any(|f| f.name == "main");
    if !has_main {
        return Err("No main() found".into());
    }
    Ok(())
}
