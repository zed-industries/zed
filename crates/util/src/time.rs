use std::time::Duration;

pub fn duration_alt_display(duration: Duration) -> String {
    if duration < Duration::from_secs(60) {
        format!("{}s", duration.as_secs())
    } else {
        duration_clock_format(duration)
    }
}

fn duration_clock_format(duration: Duration) -> String {
    let hours = duration.as_secs() / 3600;
    let minutes = (duration.as_secs() % 3600) / 60;
    let seconds = duration.as_secs() % 60;

    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else if minutes > 0 {
        format!("{minutes}:{seconds:02}")
    } else {
        format!("{seconds}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_to_clock_format() {
        use duration_clock_format as f;
        assert_eq!("0", f(Duration::from_secs(0)));
        assert_eq!("59", f(Duration::from_secs(59)));
        assert_eq!("1:00", f(Duration::from_secs(60)));
        assert_eq!("10:00", f(Duration::from_secs(600)));
        assert_eq!("1:00:00", f(Duration::from_secs(3600)));
        assert_eq!("3:02:01", f(Duration::from_secs(3600 * 3 + 60 * 2 + 1)));
        assert_eq!("23:59:59", f(Duration::from_secs(3600 * 24 - 1)));
        assert_eq!("100:00:00", f(Duration::from_secs(3600 * 100)));
    }
}
