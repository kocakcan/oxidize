///                                     Macros
/// Declarative Macros (macro_rules!)
/// Work like a match but matching against code structure, not runtime values. This is what vec!,
/// println! are built from.
///
///     macro_rules! vec { 
///         ( $( $x:expr ),* ) => {
///             {
///                 let mut temp_vec = Vec::new();
///                 $( temp_vec.push($x); )*
///                 temp_vec
///             }
///         };
///     }
/// - $x:expr -> captures a Rust expression, bound to variable $x
/// - $(...),* -> "repeat this pattern, comma-separated, zero or more times"
/// - Result: vec![1, 2, 3] literally expands to three .push() calls at compile time.
/// Why not just use a function? Functions need a fixed number/type of args. Macros can take any
/// number, because they operate on raw tokens before type-checking happens.
///
/// Procedural Macros
/// Take tokens in, spit tokens out more like a compiler plugin than a match.
/// Three flavors:
///
/// Type            Trigger                 Use case
/// Custom derive   #[derive(HelloMacro)]   Auto-generate trait impls (huge in practice)
///                                         serde's #[derive(Serialize)] is this
/// Attribute-like  #[route(GET,"/")]       Custom attributes on any item, not just structs
///                                         /enums
/// Function-like   sql!(SELECT * ...)      Looks like a function call, but does arbitrary token
///                                         parsin (e.g. validate SQL syntax at compile time)
/// Mechanically, all procedural macros:
///     1. Live in their own crate with proc-macro = true
///     2. Receive a TokenStream (the code being annotated)
///     3. Parse it with syn (tokens -> structured AST)
///     4. Build new code with quote! (AST -> tokens again)
///     5. Return the generated TokenStream, which replaces/adds to the original code
///
/// Key mental model
/// Macros run before type-checking, at compile time. They write code, not values. That's the whole
/// reason they exist: things like "generate a trait impl for whatever struct this is attached to"
/// are literally impossible as a function, because a function only runs after compilation, when
/// it's too late to add new trait impls.
macro_rules! max {
    ($x:expr) => {  /* base case: single value */
        $x
    };
    ($x:expr, $($rest:expr),+) => {
        {
            let a = $x;
            let b = max!($($rest),+);
            if a > b { a } else { b }
        }
    };
}

fn main() {
    println!("{}", max!(3));
    println!("{}", max!(3, 7));
    println!("{}", max!(3, 7, 2, 9, 1));
}

