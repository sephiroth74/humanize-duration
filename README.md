# humanize-duration

[![crates.io](https://img.shields.io/crates/v/humanize-duration.svg)](https://crates.io/crates/humanize-duration)
[![ci](https://github.com/sephiroth74/humanize-duration/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/sephiroth74/humanize-duration/actions/workflows/rust.yml)

Convert time Duration to human-readable format with the ability to truncate the output string to a specific time
unit. <br />
It supports `time::Duration`, `core::time::Duration` and `chrono::Duration` (using the chrono feature)

Example:

```rust
    let duration = time::Duration::nanoseconds(131_200_001_301_021_123);
    let human = duration.human(Truncate::Nano);
    println!("duration: {}", human);
```

    It will print: 4years 1month 27days 2h 36m 17s 301ms 21µs 123ns

While:
```rust
    let human2 = duration.human(Truncate::Day);
    println!("duration: {}", human2);
```

    It will print: 4years 1month 27days


Custom formatting is also possible. For example:

```rust
    use time::Duration as TimeDuration;
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
    let human = duration.human_with_format(Truncate::Nano, MyFormatter);

    println!("human: {human}");
```

    It will print: 1 giorno 17 ore 45 minuti 45 secondi 202 millisecondi 557 microsecondi 1 nanosecondo