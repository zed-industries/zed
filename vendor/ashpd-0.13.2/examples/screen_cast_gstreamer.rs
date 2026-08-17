use std::os::fd::{AsRawFd, OwnedFd};

use ashpd::desktop::screencast::{
    CursorMode, Screencast, SelectSourcesOptions, SourceType, Stream,
};
use gst::prelude::*;

async fn open_portal() -> ashpd::Result<(Stream, OwnedFd)> {
    let proxy = Screencast::new().await?;
    let session = proxy.create_session(Default::default()).await?;
    proxy
        .select_sources(
            &session,
            SelectSourcesOptions::default()
                .set_cursor_mode(CursorMode::Embedded)
                .set_sources(SourceType::Monitor | SourceType::Window | SourceType::Virtual)
                .set_multiple(false)
                .set_restore_token(None)
                .set_persist_mode(ashpd::desktop::PersistMode::ExplicitlyRevoked),
        )
        .await?;

    let response = proxy
        .start(&session, None, Default::default())
        .await?
        .response()?;
    let stream = response
        .streams()
        .first()
        .expect("No stream found or selected")
        .to_owned();

    let fd = proxy
        .open_pipe_wire_remote(&session, Default::default())
        .await?;

    Ok((stream, fd))
}

#[tokio::main]
async fn main() -> ashpd::Result<()> {
    gst::init().unwrap();

    let (stream, stream_fd) = open_portal().await?;
    let pipewire_node_id = &stream.pipe_wire_node_id();
    let stream_raw_fd = &stream_fd.as_raw_fd();

    let pipewire_element = gst::ElementFactory::make("pipewiresrc")
        .property("fd", stream_raw_fd)
        .property("path", pipewire_node_id.to_string())
        .build()
        .unwrap();
    let convert = gst::ElementFactory::make("videoconvert").build().unwrap();
    let wayland_sink = gst::ElementFactory::make("waylandsink").build().unwrap();

    let pipeline = gst::Pipeline::default();
    pipeline
        .add_many([&pipewire_element, &convert, &wayland_sink])
        .unwrap();
    gst::Element::link_many([&pipewire_element, &convert, &wayland_sink]).unwrap();

    pipeline.set_state(gst::State::Playing).unwrap();

    let bus = pipeline.bus().unwrap();

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => {
                println!("EOS");
                break;
            }
            MessageView::Error(err) => {
                pipeline.set_state(gst::State::Null).unwrap();
                eprintln!(
                    "Got error from {}: {} ({})",
                    msg.src()
                        .map(|s| String::from(s.path_string()))
                        .unwrap_or_else(|| "None".into()),
                    err.error(),
                    err.debug().unwrap_or_else(|| "".into()),
                );
                break;
            }
            _ => (),
        }
    }

    Ok(())
}
