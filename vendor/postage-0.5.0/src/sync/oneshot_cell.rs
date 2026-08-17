use atomic::Ordering;

use super::state_cell::StateCell;

#[derive(Copy, Clone)]
enum State {
    None,
    Writing,
    Ready,
    Taken,
}

pub enum TryRecvError {
    Pending,
    Closed,
}

pub struct OneshotCell<T> {
    state: StateCell<State, T>,
}

impl<T> OneshotCell<T> {
    pub fn new() -> Self {
        Self {
            state: StateCell::new(State::None),
        }
    }

    pub fn send(&self, value: T) -> Result<(), T> {
        unsafe {
            self.state
                .compare_store(
                    State::None,
                    State::Writing,
                    value,
                    State::Ready,
                    Ordering::AcqRel,
                    Ordering::Relaxed,
                )
                .map_err(|err| err.1)?;
        }

        Ok(())
    }

    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        unsafe {
            match self.state.compare_take(
                State::Ready,
                State::Taken,
                Ordering::AcqRel,
                Ordering::Relaxed,
            ) {
                Ok(v) => Ok(v),
                Err(e) => match e {
                    State::None => Err(TryRecvError::Pending),
                    State::Writing => Err(TryRecvError::Pending),
                    State::Ready => unreachable!(),
                    State::Taken => Err(TryRecvError::Closed),
                },
            }
        }
    }
}
