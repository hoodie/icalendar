use jiff::{SignedDuration, Span};

pub(crate) fn span_from_signed(duration: SignedDuration) -> Result<Span, String> {
    Span::try_from(duration).map_err(|error| error.to_string())
}

pub(crate) fn format_duration(span: Span) -> Result<String, String> {
    if span.get_years() != 0
        || span.get_months() != 0
        || span.get_milliseconds() != 0
        || span.get_microseconds() != 0
        || span.get_nanoseconds() != 0
    {
        return Err("iCalendar durations cannot contain years, months or subseconds".into());
    }
    let sign = if span.is_negative() { "-" } else { "" };
    let weeks = i64::from(span.get_weeks()).abs();
    let days = i64::from(span.get_days()).abs();
    let hours = i64::from(span.get_hours()).abs();
    let minutes = span.get_minutes().abs();
    let seconds = span.get_seconds().abs();
    if weeks > 0 && (days > 0 || hours > 0 || minutes > 0 || seconds > 0) {
        let days = weeks
            .checked_mul(7)
            .and_then(|weeks| weeks.checked_add(days))
            .ok_or("duration exceeds iCalendar range")?;
        let (span_hours, span_minutes, span_seconds) =
            (span.get_hours(), span.get_minutes(), span.get_seconds());
        let normalized = Span::new()
            .try_days(if span.is_negative() { -days } else { days })
            .and_then(|span| span.try_hours(span_hours))
            .and_then(|span| span.try_minutes(span_minutes))
            .and_then(|span| span.try_seconds(span_seconds))
            .map_err(|error| error.to_string())?;
        return format_duration(normalized);
    }
    if weeks > 0 {
        return Ok(format!("{sign}P{weeks}W"));
    }
    let mut result = format!("{sign}P");
    if days > 0 {
        result.push_str(&format!("{days}D"));
    }
    if hours > 0 || minutes > 0 || seconds > 0 || days == 0 {
        result.push('T');
        if hours > 0 {
            result.push_str(&format!("{hours}H"));
        }
        if minutes > 0 {
            result.push_str(&format!("{minutes}M"));
        }
        if seconds > 0 || (hours == 0 && minutes == 0) {
            result.push_str(&format!("{seconds}S"));
        }
    }
    Ok(result)
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
mod tests {
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
        assert!(format_duration(1.month()).is_err());
        assert_eq!(format_duration(1.week().days(1)).unwrap(), "P8D");
    }
}
