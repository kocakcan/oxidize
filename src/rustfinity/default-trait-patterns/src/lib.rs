/// Application configuration with sensible defaults.
///
/// Implement `Default` manually with the following default values:
/// - theme: "light"
/// - dark_mode: false
/// - font_size: 14
/// - max_connections: 100
#[derive(Debug, Clone, PartialEq)]
pub struct AppConfig {
    pub theme: String,
    pub dark_mode: bool,
    pub font_size: u32,
    pub max_connections: usize,
}

// TODO: Implement Default for AppConfig with custom default values
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "light".into(),
            dark_mode: false,
            font_size: 14,
            max_connections: 100,
        }
    }
}

/// A simple counter starting at zero.
#[derive(Debug, Clone, PartialEq)]
pub struct Counter {
    pub count: i32,
}

// TODO: Implement Default for Counter (count should be 0)
impl Default for Counter {
    fn default() -> Self {
        Self { count: 0 }
    }
}

impl Counter {
    /// Creates a new counter with the specified starting value.
    pub fn new(count: i32) -> Self {
        Counter { count }
    }

    /// Increments the counter by 1.
    pub fn increment(&mut self) {
        self.count += 1;
    }

    /// Decrements the counter by 1.
    pub fn decrement(&mut self) {
        self.count -= 1;
    }
}

/// A value constrained between minimum and maximum bounds.
///
/// Implement `Default` where T: Default + Clone, setting all fields to T::default().
#[derive(Debug, Clone, PartialEq)]
pub struct BoundedValue<T> {
    pub value: T,
    pub min: T,
    pub max: T,
}

// TODO: Implement Default for BoundedValue<T> where T: Default + Clone
impl<T: Default + Clone> Default for BoundedValue<T> {
    fn default() -> Self {
        Self {
            value: T::default(),
            min: T::default(),
            max: T::default(),
        }
    }
}

impl<T> BoundedValue<T> {
    /// Creates a new BoundedValue with specified value, min, and max.
    pub fn new(value: T, min: T, max: T) -> Self {
        BoundedValue { value, min, max }
    }
}

impl<T: Ord + Clone> BoundedValue<T> {
    /// Clamps the value to be within the min and max bounds.
    pub fn clamp(&mut self) {
        if self.value < self.min {
            self.value = self.min.clone();
        } else if self.value > self.max {
            self.value = self.max.clone();
        }
    }
}

/// Status of an operation.
///
/// Implement `Default` to return `Status::Pending`.
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Pending,
    Active,
    Completed,
    Failed,
}

// TODO: Implement Default for Status (should return Status::Pending)
impl Default for Status {
    fn default() -> Self {
        Status::Pending
    }
}

/// Partial configuration with optional fields.
///
/// Used with `merge_with_defaults` to create a complete `AppConfig`.
#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub theme: Option<String>,
    pub dark_mode: Option<bool>,
    pub font_size: Option<u32>,
    pub max_connections: Option<usize>,
}

/// Creates a default instance of any type implementing `Default`.
///
/// # Examples
///
/// ```ignore
/// let s: String = create_with_defaults();
/// assert_eq!(s, "");
/// ```
pub fn create_with_defaults<T: Default>() -> T {
    // TODO
    T::default()
}

/// Merges a partial configuration with defaults.
///
/// Any `None` values in `partial` should be replaced with the corresponding
/// default values from `AppConfig::default()`.
pub fn merge_with_defaults(partial: PartialConfig) -> AppConfig {
    // TODO
    let defaults: AppConfig = create_with_defaults();

    AppConfig {
        theme: partial.theme.unwrap_or(defaults.theme),
        dark_mode: partial.dark_mode.unwrap_or(defaults.dark_mode),
        font_size: partial.font_size.unwrap_or(defaults.font_size),
        max_connections: partial.max_connections.unwrap_or(defaults.max_connections),
    }
}

/// Creates a vector with `count` elements, each initialized to `T::default()`.
///
/// # Examples
///
/// ```ignore
/// let zeros: Vec<i32> = default_vec(5);
/// assert_eq!(zeros, vec![0, 0, 0, 0, 0]);
/// ```
pub fn default_vec<T: Default>(count: usize) -> Vec<T> {
    // TODO
    (0..count).map(|_| T::default()).collect()
}

pub fn main() {
    // AppConfig with defaults
    let config = AppConfig::default();
    println!("Default config: {:?}", config);

    // Struct update syntax with Default
    let custom = AppConfig {
        dark_mode: true,
        ..Default::default()
    };
    println!("Custom config: {:?}", custom);

    // Counter with default
    let counter = Counter::default();
    println!("Default counter: {:?}", counter);

    // BoundedValue with default
    let bounded: BoundedValue<i32> = BoundedValue::default();
    println!("Default bounded value: {:?}", bounded);

    // Status with default
    let status = Status::default();
    println!("Default status: {:?}", status);

    // Generic function with Default
    let s: String = create_with_defaults();
    println!("Default string: {:?}", s);

    // Merging with defaults
    let partial = PartialConfig {
        theme: Some("dark".to_string()),
        dark_mode: None,
        font_size: Some(16),
        max_connections: None,
    };
    let merged = merge_with_defaults(partial);
    println!("Merged config: {:?}", merged);

    // Creating default-initialized vectors
    let zeros: Vec<i32> = default_vec(5);
    println!("Default vec: {:?}", zeros);
}
