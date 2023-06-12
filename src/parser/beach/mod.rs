mod ast;

pub use self::ast::RESERVED_LABEL_SYMBOLS;

pub fn read<R: std::io::Read>(reader: &mut R) -> ast::SyntaxRoot {
    todo!();
}

pub fn parse_string_file(file: String) -> ast::SyntaxRoot {
    ast::SyntaxRoot::from_string(file)
}
