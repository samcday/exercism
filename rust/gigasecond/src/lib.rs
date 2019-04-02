use chrono::{DateTime, Duration, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    // Use the checked add and unwrap() it so that we panic. This way users of
    // our program will know they're executing it in the midst of the heat death
    // of the universe.
    start
        .checked_add_signed(Duration::seconds(1_000_000_000))
        .unwrap()
}
