use crate::types::{DefaultFormatter, DurationParts};
use crate::{Duration, FormattedDuration, Formatter, Truncate};

pub static SECONDS_IN_YEAR: i64 = 31_556_952; // 31_557_600

pub trait DurationExt {
	fn human(&self, truncate: Truncate) -> FormattedDuration;
	fn human_with_format<T: Formatter + 'static>(&self, truncate: Truncate, fmt: T) -> FormattedDuration;
}

impl DurationExt for time::Duration {
	fn human(&self, truncate: Truncate) -> FormattedDuration {
		self.human_with_format(truncate, DefaultFormatter)
	}

	fn human_with_format<T: Formatter + 'static>(&self, truncate: Truncate, fmt: T) -> FormattedDuration {
		let duration: Duration = (*self).into();
		FormattedDuration {
			duration,
			truncate_option: truncate,
			formatter: Box::new(fmt),
		}
	}
}

impl DurationExt for core::time::Duration {
	fn human(&self, truncate: Truncate) -> FormattedDuration {
		self.human_with_format(truncate, DefaultFormatter)
	}

	fn human_with_format<T: Formatter + 'static>(&self, truncate: Truncate, fmt: T) -> FormattedDuration {
		let duration: Duration = (*self).into();
		FormattedDuration {
			duration,
			truncate_option: truncate,
			formatter: Box::new(fmt),
		}
	}
}

#[cfg(feature = "chrono")]
impl DurationExt for chrono::Duration {
	fn human(&self, truncate: Truncate) -> FormattedDuration {
		self.human_with_format(truncate, DefaultFormatter)
	}

	fn human_with_format<T: Formatter + 'static>(&self, truncate: Truncate, fmt: T) -> FormattedDuration {
		let duration: Duration = (*self).into();
		FormattedDuration {
			duration,
			truncate_option: truncate,
			formatter: Box::new(fmt),
		}
	}
}

impl From<Duration> for DurationParts {
	fn from(value: Duration) -> Self {
		let original_seconds = value.secs;
		let original_nanos = value.nanos;

		let years = original_seconds / SECONDS_IN_YEAR;
		let ydays = original_seconds % SECONDS_IN_YEAR;
		let months = ydays / 2_630_016; // 30.44d
		let mdays = ydays % 2_630_016;
		let days = mdays / 86400;
		let day_secs = mdays % 86400;
		let hours = day_secs / 3600;
		let minutes = day_secs % 3600 / 60;
		let seconds = day_secs % 60;
		let millis = original_nanos / 1_000_000;
		let micros = original_nanos / 1000 % 1000;
		let nanos = original_nanos % 1000;

		DurationParts {
			original_seconds,
			original_nanos,
			years,
			months,
			days,
			hours,
			minutes,
			seconds,
			millis,
			micros,
			nanos,
		}
	}
}
