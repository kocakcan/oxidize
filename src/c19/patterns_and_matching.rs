/// Patterns and Matching
///
/// Patterns are syntax for matching against the shape of data. They combine:
///
///     - Literals
///     - Destructured arrays, enums, structs, tuples
///     - Variables, wildcards, placholders
/// 
/// x
/// (a, 3)
/// Some(Color::Red)
///
/// Compare a pattern to a value if it matches, the code runs and named pieces are usable. If not,
/// it doesn't.
match coin {
    Coin::Quarter   => 25,  // pattern matches -> code runs
    Coin::Dime      => 10,
    -               => 0,   // wildcard -> catches the rest
