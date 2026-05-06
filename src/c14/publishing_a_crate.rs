/// Making Useful Documentation Comments
///
/// Accurately documenting your packages will help other users know how and when to use them, so
/// it's worth investing the time to write documentation. Rust has a particular kind of comments
/// for documentation, known conveniently as a documentation comment, that will generate HTML
/// documentation. The HTML displays the contents of documentation comments for public API items
/// intended for programmers interested in knowing how to use your crate as opposed to how your
/// crate is implemented.
///
/// Documentation comments use three slashes (///), instead of two and support Markdown notation
/// for formatting the text. Place documentation comments just before the item they're documenting.
/// Listing 14-1 shows documentation comments for an add_one function in a crate named my_crate.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
///     pub fn add_one(x: i32) -> i32 {
///         x + 1
///     }
///     Listing 14-1: A documentation comment for a function
/// Here, we give a description of what the add_one function does, start a section with the heading
/// Examples, and then provide code that demonstrates how to use the add_one function. We can
/// generate the HTML documentation from this documentation comment by running cargo doc. This
/// comment runs the rustdoc tool distributed with Rust and puts the generated HTML documentation
/// in the target/doc directory.
///
/// For convenience, running cargo doc --open will build the HTML for your current crate's
/// documentation (as well as the documentation for all of your crate's dependencies) and open the
/// result in a browser. Navigate to the add_one function and you'll see how the text in the
/// documentation comments is rendered.
///
/// Commonly Used Sections
///
/// We used the # Examples Markdown heading in Listing 14-1 to create a section in the HTML with the title
/// "Examples". Here are some other sections that crate authors commonly use in their documentation:
///
///     - Panics: These are the scenarios in which the function being documented could panic.
///       Callers of the function who don't want their programs to panic should make sure they don't
///       call the function in these situtations.
///     - Errors: If the function returns a Result, describing the kinds of errors that might occur and
///       what conditions might cause those errors to be returned can be helpful to callers so that
///       they can write code to handle the different kinds of errors in different ways.
///     - Safety: If the function is unsafe to call, there should be a section explaining why the function is
///       unsafe and covering the invariants that the function expect callers to uphold.
/// Most documentation comments don't need all of these sections, but this is a good checklist to remind you of
/// the aspects of your code users will be interested in knowing about.
///
/// Documentation Comments are Tests
///
/// Adding example code blocks in your documentation comments can help demonstrate how to use
/// you library and has an additional bonus: Running cargo test will run the code examples in
/// your documentation as tests! Nothing is better than documentation with examples. But nothing
/// is worse than your examples that don't work because the code has changed since the documentation
/// was written. If we run cargo test with the documentation for the add_one function from Listing
/// 14-1, we will see a section in the test results.
///
/// Now, if we change either the function or the example so that assert_eq! in the example
/// panics, and run cargo test again, we'll see that the doc tests catch that the example and the code
/// are out of sync with each other!
///
/// Contained Item Comments
///
/// They style of doc comment //! adds documentation to the item that contains the comments rather than
/// to the items following the comments. We typically use these doc comments inside the
/// crate root file (src/lib.rs by convention) or inside a module to document the crate or the module
/// as a whole.
///
/// For example, to add documentation that describes the purpose of the my_crate crate that
/// contains the add_one function, we add documentation comments that start with //! to the beginning
/// of the src/lib.rs file, as shown in Listing 14-2.
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.

/// Notice there isn't any code after the last line that begins with //!. Because we started the
/// comments with //! instead of ///, we're documenting the item that contains this comment
/// rather than an item that follows this comment. In this case, that item is the src/lib.rs file, which is
/// the crate root. These comments describe the entire crate.
///
/// When we run cargo doc --open, these comments will display on the front page of the documentation for my_crate
/// above the list of public items in the crate.
///
/// Documentation comments within items are useful for describing crates and modules especially. Use them to expplain
/// the overall purpose of the container to help your users understand the crate's organization.
