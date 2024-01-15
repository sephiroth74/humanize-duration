#[macro_export]
macro_rules! unit {
	($unit_name:tt, $one:expr) => {
		pub struct $unit_name;
		impl Unit for $unit_name {
			fn one(&self) -> &'static str {
				$one
			}

			fn many(&self) -> &'static str {
				$one
			}

			fn format(
				&self,
				f: &mut std::fmt::Formatter<'_>,
				value: u64,
				allow_zero: bool,
				started: &mut bool,
			) -> std::fmt::Result {
				if value != 0 || (allow_zero && !*started) {
					if *started {
						f.write_str(" ")?;
					}
					write!(f, "{}{}", value, $one)?;
					*started = true;
				}
				Ok(())
			}
		}
	};
	($unit_name:tt, $one:expr, $many:expr) => {
		pub struct $unit_name;
		impl Unit for $unit_name {
			fn one(&self) -> &'static str {
				$one
			}

			fn many(&self) -> &'static str {
				$many
			}

			fn format(
				&self,
				f: &mut std::fmt::Formatter<'_>,
				value: u64,
				allow_zero: bool,
				started: &mut bool,
			) -> std::fmt::Result {
				if value != 0 || (allow_zero && !*started) {
					if *started {
						f.write_str(" ")?;
					}
					if value > 1 || value == 0 {
						write!(f, "{}{}", value, $many)?;
					} else {
						write!(f, "{}{}", value, $one)?;
					}
					*started = true;
				}
				Ok(())
			}
		}
	};
}
