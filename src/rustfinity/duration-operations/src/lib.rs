use std::time::Duration;

/// Creates a Duration from a number of minutes.
pub fn from_minutes(minutes: u64) -> Duration {
    // TODO: Implement this function
    Duration::from_secs(minutes * 60)
}

/// Creates a Duration from a number of hours.
pub fn from_hours(hours: u64) -> Duration {
    // TODO: Implement this function
    Duration::from_secs(hours * 3600)
}

/// Converts a Duration to total minutes (truncated).
pub fn to_minutes(duration: Duration) -> u64 {
    // TODO: Implement this function
    duration.as_secs() / 60
}

/// Converts a Duration to total hours (truncated).
pub fn to_hours(duration: Duration) -> u64 {
    // TODO: Implement this function
    duration.as_secs() / 3600
}

/// Formats a duration as "Xh Ym Zs".
/// Omits zero components at the start (e.g., "30m 45s" not "0h 30m 45s"),
/// but always shows seconds.
pub fn format_duration(duration: Duration) -> String {
    // TODO: Implement this function
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let remaining = total_seconds % 3600;
    let minutes = remaining / 60;
    let seconds = remaining % 60;

    if hours == 0 && minutes == 0 {
        format!("{}s", seconds)
    } else if hours == 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}h {}m {}s", hours, minutes, seconds)
    }
}

/// Sums all durations in a slice.
/// Returns Duration::ZERO for an empty slice.
pub fn add_durations(durations: &[Duration]) -> Duration {
    // TODO: Implement this function
    durations.iter().sum()
}

/// Calculates the average duration.
/// Returns None for an empty slice.
pub fn average_duration(durations: &[Duration]) -> Option<Duration> {
    // TODO: Implement this function
    if durations.is_empty() {
        None
    } else {
        Some(durations.iter().sum::<Duration>() / durations.len() as u32)
    }
}

/// Checks if the first duration is strictly longer than the second.
pub fn is_longer_than(d1: Duration, d2: Duration) -> bool {
    // TODO: Implement this function
    d1 > d2
}

pub fn main() {
    // Creating durations from minutes and hours
    let thirty_min = from_minutes(30);
    println!("30 minutes = {:?}", thirty_min);

    let two_hours = from_hours(2);
    println!("2 hours = {:?}", two_hours);

    // Converting to minutes and hours
    let d = Duration::from_secs(7500);
    println!(
        "{:?} = {} minutes = {} hours",
        d,
        to_minutes(d),
        to_hours(d)
    );

    // Formatting durations
    let d1 = Duration::from_secs(9045);
    let d2 = Duration::from_secs(45);
    let d3 = Duration::from_secs(1845);
    println!(
        "Formatted: {}, {}, {}",
        format_duration(d1),
        format_duration(d2),
        format_duration(d3)
    );

    // Adding durations
    let durations = vec![
        Duration::from_secs(60),
        Duration::from_secs(120),
        Duration::from_secs(180),
    ];
    println!("Sum: {:?}", add_durations(&durations));

    // Average duration
    println!("Average: {:?}", average_duration(&durations));

    // Comparison
    let short = Duration::from_secs(5);
    let long = Duration::from_secs(10);
    println!("{:?} > {:?}? {}", long, short, is_longer_than(long, short));
}
