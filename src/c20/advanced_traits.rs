///                                 Advanced Traits
/// 1. Associated Types
/// A placeholder type declared inside a trait (type Item;) that the implementor fills in with one
/// concrete type.
///
///     trait Iterator {
///         type Item;
///         fn next(&mut self) -> Option<Self::Item>;
///     }
/// Key distinction vs. generics: With trait Iterator<T>, you could implement Iterator<u32> and
/// Iterator<String> for the same type (multiple impls). With an associated type, you get exactly
/// one implementation per type, because the type is fixed once, not parameterized per-call. Use
/// associated types when a trait should have one "true" type per implementor.
///
/// 2. Default Generic Type Parameters (Operator Overloading)
/// Traits like Add<Rhs=Self> let you overload operators (+, -, etc.) by implementing traits from
/// std::ops. Rhs=Self means "if unspecified, assume the right-hand side is the same type as the
/// left."
///
///     impl Add<Meters> for Millimeters {
///         type Output = Millimeters;
///         fn add(self, other: Meters) -> Millimeters { ... }
///     }
/// This lets you override the default when adding mismatched types (Millimeters + Meters), while
/// the common case (Point + Point) needs no extra annotation.
///
/// 3. Disambiguating Same-Named Methods
/// Two traits or a trait + inherent impl can define methods with identical names. Rust resolves
/// calls by:
///
///     - Methods (have self) -> person.fly() picks the inherent method first; use
///     Pilot::fly(&person) to force a trait's version.
///     - Associated functions (no self) -> ambiguous without a receiver to infer from, so you need
///     fully qualified syntax:
///
///         <Dog as Animal>::baby_name()
///         General form: <Type as Trait>::function(args...)
///
/// 4. Supertraits
/// A trait can require another trait as a prerequisite, letting it reuse that trait's
/// functionality:
///
///     trait OutlinePrint: fmt::Display {
///         fn outline_print(&self) {
///             let output = self.to_string();  // relies on Display
///             ...
///         }
///     }
/// Any type implementing OutlinePrint must also implement Display, or the compiler rejects it.
///
/// 5. Newtype Pattern
/// Works around the orphan rule (can't impl a foreign trait on a foreign type) by wrapping the
/// foreign type in a local tuple struct:
///
///     struct Wrapper(Vec<String>);
///     impl fmt::Display for Wrapper { ... }   // Display is foreign, Vec<T> is foreign, but
///                                             // Wrapper is local
/// Trade-off: Wrapper loses the inner type's methods unless you manually delegate them (or
/// implement Deref).
