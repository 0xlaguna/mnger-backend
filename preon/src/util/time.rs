use chrono::{DateTime, FixedOffset, TimeDelta, Utc};

/// The Time, a set of utility tools for handling time. Purely based on chrono
///
///
pub struct Time;

impl Time {
    pub fn now_with_offset() -> DateTime<FixedOffset> {
        Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap())
    }

    pub fn now_plus_days(days: i64) -> DateTime<FixedOffset> {
        Self::now_with_offset() + TimeDelta::try_days(days).unwrap()
    }
}
