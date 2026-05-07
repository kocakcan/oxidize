/// Smart Pointers
///
/// A pointer is a general concept for a variable that contains an address in memory. This address
/// refers to, or "points at," some other data. The most common kind of pointer in Rust is a
/// reference. References are indicated by the & symbol and borrow the value they point to. They
/// don't have any special capabilities other than referring to data, and they have no overhead.
///
/// Smart pointers, on the other hand, are data structures that act like a pointer but also have
/// additional metadata and capabilities. Rust has a variety of smart pointers defined in the
/// standard library that provide functionality beyond that provided by references.
///
/// In Rust, with its concept of ownership and borrowing, there is an additional difference between
/// references and smart pointers: While references only borrow data, in many cases smart pointers
/// own the data they point to.
///
/// Smart pointers are usually implemented using structs. Unlike an ordinary struct, smart pointers
/// implement the Deref and Drop traits. The Deref trait allows an instance of the smart pointer
/// struct to behave like a reference so that you can write your code to work with either
/// references or smart pointers. The Drop trait allows you to customize the code that's run when
/// an instance of the smart pointer goes out of scope.
///
///     - Box<T>, for allocating values on the heap
///     - Rc<T>, a reference counting type that enables multiple ownership
///     - Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing
///     rules at runtime instead of compile time
///
/// Using Box<T> to Point to Data on the Heap
///
/// The most straightforward smart pointer is a box, whose type is written Box<T>. Boxes allow you
/// to store data on the heap rather than the stack. What remains on the stack is the pointer to
/// the heap data.
///
/// Boxes don't have performance overhead, other than storing their data on the heap instead of on
/// the stack. But they don't have many extra capabilities either. You'll use the most often in
/// these situations:
///
///     - When you have a type whose size can't be known at compile time, and you want to use a
///     value of that type in a context that requires an exact size
///     - When you have a large amount of data, and you want to transfer ownership but ensure that
///     the data won't be copied when you do so
///     - When you want to own a value, and you care only that it's a type that implements a
///     particular trait rather than being of a specific type
///
/// Storing Data on the Heap
///
///     fn main() {
///         let b = Box::new(5);
///         println!("b = {b}");
///     }
///     Listing 15-1: Storing an i32 value on the heap using a box
/// We define the variable b to have the value of a Box that points to the value 5, which is
/// allocated on the heap. This program will print b = 5; in this case, we can access the data in
/// the box similarly to how we would if this data were on the stack. Just like any owned value,
/// when a box goes out of scope, as b does at the end of main, it will be deallocated. The
/// deallocation happens both for the box (stored on the stack) and the data it points to (stored
/// on the heap).
///
/// Putting a single value on the heap isn't very useful, so you won't use boxes by themselves in
/// this way very often. Having values like a single i32 on the stack, where they're stored by
/// default, is more appropriate in the majority of situations.
///
/// Enabling Recursive Types with Boxes
///
/// A value of a recursive type can have another value of the same type as part of itself.
/// Recursive types pose an issue because Rust needs to know at compile time how much space a type
/// takes up. However, the nesting of values of recursive types could theoretically continue
/// infinitely, so Rust can't know how much space the value needs. Because boxes have a known size,
/// we can enable recursive types by inserting a box in the recursive type definition.
///
/// Understanding the Cons List
///
/// A const list is a data structure is made up of nested pairs, and is the Lisp version of a
/// linked list. Its name comes from the cons (short for construct function) in Lisp that
/// constructs a new pair from its two arguments. By calling cons on a pair consisting of a value
/// and another pair, we can construct cons lists made up of recursive pairs.
///
///     (1, (2, (3, Nil)))
/// Each item in a cons list contains two elements: the value of the current item and of the next
/// item. The last item in the list contains only a value called Nil without a next item. A cons
/// list is produced by recursively calling the const function. The canonical name to denote the
/// base case of the recursion is Nil. Note that this is not the same as the "null" or "nil"
/// concept, which is an invalid or absent value.
///
/// The cons list isn't a commonly used data structure in Rust. Most of the time when you have a
/// list of items in Rust, Vec<T> is a better choice to use. Other, more complex recursive data
/// types are useful in various situations, but by starting with the cons list we can explore how
/// boxes let us define a recursive data types without much distraction.
///
/// Listing 15-2 contains an enum definition for a cons list. Note that this code won't compile
/// yet, because the List type doesn't have a known size.
///
///     enum List {
///         Cons(i32, List),
///         Nil,
///     }
///     Listing 15-2: The first attempt at defining an enum to represent a cons list data structure
///     of i32 values
/// This cons list could have been implemented over generic types. Using the List type to store the
/// list 1, 2, 3 would look like the code in Listing 15-3.
///
///     use crate::List::{Cons, Nil};
///
///     fn main() {
///         let list = Cons(1, Cons(2, Cons(3, Nil)));
///     }
///     Listing 15-3: Using the List enum to store the list 1, 2, 3
/// The first Cons value holds 1 and another List value. This List value is another Cons value that
/// holds 2 and another List value. This List value is one more Cons value that hold 3 and a List
/// value, which is finally Nul, the non-recursive variant that signals the end of the list.
///
///     error: recursive type `List` has infinite size
/// The error shows this type "has infinite size." The reason is that we've defined List with a
/// variant that is recursive: It holds another value of itself directly. As a result, Rust can't
/// figure out how much space it needs to store a List value.
///
/// Computing the Size of a Non-Recursive Type
///
///     enum Message {
///         Quit,
///         Move { x: i32, y: i32 },
///         Write(String),
///         ChangeColor(i32, i32, i32),
///     }
/// To determine how much space to allocate for a Message value, Rust goes through each of the
/// variants to see which variants needs the most space. Rust sees that Message::Quit doesn't need
/// any space, Message::Move needs enough space to store two i32 values, and so forth. Because only
/// one variant will be used, the most space a Message value will need is the space it would take
/// to store the largest of its variants.
///
/// Contrast this with what happens when Rust tries to determine how much space a recursive type
/// like the List enum in Listing 15-2 needs. The compiler starts by looking at the Cons variant,
/// which holds a value of type i32 and a value of type List. Therefore, Cons needs an amount of
/// space equal to the size of an i32 plus the size of a List. To figure out how much memory the
/// List type needs, the compiler looks at the variants, starting with the Cons variant. The Cons
/// variant hold a value of type i32 and a value of type List, and this process continues
/// infinitely.
///
/// Getting a Recursive Type with a Known Size
///
/// Because Rust can't figure out how much space to allocate for recursively defined types, the
/// compiler gives an error with this helpful suggestion:
///
///     help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
/// In this suggestion, indirection means that instead of storing a value directly, we should
/// change the data structure to store the value indirectly by storing a pointer to the value instead.
///
/// Because a Box<T> is a pointer, Rust always knows how much space a Box<T> needs: A pointer's
/// size doesn't change based on the amount of data it's pointing to. This means we can put a
/// Box<T> inside the Cons variant instead of another List value directly. The Box<T> will point to
/// the next List value that will be on the heap rather than inside the Cons variant. Conceptually,
/// we still have a list created with lists holding other lists, but this implementation is now
/// more like placing the items next to one another rather than inside one another.
///
///     enum List {
///         Cons(i32, Box<List>),
///         Nil,
///     }
///
///     use crate::List::{Cons, Nil};
///
///     fn main() {
///         let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
///     }
///     Listing 15-5: The definition of List that uses Box<T> in order to have a known size
/// The Cons variant needs the size of an i32 plus the space to store the box's pointer data. The
/// Nil variant stores no values, so it needs less space on the stack than the Cons variant. We
/// know how that any List value will take up the size of an i32 plus the size of a box's pointer
/// data. By using a box, we've broken the infinite, recursive chain, so the compiler can figure
/// out the size it needs to store a List value.
///
/// Boxes provide only the indirection and heap allocation; they don't have any other special
/// capabilities. They also don't have the performance overhead that these special capabilities
/// incur, so they can be useful in cases like the cons list where the indirection is the only
/// feature we need.
///
/// The Box<T> type is a smart pointer because it implements the Deref trait, which allows Box<T>
/// values to be treated like references. When a Box<T> values goes out of scope, the heap data
/// that the box is pointing to is cleaned up as well because of the Drop trait implementation.
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
