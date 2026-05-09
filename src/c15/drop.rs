/// Running Code on Cleanup with the Drop Trait
///
/// The second trait important to the smart pointer pattern is Drop, which lets you customize what
/// happens when a value is about to go out of scope. You can provide an implementation for the Drop
/// trait or any type, and that code can be used to release resources like files or network
/// connections.
///
/// We're introducing Drop in the context of smart pointers because the functionality of the Drop
/// trait is almost always used when implementing a smart pointer. For example, when a Box<T> is
/// droppped, it will deallocate the space on the heap that the box points to.
///
/// You specify the code to run when a value goes out of scope by implementing the Drop trait. The
/// Drop trait requires you to implement one method named drop that takes a mutable reference to
/// self. To see when Rust calls drop, let's implement drop with println! statements for now.
///
/// Listing 15-14 shows CustomSmartPointer struct whose only custom functionality is that it will
/// print Dropping CustomSmartPointer! when the instance goes out of scope, to show when Rust runs
/// the drop method.
///
///     struct CustomSmartPointer {
///         data: String,
///     }
///
///     impl Drop for CustomSmartPointer {
///         fn drop(&mut self) {
///             println!("Dropping CustomSmartPointer with data `{}`", self.data);
///         }
///     }
///
///     fn main() {
///         let c = CustomSmartPointer {
///             data: String::from("my stuff"),
///         };
///         let d = CustomSmartPointer {
///             data: String::from("other stuff"),
///         };
///         println!("CustomSmartPointers created");
///     }
///     Listing 15-14: A CustomSmartPointer struct that implements the Drop trait where we would put
///     our cleanup code
/// The Drop trait is included in the prelude, so we don't need to bring it into scope. We implement
/// the Drop trait on CustomSmartPointer and provide an implementation for the drop method that
/// calls println!. The body of the drop method is where you would place any logic that you wanted
/// to run when an instance of your type goes out of scope.
///
/// In main, we create two instances of CustomSmartPointer and then print CustomSmartPointers
/// created. At the end of main, our instances of CustomSmartPointer will go out of scope, and Rust
/// will call the code we put in the drop method, printing our final message. Note that we didn't
/// need to drop method explicitly.
///
/// Rust automatically called drop for us when our instances went out of scope, calling the code we
/// specified. Variables are dropped in the reverse order of their creation, so d was dropped before
/// c.
///
/// Unfortunately, it's not straightforward to disable the automatic drop functionality. Disabling
/// drop isn't usually necessary; the whole point of the Drop trait is that it's taken care of
/// automatically. Occasionally, however, you might want to clean up a value early. One example is
/// when using smart pointers that manage locks: You might want to force the drop method that
/// releases the lock so that other code in the same scope can acquire the lock. Rust doesn't let
/// you call the Drop trait's drop method manually; instead, you have to call the std::mem::drop
/// function provided by the standard library if you want to force a value to be dropped before the
/// end of its scope.
///
/// Trying to call the Drop trait's drop method manually by modifying the main function from Listing
/// 15-14 won't work, as shown in Listing 15-15.
///
///     fn main() {
///         let c = CustomSmartPointer {
///             data: String::from("some data"),
///         };
///         println!("CustomSmartPointer created");
///         c.drop();
///         println!("CustomSmartPointer dropped before the end of main");
///     }
///     Listing 15-15: Attempting to call the drop method from the Drop trait manually to clean up early
/// When we try to compile this code, we'll get this error:
///
///     error: explicit use of destructor method
/// Rust doesn't let us call drop explicitly, because Rust would still automatically call drop on
/// the value at the end of main. This would cause a double free error because Rust would be trying
/// to clean up the same value twice.
///
/// We can't disable the automatic insertion of drop when a value goes out of scope, and we can't
/// call the drop method explicitly. So, if we need to force a value to be cleaned up early, we use
/// the std::mem::drop function.
///
/// The std::mem::drop function is different from the drop method in the Drop trait. We call it by
/// passing as an argument the value we want to force-drop. The function is in the prelude so we can
/// modify main in Listing  15-15 to call the drop function, as shown in Listing 15-16.
///
///     fn main() {
///         let c = CustomSmartPointer {
///             data: String::from("some data"),
///         };
///         println!("CustomSmartPointer created");
///         drop(c);
///         println!("CustomSmartPointer dropped before the end of main");
///     }
///     Listing 15-16: Calling std::mem::drop to explicitly drop a value before it goes out of scope
/// The text Dropping CustomSmartPointer with data `some data`! is printed between the
/// CustomSmartPointer created and CustomSmartPointer dropped before the end of main text, showing
/// that the drop method code is called to drop c at that point.
///
/// You can use code specified in a Drop trait implementation in many ways to make cleanup
/// convenient and safe: For instance, you could use it to create your own memory allocator! With
/// the Drop trait and Rust's ownership system, you don't have to remember to clean up, because Rust
/// does it automatically.
///
/// You also don't have to worry about the problems resulting from accidentally cleaning up values
/// still in use: The ownership system that makes sure references are always valid also ensures that
/// drop gets called only once when the value is no longer being used.
use std::mem::drop;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    {
        let e = CustomSmartPointer {
            data: String::from("this will be dropped first"),
        };
    }
    println!("CustomSmartPointers created");
}
