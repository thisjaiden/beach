use crate::utils::StringReader;

pub struct SyntaxRoot {
    pub symbols: Vec<Symbol>
}

pub enum Symbol {
    Comment(String), //        // ARG
    Comments(String), //       /*ARG*/
    Braced(Vec<Symbol>), //    {ARGS}
    Bracketed(Vec<Symbol>), // [ARGS]
    Enclosed(Vec<Symbol>), //  (ARGS)
    Keyword(Keyword), //       ARG
    PhraseEnd, //              ;
    Alias, //                  =>
    ExportedAlias, //          =>!
    Returns, //                ->
    Child, //                  ~
    Parent, //                 <-
}

impl Symbol {
    pub fn find(reader: &mut StringReader) -> Symbol {
        let first_char = reader.next_non_whitespace_char();
        match first_char {
            '{' => return Symbol::Braced(
                Symbol::read_all_symbols(
                    &mut StringReader::from_string(
                        reader.read_until('}')
                    )
                )
            ),
            '[' => return Symbol::Bracketed(
                Symbol::read_all_symbols(
                    &mut StringReader::from_string(
                        reader.read_until(']')
                    )
                )
            ),
            '(' => return Symbol::Enclosed(
                Symbol::read_all_symbols(
                    &mut StringReader::from_string(
                        reader.read_until(')')
                    )
                )
            ),
            ';' => return Symbol::PhraseEnd,
            '~' => return Symbol::Child,
            _ => todo!()
        }
    }
    pub fn read_all_symbols(reader: &mut StringReader) -> Vec<Symbol> {
        todo!();
    }
}

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