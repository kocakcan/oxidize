/// Adds a Display-able object into vector of
/// Display trait objects
use std::fmt::Display;

fn add_displayable<T: Display>(
    v: &mut Vec<Box<dyn Display>>,
    t: T
) {
    v.push(Box::new(t));
}

// Context: When casting Box<T> to Box<dyn Display> (implicitly in v.push(..)), Rust requires that
// the trait object dyn Display must outlive the vector. However, the lifetime of T is unspecified,
// so T may not live long enough.

let mut v: Vec<Box<dyn Display>> = Vec::new();
{
    let n = 0;
    add_displayable(&mut v, &n);
}
println!("{}", v[0]);

// Context: This program is memory-unsafe. To create a memory safety violation, the program must
// create a type T that contains reference which do not outlive the vector. For example, adding &n
// where n goes out of scope before v means that the later use of v[0] is a read of deallocated
// memory.

fn add_displayable<'a, T: Display + 'a>(
    v: &mut Vec<Box<dyn Display + 'a>>,
    t: T
) {
    v.push(Box::new(t));
}

// Context: The most idiomatic fix is to specify how the lifetime of T should relate to the
// lifetime of the trait object dyn Display. Here, that means adding a lifetime parameter 'a,
// saying that T outlives 'a, and that the trait objects also live for at least 'a.
//
// Using a 'static bound is the same thing but worse: it is simply less expressive, disallowing the
// vector's trait objects from ever holding references. Putting the lifetime 'a on the &'a mut
// Vec<..> reference is not correct and does not fix the compiler error.
