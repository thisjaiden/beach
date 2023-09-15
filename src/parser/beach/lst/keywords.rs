pub const KEYWORDS: &[&str] = &[
    "system", "main", "disable", "let", "return",
    "file", "library", "for"
];

pub const KEYWORDS_TYPED: &[Keyword] = &[
    Keyword::Ksystem, Keyword::Kmain, Keyword::Kdisable, Keyword::Klet, Keyword::Kreturn,
    Keyword::Kfile, Keyword::Klibrary, Keyword::Kfor
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Kmain, // main loop
    Kdisable, // disable contextual refrence or import
    Ksystem, // std import
    Klet, // variables
    Kfile, // import project file
    Klibrary, // import external library
    Kfor, // for loops
    Kreturn, // ...
}
