use std::path::Path;
use std::ptr::{self, NonNull};
use std::time::Duration;

use anyhow::{Context as _, Result};
use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{AnyThread as _, MainThreadMarker};
use objc2_av_foundation::{
    AVMediaTypeVideo, AVPlayer, AVPlayerItem, AVPlayerItemStatus, AVPlayerItemVideoOutput,
    AVURLAsset, AVVideoColorPrimaries_ITU_R_709_2, AVVideoColorPrimariesKey,
    AVVideoColorPropertiesKey, AVVideoComposition, AVVideoTransferFunction_ITU_R_709_2,
    AVVideoTransferFunctionKey, AVVideoYCbCrMatrix_ITU_R_601_4, AVVideoYCbCrMatrixKey,
};
use objc2_core_foundation::{CFRetained, CFString};
use objc2_core_media::{CMTime, CMTimeFlags, kCMTimeZero};
use objc2_core_video::{
    CVPixelBuffer, kCVPixelBufferMetalCompatibilityKey, kCVPixelBufferPixelFormatTypeKey,
    kCVPixelFormatType_420YpCbCr8BiPlanarFullRange,
};
use objc2_foundation::{NSDictionary, NSNumber, NSString, NSURL};

use crate::PlayerState;

/// A decoded frame that GPUI's `surface` element draws without copying.
pub type Frame = CFRetained<CVPixelBuffer>;

const SEEK_TIMESCALE: i32 = 600;
/// AVPlayer stops a few milliseconds before the reported duration.
const END_TOLERANCE: Duration = Duration::from_millis(100);

unsafe extern "C" {
    fn mach_absolute_time() -> u64;
}

/// Plays a video file with AVFoundation, which decodes in hardware and outputs the audio.
pub struct Player {
    player: Retained<AVPlayer>,
    item: Retained<AVPlayerItem>,
    output: Retained<AVPlayerItemVideoOutput>,
}

impl Player {
    pub fn open(path: &Path) -> Result<Self> {
        let main_thread =
            MainThreadMarker::new().context("AVPlayer must be created on the main thread")?;
        // SAFETY: the dictionaries hold the value types AVFoundation documents for each key, and
        // every object is used on the main thread that created it.
        unsafe {
            let url = NSURL::fileURLWithPath(&NSString::from_str(&path.to_string_lossy()));
            let asset = AVURLAsset::URLAssetWithURL_options(&url, None);
            let item = AVPlayerItem::playerItemWithAsset(&asset, main_thread);
            if has_rotated_video(&asset) {
                // The video output ignores a track's rotation (phone videos are stored sideways
                // with a rotation flag), but it does apply a video composition.
                #[allow(deprecated)]
                let composition = AVVideoComposition::videoCompositionWithPropertiesOfAsset(&asset);
                item.setVideoComposition(Some(&composition));
            }
            let output = AVPlayerItemVideoOutput::initWithOutputSettings(
                AVPlayerItemVideoOutput::alloc(),
                Some(&output_settings()),
            );
            item.addOutput(&output);
            let player = AVPlayer::playerWithPlayerItem(Some(&item), main_thread);
            Ok(Self {
                player,
                item,
                output,
            })
        }
    }

    pub fn state(&self) -> PlayerState {
        // SAFETY: plain property reads on main-thread objects.
        unsafe {
            match self.item.status() {
                AVPlayerItemStatus::ReadyToPlay => PlayerState::Ready,
                AVPlayerItemStatus::Failed => PlayerState::Failed(
                    self.item
                        .error()
                        .map(|error| error.localizedDescription().to_string())
                        .unwrap_or_else(|| "The video cannot be played".to_string()),
                ),
                _ => PlayerState::Loading,
            }
        }
    }

    pub fn duration(&self) -> Option<Duration> {
        // SAFETY: plain property read on a main-thread object.
        duration_from_time(unsafe { self.item.duration() })
    }

    pub fn has_video(&self) -> bool {
        // SAFETY: plain property read on a main-thread object.
        let size = unsafe { self.item.presentationSize() };
        size.width > 0.0 && size.height > 0.0
    }

    pub fn position(&self) -> Duration {
        // SAFETY: plain property read on a main-thread object.
        duration_from_time(unsafe { self.player.currentTime() }).unwrap_or_default()
    }

    pub fn is_playing(&self) -> bool {
        // SAFETY: plain property read on a main-thread object.
        unsafe { self.player.rate() != 0.0 }
    }

    pub fn is_finished(&self) -> bool {
        !self.is_playing()
            && self
                .duration()
                .is_some_and(|duration| self.position() + END_TOLERANCE >= duration)
    }

    pub fn play(&self) {
        if self.is_finished() {
            self.seek(Duration::ZERO);
        }
        // SAFETY: called on the main thread that owns the player.
        unsafe { self.player.play() }
    }

    pub fn pause(&self) {
        // SAFETY: called on the main thread that owns the player.
        unsafe { self.player.pause() }
    }

    /// Seeks to the exact frame, so a paused video shows the frame at `position`.
    pub fn seek(&self, position: Duration) {
        let time = CMTime {
            value: (position.as_secs_f64() * f64::from(SEEK_TIMESCALE)).round() as i64,
            timescale: SEEK_TIMESCALE,
            flags: CMTimeFlags::Valid,
            epoch: 0,
        };
        // SAFETY: called on the main thread that owns the player; kCMTimeZero is a constant.
        unsafe {
            self.player
                .seekToTime_toleranceBefore_toleranceAfter(time, kCMTimeZero, kCMTimeZero)
        }
    }

    pub fn set_volume(&self, volume: f32) {
        // SAFETY: called on the main thread that owns the player.
        unsafe { self.player.setVolume(volume) }
    }

    /// Returns the frame due now if it differs from the last returned frame.
    pub fn take_frame(&self) -> Option<Frame> {
        // SAFETY: the video output is thread-safe; mach_absolute_time has no preconditions.
        unsafe {
            let time = self
                .output
                .itemTimeForMachAbsoluteTime(mach_absolute_time() as i64);
            if !self.output.hasNewPixelBufferForItemTime(time) {
                return None;
            }
            let buffer = self
                .output
                .copyPixelBufferForItemTime_itemTimeForDisplay(time, ptr::null_mut())?;
            // GPUI takes CoreFoundation ownership; retaining once and dropping the
            // Objective-C handle keeps the reference count balanced.
            Some(CFRetained::retain(NonNull::from(&*buffer)))
        }
    }
}

/// Asks for bi-planar full-range 4:2:0 frames with a BT.601 matrix: GPUI's Metal surface shader
/// accepts only that pixel format and converts with BT.601 coefficients, so AVFoundation does the
/// color conversion for BT.709 (HD) sources.
unsafe fn output_settings() -> Retained<NSDictionary<NSString, AnyObject>> {
    // SAFETY: the CoreVideo keys are CFStrings, which are toll-free bridged to NSString, and the
    // AVFoundation keys are non-null constants on supported systems.
    unsafe {
        let bridge =
            |key: &CFString| -> &NSString { &*(key as *const CFString as *const NSString) };
        let color_properties: Retained<NSDictionary<NSString, AnyObject>> =
            NSDictionary::from_slices(
                &[
                    AVVideoColorPrimariesKey.expect("AVFoundation constant"),
                    AVVideoTransferFunctionKey.expect("AVFoundation constant"),
                    AVVideoYCbCrMatrixKey.expect("AVFoundation constant"),
                ],
                &[
                    AVVideoColorPrimaries_ITU_R_709_2
                        .expect("AVFoundation constant")
                        .as_ref(),
                    AVVideoTransferFunction_ITU_R_709_2
                        .expect("AVFoundation constant")
                        .as_ref(),
                    AVVideoYCbCrMatrix_ITU_R_601_4
                        .expect("AVFoundation constant")
                        .as_ref(),
                ],
            );
        let pixel_format = NSNumber::new_u32(kCVPixelFormatType_420YpCbCr8BiPlanarFullRange);
        let metal_compatible = NSNumber::new_bool(true);
        NSDictionary::from_slices(
            &[
                bridge(kCVPixelBufferPixelFormatTypeKey),
                bridge(kCVPixelBufferMetalCompatibilityKey),
                AVVideoColorPropertiesKey.expect("AVFoundation constant"),
            ],
            &[
                pixel_format.as_ref(),
                metal_compatible.as_ref(),
                color_properties.as_ref(),
            ],
        )
    }
}

unsafe fn has_rotated_video(asset: &AVURLAsset) -> bool {
    // SAFETY: the asset is a local file, so loading its tracks synchronously does not wait on
    // the network.
    unsafe {
        let Some(media_type) = AVMediaTypeVideo else {
            return false;
        };
        #[allow(deprecated)]
        let tracks = asset.tracksWithMediaType(media_type);
        tracks.firstObject().is_some_and(|track| {
            let transform = track.preferredTransform();
            transform.a != 1.0 || transform.b != 0.0 || transform.c != 0.0 || transform.d != 1.0
        })
    }
}

fn duration_from_time(time: CMTime) -> Option<Duration> {
    let valid = time.flags.contains(CMTimeFlags::Valid)
        && !time.flags.contains(CMTimeFlags::Indefinite)
        && time.timescale > 0
        && time.value >= 0;
    valid.then(|| Duration::from_secs_f64(time.value as f64 / f64::from(time.timescale)))
}
