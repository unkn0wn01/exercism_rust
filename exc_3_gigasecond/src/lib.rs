use std::{ops::Add, time::Duration};

use time::PrimitiveDateTime as DateTime;

const ONE_GIGA_SECOND: u64 = 1000000000;
// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.add(Duration::from_secs(ONE_GIGA_SECOND))
}
