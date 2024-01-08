use chrono::{DateTime, Duration, Utc};
use std::time;

const COALESCE_TIMEOUT: time::Duration = time::Duration::from_secs(20);
const SIMULATED_DURATION_FOR_SINGLE_EVENT: time::Duration = time::Duration::from_millis(1);

pub struct EventCoalescer {
    environment: Option<&'static str>,
    period_start: Option<DateTime<Utc>>,
    period_end: Option<DateTime<Utc>>,
}

impl EventCoalescer {
    pub fn new() -> Self {
        Self {
            environment: None,
            period_start: None,
            period_end: None,
        }
    }

    pub fn log_event(
        &mut self,
        environment: &'static str,
    ) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
        self.log_event_with_time(Utc::now(), environment)
    }

    fn log_event_with_time(
        &mut self,
        log_time: DateTime<Utc>,
        environment: &'static str,
    ) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
        let coalesce_timeout = Duration::from_std(COALESCE_TIMEOUT).unwrap();

        let Some(period_start) = self.period_start else {
            self.period_start = Some(log_time);
            self.environment = Some(environment);
            return None;
        };

        let period_end = self
            .period_end
            .unwrap_or(period_start + SIMULATED_DURATION_FOR_SINGLE_EVENT);
        let within_timeout = log_time - period_end < coalesce_timeout;
        let environment_is_same = self.environment == Some(environment);
        let should_coaelesce = !within_timeout || !environment_is_same;

        if should_coaelesce {
            self.period_start = Some(log_time);
            self.period_end = None;
            self.environment = Some(environment);
            return Some((
                period_start,
                if within_timeout { log_time } else { period_end },
            ));
        }

        self.period_end = Some(log_time);

        None
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn test_same_context_exceeding_timeout() {
        let environment_1 = "environment_1";
        let mut event_coalescer = EventCoalescer::new();

        assert_eq!(event_coalescer.period_start, None);
        assert_eq!(event_coalescer.period_end, None);
        assert_eq!(event_coalescer.environment, None);

        let period_start = Utc.with_ymd_and_hms(1990, 4, 12, 0, 0, 0).unwrap();
        let coalesced_duration = event_coalescer.log_event_with_time(period_start, environment_1);

        assert_eq!(coalesced_duration, None);
        assert_eq!(event_coalescer.period_start, Some(period_start));
        assert_eq!(event_coalescer.period_end, None);
        assert_eq!(event_coalescer.environment, Some(environment_1));

        let within_timeout_adjustment = Duration::from_std(COALESCE_TIMEOUT / 2).unwrap();
        let mut period_end = period_start;

        // Ensure that many calls within the timeout don't start a new period
        for _ in 0..100 {
            period_end += within_timeout_adjustment;
            let coalesced_duration = event_coalescer.log_event_with_time(period_end, environment_1);

            assert_eq!(coalesced_duration, None);
            assert_eq!(event_coalescer.period_start, Some(period_start));
            assert_eq!(event_coalescer.period_end, Some(period_end));
            assert_eq!(event_coalescer.environment, Some(environment_1));
        }

        let exceed_timeout_adjustment = Duration::from_std(COALESCE_TIMEOUT * 2).unwrap();
        // Logging an event exceeding the timeout should start a new period
        let new_period_start = period_end + exceed_timeout_adjustment;
        let coalesced_duration =
            event_coalescer.log_event_with_time(new_period_start, environment_1);

        assert_eq!(coalesced_duration, Some((period_start, period_end)));
        assert_eq!(event_coalescer.period_start, Some(new_period_start));
        assert_eq!(event_coalescer.period_end, None);
        assert_eq!(event_coalescer.environment, Some(environment_1));
    }

    #[test]
    fn test_different_environment_under_timeout() {
        let environment_1 = "environment_1";
        let mut event_coalescer = EventCoalescer::new();

        assert_eq!(event_coalescer.period_start, None);
        assert_eq!(event_coalescer.period_end, None);
        assert_eq!(event_coalescer.environment, None);

        let period_start = Utc.with_ymd_and_hms(1990, 4, 12, 0, 0, 0).unwrap();
        let coalesced_duration = event_coalescer.log_event_with_time(period_start, environment_1);

        assert_eq!(coalesced_duration, None);
        assert_eq!(event_coalescer.period_start, Some(period_start));
        assert_eq!(event_coalescer.period_end, None);
        assert_eq!(event_coalescer.environment, Some(environment_1));

        let within_timeout_adjustment = Duration::from_std(COALESCE_TIMEOUT / 2).unwrap();
        let period_end = period_start + within_timeout_adjustment;
        let coalesced_duration = event_coalescer.log_event_with_time(period_end, environment_1);

        assert_eq!(coalesced_duration, None);
        assert_eq!(event_coalescer.period_start, Some(period_start));
        assert_eq!(event_coalescer.period_end, Some(period_end));
        assert_eq!(event_coalescer.environment, Some(environment_1));

        // Logging an event within the timeout but with a different environment should start a new period
        let period_end = period_end + within_timeout_adjustment;
        let environment_2 = "environment_2";
        let coalesced_duration = event_coalescer.log_event_with_time(period_end, environment_2);

        assert_eq!(coalesced_duration, Some((period_start, period_end)));
        assert_eq!(event_coalescer.period_start, Some(period_end));
        assert_eq!(event_coalescer.period_end, None);
        assert_eq!(event_coalescer.environment, Some(environment_2));
    }

    #[test]
    fn test_switching_environment_while_within_timeout() {
        let environment_1 = "environment_1";
        let mut event_coalescer = EventCoalescer::new();

        assert_eq!(event_coalescer.period_start, None);
        assert_eq!(event_coalescer.period_end, None);
        assert_eq!(event_coalescer.environment, None);

        let period_start = Utc.with_ymd_and_hms(1990, 4, 12, 0, 0, 0).unwrap();
        let coalesced_duration = event_coalescer.log_event_with_time(period_start, environment_1);

        assert_eq!(coalesced_duration, None);
        assert_eq!(event_coalescer.period_start, Some(period_start));
        assert_eq!(event_coalescer.period_end, None);
        assert_eq!(event_coalescer.environment, Some(environment_1));

        let within_timeout_adjustment = Duration::from_std(COALESCE_TIMEOUT / 2).unwrap();
        let period_end = period_start + within_timeout_adjustment;
        let environment_2 = "environment_2";
        let coalesced_duration = event_coalescer.log_event_with_time(period_end, environment_2);

        assert_eq!(coalesced_duration, Some((period_start, period_end)));
        assert_eq!(event_coalescer.period_start, Some(period_end));
        assert_eq!(event_coalescer.period_end, None);
        assert_eq!(event_coalescer.environment, Some(environment_2));
    }
    // 0                   20                  40                  60
    // |-------------------|-------------------|-------------------|-------------------
    // |--------|----------env change
    //          |-------------------
    // |period_start       |period_end
    //                     |new_period_start

    #[test]
    fn test_switching_environment_while_exceeding_timeout() {
        let environment_1 = "environment_1";
        let mut event_coalescer = EventCoalescer::new();

        assert_eq!(event_coalescer.period_start, None);
        assert_eq!(event_coalescer.period_end, None);
        assert_eq!(event_coalescer.environment, None);

        let period_start = Utc.with_ymd_and_hms(1990, 4, 12, 0, 0, 0).unwrap();
        let coalesced_duration = event_coalescer.log_event_with_time(period_start, environment_1);

        assert_eq!(coalesced_duration, None);
        assert_eq!(event_coalescer.period_start, Some(period_start));
        assert_eq!(event_coalescer.period_end, None);
        assert_eq!(event_coalescer.environment, Some(environment_1));

        let exceed_timeout_adjustment = Duration::from_std(COALESCE_TIMEOUT * 2).unwrap();
        let period_end = period_start + exceed_timeout_adjustment;
        let environment_2 = "environment_2";
        let coalesced_duration = event_coalescer.log_event_with_time(period_end, environment_2);

        assert_eq!(
            coalesced_duration,
            Some((
                period_start,
                period_start + SIMULATED_DURATION_FOR_SINGLE_EVENT
            ))
        );
        assert_eq!(event_coalescer.period_start, Some(period_end));
        assert_eq!(event_coalescer.period_end, None);
        assert_eq!(event_coalescer.environment, Some(environment_2));
    }
    // 0                   20                  40                  60
    // |-------------------|-------------------|-------------------|-------------------
    // |--------|----------------------------------------env change
    //          |-------------------|
    // |period_start                |period_end
    //                                                   |new_period_start
}
