pub const KEYWORDS: [&'static str; 5] = [
    "needs", "main", "disable", "wants", "var"
];

pub const KEYWORDS_TYPED: [Keyword; 5] = [
    Keyword::needs, Keyword::main, Keyword::disable, Keyword::wants, Keyword::var
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
}
