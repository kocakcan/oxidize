///                         Advanced Functions and Closures
/// 1. Function Pointers (fn)
/// fn is a type (lowercase, not the Fn trait) representing a plain function that can be passed
/// around like a value:
///
///     fn add_one(x: i32) -> i32 { x + 1 }
///     fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
///         f(arg) + f(arg)
///     }
///     do_twice(add_one, 5);   // 12
/// Key facts:
///     - fn implements Fn, FnMut, and FnOnce so a plain function can always be passed where a
///     closure is expected. The reverse isn't true (a capturing closure isn't a fn).
///     - Best practice: write functions generic over Fn/FnMut/FnOnce so they accept either
///     closures or function pointers. Use plain fn params only when you specifically need to
///     exclude closures (e.g., FFI with C, which has no closures).
///     - Enum variant constructors are functions pointers too:
///         (0u32..20).map(Status::Value).collect();    // Status::Value used as fn
///
/// 2. Returning Closures
/// You can't return Fn/FnMut/FnOnce directly as trait types due to no concrete size. Solutions:
/// a) impl Trait -> works if all code paths return the same underlying closure type:
///
///     fn returns_closure() -> impl Fn(i32) -> i32 {
///         |x| x + 1
///     }
/// b) The gotcha: two functions returning impl Fn(i32) -> i32 are not interchangeable, even with
/// identical signatures. Each impl Trait is a distinct opaque type under the hood. So you can't
/// put them in the same Vec:
///
///     let handlers = vec![returns_closure(),
///     returns_initialized_closure(123)];  // mismatched types
/// c) Fix: Box<dyn Fn(i32) -> i32> -> a trait object, which erases the concrete closure type
/// entirely, so any closure matching the signature fits:
/// Now both functions return the same type (Box<dyn Fn(i32) -> i32>), so the can coexist in a Vec.
/// Rule of thumb: impl Trait return = "one specific hidden type, decided at compile time, zero
/// cost." Box<dyn Trait> return = "any type implementing this trait, decided possibly different
/// each call, small heap-allocation cost."
