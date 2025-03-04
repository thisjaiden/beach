pub mod beach;
pub mod aarch64;

use crate::parser::beach::ast::user_token_format;

pub fn parse_string_file(file: String) -> user_token_format::Syntax {
    user_token_format::Syntax::from_string(file)
}
