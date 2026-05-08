// Treating Smart Pointers Like Regular References
//
// Implementing the Deref trait allows you to customize the behaviour of the dereference operator *
// (not to be confused with the multiplication or glob operator). By implementing Deref in such a
// way that a smart pointer can be treated like a regular reference, you can write code that
// operates on references and use that code with smart pointers too.
//
// Following the Reference to the Value
//
// A regular reference is a type of pointer, and one way to think of a pointer is as an arrow to a
// value stored somewhere else. In Listing 15-6, we create a reference to an i32 value and then use
// the dereference operator to follow the reference to the value.
//
//  fn main() {
//      let x = 5;
//      let y = &x;
//
//      assert_eq!(5, x);
//      assert_eq!(5, *x);
//  }
//  Listing 15-6: Using the dereference operator to follow a reference to an i32 value
// The variable x holds an i32 value 5. We set y equal to a reference to x. We can assert that x is
// equal to 5. However, if we want to make an assertion about the value in y, we have to use *y to
// follow the reference to the value it's pointing to (hence, dereference) so that the compiler can
// compare the actual value. Once we dereference y, we have access to the integer value y is
// pointing to that we can compare with 5.
//
// If we tried to write assert_eq!(5, y); Instead, we would get this compilation error:
//
//  error: can't compare `{integer}` with `&{integer}`
// Comparing a number and a reference to a number isn't allowed because they're different types. We
// must use the dereference operator to follow the reference to the value it's pointing to.
//
// Using Box<T> Like a Reference
//
// We can rewrite the code in Listing 15-6 to use a Box<T> instead of a reference; the dereference
// operator used on the Box<T> in Listing 15-7 functions in the same way as the dereference
// operator used on the reference in Listing 15-6.
//
//      fn main() {
//          let x = 5;
//          let y = Box::new(x);
//
//          assert_eq!(5, x);
//          assert_eq!(5, *y);
//      }
//      Listing 15-7: Using the dereference operator on a Box<i32>
// The main difference between Listing 15-7 and Listing 15-6 is that here we set y to be an
// instance of a box pointing to a copied value of x rather than a reference pointing to the value
// of x. In the last assertion, we can use the dereference operator to follow the box's pointer in
// the same way that we did when y was a reference.
//
// Defining Our Own Smart Pointer
//
// Let's build a wrapper type similar to the Box<T> type provided by the standard library to
// experience how smart pointer types behave differently from references by default.
//
//  Note: There's one big difference between the MyBox<T> type we're aboutt to build and the real
//  Box<T>: Our version will not store its data on the heap. We are focusing this example on Deref,
//  so where the data is actually stored is less important than the pointer-like behaviour.
// The Box<T> type is ultimately defined as a tuple struct with one element, so Listing 15-8
// defines a MyBox<T> type in the same way.
//
//      struct MyBox<T>(T);
//
//      impl<T> MyBox<T> {
//          fn new(x: T) -> MyBox<T> {
//              MyBox(x)
//          }
//      }
//      Listing 15-8: Defining a MyBox<T> type
// We define a struct named MyBox and declare a generic parameter T because we want our type to
// hold values of any type. The MyBox type is a tuple struct with one element of type T. The
// MyBox::new function takes one parameter of type T and returns a MyBox instance that holds the
// value passed in.
//
//      fn main() {
//          let x = 5;
//          let y = MyBox::new(x);
//
//          assert_eq!(5, x);
//          assert_eq!(5, *y);
//      }
//      Listing 15-9: Attempting to use MyBox<T> in the same way we used references and Box<T>
//
//      error: type `MyBox<{integer}>` cannot be dereferenced
// Our MyBox<T> type can't be dereferenced because we haven't implemented trait ability on our
// type. To enable dereferencing with the * operator, we implement the Deref trait.
//
//
