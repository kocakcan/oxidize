/// Parses a string as a decimal (base 10) i32.
///
/// Returns `Some(value)` on success, `None` on failure.
/// Handles positive and negative numbers and leading/trailing whitespace.
pub fn parse_decimal(s: &str) -> Option<i32> {
    // TODO
    s.trim().parse::<i32>().ok()
}

/// Parses a binary string (e.g., "1010") as a u32.
///
/// The input should NOT include the "0b" prefix.
/// Returns `Some(value)` on success, `None` on failure.
pub fn parse_binary(s: &str) -> Option<u32> {
    // TODO
    u32::from_str_radix(s.trim(), 2).ok()
}

/// Parses a hexadecimal string (e.g., "FF" or "ff") as a u32.
///
/// The input should NOT include the "0x" prefix.
/// Case-insensitive: both "FF" and "ff" work.
/// Returns `Some(value)` on success, `None` on failure.
pub fn parse_hex(s: &str) -> Option<u32> {
    // TODO
    u32::from_str_radix(s.trim(), 16).ok()
}

/// Parses an octal string (e.g., "77") as a u32.
///
/// The input should NOT include the "0o" prefix.
/// Returns `Some(value)` on success, `None` on failure.
pub fn parse_octal(s: &str) -> Option<u32> {
    // TODO
    u32::from_str_radix(s.trim(), 8).ok()
}

/// Parses a string with any radix from 2 to 36.
///
/// Returns `Some(value)` on success, `None` on failure.
/// Valid radix values are 2 through 36.
pub fn parse_with_radix(s: &str, radix: u32) -> Option<u32> {
    // TODO
    if !(2..=36).contains(&radix) {
        return None;
    }
    u32::from_str_radix(s.trim(), radix).ok()
}

/// Parses a comma-separated string of integers.
///
/// Returns a `Vec<i32>` containing all successfully parsed numbers.
/// Values that fail to parse are silently skipped.
pub fn parse_multiple(s: &str) -> Vec<i32> {
    // TODO
    s.split(',')
        .filter_map(|chunk| chunk.trim().parse::<i32>().ok())
        .collect()
}

/// Parses a string as a u8 (0-255).
///
/// Returns `Ok(value)` on success.
/// Returns `Err(String)` with a descriptive error message on failure.
pub fn try_parse_u8(s: &str) -> Result<u8, String> {
    // TODO
    let trimmed = s.trim();
    trimmed
        .parse::<u8>()
        .map_err(|e| format!("Failed to parse '{}': {}", trimmed, e))
}

/// Automatically detects the base and parses the integer.
///
/// - Strings starting with "0x" or "0X" are hexadecimal
/// - Strings starting with "0b" or "0B" are binary
/// - Strings starting with "0o" or "0O" are octal
/// - All other strings are decimal
///
/// Returns `Some(i64)` on success, `None` on failure.
pub fn detect_and_parse(s: &str) -> Option<i64> {
    // TODO
    let s = s.trim();
    if let Some(rest) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        i64::from_str_radix(rest, 16).ok()
    } else if let Some(rest) = s.strip_prefix("0b").or_else(|| s.strip_prefix("0B")) {
        i64::from_str_radix(rest, 2).ok()
    } else if let Some(rest) = s.strip_prefix("0o").or_else(|| s.strip_prefix("0O")) {
        i64::from_str_radix(rest, 8).ok()
    } else {
        s.parse::<i64>().ok()
    }
}

// Example usage
pub fn main() {
    // Decimal parsing
    println!("parse_decimal(\"42\") = {:?}", parse_decimal("42"));
    println!("parse_decimal(\"-17\") = {:?}", parse_decimal("-17"));

    // Binary parsing
    println!("parse_binary(\"1010\") = {:?}", parse_binary("1010"));

    // Hex parsing
    println!("parse_hex(\"FF\") = {:?}", parse_hex("FF"));

    // Octal parsing
    println!("parse_octal(\"77\") = {:?}", parse_octal("77"));

    // Custom radix
    println!(
        "parse_with_radix(\"Z\", 36) = {:?}",
        parse_with_radix("Z", 36)
    );

    // Multiple values
    println!(
        "parse_multiple(\"1, 2, 3\") = {:?}",
        parse_multiple("1, 2, 3")
    );

    // Safe u8 parsing
    println!("try_parse_u8(\"100\") = {:?}", try_parse_u8("100"));
    println!("try_parse_u8(\"256\") = {:?}", try_parse_u8("256"));

    // Auto-detect
    println!(
        "detect_and_parse(\"0xFF\") = {:?}",
        detect_and_parse("0xFF")
    );
    println!(
        "detect_and_parse(\"0b1010\") = {:?}",
        detect_and_parse("0b1010")
    );
}
