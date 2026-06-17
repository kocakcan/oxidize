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
///
/// Defining Post and Creating a New Instance
///
/// We know we need a public Post struct that holds some content, so we'll start with the
/// definition of the struct and an associated public new function to create an instance of Post.
/// We'll also make a private State trait that will define the behaviour that all the state objects
/// for a Post must have.
///
/// Then, Post will hold a trait object of Box<dyn State> inside an Option<T> in a private field
/// named state to hold the state object.
///
/// The State trait defines the behaviour shared by different post states. The state objects are
/// Draft, PendingReview, and Published, and they will all implement the State trait. For now, the
/// trait doesn't have any methods, and we'll start by defining just the Draft state because that
/// is the state we want a post to start in.
///
/// When we create a new Post, we set its state field to a Some value that holds a Box. This Box
/// points to a new instance of the Draft struct. This ensures that whenever we create a new
/// instance of Post, it will start out as a draft. Because the state field of Post is private,
/// there is no way to create a Post in any other state. In the Post::new function, we set the
/// content field to a new, empty String.
///
/// Storing the Text of the Post Content
///
/// We want to be able to call a method named add_text and pass it a &str that is then added as the
/// text content of the blog post. We implement this as a method, rather than exposing the content
/// field as pub, so that later we can implement a method that will control how the content field's
/// data is read.
///
/// The add_text method takes a mutable reference to self because we're changing the Post instance
/// that we're calling add_text on. We then call push_str on the String in content and pass the
/// text argument to add to the saved content. This behaviour doesn't depend on the state the post
/// is in, so it's not part of the state pattern. The add_text method doesn't interact with the
/// state field at all, but it is part of the behaviour we want to support.
///
/// Ensuring That the Content of a Draft Post is Empty
///
/// Even after we've called add_text and added some content to our post, we still want the content
/// method to return an empty string slice because the post is still in the draft state.
///
/// Summary
///
/// Regardless of whether you think Rust is an object-oriented language, you now know that you can
/// use trait objects to get some object-oriented features in Rust. Dynamic dispatch can give your
/// code some flexibility in exchange for a bit of runtime performance. You can use this
/// flexibility to implement object-oriented patterns that can help your code's maintainability.
/// Rust also has other features, like ownership, that object-oriented languages don't have. An
/// object-oriented pattern won't always be the best way to take advantage of Rust's strengths, but
/// it is an available option.
///
/// Context: The struct/trait approach is extensible in the sense that an API client could
/// potentially create a new state (such as Retracted) without changing the core API functionality.
/// When adding this state, the methods for other states do not need to be changed. Whereas with
/// enums, a client cannot add a new branch to the enum. Moreover, all match expressions must be
/// updated when a state is added.
///
/// A match is not likely to be slower than dynamic dispatch. A match is a simple branch based on
/// an enum's tag, while dynamic dispatch requires a layer of indirection through a trait object's
/// virtual table with non-inlined function calls.
///
/// An API client cannot add a new method for existing states in the struct/trait approach, they
/// can only add new state. The methods are fixed by the API author's trait definition. Note that
/// you could add a new method which only builds on existing methods via extension traits, such as:
///
///     trait StateExt {
///         fn request_review_twice(self: Box<Self>) -> Box<dyn State>;
///     }
///
///     impl<S: State> StateExt for S {
///         fn request_review_twice(self: Box<Self>) -> Box<dyn State> {
///             self.request_review().request_review()
///         }
///     }
/// But these extensions cannot read the private data of the states.
pub struct Post {
    // state: Option<Box<dyn State>>,
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // ""
        // self.state.as_ref().unwrap().content(self)
        &self.content
    }

    // pub fn request_review(&mut self) {
    //     if let Some(s) = self.state.take() {
    //         self.state = Some(s.request_review())
    //     }
    // }

    // pub fn approve(&mut self) {
    //     if let Some(s) = self.state.take() {
    //         self.state = Some(s.approve())
    //     }
    // }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
