use std::rc::Rc;

struct Example;

impl Drop for Example {
    fn drop(&mut self) {
        println!("drop");
    }
}

fn main() {
    let x = Rc::new(Example);
    let y = Rc::clone(&x);
    println!("A");
    drop(x);
    println!("B");
    drop(y);
    println!("C");
}

// Context: The value inside a reference-counted pointer is only ever dropped once, so "drop" os
// only printed once. The initial drop(x) decrements the reference count, but does not drop the
// value because y is still live. Then dropping y finds that the reference count is 0, and drops Example.

// fn main() {
//     let n = Rc::new(1);
//     let mut n2 = Rc::clone(&n);
//     *n2 += 1; // cannot assing to data in an `Rc`
//     println!("{}", n);
// }
