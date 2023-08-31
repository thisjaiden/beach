// literal syntax tree
mod lst;
// abstract syntax tree
mod ast;
// intermediate representation
mod ir;

pub use self::lst::RESERVED_LABEL_SYMBOLS;

pub fn read<R: std::io::Read>(reader: &mut R) -> lst::Syntax {
    todo!();
}

pub fn parse_string_file(file: String) -> lst::Syntax {
    lst::Syntax::from_string(file)
}

pub fn abstract_syntax(syntax: lst::Syntax) -> ast::Program {
    ast::Program::from_lst(syntax)
}

pub fn intermediate_representation(ast: ast::Program) -> ir::Executable {
    ir::Executable::from_ast(ast)
}
