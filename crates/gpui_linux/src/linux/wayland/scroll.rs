use std::{collections::VecDeque, time::Instant};

use super::window::WaylandWindowStatePtr;
use gpui::{
    Modifiers, Pixels, PlatformInput, Point, ScrollDelta, ScrollWheelEvent, TouchPhase, point, px,
};

const KINETIC_SCROLLING_FRICTION: f32 = 4.0;
const KINETIC_SCROLLING_HISTORY_WINDOW: u32 = 150;
const KINETIC_SCROLLING_MIN_VELOCITY: Pixels = px(0.1);
const KINETIC_SCROLLING_ASSUME_STOPPED_INTERVAL: u32 = 40;

pub struct KineticScrollController {
    state: State,
}

enum State {
    Idling,
    FingerScrolling(FingerScrollState),
    KineticScrolling(KineticScrollState),
}

#[derive(Clone)]
pub struct KineticScrollTarget {
    pub window: WaylandWindowStatePtr,
    pub position: Point<Pixels>,
    pub modifiers: Modifiers,
}

#[derive(Clone, Copy)]
pub struct ScrollUpdate {
    pub touch_phase: TouchPhase,
    pub displacement: Point<Pixels>,
}

impl ScrollUpdate {
    pub fn new(touch_phase: TouchPhase, displacement: Point<Pixels>) -> Self {
        Self {
            touch_phase,
            displacement,
        }
    }

    pub fn platform_input(
        &self,
        target: &KineticScrollTarget,
    ) -> (WaylandWindowStatePtr, PlatformInput) {
        let window = target.window.clone();
        let input = PlatformInput::ScrollWheel(ScrollWheelEvent {
            position: target.position,
            delta: ScrollDelta::Pixels(self.displacement),
            modifiers: target.modifiers,
            touch_phase: self.touch_phase,
        });
        (window, input)
    }
}

impl KineticScrollController {
    pub fn new() -> Self {
        Self {
            state: State::Idling,
        }
    }

    pub fn finger_scroll(&mut self, time: u32, displacement: Point<Pixels>) {
        if let State::FingerScrolling(state) = &mut self.state {
            state.add_sample(time, displacement);
            return;
        }
        let mut new_state = FingerScrollState::new();
        new_state.add_sample(time, displacement);
        self.state = State::FingerScrolling(new_state);
    }

    pub fn stop_finger_scroll(&mut self, time: u32) -> Option<ScrollUpdate> {
        match &mut self.state {
            State::Idling | State::KineticScrolling(_) => None,
            State::FingerScrolling(state) => {
                if let Some(last_arrival) = state.last_arrival
                    && time.wrapping_sub(last_arrival) > KINETIC_SCROLLING_ASSUME_STOPPED_INTERVAL
                {
                    state.clear();
                }
                let velocity = state.estimate_release_velocity();
                if kinetic_scrolling_ended(velocity) {
                    self.state = State::Idling;
                    Some(ScrollUpdate::new(TouchPhase::Ended, Point::default()))
                } else {
                    self.state =
                        State::KineticScrolling(KineticScrollState::new(velocity, Instant::now()));
                    None
                }
            }
        }
    }

    pub fn cancel_kinetic_scroll(&mut self) -> Option<ScrollUpdate> {
        if matches!(self.state, State::KineticScrolling(_)) {
            self.state = State::Idling;
            Some(ScrollUpdate::new(TouchPhase::Ended, Point::default()))
        } else {
            None
        }
    }

    pub fn tick(&mut self) -> Option<ScrollUpdate> {
        if let State::KineticScrolling(state) = &mut self.state {
            let displacement = state.tick(Instant::now());
            let new_velocity = state.velocity;
            let touch_phase;
            if kinetic_scrolling_ended(new_velocity) {
                self.state = State::Idling;
                touch_phase = TouchPhase::Ended;
            } else {
                touch_phase = TouchPhase::Moved;
            }
            return Some(ScrollUpdate::new(touch_phase, displacement));
        }
        None
    }
}

struct Sample {
    arrival_time: u32,
    interval: u32,
    displacement: Point<Pixels>,
}

impl Sample {
    fn average_velocity(&self) -> Point<Pixels> {
        let interval = self.interval as f32 / 1000.0;
        if interval <= 1e-6 {
            return Point::default();
        }
        self.displacement / interval
    }
}

struct FingerScrollState {
    samples: VecDeque<Sample>,
    last_arrival: Option<u32>,
}

impl FingerScrollState {
    fn new() -> Self {
        Self {
            samples: VecDeque::new(),
            last_arrival: None,
        }
    }

    fn clear(&mut self) {
        self.samples.clear();
        self.last_arrival = None;
    }

    fn add_sample(&mut self, arrival_time: u32, displacement: Point<Pixels>) {
        if let Some(last_arrival) = self.last_arrival {
            let interval = arrival_time.wrapping_sub(last_arrival);
            if interval > KINETIC_SCROLLING_ASSUME_STOPPED_INTERVAL {
                self.samples.clear();
            } else {
                self.samples.push_back(Sample {
                    arrival_time,
                    interval,
                    displacement,
                });
            }
        }
        self.last_arrival = Some(arrival_time);
        while let Some(first) = self.samples.front()
            && arrival_time.wrapping_sub(first.arrival_time) > KINETIC_SCROLLING_HISTORY_WINDOW
        {
            self.samples.pop_front();
        }
    }

    fn estimate_release_velocity(&self) -> Point<Pixels> {
        let Some(first) = self.samples.front() else {
            return Point::default();
        };

        let mut last_velocity = first.average_velocity();
        let mut energy = {
            let x = last_velocity.x.to_f64();
            let y = last_velocity.y.to_f64();
            point(x.signum() * 0.5 * x * x, y.signum() * 0.5 * y * y)
        };
        for sample in self.samples.iter().skip(1) {
            let velocity = sample.average_velocity();
            let delta_v = velocity - last_velocity;
            let delta_t = sample.interval as f64 / 1000.0;
            if delta_t > 1e-6 {
                energy.x += delta_v.x.to_f64() * velocity.x.abs().to_f64();
                energy.y += delta_v.y.to_f64() * velocity.y.abs().to_f64();
            }
            let vx = energy.x.signum() * (2.0 * energy.x.abs()).sqrt();
            let vy = energy.y.signum() * (2.0 * energy.y.abs()).sqrt();
            last_velocity = point(px(vx as f32), px(vy as f32));
        }
        last_velocity
    }
}

struct KineticScrollState {
    velocity: Point<Pixels>,
    last_tick: Instant,
}

impl KineticScrollState {
    fn new(initial_velocity: Point<Pixels>, time: Instant) -> Self {
        Self {
            velocity: initial_velocity,
            last_tick: time,
        }
    }

    fn tick(&mut self, time: Instant) -> Point<Pixels> {
        let elapsed = time.duration_since(self.last_tick).as_secs_f32();
        self.last_tick = time;
        let velocity_multiplier = (-KINETIC_SCROLLING_FRICTION * elapsed).exp();
        let new_velocity = self.velocity * velocity_multiplier;
        let displacement = (self.velocity - new_velocity) / KINETIC_SCROLLING_FRICTION;
        self.velocity = new_velocity;
        displacement
    }
}

fn kinetic_scrolling_ended(velocity: Point<Pixels>) -> bool {
    velocity.x.abs() < KINETIC_SCROLLING_MIN_VELOCITY
        && velocity.y.abs() < KINETIC_SCROLLING_MIN_VELOCITY
}
