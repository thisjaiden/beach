pub const KEYWORDS: [&'static str; 5] = [
    "needs", "main", "disable", "wants", "var"
];

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
