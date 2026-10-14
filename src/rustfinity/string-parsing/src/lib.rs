use std::str::FromStr;

/// Parse a string into an i32, returning a descriptive error message on failure.
pub fn parse_int(s: &str) -> Result<i32, String> {
    // TODO: Parse the string as an i32
    s.trim().parse::<i32>().map_err(|e| e.to_string())
}

/// Parse common boolean representations (case-insensitive).
/// Accepts: "true", "false", "1", "0", "yes", "no"
pub fn parse_bool(s: &str) -> Result<bool, String> {
    // TODO: Match on the lowercase version of the string
    match s.trim().to_lowercase().as_str() {
        "true" | "1" | "yes" => Ok(true),
        "false" | "0" | "no" => Ok(false),
        _ => Err(format!("cannot parse '{}' as bool", s)),
    }
}

/// Parse a "key=value" string into a tuple.
pub fn parse_key_value(s: &str) -> Result<(String, String), String> {
    // TODO: Split the string at '=' and return (key, value)
    let (key, value) = s
        .split_once('=')
        .ok_or_else(|| format!("expected 'key=value', got '{}'", s))?;
    Ok((key.trim().to_string(), value.trim().to_string()))
}

/// A color represented by red, green, and blue components.
#[derive(Debug, PartialEq, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// TODO: Implement FromStr for Color
// The format is "r,g,b" (e.g., "255,128,0")
impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Parse the "r,g,b" format
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return Err(format!("expected 'r,g,b' got '{}'", s));
        }
        let r = parts[0].trim().parse().map_err(|_| "invalid r")?;
        let g = parts[1].trim().parse().map_err(|_| "invalid g")?;
        let b = parts[2].trim().parse().map_err(|_| "invalid b")?;
        Ok(Color { r, g, b })
    }
}

/// Parse a delimited list of values into a Vec.
pub fn parse_list<T: FromStr>(s: &str, delimiter: char) -> Result<Vec<T>, String> {
    // TODO: Split by delimiter, parse each part, collect into Result<Vec<T>, String>
    s.split(delimiter)
        .map(|part| part.trim().parse::<T>())
        .collect::<Result<Vec<T>, T::Err>>()
        .map_err(|_| "parser error".to_string())
}

pub fn main() {
    // Demonstrate parse_int
    println!("Parsing integers:");
    println!("  '42' -> {:?}", parse_int("42"));
    println!("  '-17' -> {:?}", parse_int("-17"));
    println!("  'abc' -> {:?}", parse_int("abc"));

    // Demonstrate parse_bool
    println!("\nParsing booleans:");
    println!("  'true' -> {:?}", parse_bool("true"));
    println!("  'YES' -> {:?}", parse_bool("YES"));
    println!("  '0' -> {:?}", parse_bool("0"));
    println!("  'maybe' -> {:?}", parse_bool("maybe"));

    // Demonstrate parse_key_value
    println!("\nParsing key=value pairs:");
    println!("  'name=Alice' -> {:?}", parse_key_value("name=Alice"));
    println!("  'count=42' -> {:?}", parse_key_value("count=42"));
    println!("  'invalid' -> {:?}", parse_key_value("invalid"));

    // Demonstrate Color parsing
    println!("\nParsing colors:");
    let color: Result<Color, _> = "255,128,0".parse();
    println!("  '255,128,0' -> {:?}", color);

    // Demonstrate parse_list
    println!("\nParsing lists:");
    println!("  '1,2,3' as i32 -> {:?}", parse_list::<i32>("1,2,3", ','));
}
