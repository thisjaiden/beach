pub struct SyntaxRoot {

}

enum Symbol {
    Commented(String), //      // ARG
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

enum Keyword {
    main,
    disable,
    wants,
    needs,
    k_pub,
    var,
    file,
}