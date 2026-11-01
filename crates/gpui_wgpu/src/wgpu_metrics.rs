use anyhow::{Context as _, Result};
use gpui::{GpuFrameStats, GpuMemoryStats, GpuStats};
use parking_lot::Mutex;
use std::{
    collections::VecDeque,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
        mpsc::{self, Receiver},
    },
    time::{Duration, Instant},
};

pub(crate) type SharedCounters = Arc<Mutex<Counters>>;

#[derive(Default)]
pub(crate) struct Counters {
    pub frame: GpuFrameStats,
    pub memory: GpuMemoryStats,
    pub peak: u64,
    pub retired_instance_bytes: u64,
}

impl Counters {
    pub fn created(&mut self, bytes: u64) {
        self.frame.resource_creations += 1;
        self.frame.resource_created_bytes += bytes;
        self.peak = self.peak.max(self.memory.owned_bytes());
    }
}

pub(crate) struct AtlasAllocation {
    counters: SharedCounters,
    bytes: u64,
}

impl AtlasAllocation {
    pub fn new(counters: SharedCounters, bytes: u64, created: bool) -> Self {
        {
            let mut counters = counters.lock();
            counters.memory.atlas_bytes += bytes;
            if created {
                counters.created(bytes);
            } else {
                counters.peak = counters.peak.max(counters.memory.owned_bytes());
            }
        }
        Self { counters, bytes }
    }
}

impl Drop for AtlasAllocation {
    fn drop(&mut self) {
        self.counters.lock().memory.atlas_bytes -= self.bytes;
    }
}

pub(crate) struct ReadbackAllocation {
    counters: SharedCounters,
    bytes: u64,
}

impl ReadbackAllocation {
    pub fn new(counters: SharedCounters, bytes: u64) -> Self {
        {
            let mut counters = counters.lock();
            counters.memory.render_target_bytes += bytes;
            counters.created(bytes);
        }
        Self { counters, bytes }
    }
}

impl Drop for ReadbackAllocation {
    fn drop(&mut self) {
        self.counters.lock().memory.render_target_bytes -= self.bytes;
    }
}

pub(crate) fn texture_bytes(texture: &wgpu::Texture) -> u64 {
    u64::from(texture.width())
        * u64::from(texture.height())
        * u64::from(texture.sample_count())
        * u64::from(texture.format().block_copy_size(None).unwrap_or(4))
}

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
    pub counters: SharedCounters,
    in_flight: Arc<AtomicU32>,
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

    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, memory: GpuMemoryStats) -> Self {
        Self {
            counters: Arc::new(Mutex::new(Counters {
                peak: memory.owned_bytes(),
                memory,
                ..Default::default()
            })),
            in_flight: Arc::default(),
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
        started: (Instant, u32),
        slot: Option<TimestampSlot>,
    ) {
        let mut frame = std::mem::take(&mut self.counters.lock().frame);
        frame.frames_in_flight = started.1;
        let (sender, completion) = mpsc::channel();
        queue.on_submitted_work_done({
            let in_flight = self.in_flight.clone();
            move || {
                let elapsed = started.0.elapsed();
                in_flight.fetch_sub(1, Ordering::AcqRel);
                // Disabling metrics intentionally drops the receivers.
                sender.send(elapsed).ok();
            }
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
            frame,
            submission,
            completion,
            timestamps,
        });
    }

    pub fn will_submit(&self) -> (Instant, u32) {
        (
            Instant::now(),
            self.in_flight.fetch_add(1, Ordering::AcqRel) + 1,
        )
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

    pub fn drain(&mut self, device: &wgpu::Device) -> GpuStats {
        let mut counters = self.counters.lock();
        let mut memory = counters.memory;
        let driver = device.get_internal_counters().hal;
        let driver_bytes = driver.buffer_memory.read() + driver.texture_memory.read();
        memory.driver_allocated_bytes = (driver_bytes > 0).then_some(driver_bytes as u64);
        let peak = std::mem::replace(&mut counters.peak, memory.owned_bytes());
        counters.frame = GpuFrameStats::default();
        GpuStats {
            frames: std::mem::take(&mut self.completed),
            memory: Some(memory),
            peak_owned_bytes: Some(peak),
        }
    }
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;

    #[test]
    fn collecting_oldest_does_not_wait_for_later_submissions() -> Result<()> {
        let (context, _) = crate::WgpuContext::new_headless()?;
        let mut metrics =
            GpuMetrics::new(&context.device, &context.queue, GpuMemoryStats::default());
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
