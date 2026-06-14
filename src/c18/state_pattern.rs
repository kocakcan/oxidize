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
