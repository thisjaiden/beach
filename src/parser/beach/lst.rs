use crate::utils::*;

pub mod keywords;

use self::keywords::*;

#[derive(Debug)]
pub struct SyntaxRoot {
    pub symbols: Vec<Symbol>,
    // tracks line nums, etc.
    pub annotations: Vec<Annotation>
}

impl SyntaxRoot {
    pub fn from_string(from: String) -> SyntaxRoot {
        let mut reader = StringReader::from_string(from);
        let symbols = Symbol::read_all_symbols(&mut reader);
        SyntaxRoot { symbols, annotations: vec![] }
    }
}

#[derive(Debug)]
pub struct Annotation {
    // TODO
}

#[derive(Debug, PartialEq)]
pub enum Symbol {
    Comment(String), //        //ARG
    Comments(String), //       /*ARG*/
    OpenBrace, //              {
    CloseBrace, //             }
    OpenBracket, //            [
    CloseBracket, //           ]
    OpenParenthesis, //        (
    CloseParenthesis, //       )
    Closure(Vec<Symbol>), //   |ARGS|
    Alias, //                  =>
    ExportedAlias, //          =>!
    Becomes, //                ->
    Parent, //                 <-
    Module, //                 ~
    PhraseEnd, //              ;
    Is, //                     :
    Set, //                    =
    Equals, //                 ==
    DoesNotEqual, //           !=
    ApproxEquals, //           =~
    ApproxDoesNotEqual, //     !~
    BitOr, //                  |
    BitAnd, //                 &
    BitXor, //                 ^
    LogicOr, //                ||
    LogicAnd, //               &&
    LogicXor, //               ^^
    LogicNot, //               !
    Power, //                  **
    Add, //                    +
    Subtract, //               -
    Divide, //                 /
    Multiply, //               *
    Modulo, //                 %
    LessThan, //               <
    MoreThan, //               >
    LessThanOrEqual, //        <=
    MoreThanOrEqual, //        >=
    Also, //                   ,
    LeftShift, //              <<
    RightShift, //             >>
    Compiler, //               !!
    String(String), //         "ARG"
    Keyword(Keyword), //       ARG
    Integer(Bigint), //        ARG
    Float(Bigfloat), //        ARG
    Complex(Bigcplx), //       ARG
    Label(String), //          ARG
}

// The following characters *cannot* appear in labels.
pub const RESERVED_LABEL_SYMBOLS: &[char] = &[
    ';', ',', ':', '(', ')', '[', ']', '~' // TODO: rest of the symbols that should go here
];

impl Symbol {
    pub fn next(reader: &mut StringReader) -> Option<Symbol> {
        let first_char = reader.next_non_whitespace_char()?;
        let second_char = reader.peek_char();
        let peaked_word = format!("{}{}", first_char, reader.peek_word());

        for (index, keyword) in keywords::KEYWORDS.iter().enumerate() {
            if &peaked_word.as_str() == keyword {
                reader.read_word();
                return Some(Symbol::Keyword(keywords::KEYWORDS_TYPED[index]));
            }
        }

        match first_char {
            // Exclusive one char symbols
            ';' => return Some(Symbol::PhraseEnd),
            '~' => return Some(Symbol::Module),
            '+' => return Some(Symbol::Add),
            '%' => return Some(Symbol::Modulo),
            ':' => return Some(Symbol::Is),
            '{' => return Some(Symbol::OpenBrace),
            '}' => return Some(Symbol::CloseBrace),
            '[' => return Some(Symbol::OpenBracket),
            ']' => return Some(Symbol::CloseBracket),
            '(' => return Some(Symbol::OpenParenthesis),
            ')' => return Some(Symbol::CloseParenthesis),
            ',' => return Some(Symbol::Also),
            '!' => {
                if second_char == Some('=') {
                    reader.read_char();
                    return Some(Symbol::DoesNotEqual);
                }
                else if second_char == Some('!') {
                    reader.read_char();
                    return Some(Symbol::Compiler);
                }
                else {
                    return Some(Symbol::LogicNot);
                }
            }
            '*' => {
                if second_char == Some('*') {
                    reader.read_char();
                    return Some(Symbol::Power);
                }
                else {
                    return Some(Symbol::Multiply);
                }
            }
            '"' => {
                let mut output_string = String::new();
                let mut last_char = '"';
                let mut this_char = reader.read_char()?;
                loop {
                    if this_char == '"' && last_char != '\\' {
                        break;
                    }
                    output_string += &this_char.to_string();
                    last_char = this_char;
                    this_char = reader.read_char()?;
                }
                return Some(Symbol::String(output_string));
            }
            '=' => {
                if second_char == Some('>') {
                    // throw away arrow
                    reader.read_char();
                    // check if we have a ! next
                    let next = reader.peek_char();
                    if next == Some('!') {
                        // throw away !
                        reader.read_char();
                        return Some(Symbol::ExportedAlias);
                    }
                    else {
                        return Some(Symbol::Alias);
                    }
                }
                else if second_char == Some('=') {
                    reader.read_char();
                    return Some(Symbol::Equals);
                }
                else {
                    return Some(Symbol::Set);
                }
            }
            '/' => {
                if second_char == Some('/') {
                    // throw away the next slash
                    reader.read_char();
                    // read the rest of the line to a comment
                    return Some(Symbol::Comment(
                        reader.read_line()
                    ));
                }
                else if second_char == Some('*') {
                    // throw away *
                    reader.read_char();
                    let mut comment_out = String::new();
                    loop {
                        comment_out += &reader.read_until('*');
                        let next = reader.peek_char();
                        if next == Some('/') {
                            break;
                        }
                        else {
                            comment_out += "*";
                        }
                    }
                    return Some(Symbol::Comments(comment_out));
                }
                else {
                    return Some(Symbol::Divide);
                }
            }
            '&' => {
                if second_char == Some('&') {
                    reader.read_char();
                    return Some(Symbol::LogicAnd);
                }
                else {
                    return Some(Symbol::BitAnd);
                }
            }
            '^' => {
                if second_char == Some('^') {
                    reader.read_char();
                    return Some(Symbol::LogicXor);
                }
                else {
                    return Some(Symbol::BitXor);
                }
            }
            '-' => {
                if second_char == Some('>') {
                    reader.read_char();
                    return Some(Symbol::Becomes);
                }
                else {
                    return Some(Symbol::Subtract);
                }
            }
            '<' => {
                if second_char == Some('-') {
                    reader.read_char();
                    return Some(Symbol::Parent);
                }
                else if second_char == Some('=') {
                    reader.read_char();
                    return Some(Symbol::LessThanOrEqual);
                }
                else if second_char == Some('<') {
                    reader.read_char();
                    return Some(Symbol::LeftShift);
                }
                else {
                    return Some(Symbol::LessThan);
                }
            }
            '>' => {
                if second_char == Some('=') {
                    reader.read_char();
                    return Some(Symbol::MoreThanOrEqual);
                }
                else if second_char == Some('>') {
                    reader.read_char();
                    return Some(Symbol::RightShift);
                }
                else {
                    return Some(Symbol::MoreThan);
                }
            }
            // TODO: this whole thing with `|` is problematic :(
            '|' => {
                if second_char != Some('|') && second_char != Some(' ') {
                    return Some(Symbol::Closure(
                        Symbol::read_all_symbols(
                            &mut StringReader::from_string(
                                // TODO: THIS IS PROBLEMATIC!!! (recursion)
                                reader.read_until('|')
                            )
                        )
                    ))
                }
                else if second_char == Some(' ') {
                    return Some(Symbol::BitOr);
                }
                else {
                    return Some(Symbol::LogicOr);
                }
            },
            _ => {}
        }
        // TODO: differentiate numeric values and other types
        reader.read_word();
        return Some(Symbol::Label(peaked_word));
    }
    pub fn read_all_symbols(reader: &mut StringReader) -> Vec<Symbol> {
        println!("Reading all symbols!");
        let mut symbols = vec![];
        while let Some(symbol) = Symbol::next(reader) {
            println!("Symbol::{:?}", symbol);
            symbols.push(symbol);
        }
        symbols
    }
}
