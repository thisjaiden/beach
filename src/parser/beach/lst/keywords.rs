pub const KEYWORDS: &[&str] = &[
    "system", "main", "disable", "let", "return",
    "file",
];

pub const KEYWORDS_TYPED: &[Keyword] = &[
    Keyword::system, Keyword::main, Keyword::disable, Keyword::k_let, Keyword::k_return,
    Keyword::file,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    main,
    disable,
    system,
    k_pub,
    k_let,
    file,
    k_for,
    k_return,
}
