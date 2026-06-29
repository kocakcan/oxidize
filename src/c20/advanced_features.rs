///                                 Advanced Features
///
/// Unsafe Rust
/// Safe Rust is conservative by design, it rejects valid programs when it can't prove correctness.
/// unsafe is the escape hatch for when you know better than the compiler.
///
/// The 5 Superpowers
/// Everything else still follows safe Rust rules. The borrow checker doesn't turn off.
///
///     Superpower                      Notes
///     Dereference a raw pointer       *const T / *mut T
///     Call an unsafe function         Must be inside unsafe {}
///     Access/modify mutable static    Data race risk
///     Implement an unsafe trait       You uphold the invariants
///     Access union fields             Type of data not guaranteed
///
/// Raw Pointers (*const T/*mut T)
/// Unlike references, raw pointers:
/// - Can ignore borrowing rules (multiple &mut to same location)
/// - May be null or dangling
/// - No automatic cleanup
///
/// Unsafe Functions
///
///     unsafe fn dangerous() { }
///     unsafe { dangerous(); }     // must be called in unsafe block
///     dangerous();                // compiler error
///
/// Safe Abstraction over Unsafe Code
/// Wrap unsafe in a safe API. The standard library does this constantly.
///
///     // safe Rust can't do this (borrow checker sees two &mut to same slice)
///     fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
///         (&mut values[..mid], &mut values[mid..])    // error: two mutable borrows
///     }
///     
///     // unsafe makes it work. we verify correctness manually
///     use std::slice;
///
///     fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
///         let len = values.len();
///         let ptr = values.as_mut_ptr();
///         assert!(mid <= len);
///         unsafe {
///             (
///                 slice::from_raw_parts_mut(ptr, mid),
///                 slice::from_raw_parts_mut(ptr.add(mid), len - mid),
///             )
///         }
///     }
/// The function itself stays safe to call, unsafe is contained inside.
///
/// FFI (extern)
/// Call C from Rust:
///
///     unsafe extern "C" {
///         safe fn abs(input: i32) -> i32; // marked safe (no unsafe block needed)
///     }
///
/// Expose Rust to C:
///
///     #[unsafe(no_mangle)]
///     pub extern "C" fn call_from_c() {
///         println!("called from C!");
///     }
///
/// Mutable Static Variables
///
///     static mut COUNTER: u32 = 0;
///
///     // SAFETY: only call from a single thread
///     unsafe fn add_to_count(inc: u32) {
///         unsafe { COUNTER += inc; }
///     }
/// Prefer thread-safe primitives over mutable statics.
///
/// Unsafe Traits
///
///     unsafe trait Foo { }
///     unsafe impl Foo for i32 { } // you uphold what the compiler can't verify
/// Send and Sync are the canonical examples, manually implement only when your type contains raw
/// pointers.
///
/// Rules of Thumb
/// 1. Keep unsafe blocks as small as possible
/// 2. Always write // SAFETY: comments explaining your invariants
/// 3. Wrap unsafe in safe abstractions (contain the blast radius)
/// 4. Run Miri on all unsafe code
/// 5. When in doubt read The Rustonomicon
fn main() {
    let mut num = 5;
    let r1 = &raw const num; // *const i32 (safe to create)
    let r2 = &raw mut num; // *mut i32 (safe to create)
    unsafe {
        println!("{}", *r1); // dereference (unsafe)
        println!("{}", *r2);
    }
}
