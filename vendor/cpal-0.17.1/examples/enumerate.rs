//! Enumerates all available audio hosts, devices, and their supported configurations.
//!
//! This example demonstrates:
//! - Querying available audio hosts on the system
//! - Enumerating all audio devices for each host
//! - Retrieving device IDs for persistent identification
//! - Getting device descriptions with metadata
//! - Listing supported input and output stream configurations
//!
//! Run with: `cargo run --example enumerate`

extern crate anyhow;
extern crate cpal;

use cpal::traits::{DeviceTrait, HostTrait};

fn main() -> Result<(), anyhow::Error> {
    println!("Supported hosts:\n  {:?}", cpal::ALL_HOSTS);
    let available_hosts = cpal::available_hosts();
    println!("Available hosts:\n  {available_hosts:?}");

    for host_id in available_hosts {
        println!("{}", host_id.name());
        let host = cpal::host_from_id(host_id)?;

        let default_in = host
            .default_input_device()
            .map(|dev| dev.id().unwrap())
            .map(|id| id.to_string());
        let default_out = host
            .default_output_device()
            .map(|dev| dev.id().unwrap())
            .map(|id| id.to_string());
        println!("  Default Input Device:\n    {default_in:?}");
        println!("  Default Output Device:\n    {default_out:?}");

        let devices = host.devices()?;
        println!("  Devices: ");
        for (device_index, device) in devices.enumerate() {
            let id = device
                .id()
                .map_or("Unknown ID".to_string(), |id| id.to_string());
            if let Ok(desc) = device.description() {
                println!("  {}. {id} ({})", device_index + 1, desc);
            } else {
                println!("  {}. {id}", device_index + 1);
            }

            // Input configs
            if let Ok(conf) = device.default_input_config() {
                println!("    Default input stream config:\n      {conf:?}");
            }
            let input_configs = match device.supported_input_configs() {
                Ok(f) => f.collect(),
                Err(e) => {
                    println!("    Error getting supported input configs: {e:?}");
                    Vec::new()
                }
            };
            if !input_configs.is_empty() {
                println!("    All supported input stream configs:");
                for (config_index, config) in input_configs.into_iter().enumerate() {
                    println!(
                        "      {}.{}. {:?}",
                        device_index + 1,
                        config_index + 1,
                        config
                    );
                }
            }

            // Output configs
            if let Ok(conf) = device.default_output_config() {
                println!("    Default output stream config:\n      {conf:?}");
            }
            let output_configs = match device.supported_output_configs() {
                Ok(f) => f.collect(),
                Err(e) => {
                    println!("    Error getting supported output configs: {e:?}");
                    Vec::new()
                }
            };
            if !output_configs.is_empty() {
                println!("    All supported output stream configs:");
                for (config_index, config) in output_configs.into_iter().enumerate() {
                    println!(
                        "      {}.{}. {:?}",
                        device_index + 1,
                        config_index + 1,
                        config
                    );
                }
            }
        }
    }

    Ok(())
}
