use std::time::Duration;

use ashpd::desktop::input_capture::{
    Barrier, BarrierID, Capabilities, CreateSessionOptions, InputCapture, ReleaseOptions,
};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> ashpd::Result<()> {
    let input_capture = InputCapture::new().await?;

    let (session, _capabilities) = input_capture
        .create_session(
            None,
            CreateSessionOptions::default()
                .set_capabilities(Capabilities::Keyboard | Capabilities::Pointer),
        )
        .await?;

    let zones_response = input_capture.zones(&session, Default::default()).await?;
    let zones = zones_response.response()?;
    println!("Available Zones: {:?}", zones);

    // Set up barriers on all edges of each zone, even the overlapping ones (if we
    // have multiple zones). Those will fail and that's fine. The alternative is
    // a more sophisticated algorithm to only set up barriers on the outside
    // edges.
    let barriers: Vec<Barrier> =
        zones
            .regions()
            .iter()
            .enumerate()
            .flat_map(|(n, region)| {
                let x = region.x_offset();
                let y = region.y_offset();
                let width = region.width() as i32;
                let height = region.height() as i32;

                // Create barriers for all four edges
                let edges = [
                    ("left", (x, y, x, y + height - 1)),
                    ("right", (x + width - 1, y, x + width - 1, y + height - 1)),
                    ("top", (x, y, x + width - 1, y)),
                    ("bottom", (x, y + height - 1, x + width - 1, y + height - 1)),
                ];

                edges.into_iter().enumerate().map(move |(edge_idx, (edge_name, position))| {
                // Create unique barrier ID: zone_n * 4 + edge_idx + 1
                let barrier_id = (n * 4 + edge_idx + 1) as u32;
                let id = BarrierID::new(barrier_id).expect("barrier-id must be non-zero");

                println!(
                    "Creating barrier {} ({} edge) at position {:?} for zone ({}, {}, {}x{})",
                    id,
                    edge_name,
                    position,
                    region.x_offset(),
                    region.y_offset(),
                    region.width(),
                    region.height()
                );

                Barrier::new(id, position)
            })
            })
            .collect();

    let barriers_response = input_capture
        .set_pointer_barriers(&session, &barriers, zones.zone_set(), Default::default())
        .await?;

    let result = barriers_response.response()?;
    let failed = result.failed_barriers();

    if !failed.is_empty() {
        println!("Failed barriers: {:?}", failed);
    } else {
        println!("All barriers set successfully");
    }

    // Actual input events will be coming from the EIS file descriptor, once
    // any barrier activates.
    let eifd = input_capture
        .connect_to_eis(&session, Default::default())
        .await?;
    println!("Connected to EIS, fd: {:?}", eifd);

    // Enable input capture so the barriers can trigger
    input_capture.enable(&session, Default::default()).await?;
    println!("Input capture enabled - move cursor to any edge to trigger");

    // Listen for activation events, once we get the event we'll see EI events
    // flow too.
    let mut activated_stream = input_capture.receive_activated().await?;
    let mut deactivated_stream = input_capture.receive_deactivated().await?;

    println!("\nWaiting for input capture to be triggered...");
    println!("Press Ctrl+C to exit\n");

    loop {
        tokio::select! {
            Some(activated) = activated_stream.next() => {
                println!("Input capture ACTIVATED!");
                println!("  Activation ID: {:?}", activated.activation_id());
                println!("  Cursor position: {:?}", activated.cursor_position());
                println!("  Barrier ID: {:?}", activated.barrier_id());
                println!("  (Input events would now be captured via libei)");

                // Once InputCapture activates, we can't interact with the machine anymore so
                // force a InputCapture.Release after 5s.
                println!("  Waiting 5 seconds before releasing...");
                tokio::time::sleep(Duration::from_secs(5)).await;

                // Release the input capture
                input_capture.release(&session, ReleaseOptions::default().set_activation_id(activated.activation_id())).await?;
                println!("  Released input capture");

                // And re-enable so we can play this game again. But wait a second first
                // to give the user a chance to move away from the barrier.
                tokio::time::sleep(Duration::from_secs(1)).await;
                input_capture.enable(&session, Default::default()).await?;
            }
            Some(deactivated) = deactivated_stream.next() => {
                println!("Input capture DEACTIVATED");
                println!("  Activation ID: {:?}", deactivated.activation_id());
            }
        }
    }
}
