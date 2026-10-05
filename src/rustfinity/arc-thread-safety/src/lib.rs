use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, Weak};

/// Creates a new atomic reference-counted value.
///
/// Wraps the given value in an `Arc<T>` for thread-safe shared ownership.
pub fn create_arc<T>(value: T) -> Arc<T> {
    Arc::new(value)
}

/// Clones an atomic reference-counted pointer.
///
/// This atomically increments the reference count without cloning the underlying data.
pub fn clone_arc<T>(arc: &Arc<T>) -> Arc<T> {
    Arc::clone(arc)
}

/// Returns the strong reference count of an Arc.
pub fn get_strong_count<T>(arc: &Arc<T>) -> usize {
    Arc::strong_count(arc)
}

/// Gets a clone of the value inside an Arc.
pub fn get_value<T: Clone>(arc: &Arc<T>) -> T {
    (**arc).clone()
}

/// A thread-safe configuration that can be shared across threads.
#[derive(Debug, Clone)]
pub struct SharedConfig {
    app_name: String,
    max_connections: usize,
    debug_mode: bool,
}

impl SharedConfig {
    /// Creates a new shared configuration wrapped in Arc.
    pub fn new(app_name: String, max_connections: usize, debug_mode: bool) -> Arc<Self> {
        Arc::new(SharedConfig {
            app_name,
            max_connections,
            debug_mode,
        })
    }

    /// Returns the application name.
    pub fn app_name(&self) -> &str {
        &self.app_name
    }

    /// Returns the maximum number of connections.
    pub fn max_connections(&self) -> usize {
        self.max_connections
    }

    /// Returns whether debug mode is enabled.
    pub fn debug_mode(&self) -> bool {
        self.debug_mode
    }
}

/// Creates a weak reference from an Arc.
pub fn create_weak<T>(arc: &Arc<T>) -> Weak<T> {
    Arc::downgrade(arc)
}

/// Attempts to upgrade a weak reference to a strong reference.
pub fn upgrade_weak<T>(weak: &Weak<T>) -> Option<Arc<T>> {
    weak.upgrade()
}

/// Returns the weak reference count of an Arc.
pub fn get_weak_count<T>(arc: &Arc<T>) -> usize {
    Arc::weak_count(arc)
}

/// A thread-safe counter using atomic operations.
#[derive(Debug)]
pub struct AtomicCounter {
    value: Arc<AtomicUsize>,
}

impl AtomicCounter {
    /// Creates a new counter with initial value 0.
    pub fn new() -> Self {
        Self::new_with_value(0)
    }

    /// Creates a new counter with a specific initial value.
    pub fn new_with_value(value: usize) -> Self {
        AtomicCounter {
            value: Arc::new(AtomicUsize::new(value)),
        }
    }

    /// Gets the current value of the counter.
    pub fn get(&self) -> usize {
        self.value.load(Ordering::SeqCst)
    }

    /// Increments the counter by 1 and returns the previous value.
    pub fn increment(&self) -> usize {
        self.value.fetch_add(1, Ordering::SeqCst)
    }

    /// Decrements the counter by 1 and returns the previous value.
    pub fn decrement(&self) -> usize {
        self.value.fetch_sub(1, Ordering::SeqCst)
    }

    /// Adds a value to the counter and returns the previous value.
    pub fn add(&self, val: usize) -> usize {
        self.value.fetch_add(val, Ordering::SeqCst)
    }

    /// Creates another handle to the same counter.
    pub fn clone_counter(&self) -> Self {
        AtomicCounter {
            value: Arc::clone(&self.value),
        }
    }
}

impl Default for AtomicCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// A thread-safe vector using Arc and Mutex.
#[derive(Debug)]
pub struct SharedVec<T> {
    data: Arc<Mutex<Vec<T>>>,
}

impl<T> SharedVec<T> {
    /// Creates a new empty shared vector.
    pub fn new() -> Self {
        SharedVec {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Adds an element to the end of the vector.
    pub fn push(&self, value: T) {
        self.data.lock().unwrap().push(value);
    }

    /// Removes and returns the last element, or None if empty.
    pub fn pop(&self) -> Option<T> {
        self.data.lock().unwrap().pop()
    }

    /// Returns the current number of elements.
    pub fn len(&self) -> usize {
        self.data.lock().unwrap().len()
    }

    /// Returns true if the vector is empty.
    pub fn is_empty(&self) -> bool {
        self.data.lock().unwrap().is_empty()
    }

    /// Creates another handle to the same vector.
    pub fn clone_vec(&self) -> Self {
        SharedVec {
            data: Arc::clone(&self.data),
        }
    }
}

impl<T: Clone> SharedVec<T> {
    /// Gets a clone of the element at the specified index.
    pub fn get(&self, index: usize) -> Option<T> {
        self.data.lock().unwrap().get(index).cloned()
    }
}

impl<T> Default for SharedVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

// Example usage
pub fn main() {
    // Basic Arc operations
    let shared = create_arc(42);
    println!("Created arc with value: {}", *shared);
    println!("Strong count: {}", get_strong_count(&shared));

    let _cloned = clone_arc(&shared);
    println!("After clone, strong count: {}", get_strong_count(&shared));

    // Shared configuration
    let config = SharedConfig::new("MyApp".to_string(), 100, true);
    println!(
        "App: {}, Max connections: {}",
        config.app_name(),
        config.max_connections()
    );

    // Atomic counter
    let counter = AtomicCounter::new();
    counter.increment();
    counter.increment();
    println!("Counter value: {}", counter.get());

    // Shared vector
    let vec: SharedVec<i32> = SharedVec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("Vec length: {}", vec.len());
    println!("Element at index 1: {:?}", vec.get(1));
}
