use std::fmt;

/// A point in 2D space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

// TODO: Implement the Display trait for Point
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Use write!(f, ...) to format the point
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// A color represented as either a named color or custom RGB values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}

// TODO: Implement the Display trait for Color
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Match on self and format each variant appropriately
        match self {
            Color::Red => write!(f, "Red"),
            Color::Green => write!(f, "Green"),
            Color::Blue => write!(f, "Blue"),
            Color::Custom(r, g, b) => write!(f, "RGB({}, {}, {})", r, g, b),
        }
    }
}

/// A temperature in either Celsius or Fahrenheit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

// TODO: Implement the Display trait for Temperature
impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Match on self and format each variant
        match self {
            Temperature::Celsius(c) => write!(f, "{}°C", c),
            Temperature::Fahrenheit(h) => write!(f, "{}°F", h),
        }
    }
}

/// Money represented in cents with a currency code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Money {
    pub amount: i64, // Amount in cents (can be negative)
    pub currency: String,
}

// TODO: Implement the Display trait for Money
impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Convert cents to dollars/cents
        let negative = self.amount.is_negative();
        let cents = self.amount.abs();
        let sign = if negative { "-" } else { "" };
        let whole = cents / 100;
        let frac = cents % 100;

        match self.currency.as_str() {
            "USD" => write!(f, "{}${}.{:02}", sign, whole, frac),
            "EUR" => write!(f, "{}€{}.{:02}", sign, whole, frac),
            _ => write!(f, "{}{}.{:02} {}", sign, whole, frac, self.currency),
        }
    }
}

/// A person with a name and age.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

// TODO: Implement the Display trait for Person
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Format the person
        write!(f, "{} (age {})", self.name, self.age)
    }
}

/// Converts a slice of displayable items to a comma-separated string in square brackets.
///
/// Examples:
/// - [1, 2, 3] -> "[1, 2, 3]"
/// - [] -> "[]"
/// - ["hello"] -> "[hello]"
pub fn list_to_string<T: fmt::Display>(items: &[T]) -> String {
    // TODO: Iterate over items, convert each to string, and join with ", "
    format!(
        "[{}]",
        items
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    )
}

/// Formats data as a simple text table with headers and rows.
///
/// Each row is formatted with cells separated by " | ", and the header row
/// is followed by a separator line of dashes.
///
/// Example output:
/// ```text
/// Name | Age
/// -----+----
/// Alice | 30
/// Bob | 25
/// ```
pub fn format_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    // TODO: Format as a text table with headers and rows
    let header_line = headers.join(" | ");
    let separator = "-".repeat(header_line.len());
    let row_lines: Vec<String> = rows.iter().map(|row| row.join(" | ")).collect();
    let mut lines = vec![header_line, separator];
    lines.extend(row_lines);
    lines.join("\n")
}

// Example usage
pub fn main() {
    // Point
    let point = Point { x: 3, y: -4 };
    println!("Point: {}", point);

    // Color
    let red = Color::Red;
    let custom = Color::Custom(255, 128, 0);
    println!("Colors: {}, {}", red, custom);

    // Temperature
    let celsius = Temperature::Celsius(25.5);
    let fahrenheit = Temperature::Fahrenheit(77.0);
    println!("Temperatures: {}, {}", celsius, fahrenheit);

    // Money
    let usd = Money {
        amount: 1234,
        currency: "USD".to_string(),
    };
    let eur = Money {
        amount: 5000,
        currency: "EUR".to_string(),
    };
    println!("Money: {}, {}", usd, eur);

    // Person
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    println!("Person: {}", person);

    // list_to_string
    let numbers = vec![1, 2, 3];
    println!("List: {}", list_to_string(&numbers));

    // format_table
    let headers = vec!["Name", "Age"];
    let rows = vec![
        vec!["Alice".to_string(), "30".to_string()],
        vec!["Bob".to_string(), "25".to_string()],
    ];
    println!("Table:\n{}", format_table(&headers, &rows));
}
