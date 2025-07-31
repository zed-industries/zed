use std::time;
use std::{sync::Arc, time::Instant};

use clock::SystemClock;

const COALESCE_TIMEOUT: time::Duration = time::Duration::from_secs(20);
const SIMULATED_DURATION_FOR_SINGLE_EVENT: time::Duration = time::Duration::from_millis(1);

#[derive(Debug, PartialEq)]
struct PeriodData {
    environment: &'static str,
    start: Instant,
    end: Option<Instant>,
}

pub struct EventCoalescer {
    clock: Arc<dyn SystemClock>,
    state: Option<PeriodData>,
}

impl EventCoalescer {
    pub fn new(clock: Arc<dyn SystemClock>) -> Self {
        Self { clock, state: None }
    }

    pub fn log_event(
        &mut self,
        environment: &'static str,
    ) -> Option<(Instant, Instant, &'static str)> {
        let log_time = self.clock.utc_now();

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
        let within_timeout = log_time - period_end < COALESCE_TIMEOUT;
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
    use clock::FakeSystemClock;

    use super::*;

    #[test]
    fn test_same_context_exceeding_timeout() {
        let clock = Arc::new(FakeSystemClock::new());
        let environment_1 = "environment_1";
        let mut event_coalescer = EventCoalescer::new(clock.clone());

        assert_eq!(event_coalescer.state, None);

        let period_start = clock.utc_now();
        let period_data = event_coalescer.log_event(environment_1);

        assert_eq!(period_data, None);
        assert_eq!(
            event_coalescer.state,
            Some(PeriodData {
                start: period_start,
                end: None,
                environment: environment_1,
            })
        );

        let within_timeout_adjustment = COALESCE_TIMEOUT / 2;

        // Ensure that many calls within the timeout don't start a new period
        for _ in 0..100 {
            clock.advance(within_timeout_adjustment);
            let period_data = event_coalescer.log_event(environment_1);
            let period_end = clock.utc_now();

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

        let period_end = clock.utc_now();
        let exceed_timeout_adjustment = COALESCE_TIMEOUT * 2;
        // Logging an event exceeding the timeout should start a new period
        clock.advance(exceed_timeout_adjustment);
        let new_period_start = clock.utc_now();
        let period_data = event_coalescer.log_event(environment_1);

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
        let clock = Arc::new(FakeSystemClock::new());
        let environment_1 = "environment_1";
        let mut event_coalescer = EventCoalescer::new(clock.clone());

        assert_eq!(event_coalescer.state, None);

        let period_start = clock.utc_now();
        let period_data = event_coalescer.log_event(environment_1);

        assert_eq!(period_data, None);
        assert_eq!(
            event_coalescer.state,
            Some(PeriodData {
                start: period_start,
                end: None,
                environment: environment_1,
            })
        );

        let within_timeout_adjustment = COALESCE_TIMEOUT / 2;
        clock.advance(within_timeout_adjustment);
        let period_end = clock.utc_now();
        let period_data = event_coalescer.log_event(environment_1);

        assert_eq!(period_data, None);
        assert_eq!(
            event_coalescer.state,
            Some(PeriodData {
                start: period_start,
                end: Some(period_end),
                environment: environment_1,
            })
        );

        clock.advance(within_timeout_adjustment);

        // Logging an event within the timeout but with a different environment should start a new period
        let period_end = clock.utc_now();
        let environment_2 = "environment_2";
        let period_data = event_coalescer.log_event(environment_2);

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
        let clock = Arc::new(FakeSystemClock::new());
        let environment_1 = "environment_1";
        let mut event_coalescer = EventCoalescer::new(clock.clone());

        assert_eq!(event_coalescer.state, None);

        let period_start = clock.utc_now();
        let period_data = event_coalescer.log_event(environment_1);

        assert_eq!(period_data, None);
        assert_eq!(
            event_coalescer.state,
            Some(PeriodData {
                start: period_start,
                end: None,
                environment: environment_1,
            })
        );

        let within_timeout_adjustment = COALESCE_TIMEOUT / 2;
        clock.advance(within_timeout_adjustment);
        let period_end = clock.utc_now();
        let environment_2 = "environment_2";
        let period_data = event_coalescer.log_event(environment_2);

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

    // 0                   20                  40                  60
    // |-------------------|-------------------|-------------------|-------------------
    // |--------|----------env change
    //          |-------------------
    // |period_start       |period_end
    //                     |new_period_start

    #[test]
    fn test_switching_environment_while_exceeding_timeout() {
        let clock = Arc::new(FakeSystemClock::new());
        let environment_1 = "environment_1";
        let mut event_coalescer = EventCoalescer::new(clock.clone());

        assert_eq!(event_coalescer.state, None);

        let period_start = clock.utc_now();
        let period_data = event_coalescer.log_event(environment_1);

        assert_eq!(period_data, None);
        assert_eq!(
            event_coalescer.state,
            Some(PeriodData {
                start: period_start,
                end: None,
                environment: environment_1,
            })
        );

        let exceed_timeout_adjustment = COALESCE_TIMEOUT * 2;
        clock.advance(exceed_timeout_adjustment);
        let period_end = clock.utc_now();
        let environment_2 = "environment_2";
        let period_data = event_coalescer.log_event(environment_2);

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
