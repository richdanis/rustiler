use super::parser::{Function, Program, Statement};
use std::fs;

pub fn generate_assembly(ast: Program) {
    let mut code = String::new();

    generate_function(ast.function, &mut code);

    fs::write("main.s", code ).expect("Unable to write file");

}

fn generate_function(function: Function, code: &mut String) {
    
    let mut line = format!(" .globl _{}\n",function.identifier);
    code.push_str(&line);
    line = format!("_{}:\n", function.identifier);
    code.push_str(&line);

    generate_statements(function.statement, code, " ");

}

fn generate_statements(statement: Statement, code: &mut String, offset: &str) {
    let mut line = format!("{offset}movl    ${}, %eax\n", statement.expression.value);
    code.push_str(&line);
    line = format!("{offset}ret");
    code.push_str(&line);

}

