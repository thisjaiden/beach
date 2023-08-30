pub const KEYWORDS: &[&str] = &[
    "needs", "main", "disable", "wants", "var",
    "return",
];

pub const KEYWORDS_TYPED: &[Keyword] = &[
    Keyword::needs, Keyword::main, Keyword::disable, Keyword::wants, Keyword::var,
    Keyword::k_return,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    main,
    disable,
    wants,
    needs,
    k_pub,
    var,
    file,
    k_for,
    k_return,
}
