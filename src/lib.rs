use crate::types::DurationParts;

mod impls;
mod r#macro;
mod prelude;
mod types;

#[derive(Copy, Clone)]
pub(crate) struct Duration {
	secs: i64,
	nanos: i32,
}

pub struct FormattedDuration {
	duration: Duration,
	truncate_option: Truncate,
	formatter: Box<dyn Formatter>,
}

pub trait Formatter {
	fn get(&self, truncate: Truncate) -> Box<dyn Unit>;
	fn format(&self, f: &mut std::fmt::Formatter<'_>, parts: DurationParts, truncate: Truncate) -> core::fmt::Result;
}

pub trait Unit {
	fn one(&self) -> &'static str;
	fn many(&self) -> &'static str;
	fn format(&self, f: &mut std::fmt::Formatter<'_>, value: u64, allow_zero: bool, started: &mut bool) -> std::fmt::Result;
}

unit!(Year, "year", "years");
unit!(Month, "month", "months");
unit!(Day, "day", "days");
unit!(Hour, "h");
unit!(Minute, "m");
unit!(Second, "s");
unit!(Millis, "ms");
unit!(Micro, "µs");
unit!(Nano, "ns");

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum Truncate {
	Nano,
	Micro,
	Millis,
	Second,
	Minute,
	Hour,
	Day,
	Month,
	Year,
}

#[cfg(test)]
mod tests {
	use core::time::Duration as StdDuration;

	#[cfg(feature = "chrono")]
	use chrono::Duration as ChronoDuration;
	use time::Duration as TimeDuration;

	use crate::prelude::DurationExt;
	use crate::types::DefaultFormatter;
	use crate::{Duration, Truncate};

	#[test]
	fn test_nano() {
		let duration = time::Duration::nanoseconds(131_200_001_301_021_123);
		println!("duration: {}", duration);

		let human = duration.human(Truncate::Nano);
		println!("duration: {human}");
		assert_eq!("4years 1month 27days 2h 36m 17s 301ms 21µs 123ns", human.to_string());

		let human2 = duration.human(Truncate::Day);
		println!("duration with days: {human2}");
		assert_eq!("4years 1month 27days", human2.to_string());
	}

	#[test]
	fn test_micro() {
		let duration = time::Duration::microseconds(123);
		println!("duration: {}", duration);

		let human = duration.human(Truncate::Micro);
		println!("duration: {:}", human);

		assert_eq!("123µs", duration.to_string());
		assert_eq!("123µs", human.to_string());
	}

	#[test]
	fn test_millis() {
		let duration = time::Duration::milliseconds(200_200_111);
		println!("duration: {}", duration);

		let human = duration.human_with_format(Truncate::Millis, DefaultFormatter);
		println!("human: {}", human);

		assert_eq!("2d7h36m40s111ms", duration.to_string());
		assert_eq!("2days 7h 36m 40s 111ms", human.to_string());
	}

	#[test]
	fn test_seconds() {
		let duration = time::Duration::seconds(31_556_952);
		let human = duration.human_with_format(Truncate::Second, DefaultFormatter);
		println!("human: {}", human);
		assert_eq!("1year", human.to_string());
	}

	#[test]
	fn test_minutes() {
		let duration = time::Duration::seconds(556_952);
		let human = duration.human_with_format(Truncate::Minute, DefaultFormatter);
		println!("human: {}", human);
		assert_eq!("6days 10h 42m", human.to_string());
	}

	#[test]
	fn test_hours() {
		let duration = time::Duration::seconds(556_952);
		let human = duration.human_with_format(Truncate::Hour, DefaultFormatter);
		println!("human: {}", human);
		assert_eq!("6days 10h", human.to_string());
	}

	#[test]
	fn test_days() {
		let duration = time::Duration::seconds(556_952);
		let human = duration.human_with_format(Truncate::Day, DefaultFormatter);
		println!("human: {}", human);
		assert_eq!("6days", human.to_string());
	}

	#[test]
	fn test_months() {
		let duration = time::Duration::seconds(556_952);
		let human = duration.human_with_format(Truncate::Month, DefaultFormatter);
		println!("human: {}", human);
		assert_eq!("0months", human.to_string());
	}

	#[test]
	fn test_years() {
		let duration = time::Duration::seconds(456_999_556_952);
		println!("{duration}");

		let human = duration.human_with_format(Truncate::Year, DefaultFormatter);
		println!("human: {human}");
		assert_eq!("14481years", human.to_string());
	}

	#[cfg(feature = "chrono")]
	#[test]
	fn test_chrono_duration() {
		let duration = chrono::Duration::nanoseconds(9223372036854775807);
		let std_duration: StdDuration = duration.to_std().unwrap();
		let time_duration: TimeDuration = time::Duration::try_from(std_duration).unwrap();
		println!("duration: {duration}");
		println!("std duration: {time_duration}");

		let human = duration.human(Truncate::Nano);
		println!("human: {human}");

		assert_eq!("292years 3months 9days 20h 40m 4s 854ms 775µs 807ns", human.to_string());
	}

	#[cfg(feature = "chrono")]
	#[test]
	fn test_convert_chrono_duration() {
		let duration: ChronoDuration = chrono::Duration::nanoseconds(9223372036854775807);
		let converted: Duration = duration.into();
		let duration2: ChronoDuration = converted.into();
		assert_eq!(duration, duration2);
	}

	#[test]
	fn test_convert_time_duration() {
		let duration: TimeDuration = time::Duration::nanoseconds(123_456_789);
		let converted: Duration = duration.into();
		let duration2: TimeDuration = converted.into();
		assert_eq!(duration, duration2);
	}

	#[test]
	fn test_convert_std_time_duration() {
		let duration: StdDuration = StdDuration::new(123_456_789, 999);
		let converted: Duration = duration.into();
		let duration2: StdDuration = converted.into();
		assert_eq!(duration, duration2);
	}
}
