use std::borrow::Cow;

// =============================================================================
// Part 1: Basic Cow Usage
// =============================================================================

/// Converts a string to uppercase only if it contains lowercase letters.
///
/// Returns `Cow::Borrowed` if the string is already all uppercase (or empty),
/// `Cow::Owned` if conversion was needed.
pub fn maybe_uppercase(s: &str) -> Cow<'_, str> {
    // TODO: Check if the string contains any lowercase letters
    // If yes, return Cow::Owned with the uppercase version
    // If no, return Cow::Borrowed with the original string
    for c in s.chars() {
        if c.is_lowercase() {
            return Cow::Owned(s.to_uppercase());
        } else {
            continue;
        }
    }
    Cow::Borrowed(s)
}

/// Ensures a string ends with the given suffix.
///
/// Returns `Cow::Borrowed` if the suffix is already present,
/// `Cow::Owned` with suffix appended otherwise.
pub fn ensure_suffix<'a>(s: &'a str, suffix: &str) -> Cow<'a, str> {
    // TODO: Check if string already ends with suffix
    // If yes, return Cow::Borrowed
    // If no, return Cow::Owned with suffix appended
    if s.ends_with(suffix) {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(format!("{s}{suffix}"))
    }
}

/// Trims whitespace and converts to lowercase.
///
/// Returns `Cow::Borrowed` only if the string is already trimmed and lowercase,
/// `Cow::Owned` otherwise.
pub fn trim_and_lowercase(s: &str) -> Cow<'_, str> {
    // TODO: Check if the string needs trimming or lowercasing
    // Only return Cow::Borrowed if absolutely no change is needed
    // Otherwise return Cow::Owned with trimmed and lowercased string
    for c in s.chars() {
        if c.is_lowercase() || c.is_whitespace() {
            return Cow::Owned(s.trim().to_lowercase());
        } else {
            continue;
        }
    }
    Cow::Borrowed(s)
}

// =============================================================================
// Part 2: Cow with Collections
// =============================================================================

/// Removes all zeros from a slice.
///
/// Returns `Cow::Borrowed` if no zeros exist,
/// `Cow::Owned` with zeros filtered out.
pub fn remove_zeros(values: &[i32]) -> Cow<'_, [i32]> {
    // TODO: Check if the slice contains any zeros
    // If no zeros, return Cow::Borrowed
    // If has zeros, return Cow::Owned with zeros filtered out
    for value in values {
        if *value == 0 {
            return Cow::Owned(values.iter().filter(|i| **i != 0).copied().collect());
        } else {
            continue;
        }
    }
    Cow::Borrowed(values)
}

/// Removes consecutive duplicates from a sorted slice.
///
/// Returns `Cow::Borrowed` if no consecutive duplicates exist,
/// `Cow::Owned` with duplicates removed.
pub fn deduplicate_sorted<T: Clone + PartialEq>(values: &[T]) -> Cow<'_, [T]> {
    // TODO: Check if there are any consecutive duplicates
    // If no duplicates, return Cow::Borrowed
    // If has duplicates, return Cow::Owned with duplicates removed
    if !values.windows(2).any(|w| w[0] == w[1]) {
        return Cow::Borrowed(values);
    }
    let mut owned = values.to_vec();
    owned.dedup();
    Cow::Owned(owned)
}

/// Clamps all values to a range [min, max].
///
/// Returns `Cow::Borrowed` if all values are already in range,
/// `Cow::Owned` with clamped values.
pub fn clamp_values(values: &[i32], min: i32, max: i32) -> Cow<'_, [i32]> {
    // TODO: Check if any values are outside the range
    // If all in range, return Cow::Borrowed
    // If some out of range, return Cow::Owned with clamped values
    if values.iter().any(|&v| v < min || v > max) {
        Cow::Owned(values.iter().map(|&p| p.clamp(min, max)).collect())
    } else {
        Cow::Borrowed(values)
    }
}

// =============================================================================
// Part 3: Modifying Cow with to_mut()
// =============================================================================

/// Ensures a string has at least `min_len` characters, padding with `pad_char` if needed.
///
/// Uses `to_mut()` to modify efficiently.
pub fn ensure_capacity<'a>(mut s: Cow<'a, str>, min_len: usize, pad_char: char) -> Cow<'a, str> {
    // TODO: Check current character count
    // If less than min_len, use s.to_mut() to get mutable access and push pad_char enough times
    // Return the (potentially modified) Cow
    let missing = min_len.saturating_sub(s.chars().count());

    if missing > 0 {
        s.to_mut().extend(std::iter::repeat_n(pad_char, missing));
    }
    s
}

/// Applies a transformation function only if the predicate returns true.
///
/// Uses `to_mut()` for efficient in-place modification when already owned.
pub fn modify_if_needed<'a, P, T>(mut s: Cow<'a, str>, predicate: P, transform: T) -> Cow<'a, str>
where
    P: FnOnce(&str) -> bool,
    T: Fn(char) -> char,
{
    // TODO: Check if predicate returns true for the string
    // If yes, transform all characters and use to_mut() to update
    // Return the (potentially modified) Cow
    unimplemented!()
}

// =============================================================================
// Part 4: Cow in Data Structures
// =============================================================================

/// A text processor that demonstrates Cow usage in a struct.
#[derive(Debug, Clone)]
pub struct TextProcessor<'a> {
    text: Cow<'a, str>,
}

impl<'a> TextProcessor<'a> {
    /// Creates a new TextProcessor with borrowed text.
    pub fn new(text: &'a str) -> Self {
        // TODO: Create TextProcessor with Cow::Borrowed
        unimplemented!()
    }

    /// Creates a new TextProcessor with owned text.
    pub fn from_owned(text: String) -> Self {
        // TODO: Create TextProcessor with Cow::Owned
        unimplemented!()
    }

    /// Processes the text by trimming and normalizing whitespace.
    ///
    /// Multiple consecutive whitespace characters are collapsed to single spaces.
    pub fn process(&mut self) {
        // TODO: Trim the text and collapse multiple whitespace to single spaces
        // Only modify if needed
        // Update self.text to Cow::Owned if changes were made
        unimplemented!()
    }

    /// Returns a reference to the current text.
    pub fn as_str(&self) -> &str {
        // TODO: Return a reference to the inner text
        unimplemented!()
    }

    /// Converts the processor into a String.
    pub fn into_string(self) -> String {
        // TODO: Use into_owned() to convert to String
        unimplemented!()
    }

    /// Checks if the text is currently borrowed.
    pub fn is_borrowed(&self) -> bool {
        // TODO: Return true if self.text is Cow::Borrowed
        // Hint: use matches! macro
        unimplemented!()
    }

    /// Appends text to the processor.
    ///
    /// This will convert to owned if currently borrowed.
    pub fn append(&mut self, text: &str) {
        // TODO: Use to_mut() to get mutable access and append
        unimplemented!()
    }

    /// Gets the length of the text.
    pub fn len(&self) -> usize {
        // TODO: Return the byte length of the text
        unimplemented!()
    }

    /// Checks if the text is empty.
    pub fn is_empty(&self) -> bool {
        // TODO: Return true if text is empty
        unimplemented!()
    }
}

impl Default for TextProcessor<'_> {
    fn default() -> Self {
        // TODO: Return a TextProcessor with an empty owned String
        unimplemented!()
    }
}

// Example usage
pub fn main() {
    // Part 1: Basic Cow usage
    let result = maybe_uppercase("HELLO");
    println!("maybe_uppercase(HELLO): {:?}", result);

    let result = maybe_uppercase("Hello");
    println!("maybe_uppercase(Hello): {:?}", result);

    let result = ensure_suffix("file.txt", ".txt");
    println!("ensure_suffix already has: {:?}", result);

    let result = ensure_suffix("file", ".txt");
    println!("ensure_suffix needs: {:?}", result);

    // Part 2: Collections
    let data = [1, 2, 3, 4];
    let result = remove_zeros(&data);
    println!("remove_zeros (no zeros): {:?}", result);

    let data = [0, 1, 0, 2];
    let result = remove_zeros(&data);
    println!("remove_zeros (has zeros): {:?}", result);

    // Part 3: to_mut()
    let cow: Cow<str> = Cow::Borrowed("hi");
    let result = ensure_capacity(cow, 5, '!');
    println!("ensure_capacity: {:?}", result);

    // Part 4: TextProcessor
    let mut processor = TextProcessor::new("  hello   world  ");
    println!("Is borrowed: {}", processor.is_borrowed());
    processor.process();
    println!("After process: {}", processor.as_str());
    println!("Is borrowed: {}", processor.is_borrowed());
}
