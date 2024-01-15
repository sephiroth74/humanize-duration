# humanize-duration

Convert time Duration to human-readable format with the ability to truncate the output string to a specific time
unit. <br />
It supports `time::Duration`, `core::time::Duration` and `chrono::Duration` (using the chrono feature)

Example:

		let duration = time::Duration::nanoseconds(131_200_001_301_021_123);

		let human = duration.human(Truncate::Nano);
		println!("duration: {}", human);
		// it will print: 4years 1month 27days 2h 36m 17s 301ms 21µs 123ns

		let human2 = duration.human(Truncate::Day);
		println!("duration: {}", human2);
		// It will print: 4years 1month 27days