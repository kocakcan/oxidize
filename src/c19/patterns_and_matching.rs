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
// match coin {
//     Coin::Quarter   => 25,  // pattern matches -> code runs
//     Coin::Dime      => 10,
//     -               => 0,   // wildcard -> catches the rest
/// Where Patterns Are Used
///
/// Patterns appear in six places. Each is ju a variation of PATTERN = VALUE.
///
/// 1. match Arms
/// Must be exhaustive. _ catches the rest.
///
///     match x {
///         None    => None,
///         Some(i) => Some(i + 1),
///     }
///
/// 2. let Statements
/// Every let is a pattern. Always has been.
///
///     let x = 5;                  // x is the pattern
///     let (x, y, z) = (1, 2, 3)   // destructive - must match element count
///
/// 3. if let Expressions
/// Flexible, chainable but not exhaustive-checked by the compiler.
///
///     if let Some(color) = favorite_color {
///         // ...
///     } else if let Ok(age) = age {
///         // age is shadowed here (new scope)
///     } else {
///         // fallback
///     }
///
/// 4. while let Loops
/// Runs as long as the pattern matches.
///
///     while let Ok(value) = rx.recv() {
///         println!("{value}");
///     }
///
/// 5. for Loops
/// The variable after for is the pattern.
///
///     for (index, value) in v.iter().enumerate() {
///         println!("{value} is at index {index}");
///     }
///
/// 6. Function Parameters
/// Parameters are patterns. Destructuring works inline.
///
///     fn print_coordinates(&(x, y): &(i32, i32)) {
///         println!("({x}, {y})");
///     }
///
/// NOTE: Anywhere Rust binds a name to a value, a pattern is at work.
///
/// Refutability
///
/// A pattern either always matches or might not.
///
///     Two Kinds
///
///     Kind        Definition                  Example
///     Irrefutable Always matches, cannot fail let x = 5;
///     Refutable   May fail to match,          if let Some(x) = val;
///
/// Where Each Belongs
///
///     // Irrefutable required (let, for, fn params)
///     let x = 5;
///     let (a, b) = (1, 2);
///
///     // Refutable allowed (if let, while let, let...else)
///     if let Some(x) = value { ... }
///     while let Ok(y) = rx.recv() { ... }
///
///     // Refutable in let (compiler error)
///     let Some(x) = some_option_value;
///     //  ^^^^^^^ pattern `None` not covered
///
/// The Fix: let...else
/// When you need a refutable pattern in a binding, use let...else to handle the non-matching case.
///
///     // Correct
///     let Some(x) = some_option_value else {
///         return;
///     }
///
/// Compiler Warns Both Ways
///
///     // Refutable in let -> error
///     let Some(x) = value;
///
///     // Irrefutable in let...else -> warning
///     let x = 5 else { return; }  // else is useless
///
/// NOTE: Use the right tool for the job.
/// -> Certain binding? -> let
/// -> Might not watch  -> if let, while let, or let...else
/// -> Never mix them up, the compiler won't let you forget
fn main() {
    // since slices don't have a fixed length, any pattern which asserts that x must have at least
    // one element is refutable.
    let x: &[(x, y)] = &[(0, 1)];
    let x: &[(x, y), ..)] = &[(0, 1)];

    let mut v = vec![(1, 2), (3, 4)].into_iter();
    let mut sum = 0;
    while let Some(t) = v.next() {
        let (_, n) = t;
        sum += n;
    }
    println!("{sum}");
}

