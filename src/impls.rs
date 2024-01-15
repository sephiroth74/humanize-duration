use std::fmt::{Display, Formatter};

use crate::types::{DefaultFormatter, DurationParts};
use crate::{Day, Duration, FormattedDuration, Hour, Micro, Millis, Minute, Month, Nano, Second, Truncate, Unit, Year};

impl Default for Truncate {
	fn default() -> Self {
		Truncate::Millis
	}
}

impl Display for FormattedDuration {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let parts: DurationParts = self.duration.into();
		self.formatter.format(f, parts, self.truncate_option)
	}
}

impl From<time::Duration> for Duration {
	fn from(value: time::Duration) -> Self {
		Duration {
			secs: value.as_seconds_f64() as i64,
			nanos: value.subsec_nanoseconds(),
		}
	}
}

impl From<core::time::Duration> for Duration {
	fn from(value: core::time::Duration) -> Self {
		Duration {
			secs: value.as_secs() as i64,
			nanos: value.subsec_nanos() as i32,
		}
	}
}

#[cfg(feature = "chrono")]
impl From<chrono::Duration> for Duration {
	fn from(value: chrono::Duration) -> Self {
		let secs = value.num_seconds();
		let d = value
			.checked_sub(&chrono::Duration::seconds(secs))
			.unwrap_or(chrono::Duration::zero());
		let nanos = d.num_nanoseconds().unwrap_or(0) as i32;

		Duration { secs, nanos }
	}
}

impl From<Duration> for time::Duration {
	fn from(value: Duration) -> Self {
		time::Duration::new(value.secs, value.nanos)
	}
}

impl From<Duration> for core::time::Duration {
	fn from(value: Duration) -> Self {
		core::time::Duration::new(value.secs.abs() as u64, value.nanos.abs() as u32)
	}
}

#[cfg(feature = "chrono")]
impl From<Duration> for chrono::Duration {
	fn from(value: Duration) -> Self {
		chrono::Duration::seconds(value.secs).add(chrono::Duration::nanoseconds(value.nanos as i64))
	}
}

impl DurationParts {
	pub(crate) fn is_empty(&self) -> bool {
		self.original_seconds == 0 && self.original_nanos == 0
	}
}

impl crate::Formatter for DefaultFormatter {
	fn get(&self, truncate: Truncate) -> Box<dyn Unit> {
		match truncate {
			Truncate::Nano => Box::new(Nano),
			Truncate::Micro => Box::new(Micro),
			Truncate::Millis => Box::new(Millis),
			Truncate::Second => Box::new(Second),
			Truncate::Minute => Box::new(Minute),
			Truncate::Hour => Box::new(Hour),
			Truncate::Day => Box::new(Day),
			Truncate::Month => Box::new(Month),
			Truncate::Year => Box::new(Year),
		}
	}

	fn format(&self, f: &mut std::fmt::Formatter<'_>, parts: DurationParts, truncate: Truncate) -> core::fmt::Result {
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
