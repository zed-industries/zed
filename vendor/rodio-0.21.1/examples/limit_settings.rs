//! Example demonstrating the new LimitSettings API for audio limiting.
//!
//! This example shows how to use the LimitSettings struct with the builder
//! to configure audio limiting parameters.

use rodio::source::{LimitSettings, SineWave, Source};
use rodio::Sample;
use std::time::Duration;

fn main() {
    println!("Example 1: Default LimitSettings");
    let default_limiting = LimitSettings::default();
    println!("  Threshold: {} dB", default_limiting.threshold);
    println!("  Knee width: {} dB", default_limiting.knee_width);
    println!("  Attack: {:?}", default_limiting.attack);
    println!("  Release: {:?}", default_limiting.release);
    println!();

    println!("Example 2: Custom LimitSettings with builder pattern");
    let custom_limiting = LimitSettings::new()
        .with_threshold(-3.0)
        .with_knee_width(2.0)
        .with_attack(Duration::from_millis(10))
        .with_release(Duration::from_millis(50));

    println!("  Threshold: {} dB", custom_limiting.threshold);
    println!("  Knee width: {} dB", custom_limiting.knee_width);
    println!("  Attack: {:?}", custom_limiting.attack);
    println!("  Release: {:?}", custom_limiting.release);
    println!();

    println!("Example 3: Applying limiter to a sine wave with default settings");

    // Create a sine wave at 440 Hz
    let sine_wave = SineWave::new(440.0)
        .amplify(2.0) // Amplify to cause limiting
        .take_duration(Duration::from_millis(100));

    // Apply limiting with default settings (simplest usage)
    let limited_wave = sine_wave.limit(LimitSettings::default());

    // Collect some samples to demonstrate
    let samples: Vec<Sample> = limited_wave.take(100).collect();
    println!("  Generated {} limited samples", samples.len());

    // Show peak reduction
    let max_sample: Sample = samples.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
    println!("  Peak amplitude after limiting: {max_sample:.3}");
    println!();

    println!("Example 4: Custom settings with builder pattern");

    // Create another sine wave for custom limiting
    let sine_wave2 = SineWave::new(880.0)
        .amplify(1.8)
        .take_duration(Duration::from_millis(50));

    // Apply the custom settings from Example 2
    let custom_limited = sine_wave2.limit(custom_limiting);
    let custom_samples: Vec<Sample> = custom_limited.take(50).collect();
    println!(
        "  Generated {} samples with custom settings",
        custom_samples.len()
    );
    println!();

    println!("Example 5: Comparing different limiting scenarios");

    let gentle_limiting = LimitSettings::default()
        .with_threshold(-6.0) // Higher threshold (less limiting)
        .with_knee_width(8.0) // Wide knee (softer)
        .with_attack(Duration::from_millis(20)) // Slower attack
        .with_release(Duration::from_millis(200)); // Slower release

    let aggressive_limiting = LimitSettings::default()
        .with_threshold(-1.0) // Lower threshold (more limiting)
        .with_knee_width(1.0) // Narrow knee (harder)
        .with_attack(Duration::from_millis(2)) // Fast attack
        .with_release(Duration::from_millis(20)); // Fast release

    println!("  Gentle limiting:");
    println!(
        "    Threshold: {} dB, Knee: {} dB",
        gentle_limiting.threshold, gentle_limiting.knee_width
    );
    println!(
        "    Attack: {:?}, Release: {:?}",
        gentle_limiting.attack, gentle_limiting.release
    );

    println!("  Aggressive limiting:");
    println!(
        "    Threshold: {} dB, Knee: {} dB",
        aggressive_limiting.threshold, aggressive_limiting.knee_width
    );
    println!(
        "    Attack: {:?}, Release: {:?}",
        aggressive_limiting.attack, aggressive_limiting.release
    );
    println!();

    println!("Example 6: Limiting with -6dB threshold");

    // Create a sine wave that will definitely trigger limiting
    const AMPLITUDE: Sample = 2.5; // High amplitude to ensure limiting occurs
    let test_sine = SineWave::new(440.0)
        .amplify(AMPLITUDE)
        .take_duration(Duration::from_millis(100)); // 100ms = ~4410 samples

    // Apply limiting with -6dB threshold (should limit to ~0.5)
    let strict_limiting = LimitSettings::default()
        .with_threshold(-6.0)
        .with_knee_width(0.5) // Narrow knee for precise limiting
        .with_attack(Duration::from_millis(3)) // Fast attack
        .with_release(Duration::from_millis(12)); // Moderate release

    let limited_sine = test_sine.limit(strict_limiting.clone());
    let test_samples: Vec<Sample> = limited_sine.take(4410).collect();

    // Analyze peaks at different time periods
    let early_peak: Sample = test_samples[0..500]
        .iter()
        .fold(0.0, |acc, &x| acc.max(x.abs()));
    let mid_peak: Sample = test_samples[1000..1500]
        .iter()
        .fold(0.0, |acc, &x| acc.max(x.abs()));
    let settled_peak: Sample = test_samples[2000..]
        .iter()
        .fold(0.0, |acc, &x| acc.max(x.abs()));

    // With -6dB threshold, ALL samples are well below 1.0!
    let target_linear = (10.0 as Sample).powf(strict_limiting.threshold / 20.0);
    let max_settled: Sample = test_samples[2000..]
        .iter()
        .fold(0.0, |acc, &x| acc.max(x.abs()));

    println!(
        "  {}dB threshold limiting results:",
        strict_limiting.threshold
    );
    println!("    Original max amplitude: {AMPLITUDE}");
    println!("    Target threshold: {target_linear:.3}");
    println!("    Early peak (0-500 samples): {early_peak:.3}");
    println!("    Mid peak (1000-1500 samples): {mid_peak:.3}");
    println!("    Settled peak (2000+ samples): {settled_peak:.3}");
    println!("    ALL samples now well below 1.0: max = {max_settled:.3}");
}
