#![deny(clippy::all, clippy::pedantic)]

use chrono::{DateTime, Duration, Utc};

const ONE_BILLYUN: i64 = 1_000_000_000;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(ONE_BILLYUN)
}
