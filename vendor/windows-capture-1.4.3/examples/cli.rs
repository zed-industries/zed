use std::{
    io::{self, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use clap::Parser;

use windows_capture::{
    capture::{Context, GraphicsCaptureApiHandler},
    encoder::{AudioSettingsBuilder, ContainerSettingsBuilder, VideoEncoder, VideoSettingsBuilder},
    frame::Frame,
    graphics_capture_api::InternalCaptureControl,
    monitor::Monitor,
    settings::{ColorFormat, CursorCaptureSettings, DrawBorderSettings, Settings},
    window::Window,
};

use windows::Graphics::Capture::GraphicsCaptureItem;

// Struct to hold capture settings
struct CaptureSettings {
    stop_flag: Arc<AtomicBool>,
    width: u32,
    height: u32,
    path: String,
    bitrate: u32,
    frame_rate: u32,
}

// This struct will be used to handle the capture events.
struct Capture {
    // The video encoder that will be used to encode the frames.
    encoder: Option<VideoEncoder>,
    // To measure the time the capture has been running
    start: Instant,
    // To count the number of frames captured since last reset
    frame_count_since_reset: u64,
    // To store the time when frame count was last reset
    last_reset: Instant,
    // Capture settings including stop flag and video settings
    settings: CaptureSettings,
}

impl GraphicsCaptureApiHandler for Capture {
    // The type of flags used to get the values from the settings.
    type Flags = CaptureSettings;

    // The type of error that can occur during capture, the error will be returned from `CaptureControl` and `start` functions.
    type Error = Box<dyn std::error::Error + Send + Sync>;

    // Function that will be called to create the struct. The flags can be passed from settings.
    fn new(ctx: Context<Self::Flags>) -> Result<Self, Self::Error> {
        println!("Capture started.");

        let video_settings = VideoSettingsBuilder::new(ctx.flags.width, ctx.flags.height)
            .bitrate(ctx.flags.bitrate)
            .frame_rate(ctx.flags.frame_rate);

        let encoder = VideoEncoder::new(
            video_settings,
            AudioSettingsBuilder::default().disabled(true),
            ContainerSettingsBuilder::default(),
            &ctx.flags.path,
        )?;

        Ok(Self {
            encoder: Some(encoder),
            start: Instant::now(),
            frame_count_since_reset: 0,
            last_reset: Instant::now(),
            settings: ctx.flags,
        })
    }

    // Called every time a new frame is available.
    fn on_frame_arrived(
        &mut self,
        frame: &mut Frame,
        capture_control: InternalCaptureControl,
    ) -> Result<(), Self::Error> {
        self.frame_count_since_reset += 1;

        // Calculate elapsed time since last reset
        let elapsed_since_reset = self.last_reset.elapsed();
        let fps = self.frame_count_since_reset as f64 / elapsed_since_reset.as_secs_f64();
        // Print the FPS
        print!(
            "\rRecording for: {:.2} seconds | FPS: {:.2}",
            self.start.elapsed().as_secs_f64(),
            fps
        );
        io::stdout().flush()?;

        // Send the frame to the video encoder
        self.encoder.as_mut().unwrap().send_frame(frame)?;

        // Stop the capture if stop_flag is set
        if self.settings.stop_flag.load(Ordering::SeqCst) {
            // Finish the encoder and save the video.
            self.encoder.take().unwrap().finish()?;

            capture_control.stop();

            println!("\nRecording stopped by user.");
        }

        if elapsed_since_reset >= Duration::from_secs(1) {
            // Reset frame count and last_reset time
            self.frame_count_since_reset = 0;
            self.last_reset = Instant::now();
        }

        Ok(())
    }

    // Optional handler called when the capture item (usually a window) closes.
    fn on_closed(&mut self) -> Result<(), Self::Error> {
        println!("Capture Session Closed");

        Ok(())
    }
}

#[derive(Parser)]
#[command(name = "Screen Capture")]
#[command(version = "1.0")]
#[command(author = "Your Name")]
#[command(about = "Captures the screen")]
struct Cli {
    /// Window name to capture
    #[arg(long, conflicts_with = "monitor_index")]
    window_name: Option<String>,

    /// Monitor index to capture
    #[arg(long, conflicts_with = "window_name")]
    monitor_index: Option<u32>,

    /// Cursor capture settings: always, never, default
    #[arg(long, default_value = "default")]
    cursor_capture: String,

    /// Draw border settings: always, never, default
    #[arg(long, default_value = "default")]
    draw_border: String,

    /// Output file path
    #[arg(long, default_value = "video.mp4")]
    path: String,

    /// Video bitrate in bits per second
    #[arg(long, default_value_t = 15_000_000)]
    bitrate: u32,

    /// Video frame rate
    #[arg(long, default_value_t = 60)]
    frame_rate: u32,
}

fn parse_cursor_capture(s: &str) -> CursorCaptureSettings {
    match s.to_lowercase().as_str() {
        "always" => CursorCaptureSettings::WithCursor,
        "never" => CursorCaptureSettings::WithoutCursor,
        "default" => CursorCaptureSettings::Default,
        _ => {
            eprintln!("Invalid cursor_capture value: {}", s);
            std::process::exit(1);
        }
    }
}

fn parse_draw_border(s: &str) -> DrawBorderSettings {
    match s.to_lowercase().as_str() {
        "always" => DrawBorderSettings::WithBorder,
        "never" => DrawBorderSettings::WithoutBorder,
        "default" => DrawBorderSettings::Default,
        _ => {
            eprintln!("Invalid draw_border value: {}", s);
            std::process::exit(1);
        }
    }
}

fn start_capture<T>(
    capture_item: T,
    cursor_capture: CursorCaptureSettings,
    draw_border: DrawBorderSettings,
    settings: CaptureSettings,
) where
    T: TryInto<GraphicsCaptureItem>,
{
    let capture_settings = Settings::new(
        capture_item,
        cursor_capture,
        draw_border,
        ColorFormat::Rgba8,
        settings,
    );

    // Starts the capture and takes control of the current thread.
    // The errors from handler trait will end up here
    Capture::start(capture_settings).expect("Screen Capture Failed");
}

fn main() {
    let cli = Cli::parse();

    let cursor_capture = parse_cursor_capture(&cli.cursor_capture);
    let draw_border = parse_draw_border(&cli.draw_border);

    let stop_flag = Arc::new(AtomicBool::new(false));

    // Set up Ctrl+C handler
    {
        let stop_flag = stop_flag.clone();
        ctrlc::set_handler(move || {
            stop_flag.store(true, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");
    }

    if let Some(window_name) = cli.window_name {
        // May use Window::foreground() instead
        let capture_item = Window::from_contains_name(&window_name).expect("Window not found!");

        // Automatically detect window's width and height
        let rect = capture_item.rect().expect("Failed to get window rect");
        let width = (rect.right - rect.left) as u32;
        let height = (rect.bottom - rect.top) as u32;

        let capture_settings = CaptureSettings {
            stop_flag: stop_flag.clone(),
            width,
            height,
            path: cli.path.clone(),
            bitrate: cli.bitrate,
            frame_rate: cli.frame_rate,
        };

        println!(
            "Window title: {}",
            capture_item.title().expect("Failed to get window title")
        );
        println!("Window size: {}x{}", width, height);

        start_capture(capture_item, cursor_capture, draw_border, capture_settings);
    } else if let Some(index) = cli.monitor_index {
        // May use Monitor::primary() instead
        let capture_item =
            Monitor::from_index(usize::try_from(index).unwrap()).expect("Monitor not found!");

        // Automatically detect monitor's width and height
        let width = capture_item.width().expect("Failed to get monitor width");
        let height = capture_item.height().expect("Failed to get monitor height");

        let capture_settings = CaptureSettings {
            stop_flag: stop_flag.clone(),
            width,
            height,
            path: cli.path.clone(),
            bitrate: cli.bitrate,
            frame_rate: cli.frame_rate,
        };

        println!("Monitor index: {}", index);
        println!("Monitor size: {}x{}", width, height);

        start_capture(capture_item, cursor_capture, draw_border, capture_settings);
    } else {
        eprintln!("Either --window-name or --monitor-index must be provided");
        std::process::exit(1);
    }
}
