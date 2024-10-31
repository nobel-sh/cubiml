pub mod ast;

use ast::*;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar);


pub fn parse(source: &str) -> Result<Program, String> {
    println!("{}", source);
    let parsed = grammar::ScriptParser::new().parse(source).unwrap();
    Ok(parsed)
}

fn main() {
    parse("let x = true; let y = x ").unwrap();
}
