/// Functional Language Features: Iterators and Closures
///
/// Rust's design has taken inspiration from many existing languages and techniques, and one
/// significant influence is functional programming. Programming in a functional style often
/// includes using functions as values by passing them in arguments, returning them from other
/// functions, assigning them to variables for later execution, and so forth.
///
/// Closures: Anonymous Functions That Capture Their Environment
///
/// Rust's closures are anonymous functions you can save in a variable or pass as arguments to other
/// functions. You can create the closure in one place and then call the closure elsewhere to
/// evaluate it in a different context. Unlike functions, closures can capture values from the scope
/// in which they're defined.
///
/// Capturing the Environment with Closures
///
/// Here's the scenario: Every so often, our t-shirt company gives away an exclusive,
/// limited-edition shirt to someone on our mailing list as a promotion. People on the mailing list
/// can optionally add their favourite color to their profile. If the person chosen for a free shirt
/// has their favourite color set, they get that color shirt. If the person hasn't specified a
/// favourite color, they get whatever color the company has the most of.
///
/// The store defined in main has two blue shirts and one red shirt remaining to distribute for
/// this limited-edition promotion. We call the giveaway method for a user with a preference for a
/// red shirt and a user without any preference.
///
/// Again, this code could be implemented in many ways, to focus on closures, we've stuck to
/// concepts you've already learned, except for the body of the giveaway method that uses a
/// closure. In the giveaway method, we get the user preference as a parameter of type
/// Option<ShirtColor> and call the unwrap_or_else method on user_preference. The unwrap_or_else
/// method on Option<T> is defined by the standard library. It takes one argument: a closure
/// without any arguments that return a value T (the same type stored in the Some variant of the
/// Option<T>, in this case ShirtColor). If the Option<T> is the Some variant, unwrap_or_else
/// returns the value from within the Some. If the Option<T> is the None variant, unwrap_or_else
/// calls the closure returns the values returned by the closure.
///
/// We specify the closure expression || self.most_stocked() as the argument to unwrap_or_else.
/// This is a closure that takes no parameters itself (if the closure had parameters, they would
/// appear between the two vertical bars). The body of the closure calls self.most_stocked(). We're
/// defining the closure here, and the implementation of unwrap_or_else will evaluate the closure
/// later if the result is needed.
///
/// One interesting aspect here is that we've passed a closure that calls self.most_stocked() on
/// the current Inventory instance. The standard library didn't need to know anything about the
/// Inventory or ShirtColor types we defined, or the logic we want to use in this scenario. The
/// closure captures an immutable reference to the self Inventory instance and passes it with the
/// code we specify to the unwrap_or_else method. Function, on the other hand, are not able to
/// capture their environment in this way.
///
/// Closure Type Inference and Annotation
///
/// There are more differences between functions and closures. Closures don't usually require you
/// to annotate the types of the parameters or the return values like fn function do. Type
/// annotations are required on functions because the types are part of an explicit interface
/// exposed to your users. Defining this interface rigidly is important for ensuring that everyone
/// agrees on what type of values a function uses and returns. Closures, on the other hand, aren't
/// used in an exposed interface like this: they're stored in variables and used without naming
/// them and exposing them to users of our library.
///
/// Closures are typically short and relevant only within a narrow context rather than in any
/// arbitrary scenario. Within these limited contexts, the compiler can infer the types of the
/// parameters and the return type, similar to how it's able to infer the types of most variables
/// (there are rare cases where the compiler need closure type annotations too).
///
/// As with variables, we can add type annotations if we want to increase explicitness and clarity
/// at the cost of being more verbose than is strictly necessary. Annotating the types for a
/// closure would look like the definition shown in Listing 13-2. In this example, we're defining a
/// closure and storing it in a variable rather than defining the closure in the spot we pass it as
/// an argument as we did in Listing 13-1.
///
///     let expensive_closure = |num: u32| -> u32 {
///         println!("calculating slowly...");
///         thread::sleep(Duration::from_secs(2));
///         num
///     };
///     Listing 13-2: Adding optional type annotations of the parameter and return value types in
///     the closure
/// With type annotations added, the syntax of closures look more similar to the syntax of
/// functions. Here we define a function that adds 1 to its parameter and a closure that has the
/// same behaviour, for comparison. We've added some spaces to line up the relevant parts. This
/// illustrates how closure syntax is similar to function syntax except for the use of pipes and
/// the amount of syntax that is optional:
///
///     fn  add_one_v1   (x: i32) -> u32 { x + 1 }
///     let add_one_v2 = |x: u32| -> u32 { x + 1 };
///     let add_one_v3 = |x|             { x + 1 };
///     let add_one_v4 = |x|               x + 1  ;
/// The first line shows a function definition, and the second line shows a fully annotated closure
/// definition. In the third line, we remove the type annotations from the closure definition. In
/// the fourth line, we remove the brackets, which are optional because the closure body has only
/// one expression. These are all valid definitions that will produce the same behaviour when
/// they're called. The add_one_v3 and add_one_v4 lines require the closures to be evaluated to be
/// able to compile because the types will be inferred from their usage. This is similar to let v =
/// Vec::new(); needing either type annotation or values of some type to be inserted into the Vec
/// for Rust to be able to infer the type.
///
/// For closure definitions, the compiler will infer one concrete type for each of their parameters
/// and for their return value. For instance, Listing 13-3 shows the definition of a short closure
/// that just returns the value it receives as a parameter. This closure isn't very useful except
/// for the purposes of this example. Note that we haven't added any type annotations to the
/// definition. Because there are no type annotations, we can call the closure with any type, which
/// we've done here with String the first time. If we then try to call example_closure with an
/// integer, we'll get an error.
///
///     let example_closure = |x| x;
///
///     let s = example_closure(String::from("hello"));
///     let n = example_closure(5);
///     Listing 13-3: Attempting to call a closure whose types are inferred with two different types
/// The compiler give us this error:
///
///     error: mismatched types
/// The first time we call example_closure with the String value, the compiler infers the type of x
/// and the return type of the closure to be String. Those types are then locked into the closure
/// in example_closure, and we get a type error when we try to use a different type with the same closure.
///
/// Capturing References or Moving Ownership
///
/// Closures can capture values from their environment in three ways, which directly map to the
/// three ways a function can take a parameter: borrowing immutably, borrowing mutably, and taking
/// ownership. The closure will decide which of these to use based on what the body of the function
/// does with the captured values.
///
/// In Listing 13-4, we define a closure that captures an immutable reference to the vector named
/// list because it only need a immutable reference to print the value.
///
///     fn main() {
///         let list = vec![1, 2, 3];
///         println!("Before defining closure: {list:?}");
///
///         let only_borrows = || println!("From closure: {list:?}");
///
///         println!("Before calling closure: {list:?}");
///         only_borrows();
///         println!("After calling closure: {list:?}");
///     }
///     Listing 13-4: Defining and calling a closure that captures an immutable reference
/// This example also illustrates that a variable can bind to a closure definition, and we can
/// later call the closure by using the variable name and parantheses as if the variable name were
/// a function name.
///
/// Because we can have multiple immutable references to list at the same time, list is still
/// accessible from the code before the closure definition, after the closure definition but before
/// the closure is called, and after the closure is called. This code compiles, runs, and prints.
///
/// Next, in Listing 13-4, we change the closure body so that it adds an element to the list vector.
/// The closure now captures a mutable reference:
///
///     fn main() {
///         let mut list = vec![1, 2, 3];
///         println!("Before defining closure: {list:?}");
///
///         let mut borrows_mutably = || list.push(7);
///
///         borrows_mutably();
///         println!("After calling closure: {list:?}");
///     }
///     Listing 13-5: Defining and calling a closure that captures a mutable reference
/// This code compiles, runs, and prints.
///
/// Note that there's no longer a println! between the definition and the call of the
/// borrows_mutably closure: when borrows_mutably is defined, it captures a mutable reference to
/// list. We don't use the closure again after the closure is called, so the mutable borrow ends.
/// Between the closure definition and the closure call, an immutable borrow to print isn't allowed
/// because no other borrows are allowed when there's a mutable borrow. Try adding a println! there
/// to see what error message you get!
///
/// If you want to force the closure to take ownership of the values it uses in the environment
/// even though the body of the closure doesn't strictly need ownership, you can use the move
/// keyword before the parameter list.
///
/// This technique is mostly useful when passing a closure to a new thread to move the data so that
/// it's owned by the new thread.
///
///     use std::thread;
///
///     fn main() {
///         let list = vec![1, 2, 3];
///         println!("Before defining closure: {list:?}");
///
///         thread::spawn(move || println!("From thread: {list:?}"))
///             .join()
///             .unwrap();
///     }
///     Listing 13-6: Using move to force the closure for the thread to take ownership of list
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("
        The user with preference {:?} gets {:?}",
        user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("
        The user with preference {:?} gets {:?}",
        user_pref2, giveaway2);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    // Can't do it here as borrows_mutably borrows mutably 😁
    // println!("Before calling closure: {list:?}");
    borrows_mutably();
    println!("After calling closure: {list:?}");
}
