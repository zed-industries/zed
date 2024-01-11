use chrono::{DateTime, Duration, Utc};
use std::time;

const COALESCE_TIMEOUT: time::Duration = time::Duration::from_secs(20);
const SIMULATED_DURATION_FOR_SINGLE_EVENT: time::Duration = time::Duration::from_millis(1);

#[derive(Debug, PartialEq)]
struct PeriodData {
    environment: &'static str,
    start: DateTime<Utc>,
    end: Option<DateTime<Utc>>,
}

pub struct EventCoalescer {
    state: Option<PeriodData>,
}

impl EventCoalescer {
    pub fn new() -> Self {
        Self { state: None }
    }

    pub fn log_event(
        &mut self,
        environment: &'static str,
    ) -> Option<(DateTime<Utc>, DateTime<Utc>, &'static str)> {
        self.log_event_with_time(Utc::now(), environment)
    }

    // pub fn close_current_period(&mut self) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
    //     self.environment.map(|env| self.log_event(env)).flatten()
    // }

    fn log_event_with_time(
        &mut self,
        log_time: DateTime<Utc>,
        environment: &'static str,
    ) -> Option<(DateTime<Utc>, DateTime<Utc>, &'static str)> {
        let coalesce_timeout = Duration::from_std(COALESCE_TIMEOUT).unwrap();

        let Some(state) = &mut self.state else {
            self.state = Some(PeriodData {
                start: log_time,
                end: None,
                environment,
            });
            return None;
        };

        let period_end = state
            .end
            .unwrap_or(state.start + SIMULATED_DURATION_FOR_SINGLE_EVENT);
        let within_timeout = log_time - period_end < coalesce_timeout;
        let environment_is_same = state.environment == environment;
        let should_coaelesce = !within_timeout || !environment_is_same;

        if should_coaelesce {
            let previous_environment = state.environment;
            let original_start = state.start;

            state.start = log_time;
            state.end = None;
            state.environment = environment;

            return Some((
                original_start,
                if within_timeout { log_time } else { period_end },
                previous_environment,
            ));
        }

        state.end = Some(log_time);

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

        assert_eq!(event_coalescer.state, None);

        let period_start = Utc.with_ymd_and_hms(1990, 4, 12, 0, 0, 0).unwrap();
        let period_data = event_coalescer.log_event_with_time(period_start, environment_1);

        assert_eq!(period_data, None);
        assert_eq!(
            event_coalescer.state,
            Some(PeriodData {
                start: period_start,
                end: None,
                environment: environment_1,
            })
        );

        let within_timeout_adjustment = Duration::from_std(COALESCE_TIMEOUT / 2).unwrap();
        let mut period_end = period_start;

        // Ensure that many calls within the timeout don't start a new period
        for _ in 0..100 {
            period_end += within_timeout_adjustment;
            let period_data = event_coalescer.log_event_with_time(period_end, environment_1);

            assert_eq!(period_data, None);
            assert_eq!(
                event_coalescer.state,
                Some(PeriodData {
                    start: period_start,
                    end: Some(period_end),
                    environment: environment_1,
                })
            );
        }

        let exceed_timeout_adjustment = Duration::from_std(COALESCE_TIMEOUT * 2).unwrap();
        // Logging an event exceeding the timeout should start a new period
        let new_period_start = period_end + exceed_timeout_adjustment;
        let period_data = event_coalescer.log_event_with_time(new_period_start, environment_1);

        assert_eq!(period_data, Some((period_start, period_end, environment_1)));
        assert_eq!(
            event_coalescer.state,
            Some(PeriodData {
                start: new_period_start,
                end: None,
                environment: environment_1,
            })
        );
    }

    #[test]
    fn test_different_environment_under_timeout() {
        let environment_1 = "environment_1";
        let mut event_coalescer = EventCoalescer::new();

        assert_eq!(event_coalescer.state, None);

        let period_start = Utc.with_ymd_and_hms(1990, 4, 12, 0, 0, 0).unwrap();
        let period_data = event_coalescer.log_event_with_time(period_start, environment_1);

        assert_eq!(period_data, None);
        assert_eq!(
            event_coalescer.state,
            Some(PeriodData {
                start: period_start,
                end: None,
                environment: environment_1,
            })
        );

        let within_timeout_adjustment = Duration::from_std(COALESCE_TIMEOUT / 2).unwrap();
        let period_end = period_start + within_timeout_adjustment;
        let period_data = event_coalescer.log_event_with_time(period_end, environment_1);

        assert_eq!(period_data, None);
        assert_eq!(
            event_coalescer.state,
            Some(PeriodData {
                start: period_start,
                end: Some(period_end),
                environment: environment_1,
            })
        );

        // Logging an event within the timeout but with a different environment should start a new period
        let period_end = period_end + within_timeout_adjustment;
        let environment_2 = "environment_2";
        let period_data = event_coalescer.log_event_with_time(period_end, environment_2);

        assert_eq!(period_data, Some((period_start, period_end, environment_1)));
        assert_eq!(
            event_coalescer.state,
            Some(PeriodData {
                start: period_end,
                end: None,
                environment: environment_2,
            })
        );
    }

    #[test]
    fn test_switching_environment_while_within_timeout() {
        let environment_1 = "environment_1";
        let mut event_coalescer = EventCoalescer::new();

        assert_eq!(event_coalescer.state, None);

        let period_start = Utc.with_ymd_and_hms(1990, 4, 12, 0, 0, 0).unwrap();
        let period_data = event_coalescer.log_event_with_time(period_start, environment_1);

        assert_eq!(period_data, None);
        assert_eq!(
            event_coalescer.state,
            Some(PeriodData {
                start: period_start,
                end: None,
                environment: environment_1,
            })
        );

        let within_timeout_adjustment = Duration::from_std(COALESCE_TIMEOUT / 2).unwrap();
        let period_end = period_start + within_timeout_adjustment;
        let environment_2 = "environment_2";
        let period_data = event_coalescer.log_event_with_time(period_end, environment_2);

        assert_eq!(period_data, Some((period_start, period_end, environment_1)));
        assert_eq!(
            event_coalescer.state,
            Some(PeriodData {
                start: period_end,
                end: None,
                environment: environment_2,
            })
        );
    }
    // // 0                   20                  40                  60
    // // |-------------------|-------------------|-------------------|-------------------
    // // |--------|----------env change
    // //          |-------------------
    // // |period_start       |period_end
    // //                     |new_period_start

    #[test]
    fn test_switching_environment_while_exceeding_timeout() {
        let environment_1 = "environment_1";
        let mut event_coalescer = EventCoalescer::new();

        assert_eq!(event_coalescer.state, None);

        let period_start = Utc.with_ymd_and_hms(1990, 4, 12, 0, 0, 0).unwrap();
        let period_data = event_coalescer.log_event_with_time(period_start, environment_1);

        assert_eq!(period_data, None);
        assert_eq!(
            event_coalescer.state,
            Some(PeriodData {
                start: period_start,
                end: None,
                environment: environment_1,
            })
        );

        let exceed_timeout_adjustment = Duration::from_std(COALESCE_TIMEOUT * 2).unwrap();
        let period_end = period_start + exceed_timeout_adjustment;
        let environment_2 = "environment_2";
        let period_data = event_coalescer.log_event_with_time(period_end, environment_2);

        assert_eq!(
            period_data,
            Some((
                period_start,
                period_start + SIMULATED_DURATION_FOR_SINGLE_EVENT,
                environment_1
            ))
        );
        assert_eq!(
            event_coalescer.state,
            Some(PeriodData {
                start: period_end,
                end: None,
                environment: environment_2,
            })
        );
    }
    // 0                   20                  40                  60
    // |-------------------|-------------------|-------------------|-------------------
    // |--------|----------------------------------------env change
    //          |-------------------|
    // |period_start                |period_end
    //                                                   |new_period_start
}
