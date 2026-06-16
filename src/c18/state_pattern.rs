/// Implementing an Object-Oriented Design Pattern
///
/// The state pattern is an object-oriented design pattern. The crux of the pattern is that we
/// define a set of states a value can have internally. The states are represented by a set of state
/// objects, and the value's behaviour changes based on its state.
///
/// The state objects share functionality: In Rust, of course we use structs and traits rather than
/// objects and inheritance. Each state object is responsible for its own behaviour and for
/// governing when it should change into another state. The value that holds a state object knows
/// nothing about the different behaviour of the states or when to transition between states.
///
/// The advantage of using the state pattern is that, when the business requirements of the program
/// change, we won't need to change the code the value holding the state or the code that uses the
/// value. We'll only need to update the code inside one of the state objects to change its rules or
/// perhaps and more state objects.
///
/// Attempting Traditional Object-Oriented Style
///
/// There are infinite ways to structure code to solve the same problem, each with different
/// trade-offs. The first implementation is more of a traditional object-oriented style, which is
/// possible to write in Rust, but doesn't take advantage of some of Rust's strengths.
///
/// Listing 18-11 shows this workflow in code form: This is an example usage of the API we'll
/// implement in a library crate named blog.
///
///     use blog::Post;
///
///     fn main() {
///         let mut post = Post::new();
///
///         post.add_text("I ate a salad for lunch today");
///         assert_eq!("", post.content());
///
///         post.request_review();
///         assert_eq!("", post.content());
///
///         post.approve();
///         assert_eq!("I ate a salad for lunch today", post.content());
///     }
///     Listing 18-11: Code that demonstrates the desired behaviour we want our blog crate to have
/// We want to allow the user to create a new draft blog post with Post::new. We want to allow text
/// to be added to the blog post. If we try to get the post's content immediately, before approval,
/// we shouldn't get any text because the post is still in draft. We've added assert_eq! in the
/// code for demonstration purposes.
///
/// Next, we want to enable a request for a review of the post, and we want content to return an
/// empty string while waiting for the review. When the post receives approval, it should get
/// published, meaning the text of the post will be returned when content is called.
///
/// Notice that the only type we're interacting with from the crate is the Post type. This type
/// will use the state pattern and will hold a value that will be one of three state objects
/// representing the states a post can be in--draft, review, or published. Changing from one state
/// to another will be managed internally within the Post type. The states change in response to
/// the methods called by our library's users on the Post instance, but they don't have to manage
/// the state changes directly. Also, users can't make a mistake with the states, such as
/// publishing a post before it's reviewed.
