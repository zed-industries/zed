use super::*;
use gpui::{Modifiers, ScrollSource, point, px};
use std::time::Duration;

#[test]
fn continuous_pan_coasts_on_its_original_container_and_can_be_interrupted() {
    let mut scroll = IndirectScroll::default();
    let now = Instant::now();
    let start = point(px(100.), px(200.));
    let mut event = ScrollEvent {
        position: start,
        delta: ScrollDelta::Pixels(point(px(0.), px(-20.))),
        modifiers: Modifiers::default(),
        touch_phase: TouchPhase::Started,
        source: ScrollSource::Pointer,
    };
    scroll.handle_event(&mut event, None, now);
    assert!(scroll.tick(now + Duration::from_millis(16)).is_none());

    event.position = point(px(500.), px(500.));
    event.touch_phase = TouchPhase::Ended;
    let velocity = point(px(0.), px(-1000.));
    scroll.handle_event(&mut event, Some(velocity), now);
    let mut expected_fling = GestureTuning::default().scroll_physics.fling(velocity);
    for elapsed in [Duration::from_millis(16), Duration::from_secs(10)] {
        let step = scroll.tick(now + elapsed).unwrap();
        assert_eq!(step.position, start);
        assert_eq!(step.source, ScrollSource::Pointer);
        let ScrollDelta::Pixels(delta) = step.delta else {
            panic!("momentum must use logical pixel deltas");
        };
        assert_eq!(delta, expected_fling.advance(elapsed));
        assert!(delta.y < px(0.));
        assert_eq!(
            step.touch_phase,
            if elapsed >= expected_fling.duration() {
                TouchPhase::Ended
            } else {
                TouchPhase::Moved
            }
        );
    }
    assert!(scroll.tick(now + Duration::from_secs(11)).is_none());

    scroll.handle_event(&mut event, Some(velocity), now);
    scroll.cancel(); // A direct touch or scene deactivation catches the fling.
    assert!(scroll.tick(now + Duration::from_millis(16)).is_none());
    scroll.handle_event(&mut event, Some(velocity), now);
    event.touch_phase = TouchPhase::Started;
    scroll.handle_event(&mut event, None, now);
    assert!(scroll.tick(now + Duration::from_millis(16)).is_none());
}

#[test]
fn wheel_steps_cancellation_and_slow_release_do_not_fling() {
    for (phase, velocity) in [
        (TouchPhase::Ended, None),
        (TouchPhase::Cancelled, Some(point(px(0.), px(-1000.)))),
        (TouchPhase::Ended, Some(point(px(0.), px(-20.)))),
    ] {
        let mut scroll = IndirectScroll::default();
        let now = Instant::now();
        let mut event = ScrollEvent {
            position: Point::default(),
            delta: ScrollDelta::Pixels(point(px(0.), px(-20.))),
            modifiers: Modifiers::default(),
            touch_phase: phase,
            source: ScrollSource::Pointer,
        };
        scroll.handle_event(&mut event, velocity, now);
        assert!(scroll.tick(now + Duration::from_millis(16)).is_none());
    }
}
