///                                 Advanced Types
/// 1. Newtype Pattern
/// Beyond enabling foreign trait impls, newtypes give you:
///
///     - Type safety: Millimeters and Meters can't be accidentally swapped, even though both wrap
///     u32.
///     - Encapsulation: wrap HashMap<i32, String> in a People struct and expose only the API you
///     want, callers never see the internal representation.
///
/// 2. Type Aliases (type)
/// This is a synonym, not a new type; Kilometers and i32 are interchangeable, and the compiler
/// gives you zero type-checking benefit (unlike newtype).
/// Real purpose: reduce repetition for verbose types.
///
///     type Thunk = Box<dyn Fn() + Send + 'static>;
/// Classic example: std::io::Result<T> is type Result<T> = std::result::Result<T,
/// std::io::Error>; shortens every I/O function signature while still behaving as a normal
/// Result (so ? and all its methods still work).
/// Rule of thumb: newtype = new type + compiler enforcement; alias = same type +
/// convenience/readability
///
/// 3. The Never Type (!)
/// Represents a type with no values, it marks diverging functions/expressions that never
/// return normally:
///
///     fn bar() -> ! { /* loops forever, or always panics */ }
/// Key property: ! coerces into any other type, since an expression of type ! never actually
/// produces a value to conflict with anything. This is why this compiles:
///
///     let guess: u32 = match input.parse() {
///         Ok(num) => num,     // u32
///         Err(_) => continue, // ! coerces to u32
///     }
/// Same trick applies to panic!, continue, break, and infinite loop {} (without a break).
///
/// 4. Dynamically Sized Types (DSTs) & Sized
