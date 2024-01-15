#[derive(Debug, Copy, Clone)]
pub struct DurationParts {
	pub(crate) original_seconds: i64,
	pub(crate) original_nanos: i32,
	pub(crate) years: i64,
	pub(crate) months: i64,
	pub(crate) days: i64,
	pub(crate) hours: i64,
	pub(crate) minutes: i64,
	pub(crate) seconds: i64,
	pub(crate) millis: i32,
	pub(crate) micros: i32,
	pub(crate) nanos: i32,
}

pub struct DefaultFormatter;
