mod engine;

use core::fmt;
use std::{collections::VecDeque, sync::mpsc, thread};

pub use engine::Engine;
use rodio::{ChannelCount, Sample, SampleRate, Source, nz};

use crate::engine::BLOCK_SHIFT;

const SUPPORTED_SAMPLE_RATE: SampleRate = nz!(16_000);
const SUPPORTED_CHANNEL_COUNT: ChannelCount = nz!(1);

pub struct Denoiser<S: Source> {
    inner: S,
    input_tx: mpsc::Sender<[Sample; BLOCK_SHIFT]>,
    denoised_rx: mpsc::Receiver<[Sample; BLOCK_SHIFT]>,
    ready: [Sample; BLOCK_SHIFT],
    next: usize,
    state: IterState,
    // When disabled instead of reading denoised sub-blocks from the engine through
    // `denoised_rx` we read unprocessed from this queue. This maintains the same
    // latency so we can 'trivially' re-enable
    queued: Queue,
}

impl<S: Source> fmt::Debug for Denoiser<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Denoiser")
            .field("state", &self.state)
            .finish_non_exhaustive()
    }
}

struct Queue(VecDeque<[Sample; BLOCK_SHIFT]>);

impl Queue {
    fn new() -> Self {
        Self(VecDeque::new())
    }
    fn push(&mut self, block: [Sample; BLOCK_SHIFT]) {
        self.0.push_back(block);
        self.0.resize(4, [0f32; BLOCK_SHIFT]);
    }
    fn pop(&mut self) -> [Sample; BLOCK_SHIFT] {
        debug_assert!(self.0.len() == 4);
        self.0.pop_front().expect(
            "There is no State where the queue is popped while there are less then 4 entries",
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum IterState {
    Enabled,
    StartingMidAudio { fed_to_denoiser: usize },
    Disabled,
    Startup { enabled: bool },
}

#[derive(Debug, thiserror::Error)]
pub enum DenoiserError {
    #[error("This denoiser only works on sources with samplerate 16000")]
    UnsupportedSampleRate,
    #[error("This denoiser only works on mono sources (1 channel)")]
    UnsupportedChannelCount,
}

// todo dvdsk needs constant source upstream in rodio
impl<S: Source> Denoiser<S> {
    pub fn try_new(source: S) -> Result<Self, DenoiserError> {
        if source.sample_rate() != SUPPORTED_SAMPLE_RATE {
            return Err(DenoiserError::UnsupportedSampleRate);
        }
        if source.channels() != SUPPORTED_CHANNEL_COUNT {
            return Err(DenoiserError::UnsupportedChannelCount);
        }

        let (input_tx, input_rx) = mpsc::channel();
        let (denoised_tx, denoised_rx) = mpsc::channel();

        thread::Builder::new()
            .name("NeuralDenoiser".to_owned())
            .spawn(move || {
                run_neural_denoiser(denoised_tx, input_rx);
            })
            .unwrap();

        Ok(Self {
            inner: source,
            input_tx,
            denoised_rx,
            ready: [0.0; BLOCK_SHIFT],
            state: IterState::Startup { enabled: true },
            next: BLOCK_SHIFT,
            queued: Queue::new(),
        })
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.state = match (enabled, self.state) {
            (false, IterState::StartingMidAudio { .. }) | (false, IterState::Enabled) => {
                IterState::Disabled
            }
            (false, IterState::Startup { enabled: true }) => IterState::Startup { enabled: false },
            (true, IterState::Disabled) => IterState::StartingMidAudio { fed_to_denoiser: 0 },
            (_, state) => state,
        };
    }

    fn feed(&self, sub_block: [f32; BLOCK_SHIFT]) {
        self.input_tx.send(sub_block).unwrap();
    }
}

fn run_neural_denoiser(
    denoised_tx: mpsc::Sender<[f32; BLOCK_SHIFT]>,
    input_rx: mpsc::Receiver<[f32; BLOCK_SHIFT]>,
) {
    let mut engine = Engine::new();
    loop {
        let Ok(sub_block) = input_rx.recv() else {
            // tx must have dropped, stop thread
            break;
        };

        let denoised_sub_block = engine.feed(&sub_block);
        if denoised_tx.send(denoised_sub_block).is_err() {
            break;
        }
    }
}

impl<S: Source> Source for Denoiser<S> {
    fn current_span_len(&self) -> Option<usize> {
        self.inner.current_span_len()
    }

    fn channels(&self) -> rodio::ChannelCount {
        self.inner.channels()
    }

    fn sample_rate(&self) -> rodio::SampleRate {
        self.inner.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.inner.total_duration()
    }
}

impl<S: Source> Iterator for Denoiser<S> {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.next += 1;
        if self.next < self.ready.len() {
            let sample = self.ready[self.next];
            return Some(sample);
        }

        // This is a separate function to prevent it from being inlined
        // as this code only runs once every 128 samples
        self.prepare_next_ready()
            .inspect_err(|_| {
                log::error!("Denoise engine crashed");
            })
            .ok()
            .flatten()
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Could not send or receive from denoise thread. It must have crashed")]
struct DenoiseEngineCrashed;

impl<S: Source> Denoiser<S> {
    #[cold]
    fn prepare_next_ready(&mut self) -> Result<Option<f32>, DenoiseEngineCrashed> {
        self.state = match self.state {
            IterState::Startup { enabled } => {
                // guaranteed to be coming from silence
                for _ in 0..3 {
                    let Some(sub_block) = read_sub_block(&mut self.inner) else {
                        return Ok(None);
                    };
                    self.queued.push(sub_block);
                    self.input_tx
                        .send(sub_block)
                        .map_err(|_| DenoiseEngineCrashed)?;
                }
                let Some(sub_block) = read_sub_block(&mut self.inner) else {
                    return Ok(None);
                };
                self.queued.push(sub_block);
                self.input_tx
                    .send(sub_block)
                    .map_err(|_| DenoiseEngineCrashed)?;
                // throw out old blocks that are denoised silence
                let _ = self.denoised_rx.iter().take(3).count();
                self.ready = self.denoised_rx.recv().map_err(|_| DenoiseEngineCrashed)?;

                let Some(sub_block) = read_sub_block(&mut self.inner) else {
                    return Ok(None);
                };
                self.queued.push(sub_block);
                self.feed(sub_block);

                if enabled {
                    IterState::Enabled
                } else {
                    IterState::Disabled
                }
            }
            IterState::Enabled => {
                self.ready = self.denoised_rx.recv().map_err(|_| DenoiseEngineCrashed)?;
                let Some(sub_block) = read_sub_block(&mut self.inner) else {
                    return Ok(None);
                };
                self.queued.push(sub_block);
                self.input_tx
                    .send(sub_block)
                    .map_err(|_| DenoiseEngineCrashed)?;
                IterState::Enabled
            }
            IterState::Disabled => {
                // Need to maintain the same 512 samples delay such that
                // we can re-enable at any point.
                self.ready = self.queued.pop();
                let Some(sub_block) = read_sub_block(&mut self.inner) else {
                    return Ok(None);
                };
                self.queued.push(sub_block);
                IterState::Disabled
            }
            IterState::StartingMidAudio {
                fed_to_denoiser: mut sub_blocks_fed,
            } => {
                self.ready = self.queued.pop();
                let Some(sub_block) = read_sub_block(&mut self.inner) else {
                    return Ok(None);
                };
                self.queued.push(sub_block);
                self.input_tx
                    .send(sub_block)
                    .map_err(|_| DenoiseEngineCrashed)?;
                sub_blocks_fed += 1;
                if sub_blocks_fed > 4 {
                    // throw out partially denoised blocks,
                    // next will be correctly denoised
                    let _ = self.denoised_rx.iter().take(3).count();
                    IterState::Enabled
                } else {
                    IterState::StartingMidAudio {
                        fed_to_denoiser: sub_blocks_fed,
                    }
                }
            }
        };

        self.next = 0;
        Ok(Some(self.ready[0]))
    }
}

fn read_sub_block(s: &mut impl Source) -> Option<[f32; BLOCK_SHIFT]> {
    let mut res = [0f32; BLOCK_SHIFT];
    for sample in &mut res {
        *sample = s.next()?;
    }
    Some(res)
}
