use time::PrimitiveDateTime as DateTime;
use time::Duration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let gigasecond = i64::pow(10, 9);
    start + Duration::new(gigasecond, 0)
}
