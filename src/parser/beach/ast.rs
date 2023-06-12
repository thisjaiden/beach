use crate::utils::*;

mod keywords;

use self::keywords::*;

#[derive(Debug)]
pub struct SyntaxRoot {
    pub symbols: Vec<Symbol>
}

impl SyntaxRoot {
    pub fn from_string(from: String) -> SyntaxRoot {
        let mut reader = StringReader::from_string(from);
        let symbols = Symbol::read_all_symbols(&mut reader);
        SyntaxRoot { symbols }
    }
}

#[derive(Debug)]
pub enum Symbol {
    Comment(String), //        //ARG
    Comments(String), //       /*ARG*/
    Braced(Vec<Symbol>), //    {ARGS}
    Bracketed(Vec<Symbol>), // [ARGS]
    Enclosed(Vec<Symbol>), //  (ARGS)
    Closure(Vec<Symbol>), //   |ARGS|
    Alias, //                  =>
    ExportedAlias, //          =>!
    Becomes, //                ->
    Parent, //                 <-
    Child, //                  ~
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
    Keyword(Keyword), //       ARG
    Integer(Bigint), //        ARG
    Float(Bigfloat), //        ARG
    Complex(Bigcplx), //       ARG
    Label(String), //          ARG
}

// The following characters *cannot* appear in labels.
pub const RESERVED_LABEL_SYMBOLS: &[char] = &[
    ';', ',', ':', '(', ')' // TODO: rest of the symbols that should go here
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
            '~' => return Some(Symbol::Child),
            '+' => return Some(Symbol::Add),
            '%' => return Some(Symbol::Modulo),
            ':' => return Some(Symbol::Is),
            // One or two char symbols
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
                // TODO: if it's a `*`, multi-line comment
                else {
                    return Some(Symbol::Divide);
                }
            }
            '&' => {
                if second_char == Some('&') {
                    return Some(Symbol::LogicAnd);
                }
                else {
                    return Some(Symbol::BitAnd);
                }
            }
            '-' => {
                if second_char == Some('>') {
                    return Some(Symbol::Becomes);
                }
                else {
                    return Some(Symbol::Subtract);
                }
            }
            // Enclosure symbols
            '{' => return Some(Symbol::Braced(
                vec![]
                //Symbol::read_all_symbols(
                //    &mut StringReader::from_string(
                //        reader.read_until('}')
                //    )
                //)
            )),
            '[' => return Some(Symbol::Bracketed(
                Symbol::read_all_symbols(
                    &mut StringReader::from_string(
                        reader.read_until(']')
                    )
                )
            )),
            '(' => return Some(Symbol::Enclosed(
                vec![]
                //Symbol::read_all_symbols(
                //    &mut StringReader::from_string(
                //        reader.read_until(')')
                //    )
                //)
            )),
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
