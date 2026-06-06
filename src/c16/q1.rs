use std::thread;

fn main() {
    let mut n = 1;
    let t = thread::spawn(move || {
        /* n is moved here (first copy) */
        n = n + 1;
        thread::spawn(move || {
            /* n is moved again (second copy) */
            n = n + 1;
        })
    });
    n = n + 1; /* n was copied in the spawned thread */
    t.join().unwrap().join().unwrap();
    println!("{n}");
}

// If a closure captures a reference to a value that lived less than 'static, it's possible that the
// thread would live longer than the value and violate memory safety.
