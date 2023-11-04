// literal syntax tree
mod lst;
// abstract syntax tree
mod ast;
// intermediate representation
mod ir;

pub use ir::Executable;
pub use ast::Definition;

pub use self::lst::RESERVED_LABEL_SYMBOLS;

pub fn parse_string_file(file: String) -> lst::Syntax {
    lst::Syntax::from_string(file)
}

pub fn abstract_syntax(syntax: lst::Syntax, prefix: Option<String>) -> Result<ast::Program, anyhow::Error> {
    ast::Program::from_lst(syntax, prefix)
}

pub fn intermediate_representation(ast: ast::Program) -> ir::Executable {
    ir::Executable::from_ast(ast)
}
