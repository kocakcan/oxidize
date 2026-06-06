/// Fearless Concurrency
///
/// Handling concurrent programming safely and efficiently is another of Rust's
/// major goals. Concurrent programming, in which different parts of a program
/// execute independently, and parallel programming, in which different parts o
/// f a prgoram execute at the same time, are becoming increasingly important a
/// s more computers take advantage of their multiple processors. Historically,
/// programming in these contexts has been difficult and error-prone.
///
/// Using Threads to Run Code Simultaneously
///
/// In most current operating systems, an executed program's code is run in a p
/// rocess, and the operating system will manage multiple processes at once. Wi
/// thin a program, you can also have independent parts that run simultaneously
/// . The features that run these independent parts are called threads. For exa
/// mple, a web server could have multiple threads so that it can respond to mo
/// re than one request at the same time.
///
/// Splitting the computation in your program into multiple threads to run mult
/// iple tasks at the same time can improve performance, but it also adds compl
/// exity. Because threads can run simultaneously, there's no inherent guarante
/// e about the order in which parts of your code on different threads will run
/// . This can lead to problems, such as:
///
///     - Race conditions, in which threads are accessing data or resources in
///     an inconsistent order
///     - Deadlocks, in which two threads are waiting for each other, preventin
///     g both threads from continuing
///     - Bugs that only happen in certain situations and are hard to reproduce
///     and fix reliably
///
/// The Rust standard library uses 1:1 model of thread implementation, whereby
/// a program uses one operating system thread per one language thread. There a
/// re crates that implement other models of threading that make different trad
/// e-offs to the 1:1 model.
///
/// Creating a New Thread with spawn
///
/// To create a new thread, we call the thread::spawn function and pass it a cl
/// osure containing the code we want to run in the new thread. The example in
/// Listing 16-1 prints some text from a main thread and other text from a new
/// thread.
///
/// Note that when the main thread of a Rust program completes, all spawned thr
/// eads are shut down, whether or not they have finished running. The output f
/// rom this program might be a little different every time.
///
/// The calls to thread::sleep force a thread to stop its execution for a short
/// duration, allowing a different thread to run. The threads will probably tak
/// e turns, but that isn't guaranteed: It depends on how your operating system
/// schedules the threads.
///
/// Waiting for All Threads to Finish
///
/// The code in Listing 16-1 not only stops the spawned thread prematurely most
/// of the time due to main thread ending, but because there is no guarantee on
/// the order in which threads run, we also can't guarantee that the spawned th
/// read will get to run at all!
///
/// We can fix the problem of the spawned thread not running or of it ending pr
/// ematurely by saving the return value of thread::spawn in a variable. The re
/// turn type of thread::spawn is JoinHandle<T>. A JoinHandle<T> is an owned va
/// lue that, when we call the join method on it, will wait for its thread to f
/// inish.
///
/// Using move Closures with Threads
///
/// We'll often use the move keyword with closures passed to thread::spawn beca
/// use the closure will then take ownership of the values it uses from the env
/// ironment, thus transferring ownership of those values from one thread to an
/// other.
///
/// Notice in Listing 16-1 that the closure we pass to thread::spawn takes no arguments: We're not
/// using any data from the main thread in the spawned thread's code. To use data from the main
/// thread in the spawned thread, the spawned thread's closure must capture the values it needs.
/// Listing 16-3 shows an attempt to create a vector in the main thread and use it in the spawned
/// thread. However, this won't work yet.
///
///     use std::thread;
///
///     fn main() {
///         let v = vec![1, 2, 3];
///
///         let handle = thread::spawn(|| {
///             println!("Here's a vector: {v:?}");
///         });
///         handle.join().unwrap();
///     }
///     Listing 16-3: Attempting to use a vector created by the main thread in another thread
/// The closure uses v, so it will capture v and make it part of the clousre's environment. Becuase
/// thread::spawn runs this closure in a new thread, we should be able to access v inside that new
/// thread. But when we compile this example, we get the following error:
///
///     error: closure may outlive the current function, but it borrows `v`, which is owned by the
///     current function
/// Rust infers how to capture v, and because println! only needs a reference to v, the closure
/// tries to borrow v. However, there's a problem: Rust can't tell how long the spawned thread will
/// run, so it doesn't know whether the reference to v will always be valid.
///
/// Listing 16-4 provides a scenario that's more likely to have a reference to v that won't be valid.
///
///     use std::thread;
///
///     fn main() {
///         let v = vec![1, 2, 3];
///
///         let handle = thread::spawn(|| {
///             println!("Here's a vector: {v:?}");
///         });
///         drop(v);    // oh fuck!
///         handle.join().unwrap();
///     }
///     Listing 16-4: A thread with a closure that attempts to capture a reference to v from a main
///     thread that drops v
/// If Rust allowed us to run this code, there's a possibility that the spawned thread would be
/// immediately put in the background without running at all. The spawned thread has a reference to
/// v inside, but the main thread immediately drops v, using the drop function. Then, when the
/// spawned thread starts to execute, v is no longer valid, so a reference to it is also invalid.
///
/// By adding the move keyword before the closure, we force the closure to take ownership of the
/// values it's using rather than allowing Rust to infer that it would borrow the values. The
/// modification to Listing 16-3 shown in Listing 16-5 will compile and run as we intend.
///
///     use std::thread;
///
///     fn main() {
///         let v = vec![1, 2, 3];
///
///         let handle = thread::spawn(move || {
///             println!("Here's a vector: {v:?}");
///         });
///         handle.join().unwrap();
///     }
///     Listing 16-5: Using the move keyword to force a closure to take ownership of the values it uses
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap(); /* main thread waits for spawned thread to finish */
    for i in 1..=5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    // handle.join().unwrap();
}
