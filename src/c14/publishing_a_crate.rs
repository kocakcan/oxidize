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
///
/// Exporting a Convenient Public API
///
/// The structure of your public API is a major consideration when publishing a crate. People who
/// use your crate are less familiar with the structure than you are and might have difficulty
/// finding the pieces they want to use if your crate has a large module hierarchy.
///
/// pub keyword is used to make items public and use keyword used to bring items into scope.
/// However, the structure that makes sense to you while you're developing a crate might not be
/// very convenient for your users. You might want to organize your structs in a hierarchy might
/// have trouble finding out that type exists. They might also be annoyed at having to enter use
/// my_crate::some_module::another_module::UsefulType; rather than use my_crate::UsefulType;
///
/// The good news is that if the structure isn't convenient for others to use from another library,
/// you don't have to rearrange your internal organization: Instead, you can re-export items to
/// make a public structure that's different from your private structure by using pub use.
/// Re-exporting takes a public item in one location and makes it public in another location, as if
/// it were defined in the other location instead.
///
/// For example, say we made a library named art for modeling artistic concepts. Within this
/// library are two modules: a kinds module containing two enums named PrimaryColor and
/// SecondaryColor and a utils module containing a function named mix, as shown in Listing 14-3.
//! # Art
//!
//! A library for modeling artistic concepts.
//!
//! pub mod kinds {
//!     /// The primary colors according to the RYB color model.
//!     pub enum PrimaryColor {
//!         Red,
//!         Yellow,
//!         Blue,
//!     }
//!
//!     /// The secondary colors according to the RYB color model.
//!     pub enum SecondaryColor {
//!         Orange,
//!         Green,
//!         Purple,
//!     }
//! }
//!
//! pub mod utils {
//!     use crate::kinds::*;
//!
//!     /// Combines two primary colors in equal amounts to create
//!     /// a secondary color.
//!     pub fn mix(c1: PrimaryColor, c2: SecondaryColor) -> SecondaryColor {
//!         // --snip--
//!     }
//! }
//! Listing 14-3: An art library with items organized into kinds and utils modules
/// Note that the PrimaryColor and SecondaryColor types aren't listed on the front page, nor is the
/// mix function. We have to click kinds and utils to see them.
///
/// Another crate that depends on this library would need use statements that bring the items from
/// art into scope, specifying the module structure that's currently defined. Listing 14-4 shows an
/// example of a crate that uses the PrimaryColor and mix items from the art crate.
///
///     use art::kinds::PrimaryColor;
///     use art::utils::mix;
///
///     fn main() {
///         let red = PrimaryColor::Red;
///         let yellow = PrimaryColor::Yellow;
///         mix(red, yellow);
///     }
///     Listing 14-4: A crate using the art crate's items with its internal structure exported
/// The author of the code in Listing 14-4, which uses the art crate, had to figure out that
/// PrimaryColor is in the kinds module and mix is in the utils module. The module structure of the
/// art crate is more relevant to developers working on the art crate than to those using it. The
/// internal structure doesn't contain any useful information for someone trying to understand how
/// to use the art crate, but rather causes confusion because developers who use it have to figure
/// out where to look, and must specify the module names in the use statement.
///
/// To remove the internal organization from the public API, we can modify the art crate code in
/// Listing 14-3 to add pub use statements to re-export the items at the top level, as shown in
/// Listing 14-5.
//! # Art
//!
//! A library for modeling artistic concepts.
//!
//! pub use self::kinds::PrimaryColor;
//! pub use self::kinds::SecondaryColor;
//! pub use self::utils::mix;
//!
//! pub mod kinds {
//!     // --snip--
//! }
//!
//! pub mod utils {
//!     // --snip--
//! }
//! Listing 14-5: Adding pub use statements to re-export items
//!
//! The API documentation that cargo doc generates for this crate will now list and link re-exports
//! on the front page, making the PrimaryColor and SecondaryColor types and the mix function easier
//! to find.
//!
//! The art crate users can still see and use the internal structure from Listing 14-3 as
//! demonstrated in Listing 14-4, or they can use the more convenient structure in Listing 14-5, as
//! shown in Listing 14-6.
//!
//!     use art::PrimaryColor;
//!     use art::mix;
//!
//!     fn main() {
//!         // --snip--
//!     }
/// In cases where there are many nested modules, re-exporting the types at the top level with pub
/// use can make a significant difference in the experience of people who use the crate. Another
/// common use of pub use is to re-export definitions of a dependency in the current crate to make
/// that crate's definitions part of your crate's public API.
///
/// Adding Metadata to a New Crate
///
/// Let's say you have a crate you want to publish. Before publishing, you'll need to add some
/// metadata in the [package] section of the crate's Cargo.toml file.
///
/// Your crate will need a unique name. While you're working on a crate locally, you can name a
/// crate basically whatever you'd like. However, crate names on crates.io are allocated on a
/// first-come, first-served basis. Once a crate name is taken, no one else can publish a crate
/// with that name. Before attempting to publish a crate, search for the name you want to use. If
/// the name has been used, you will need to find another name and edit the name field in the
/// Cargo.toml file under the [package] section to use the new name for publishing like so:
///
///     [package]
///     name = "guessing_game"
/// Even if you've chosen a unique name, when you run cargo publish to publish the crate at this
/// point, you'll get a warning and then an error:
///
/// This results in an error because you're missing some crucial information: A description and
/// license are required so that people will know what your crate does and under what terms they
/// can use it. In Cargo.toml, add a description that's just a sentence or two, because it will
/// appear with your crate in search results. For the license field, you need to give a license
/// identifier value. For example, to specify that you've licensed your crate using the MIT
/// license, add the MIT identifier.
///
///     [package]
///     name = "guessing_game"
///     version = "0.1.0"
///     edition = "2024"
///     description = "A fun game where you guess what number the computer has chosen."
///     license = "MIT OR Apache-2.0"
///
/// Publishing a New Version of an Existing Crate
///
/// When you've made changes to your crate and are ready to release a new version, you change the
/// version value specified in your Cargo.toml file and republish using cargo publish command.
///
/// Deprecating Versions from Crates.io
///
/// Although you can't remove previous versions of a crate, you can prevent any future projects
/// from adding them as a new dependency. This is useful when a crate version is broken for one
/// reason or another. In such situations, Cargo supports yanking a crate version.
///
/// Yanking a version prevents new projects from depending on that version while allowing all
/// existing projects that depend on it to continue. Essentially, a yank means that all projects
/// with a Cargo.lock will not break, and any future Cargo.lock files generated will not use the
/// yanked version.
///
/// To yank a version of a crate, in the directory of the crate that you've previously published,
/// run cargo yank and specify which version you want to yank. For example, if we've published a
/// crate named guessing_game version 1.0.1 and we want to yank it, we'd run the following in the
/// project directory for guessing_game:
///
///     cargo yank --vers 1.0.1
///
/// By adding --undo to the command, you can also undo a yank and allow projects to start depending
/// on a version again:
///
///     cargo yank --vers 1.0.1 --undo
/// A yank does not delete any code. It cannot.
