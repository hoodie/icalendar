use jiff::{SignedDuration, Span};

pub(crate) fn span_from_signed(duration: SignedDuration) -> Result<Span, String> {
    Span::try_from(duration).map_err(|error| error.to_string())
}

pub(crate) fn format_duration(mut span: Span) -> Result<String, String> {
    if span.get_years() != 0
        || span.get_months() != 0
        || span.get_milliseconds() != 0
        || span.get_microseconds() != 0
        || span.get_nanoseconds() != 0
    {
        return Err("iCalendar durations cannot contain years, months or subseconds".into());
    }
    if span.get_weeks() != 0
        && (span.get_days() != 0
            || span.get_hours() != 0
            || span.get_minutes() != 0
            || span.get_seconds() != 0)
    {
        let days = i64::from(span.get_weeks()) * 7 + i64::from(span.get_days());
        span = span
            .try_days(days)
            .map_err(|error| error.to_string())?
            .weeks(0);
    }
    Ok(span.to_string())
}

pub(crate) fn parse_duration(value: &str) -> Option<Span> {
    let (negative, value) = match value.strip_prefix('-') {
        Some(value) => (true, value),
        None => (false, value.strip_prefix('+').unwrap_or(value)),
    };
    let value = value.strip_prefix('P')?;
    if value.is_empty() {
        return None;
    }
    let mut span = Span::new();
    let mut rest = value;
    let mut time = false;
    let mut previous = 0;
    let mut count = 0;
    while !rest.is_empty() {
        if let Some(after_t) = rest.strip_prefix('T') {
            if time || after_t.is_empty() {
                return None;
            }
            time = true;
            previous = 0;
            rest = after_t;
            continue;
        }
        let digits = rest.bytes().take_while(u8::is_ascii_digit).count();
        if digits == 0 || digits == rest.len() {
            return None;
        }
        let number: i64 = rest.get(..digits)?.parse().ok()?;
        let number = if negative {
            number.checked_neg()?
        } else {
            number
        };
        let unit = rest.as_bytes().get(digits).copied()?;
        let order = match (time, unit) {
            (false, b'W') if count == 0 && digits + 1 == rest.len() => 1,
            (false, b'D') => 2,
            (true, b'H') => 1,
            (true, b'M') => 2,
            (true, b'S') => 3,
            _ => return None,
        };
        if order <= previous {
            return None;
        }
        span = match (time, unit) {
            (false, b'W') => span.try_weeks(number).ok()?,
            (false, b'D') => span.try_days(number).ok()?,
            (true, b'H') => span.try_hours(number).ok()?,
            (true, b'M') => span.try_minutes(number).ok()?,
            (true, b'S') => span.try_seconds(number).ok()?,
            _ => return None,
        };
        previous = order;
        count += 1;
        rest = rest.get(digits + 1..)?;
    }
    (count > 0).then_some(span)
}

#[cfg(test)]
mod test_duration {
    use super::*;
    use jiff::ToSpan as _;

    #[test]
    fn strict_wire_durations() {
        for value in ["P1D", "PT24H", "-PT10M", "P2W", "P1DT2H3M4S"] {
            assert_eq!(
                format_duration(parse_duration(value).unwrap()).unwrap(),
                value
            );
        }
        assert_ne!(
            parse_duration("P1D").unwrap().fieldwise(),
            parse_duration("PT24H").unwrap().fieldwise()
        );
        for value in [
            "P", "PT", "P1W2D", "P1DT", "P1M", "PT1.5S", "1 hour", "PT1M1H",
        ] {
            assert!(parse_duration(value).is_none(), "{value}");
        }
    }

    #[test]
    fn rejects_unsupported_units() {
        for span in [
            1.year(),
            1.month(),
            1.millisecond(),
            1.microsecond(),
            1.nanosecond(),
        ] {
            assert!(format_duration(span).is_err(), "{span}");
        }
    }

    #[test]
    fn formats_zero_and_negative_durations() {
        assert_eq!(format_duration(Span::new()).unwrap(), "PT0S");
        assert_eq!(format_duration(-2.weeks()).unwrap(), "-P2W");
        assert_eq!(
            format_duration(-1.day().hours(2).minutes(3).seconds(4)).unwrap(),
            "-P1DT2H3M4S"
        );
    }

    #[test]
    fn normalizes_mixed_weeks() {
        for (span, expected) in [
            (1.week().days(1), "P8D"),
            (1.week().hours(2), "P7DT2H"),
            (1.week().minutes(3), "P7DT3M"),
            (1.week().seconds(4), "P7DT4S"),
            (-1.week().days(1).hours(2), "-P8DT2H"),
        ] {
            assert_eq!(format_duration(span).unwrap(), expected);
        }
    }

    #[test]
    fn rejects_mixed_weeks_exceeding_day_range() {
        assert!(format_duration(Span::new().weeks(1_000_000).days(1_000_000)).is_err());
    }
}
