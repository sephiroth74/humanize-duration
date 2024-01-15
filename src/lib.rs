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

	fn format_default(&self, f: &mut std::fmt::Formatter<'_>, parts: DurationParts, truncate: Truncate) -> core::fmt::Result {
		let ref mut started = false;

		if parts.is_empty() {
			self.get(Truncate::Second)
				.format(f, 0, truncate == Truncate::Second, started)?;
			return Ok(());
		}

		if parts.seconds < 0 {
			f.write_str("-")?;
		}

		self.get(Truncate::Year)
			.format(f, parts.years.abs() as u64, truncate == Truncate::Year, started)?;
		if truncate == Truncate::Year {
			return Ok(());
		}

		self.get(Truncate::Month)
			.format(f, parts.months.abs() as u64, truncate == Truncate::Month, started)?;
		if truncate == Truncate::Month {
			return Ok(());
		}

		self.get(Truncate::Day)
			.format(f, parts.days.abs() as u64, truncate == Truncate::Day, started)?;
		if truncate == Truncate::Day {
			return Ok(());
		}

		self.get(Truncate::Hour)
			.format(f, parts.hours.abs() as u64, truncate == Truncate::Hour, started)?;
		if truncate == Truncate::Hour {
			return Ok(());
		}

		self.get(Truncate::Minute)
			.format(f, parts.minutes.abs() as u64, truncate == Truncate::Minute, started)?;
		if truncate == Truncate::Minute {
			return Ok(());
		}

		self.get(Truncate::Second)
			.format(f, parts.seconds.abs() as u64, truncate == Truncate::Second, started)?;
		if truncate == Truncate::Second {
			return Ok(());
		}

		self.get(Truncate::Millis)
			.format(f, parts.millis.abs() as u64, truncate == Truncate::Millis, started)?;
		if truncate == Truncate::Millis {
			return Ok(());
		}

		self.get(Truncate::Micro)
			.format(f, parts.micros.abs() as u64, truncate == Truncate::Micro, started)?;
		if truncate == Truncate::Micro {
			return Ok(());
		}

		self.get(Truncate::Nano)
			.format(f, parts.nanos.abs() as u64, truncate == Truncate::Nano, started)?;

		Ok(())
	}
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
	use crate::types::{DefaultFormatter, DurationParts};
	use crate::{unit, Duration, Formatter, Truncate, Unit};

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

	#[test]
	fn test_custom_formatter() {
		struct MyFormatter;

		unit!(MyYear, " anno", " anni");
		unit!(MyMonth, " mese", " mesi");
		unit!(MyDay, " giorno", " giorni");
		unit!(MyHour, " ora", " ore");
		unit!(MyMinute, " minuto", " minuti");
		unit!(MySecond, " secondo", " secondi");
		unit!(MyMillis, " millisecondo", " millisecondi");
		unit!(MyMicro, " microsecondo", " microsecondi");
		unit!(MyNano, " nanosecondo", " nanosecondi");

		impl Formatter for MyFormatter {
			fn get(&self, truncate: Truncate) -> Box<dyn Unit> {
				match truncate {
					Truncate::Nano => Box::new(MyNano),
					Truncate::Micro => Box::new(MyMicro),
					Truncate::Millis => Box::new(MyMillis),
					Truncate::Second => Box::new(MySecond),
					Truncate::Minute => Box::new(MyMinute),
					Truncate::Hour => Box::new(MyHour),
					Truncate::Day => Box::new(MyDay),
					Truncate::Month => Box::new(MyMonth),
					Truncate::Year => Box::new(MyYear),
				}
			}

			fn format(&self, f: &mut std::fmt::Formatter<'_>, parts: DurationParts, truncate: Truncate) -> std::fmt::Result {
				self.format_default(f, parts, truncate)
			}
		}

		let duration = TimeDuration::nanoseconds(150_345_202_557_001);
		let human_default = duration.human(Truncate::Nano);
		let human = duration.human_with_format(Truncate::Nano, MyFormatter);

		println!("human default: {human_default}");
		println!("human: {human}");

		assert_eq!("1day 17h 45m 45s 202ms 557µs 1ns", human_default.to_string());
		assert_eq!(
			"1 giorno 17 ore 45 minuti 45 secondi 202 millisecondi 557 microsecondi 1 nanosecondo",
			human.to_string()
		);
	}
}
