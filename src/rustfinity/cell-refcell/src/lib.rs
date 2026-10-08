use std::cell::{Cell, Ref, RefCell, RefMut};
use std::rc::Rc;

// =============================================================================
// Part 1: Cell Basics
// =============================================================================

/// A simple counter using `Cell` for interior mutability.
///
/// `Cell<T>` provides interior mutability for `Copy` types with zero overhead.
/// All operations work through shared references (&self).
#[derive(Debug, Default)]
pub struct Counter {
    // TODO: Add a Cell<i32> field named `value`
    value: Cell<i32>,
}

impl Counter {
    /// Creates a new counter with initial value 0.
    pub fn new() -> Self {
        // TODO: Implement using Cell::new(0)
        Self {
            value: Cell::new(0),
        }
    }

    /// Gets the current count.
    pub fn get(&self) -> i32 {
        // TODO: Use Cell::get() to return the value
        self.value.get()
    }

    /// Sets the count to a specific value.
    pub fn set(&self, value: i32) {
        // TODO: Use Cell::set() to set the value
        self.value.set(value);
    }

    /// Increments the count by 1.
    pub fn increment(&self) {
        // TODO: Get current value, add 1, and set it back
        self.value.set(self.value.get() + 1);
    }

    /// Decrements the count by 1.
    pub fn decrement(&self) {
        // TODO: Get current value, subtract 1, and set it back
        self.value.set(self.value.get() - 1);
    }
}

/// A cached value that tracks how many times it's been accessed.
///
/// Uses `Cell` for both the value and access count.
#[derive(Debug)]
pub struct CachedValue<T: Copy + Default> {
    // TODO: Add a Cell<T> field named `value`
    // TODO: Add a Cell<u32> field named `access_count`
    value: Cell<T>,
    access_count: Cell<u32>,
}

impl<T: Copy + Default> CachedValue<T> {
    /// Creates a new cached value.
    pub fn new(value: T) -> Self {
        // TODO: Initialize both cells, access_count starts at 0
        Self {
            value: Cell::new(value),
            access_count: Cell::new(0),
        }
    }

    /// Gets the cached value and increments the access count.
    pub fn get(&self) -> T {
        // TODO: Increment access_count, then return the value
        self.access_count.set(self.access_count.get() + 1);
        self.value.get()
    }

    /// Sets a new value and resets the access count.
    pub fn set(&self, value: T) {
        // TODO: Set the new value and reset access_count to 0
        self.value.set(value);
        self.access_count.set(0);
    }

    /// Returns the number of times the value was accessed.
    pub fn access_count(&self) -> u32 {
        // TODO: Return the access count
        self.access_count.get()
    }
}

impl<T: Copy + Default> Default for CachedValue<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

// =============================================================================
// Part 2: RefCell Basics
// =============================================================================

/// A string that can be modified through shared references.
///
/// Uses `RefCell` for interior mutability of the String.
#[derive(Debug, Default)]
pub struct SharedString {
    // TODO: Add a RefCell<String> field named `value`
    value: RefCell<String>,
}

impl SharedString {
    /// Creates a new shared string.
    pub fn new(s: &str) -> Self {
        // TODO: Initialize with RefCell::new(s.to_string())
        Self {
            value: RefCell::new(s.to_string()),
        }
    }

    /// Gets a clone of the string.
    pub fn get(&self) -> String {
        // TODO: Use borrow() to get an immutable reference, then clone
        self.value.borrow().clone()
    }

    /// Sets a new string value.
    pub fn set(&self, s: &str) {
        // TODO: Use borrow_mut() to get a mutable reference, then assign
        *self.value.borrow_mut() = s.to_string();
    }

    /// Appends a string to the current value.
    pub fn append(&self, s: &str) {
        // TODO: Use borrow_mut() and push_str()
        self.value.borrow_mut().push_str(s);
    }

    /// Returns the length of the string.
    pub fn len(&self) -> usize {
        // TODO: Use borrow() and len()
        self.value.borrow().len()
    }

    /// Returns true if the string is empty.
    pub fn is_empty(&self) -> bool {
        // TODO: Use borrow() and is_empty()
        self.value.borrow().is_empty()
    }
}

/// A vector with interior mutability.
///
/// Uses `RefCell` to allow modification through shared references.
#[derive(Debug, Default)]
pub struct SharedVec<T> {
    // TODO: Add a RefCell<Vec<T>> field named `data`
    data: RefCell<Vec<T>>,
}

impl<T> SharedVec<T> {
    /// Creates a new empty shared vector.
    pub fn new() -> Self {
        // TODO: Initialize with RefCell::new(Vec::new())
        Self {
            data: RefCell::new(Vec::new()),
        }
    }

    /// Adds an element to the end of the vector.
    pub fn push(&self, value: T) {
        // TODO: Use borrow_mut() and push()
        self.data.borrow_mut().push(value);
    }

    /// Removes and returns the last element, or None if empty.
    pub fn pop(&self) -> Option<T> {
        // TODO: Use borrow_mut() and pop()
        self.data.borrow_mut().pop()
    }

    /// Returns the current number of elements.
    pub fn len(&self) -> usize {
        // TODO: Use borrow() and len()
        self.data.borrow().len()
    }

    /// Returns true if the vector is empty.
    pub fn is_empty(&self) -> bool {
        // TODO: Use borrow() and is_empty()
        self.data.borrow().is_empty()
    }
}

impl<T: Clone> SharedVec<T> {
    /// Gets a clone of the element at the specified index.
    pub fn get(&self, index: usize) -> Option<T> {
        // TODO: Use borrow(), get(index), and cloned()
        self.data.borrow().get(index).cloned()
    }
}

// =============================================================================
// Part 3: Safe Borrowing with try_borrow
// =============================================================================

/// A RefCell wrapper that provides safe, non-panicking borrow operations.
///
/// Uses `try_borrow` and `try_borrow_mut` internally to avoid panics.
#[derive(Debug, Default)]
pub struct SafeCell<T> {
    // TODO: Add a RefCell<T> field named `value`
    value: RefCell<T>,
}

impl<T> SafeCell<T> {
    /// Creates a new SafeCell.
    pub fn new(value: T) -> Self {
        // TODO: Initialize with RefCell::new(value)
        Self {
            value: RefCell::new(value),
        }
    }

    /// Tries to borrow the value immutably.
    ///
    /// Returns `None` if the value is currently mutably borrowed.
    pub fn try_read(&self) -> Option<Ref<'_, T>> {
        // TODO: Use try_borrow() and convert Result to Option with .ok()
        self.value.try_borrow().ok()
    }

    /// Tries to borrow the value mutably.
    ///
    /// Returns `None` if the value is currently borrowed.
    pub fn try_write(&self) -> Option<RefMut<'_, T>> {
        // TODO: Use try_borrow_mut() and convert Result to Option with .ok()
        self.value.try_borrow_mut().ok()
    }

    /// Checks if the value is currently borrowed.
    ///
    /// Returns `true` if there are any active borrows.
    pub fn is_borrowed(&self) -> bool {
        // TODO: Try to borrow mutably - if it fails, value is borrowed
        self.value.try_borrow_mut().is_err()
    }

    /// Applies a function to the value if not currently borrowed.
    ///
    /// Returns `None` if the value cannot be borrowed.
    pub fn with_value<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R,
    {
        // TODO: Use try_read() and map() to apply the function
        self.try_read().map(|r| f(&r))
    }

    /// Applies a mutating function to the value if not currently borrowed.
    ///
    /// Returns `None` if the value cannot be mutably borrowed.
    pub fn with_value_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut T) -> R,
    {
        // TODO: Use try_write() and map() to apply the function
        self.try_write().map(|mut r| f(&mut r))
    }
}

// =============================================================================
// Part 4: Combining Rc and RefCell
// =============================================================================

/// A reference-counted counter with interior mutability.
///
/// Combines `Rc` for shared ownership with `RefCell` for interior mutability.
#[derive(Debug, Default)]
pub struct SharedCounter {
    // TODO: Add a RefCell<i32> field named `value`
    value: RefCell<i32>,
}

impl SharedCounter {
    /// Creates a new shared counter wrapped in Rc.
    pub fn new() -> Rc<Self> {
        // TODO: Create a new SharedCounter and wrap it in Rc
        Rc::new(Self {
            value: RefCell::new(0),
        })
    }

    /// Gets the current value.
    pub fn get(&self) -> i32 {
        // TODO: Use borrow() and dereference
        *self.value.borrow()
    }

    /// Increments the counter by 1.
    pub fn increment(&self) {
        // TODO: Use borrow_mut() and add 1
        *self.value.borrow_mut() += 1;
    }

    /// Decrements the counter by 1.
    pub fn decrement(&self) {
        // TODO: Use borrow_mut() and subtract 1
        *self.value.borrow_mut() -= 1;
    }

    /// Adds n to the counter.
    pub fn add(&self, n: i32) {
        // TODO: Use borrow_mut() and add n
        *self.value.borrow_mut() += n;
    }
}

/// A tree node with interior mutability for children.
///
/// Combines `Rc` for shared ownership with `RefCell` for mutable children list.
#[derive(Debug)]
pub struct TreeNode<T> {
    // TODO: Add a RefCell<T> field named `value`
    // TODO: Add a RefCell<Vec<Rc<TreeNode<T>>>> field named `children`
    value: RefCell<T>,
    children: RefCell<Vec<Rc<TreeNode<T>>>>,
}

impl<T> TreeNode<T> {
    /// Creates a new tree node wrapped in Rc.
    pub fn new(value: T) -> Rc<Self> {
        // TODO: Create a new TreeNode with the value and empty children, wrapped in Rc
        Rc::new(Self {
            value: RefCell::new(value),
            children: RefCell::new(Vec::new()),
        })
    }

    /// Gets a reference to the node's value.
    pub fn value(&self) -> Ref<'_, T> {
        // TODO: Use borrow() to return Ref<T>
        self.value.borrow()
    }

    /// Sets a new value for the node.
    pub fn set_value(&self, value: T) {
        // TODO: Use borrow_mut() and assign new value
        *self.value.borrow_mut() = value;
    }

    /// Adds a child node.
    pub fn add_child(&self, child: Rc<TreeNode<T>>) {
        // TODO: Use borrow_mut() on children and push the child
        self.children.borrow_mut().push(child);
    }

    /// Returns the number of children.
    pub fn children_count(&self) -> usize {
        // TODO: Use borrow() on children and return len()
        self.children.borrow().len()
    }
}

// Example usage
pub fn main() {
    // Cell-based Counter
    let counter = Counter::new();
    counter.increment();
    counter.increment();
    println!("Counter: {}", counter.get());

    // CachedValue with access tracking
    let cached = CachedValue::new(42);
    println!(
        "Value: {} (accessed {} times)",
        cached.get(),
        cached.access_count()
    );

    // RefCell-based SharedString
    let shared = SharedString::new("Hello");
    shared.append(", World!");
    println!("String: {}", shared.get());

    // SharedVec
    let vec: SharedVec<i32> = SharedVec::new();
    vec.push(1);
    vec.push(2);
    println!("Vec length: {}", vec.len());

    // SafeCell with try_borrow
    let safe = SafeCell::new(42);
    if let Some(doubled) = safe.with_value(|v| *v * 2) {
        println!("Doubled: {}", doubled);
    }

    // Rc<RefCell> pattern with SharedCounter
    let counter = SharedCounter::new();
    let counter2 = Rc::clone(&counter);
    counter.increment();
    counter2.increment();
    println!("Shared counter: {}", counter.get());

    // Tree with interior mutability
    let root = TreeNode::new("root");
    let child = TreeNode::new("child");
    root.add_child(child);
    println!("Tree children: {}", root.children_count());
}
