/// Pattern Syntax
///
/// 1. Literals
/// Match exact values.
///
///     match x {
///         1 => println!("one"),
///         2 => println!("two"),
///         _ => println!("anything"),
///     }
///
/// 2. Named Variables
/// Irrefutable but shadow outer variables inside match/if let/while let scope.
///
///     let y = 10;
///     match x {
///         Some(y) => println!("{y}"), // new y, not outer y!
///         _ => (),
///     }
///
/// 3. Multiple Patterns - |
///
///     match x {
///         1 | 2 => println!("one or two"),
///         _ => println!("other"),
///     }
///
/// 4. Ranges ..=
/// Only for numeric and char. Compiler checks range is non-empty.
///
///     match x {
///         1..=5 => println!("one through five"),
///         'a'..='j' => println!("early ASCII"),
///         _ => (),
///     }
///
/// 5. Destructuring
///
///     Structs
///     let Point { x, y } = p;         // shorthand
///     Point { x, y: 0 } => ..         // mix literals + bindings
///
///     Enums (mirror their definition)
///     Message::Move { x, y } => ..    // struct-like
///     Message::Write(text) => ..      // tuple-like
///     Message::Quit => ..             // no data
///
///     Nested:
///     Message::ChangeColor(Color::Hsv(h, s, v)) => ..
///
///     Mixed:
///     let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
///
/// 6. Ignoring Values
///
///     Syntax  Behaviour
///     _       Wildcard -> matches all, no binding, no ownership move
///     _x      Suppresses unused warning, still binds, can move ownership
///     ..      Skip remaining fields/elements, must be unambiguous
///
///     fn foo(_: i32, y: i32) { .. }   // ignore param
///     (Some(_), Some(_)) => ..        // test shape, ignore inner
///     Point { x, .. } => ..           // ignore y, z
///     (first, ..., last) => ..        // first and last only
///     (.., second, ..) -> ..          // ambiguous, won't compile
///
/// 7. Match Guards (if <condition>)
/// Extra condition after the pattern. Not checked for exhaustiveness. Solves the shadowing
/// problem.
///
///     match num {
///         Some(x) if x % 2 == 0 => println!("even"),
///         Some(x)               => println!("odd"),
///         None                  => (),
///     }
///
///     // Avoid shadowing with guard:
///     Some(n) if n == y => ..   // compares to outer y, no shadow
/// NOTE: Guard applies to the whole | group:
/// 4 | 5 | 6 if y means (4 | 5 | 6) if y
///
/// 8. @ Bindings
/// Test a value against a pattern and bind it simultaneously.
///
///     match msg {
///         Message::Hello { id: id @ 3..=7 } => println!("in range: {id}"),
///         Message::Hello { id: 10..=12 }    => println!("another range"), // no binding
///         Message::Hello { id }             => println!("other: {id}").
///     }
///
/// TAKEAWAY: Patterns are a composable system. Literals, destructuring, guards, and @ bindings can
/// all combine, the compiler enforces correctness at every step.
