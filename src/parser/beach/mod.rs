// literal syntax tree
mod lst;
// abstract syntax tree
mod ast;

pub use self::lst::RESERVED_LABEL_SYMBOLS;

pub fn read<R: std::io::Read>(reader: &mut R) -> lst::SyntaxRoot {
    todo!();
}

pub fn parse_string_file(file: String) -> lst::SyntaxRoot {
    lst::SyntaxRoot::from_string(file)
}
