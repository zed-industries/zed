use std::{
    collections::HashSet,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
        mpsc::{Receiver, Sender, channel},
    },
    thread::{self, JoinHandle},
};

use gpui::Image;

use crate::{pdf_engine::PdfDocument, tile_cache::TileKey};

fn rotate_image_bytes(img: Arc<Image>, rotation: u16) -> anyhow::Result<Arc<Image>> {
    if rotation == 0 {
        return Ok(img);
    }
    let reader = image::ImageReader::new(std::io::Cursor::new(&img.bytes))
        .with_guessed_format()?;
    let dyn_img = reader.decode()?;
    let rotated = match rotation {
        90 => dyn_img.rotate90(),
        180 => dyn_img.rotate180(),
        270 => dyn_img.rotate270(),
        _ => dyn_img,
    };
    let mut out = Vec::new();
    rotated.write_to(&mut std::io::Cursor::new(&mut out), image::ImageFormat::Png)?;
    Ok(Arc::new(Image::from_bytes(gpui::ImageFormat::Png, out)))
}

fn render_tile_image(
    document: &Arc<dyn PdfDocument>,
    key: &TileKey,
    scale: f32,
    crop_x: u32,
    crop_y: u32,
    tile_w: u32,
    tile_h: u32,
) -> anyhow::Result<Arc<Image>> {
    if key.rotation == 0 {
        document.render_tile(key.page_index, scale, crop_x, crop_y, tile_w, tile_h)
    } else {
        let full_page = document.render_page(key.page_index, scale)?;
        let reader = image::ImageReader::new(std::io::Cursor::new(&full_page.bytes))
            .with_guessed_format()?;
        let dyn_img = reader.decode()?;
        let rotated = match key.rotation {
            90 => dyn_img.rotate90(),
            180 => dyn_img.rotate180(),
            270 => dyn_img.rotate270(),
            _ => dyn_img,
        };

        let img_w = rotated.width();
        let img_h = rotated.height();
        let cropped = if crop_x == 0 && crop_y == 0 && tile_w >= img_w && tile_h >= img_h {
            rotated
        } else {
            let cw = tile_w.min(img_w.saturating_sub(crop_x));
            let ch = tile_h.min(img_h.saturating_sub(crop_y));
            rotated.crop_imm(crop_x, crop_y, cw, ch)
        };

        let mut out = Vec::new();
        cropped.write_to(&mut std::io::Cursor::new(&mut out), image::ImageFormat::Png)?;
        Ok(Arc::new(Image::from_bytes(gpui::ImageFormat::Png, out)))
    }
}

/// Job request sent to background worker threads.
enum RenderJob {
    Tile {
        key: TileKey,
        generation: u64,
        scale: f32,
        crop_x: u32,
        crop_y: u32,
        tile_w: u32,
        tile_h: u32,
        document: Arc<dyn PdfDocument>,
    },
    PageOverview {
        page_index: usize,
        rotation: u16,
        generation: u64,
        scale: f32,
        document: Arc<dyn PdfDocument>,
    },
}

/// Message returned to the UI thread when rendering completes.
pub enum RenderResult {
    Tile {
        key: TileKey,
        generation: u64,
        image: anyhow::Result<Arc<Image>>,
    },
    PageOverview {
        page_index: usize,
        rotation: u16,
        generation: u64,
        image: anyhow::Result<Arc<Image>>,
    },
}

/// Multi-threaded worker pool for off-screen PDF rendering.
pub struct RenderPool {
    job_tx: Sender<RenderJob>,
    result_rx: Receiver<RenderResult>,
    current_generation: Arc<AtomicU64>,
    in_flight_tiles: Arc<Mutex<HashSet<TileKey>>>,
    in_flight_pages: Arc<Mutex<HashSet<(usize, u16)>>>,
    _workers: Vec<JoinHandle<()>>,
}

impl RenderPool {
    /// Creates a new worker pool with the given number of threads.
    pub fn new(num_threads: usize) -> Self {
        let (job_tx, job_rx) = channel::<RenderJob>();
        let (result_tx, result_rx) = channel::<RenderResult>();
        let current_generation = Arc::new(AtomicU64::new(0));
        let in_flight_tiles = Arc::new(Mutex::new(HashSet::new()));
        let in_flight_pages = Arc::new(Mutex::new(HashSet::new()));

        let job_rx = Arc::new(Mutex::new(job_rx));
        let mut workers = Vec::with_capacity(num_threads);

        for worker_id in 0..num_threads {
            let job_rx = Arc::clone(&job_rx);
            let result_tx = result_tx.clone();
            let current_generation = Arc::clone(&current_generation);
            let in_flight_tiles = Arc::clone(&in_flight_tiles);
            let in_flight_pages = Arc::clone(&in_flight_pages);

            let handle = thread::Builder::new()
                .name(format!("pdf-worker-{}", worker_id))
                .spawn(move || {
                    loop {
                        let job = {
                            let rx = job_rx.lock().unwrap();
                            match rx.recv() {
                                Ok(job) => job,
                                Err(_) => break, // Channel closed, shutdown
                            }
                        };

                        match job {
                            RenderJob::Tile {
                                key,
                                generation,
                                scale,
                                crop_x,
                                crop_y,
                                tile_w,
                                tile_h,
                                document,
                            } => {
                                // Fast cancellation check before rendering
                                let cur_gen = current_generation.load(Ordering::Relaxed);
                                if generation < cur_gen {
                                    in_flight_tiles.lock().unwrap().remove(&key);
                                    continue; // Drop obsolete job
                                }

                                let res = render_tile_image(
                                    &document,
                                    &key,
                                    scale,
                                    crop_x,
                                    crop_y,
                                    tile_w,
                                    tile_h,
                                );

                                in_flight_tiles.lock().unwrap().remove(&key);

                                if generation < current_generation.load(Ordering::Relaxed) {
                                    continue;
                                }

                                let _ = result_tx.send(RenderResult::Tile {
                                    key,
                                    generation,
                                    image: res,
                                });
                            }
                            RenderJob::PageOverview {
                                page_index,
                                rotation,
                                generation,
                                scale,
                                document,
                            } => {
                                let cur_gen = current_generation.load(Ordering::Relaxed);
                                if generation < cur_gen {
                                    in_flight_pages.lock().unwrap().remove(&(page_index, rotation));
                                    continue;
                                }

                                let res = document.render_page(page_index, scale);
                                let res = res.and_then(|img| rotate_image_bytes(img, rotation));

                                in_flight_pages.lock().unwrap().remove(&(page_index, rotation));

                                if generation < current_generation.load(Ordering::Relaxed) {
                                    continue;
                                }

                                let _ = result_tx.send(RenderResult::PageOverview {
                                    page_index,
                                    rotation,
                                    generation,
                                    image: res,
                                });
                            }
                        }
                    }
                })
                .expect("Failed to spawn PDF render worker thread");

            workers.push(handle);
        }

        Self {
            job_tx,
            result_rx,
            current_generation,
            in_flight_tiles,
            in_flight_pages,
            _workers: workers,
        }
    }

    /// Advances the rendering generation, causing in-flight or queued jobs from previous generations to be discarded.
    pub fn increment_generation(&self) -> u64 {
        self.in_flight_tiles.lock().unwrap().clear();
        self.in_flight_pages.lock().unwrap().clear();
        self.current_generation.fetch_add(1, Ordering::SeqCst) + 1
    }

    /// Current active generation.
    pub fn generation(&self) -> u64 {
        self.current_generation.load(Ordering::Relaxed)
    }

    /// Dispatches a tile rendering job to the worker pool if not already in flight.
    pub fn request_tile(
        &self,
        key: TileKey,
        scale: f32,
        crop_x: u32,
        crop_y: u32,
        tile_w: u32,
        tile_h: u32,
        document: Arc<dyn PdfDocument>,
    ) {
        let mut in_flight = self.in_flight_tiles.lock().unwrap();
        if !in_flight.insert(key) {
            return; // Already being rendered!
        }

        let generation = self.current_generation.load(Ordering::Relaxed);
        let job = RenderJob::Tile {
            key,
            generation,
            scale,
            crop_x,
            crop_y,
            tile_w,
            tile_h,
            document,
        };
        let _ = self.job_tx.send(job);
    }

    /// Dispatches a page overview render job (low-res preview) if not already in flight.
    pub fn request_page_overview(
        &self,
        page_index: usize,
        rotation: u16,
        scale: f32,
        document: Arc<dyn PdfDocument>,
    ) {
        let mut in_flight = self.in_flight_pages.lock().unwrap();
        if !in_flight.insert((page_index, rotation)) {
            return; // Already being rendered!
        }

        let generation = self.current_generation.load(Ordering::Relaxed);
        let job = RenderJob::PageOverview {
            page_index,
            rotation,
            generation,
            scale,
            document,
        };
        let _ = self.job_tx.send(job);
    }

    /// Tries to receive any completed tile or overview results without blocking.
    pub fn try_recv(&self) -> Option<RenderResult> {
        self.result_rx.try_recv().ok()
    }
}
