use anyhow::{Context as _, Result};
use gpui::{GpuFrameStats, GpuStats};
use std::{
    collections::VecDeque,
    sync::mpsc::{self, Receiver},
    time::{Duration, Instant},
};

pub(crate) struct TimestampSlot {
    queries: wgpu::QuerySet,
    resolve: wgpu::Buffer,
    readback: wgpu::Buffer,
}

impl TimestampSlot {
    fn new(device: &wgpu::Device) -> Self {
        Self {
            queries: device.create_query_set(&wgpu::QuerySetDescriptor {
                label: Some("benchmark timestamps"),
                ty: wgpu::QueryType::Timestamp,
                count: 2,
            }),
            resolve: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("benchmark timestamp resolve"),
                size: 16,
                usage: wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC,
                mapped_at_creation: false,
            }),
            readback: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("benchmark timestamp readback"),
                size: 16,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            }),
        }
    }

    pub fn writes(&self, first: bool, last: bool) -> wgpu::RenderPassTimestampWrites<'_> {
        wgpu::RenderPassTimestampWrites {
            query_set: &self.queries,
            beginning_of_pass_write_index: first.then_some(0),
            end_of_pass_write_index: last.then_some(1),
        }
    }

    pub fn resolve(&self, encoder: &mut wgpu::CommandEncoder) {
        encoder.resolve_query_set(&self.queries, 0..2, &self.resolve, 0);
        encoder.copy_buffer_to_buffer(&self.resolve, 0, &self.readback, 0, 16);
    }
}

struct PendingFrame {
    frame: GpuFrameStats,
    submission: wgpu::SubmissionIndex,
    completion: Receiver<Duration>,
    timestamps: Option<(TimestampSlot, Receiver<Result<Duration>>)>,
}

pub(crate) struct GpuMetrics {
    pending: VecDeque<PendingFrame>,
    completed: Vec<GpuFrameStats>,
    slots: Vec<TimestampSlot>,
    timestamp_period: Option<f32>,
}

impl GpuMetrics {
    #[cfg(all(test, target_os = "linux"))]
    pub fn disable_timestamps_for_test(&mut self) {
        self.timestamp_period = None;
    }

    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        Self {
            pending: VecDeque::new(),
            completed: Vec::new(),
            slots: Vec::new(),
            timestamp_period: device
                .features()
                .contains(wgpu::Features::TIMESTAMP_QUERY)
                .then(|| queue.get_timestamp_period()),
        }
    }

    pub fn acquire(&mut self, device: &wgpu::Device) -> Result<Option<TimestampSlot>> {
        self.collect(false)?;
        if self.timestamp_period.is_none() {
            return Ok(None);
        }
        if let Some(slot) = self.slots.pop() {
            return Ok(Some(slot));
        }
        // Grow on demand so instrumentation does not impose early CPU/GPU
        // backpressure. Match Metal's 64-command-buffer ceiling only to bound
        // benchmark allocations when the GPU falls far behind.
        if self.pending.len() >= 64 {
            device.poll(wgpu::PollType::Wait {
                submission_index: self
                    .pending
                    .front()
                    .map(|pending| pending.submission.clone()),
                timeout: Some(Duration::from_secs(30)),
            })?;
            self.collect_limit(true, 1)?;
            if let Some(slot) = self.slots.pop() {
                return Ok(Some(slot));
            }
        }
        Ok(Some(TimestampSlot::new(device)))
    }

    pub fn submitted(
        &mut self,
        queue: &wgpu::Queue,
        submission: wgpu::SubmissionIndex,
        started: Instant,
        slot: Option<TimestampSlot>,
    ) {
        let (sender, completion) = mpsc::channel();
        queue.on_submitted_work_done(move || {
            // Disabling metrics intentionally drops the receivers.
            sender.send(started.elapsed()).ok();
        });
        let timestamps = slot.zip(self.timestamp_period).map(|(slot, period)| {
            let (sender, receiver) = mpsc::channel();
            let readback = slot.readback.clone();
            slot.readback
                .map_async(wgpu::MapMode::Read, .., move |result| {
                    let mapped_successfully = result.is_ok();
                    let duration = (|| {
                        result.context("mapping GPU timestamps")?;
                        let mapped = readback.get_mapped_range(..)?;
                        let timestamps: &[u64] = bytemuck::cast_slice(&mapped);
                        let [start, end] = timestamps else {
                            anyhow::bail!("expected two GPU timestamps");
                        };
                        Ok(Duration::from_secs_f64(
                            end.wrapping_sub(*start) as f64 * f64::from(period) / 1e9,
                        ))
                    })();
                    if mapped_successfully {
                        readback.unmap();
                    }
                    sender.send(duration).ok();
                });
            (slot, receiver)
        });
        self.pending.push_back(PendingFrame {
            frame: GpuFrameStats::default(),
            submission,
            completion,
            timestamps,
        });
    }

    pub fn collect(&mut self, wait: bool) -> Result<()> {
        self.collect_limit(wait, usize::MAX)
    }

    fn collect_limit(&mut self, wait: bool, limit: usize) -> Result<()> {
        for _ in 0..limit {
            let Some(pending) = self.pending.front_mut() else {
                break;
            };
            if pending.frame.submit_to_complete.is_none() {
                if wait {
                    pending.frame.submit_to_complete =
                        Some(pending.completion.recv_timeout(Duration::from_secs(30))?);
                } else {
                    match pending.completion.try_recv() {
                        Ok(duration) => pending.frame.submit_to_complete = Some(duration),
                        Err(mpsc::TryRecvError::Empty) => break,
                        Err(error) => return Err(error.into()),
                    }
                }
            }
            if let Some((_, receiver)) = &pending.timestamps {
                let duration = if wait {
                    receiver.recv_timeout(Duration::from_secs(30))??
                } else {
                    match receiver.try_recv() {
                        Ok(duration) => duration?,
                        Err(mpsc::TryRecvError::Empty) => break,
                        Err(error) => return Err(error.into()),
                    }
                };
                pending.frame.gpu_duration = Some(duration);
            }
            if let Some(pending) = self.pending.pop_front() {
                if let Some((slot, _)) = pending.timestamps {
                    self.slots.push(slot);
                }
                self.completed.push(pending.frame);
            }
        }
        Ok(())
    }

    pub fn drain(&mut self) -> GpuStats {
        GpuStats {
            frames: std::mem::take(&mut self.completed),
        }
    }
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;

    #[test]
    fn collecting_oldest_does_not_wait_for_later_submissions() -> Result<()> {
        let (context, _) = crate::WgpuContext::new_headless()?;
        let mut metrics = GpuMetrics::new(&context.device, &context.queue);
        let (first_sender, first_completion) = mpsc::channel();
        let (_second_sender, second_completion) = mpsc::channel();
        for completion in [first_completion, second_completion] {
            metrics.pending.push_back(PendingFrame {
                frame: GpuFrameStats::default(),
                submission: context.queue.submit(std::iter::empty()),
                completion,
                timestamps: None,
            });
        }
        first_sender.send(Duration::from_millis(1))?;
        metrics.collect_limit(true, 1)?;
        assert_eq!(metrics.completed.len(), 1);
        assert_eq!(metrics.pending.len(), 1);
        Ok(())
    }
}
