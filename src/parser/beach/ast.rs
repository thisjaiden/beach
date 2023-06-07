use crate::utils::*;

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
    Comment(String), //        // ARG
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
    Keyword(Keyword), //       ARG
    Integer(Bigint), //        ARG
    Float(Bigfloat), //        ARG
    Complex(Bigcplx), //       ARG
    Label(String), //          ARG
}

impl Symbol {
    pub fn next(reader: &mut StringReader) -> Option<Symbol> {
        let first_char = reader.next_non_whitespace_char()?;
        let second_char = reader.peek_char();
        match first_char {
            // Exclusive one char symbols
            ';' => return Some(Symbol::PhraseEnd),
            '/' => return Some(Symbol::Divide),
            '~' => return Some(Symbol::Child),
            '+' => return Some(Symbol::Add),
            '%' => return Some(Symbol::Modulo),
            ':' => return Some(Symbol::Is),
            // One or two char symbols
            '&' => {
                if second_char == Some('&') {
                    return Some(Symbol::LogicAnd);
                }
                else {
                    return Some(Symbol::BitAnd);
                }
            }
            // Enclosure symbols
            '{' => return Some(Symbol::Braced(
                Symbol::read_all_symbols(
                    &mut StringReader::from_string(
                        reader.read_until('}')
                    )
                )
            )),
            '[' => return Some(Symbol::Bracketed(
                Symbol::read_all_symbols(
                    &mut StringReader::from_string(
                        reader.read_until(']')
                    )
                )
            )),
            '(' => return Some(Symbol::Enclosed(
                Symbol::read_all_symbols(
                    &mut StringReader::from_string(
                        reader.read_until(')')
                    )
                )
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
            _ => todo!()
        }
    }
    pub fn read_all_symbols(reader: &mut StringReader) -> Vec<Symbol> {
        let mut symbols = vec![];
        while let Some(symbol) = Symbol::next(reader) {
            symbols.push(symbol);
        }
        symbols
    }
}

#[derive(Debug)]
pub enum Keyword {
    main,
    disable,
    wants,
    needs,
    k_pub,
    var,
    file,
    k_for,
}