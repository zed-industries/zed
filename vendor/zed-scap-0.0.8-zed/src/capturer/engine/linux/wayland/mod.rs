use std::{
    mem::size_of,
    sync::{
        atomic::{AtomicBool, AtomicU8},
        mpsc::{sync_channel, RecvError, SendError, Sender, SyncSender},
    },
    thread::JoinHandle,
    time::Duration,
};

use anyhow::{anyhow, Context as _, Result};
use pipewire as pw;
use pw::{
    context::Context,
    main_loop::MainLoop,
    properties::properties,
    spa::{
        self,
        param::{
            format::{FormatProperties, MediaSubtype, MediaType},
            video::VideoFormat,
            ParamType,
        },
        pod::{Pod, Property},
        sys::{
            spa_buffer, spa_meta_header, SPA_META_Header, SPA_PARAM_META_size, SPA_PARAM_META_type,
        },
        utils::{Direction, SpaTypes},
    },
    stream::{StreamRef, StreamState},
};

use crate::{
    Target,
    capturer::Options,
    frame::{BGRxFrame, Frame, RGBFrame, RGBxFrame, XBGRFrame},
};

use self::portal::ScreenCastPortal;

use super::LinuxCapturerImpl;

mod portal;

// TODO: Move to wayland capturer with Arc<>
static CAPTURER_STATE: AtomicU8 = AtomicU8::new(0);
static STREAM_STATE_CHANGED_TO_ERROR: AtomicBool = AtomicBool::new(false);

#[derive(Clone)]
struct ListenerUserData {
    pub tx: Sender<Result<Frame>>,
    pub format: spa::param::video::VideoInfoRaw,
}

fn param_changed_callback(
    _stream: &StreamRef,
    user_data: &mut ListenerUserData,
    id: u32,
    param: Option<&Pod>,
) {
    let Some(param) = param else {
        return;
    };
    if id != pw::spa::param::ParamType::Format.as_raw() {
        return;
    }
    let (media_type, media_subtype) = match pw::spa::param::format_utils::parse_format(param) {
        Ok(v) => v,
        Err(_) => return,
    };

    if media_type != MediaType::Video || media_subtype != MediaSubtype::Raw {
        return;
    }

    user_data
        .format
        .parse(param)
        // TODO: Tell library user of the error
        .expect("Failed to parse format parameter");
}

fn state_changed_callback(
    _stream: &StreamRef,
    _user_data: &mut ListenerUserData,
    _old: StreamState,
    new: StreamState,
) {
    match new {
        StreamState::Error(e) => {
            log::debug!("pipewire: State changed to error({e})");
            STREAM_STATE_CHANGED_TO_ERROR.store(true, std::sync::atomic::Ordering::Relaxed);
        }
        _ => {}
    }
}

unsafe fn get_timestamp(buffer: *mut spa_buffer) -> i64 {
    let n_metas = (*buffer).n_metas;
    if n_metas > 0 {
        let mut meta_ptr = (*buffer).metas;
        let metas_end = (*buffer).metas.wrapping_add(n_metas as usize);
        while meta_ptr != metas_end {
            if (*meta_ptr).type_ == SPA_META_Header {
                let meta_header: &mut spa_meta_header =
                    &mut *((*meta_ptr).data as *mut spa_meta_header);
                return meta_header.pts;
            }
            meta_ptr = meta_ptr.wrapping_add(1);
        }
        0
    } else {
        0
    }
}

fn process_callback(stream: &StreamRef, user_data: &mut ListenerUserData) {
    let buffer = unsafe { stream.dequeue_raw_buffer() };
    let frame_result = match process_callback_impl(buffer, user_data) {
        Ok(None) => None,
        Ok(Some(frame)) => Some(Ok(frame)),
        Err(err) => Some(Err(err)),
    };
    if let Some(frame_result) = frame_result {
        match user_data.tx.send(frame_result) {
            Ok(()) => {}
            Err(SendError(_)) => {
                log::debug!("Frame receiver was dropped.")
            }
        }
    }
    unsafe { stream.queue_raw_buffer(buffer) };
}

fn process_callback_impl(
    buffer: *mut pipewire::sys::pw_buffer,
    user_data: &mut ListenerUserData,
) -> Result<Option<Frame>> {
    if buffer.is_null() {
        return Err(anyhow!("Wayland screen capture out of buffers."));
    }
    let buffer = unsafe { (*buffer).buffer };
    if buffer.is_null() {
        // TODO: This matches the behavior of the original code by not having an error here.
        log::error!("Buffer pointer unexpectedly null in Wayland screen capture.");
        return Ok(None);
    }

    let timestamp = unsafe { get_timestamp(buffer) };

    let n_datas = unsafe { (*buffer).n_datas };
    if n_datas < 1 {
        return Ok(None);
    }
    let frame_size = user_data.format.size();
    let frame_data: Vec<u8> = unsafe {
        std::slice::from_raw_parts(
            (*(*buffer).datas).data as *mut u8,
            (*(*buffer).datas).maxsize as usize,
        )
        .to_vec()
    };

    match user_data.format.format() {
        VideoFormat::RGBx => Ok(Some(Frame::RGBx(RGBxFrame {
            display_time: timestamp as u64,
            width: frame_size.width as i32,
            height: frame_size.height as i32,
            data: frame_data,
        }))),
        VideoFormat::RGB => Ok(Some(Frame::RGB(RGBFrame {
            display_time: timestamp as u64,
            width: frame_size.width as i32,
            height: frame_size.height as i32,
            data: frame_data,
        }))),
        VideoFormat::xBGR => Ok(Some(Frame::XBGR(XBGRFrame {
            display_time: timestamp as u64,
            width: frame_size.width as i32,
            height: frame_size.height as i32,
            data: frame_data,
        }))),
        VideoFormat::BGRx => Ok(Some(Frame::BGRx(BGRxFrame {
            display_time: timestamp as u64,
            width: frame_size.width as i32,
            height: frame_size.height as i32,
            data: frame_data,
        }))),
        _ => Err(anyhow!("Unsupported frame format received")),
    }
}

fn start_pipewire_capturer(
    options: Options,
    tx: Sender<Result<Frame>>,
    stream_id: u32,
) -> Result<MainLoop> {
    pw::init();

    let mainloop = MainLoop::new(None)?;
    let context = Context::new(&mainloop)?;
    let core = context.connect(None)?;

    let user_data = ListenerUserData {
        tx,
        format: Default::default(),
    };

    let stream = pw::stream::Stream::new(
        &core,
        "scap",
        properties! {
            *pw::keys::MEDIA_TYPE => "Video",
            *pw::keys::MEDIA_CATEGORY => "Capture",
            *pw::keys::MEDIA_ROLE => "Screen",
        },
    )?;

    let _listener = stream
        .add_local_listener_with_user_data(user_data.clone())
        .state_changed(state_changed_callback)
        .param_changed(param_changed_callback)
        .process(process_callback)
        .register()?;

    let obj = pw::spa::pod::object!(
        pw::spa::utils::SpaTypes::ObjectParamFormat,
        pw::spa::param::ParamType::EnumFormat,
        pw::spa::pod::property!(FormatProperties::MediaType, Id, MediaType::Video),
        pw::spa::pod::property!(FormatProperties::MediaSubtype, Id, MediaSubtype::Raw),
        pw::spa::pod::property!(
            FormatProperties::VideoFormat,
            Choice,
            Enum,
            Id,
            pw::spa::param::video::VideoFormat::RGB,
            pw::spa::param::video::VideoFormat::RGBA,
            pw::spa::param::video::VideoFormat::RGBx,
            pw::spa::param::video::VideoFormat::BGRx,
        ),
        pw::spa::pod::property!(
            FormatProperties::VideoSize,
            Choice,
            Range,
            Rectangle,
            pw::spa::utils::Rectangle {
                // Default
                width: 128,
                height: 128,
            },
            pw::spa::utils::Rectangle {
                // Min
                width: 1,
                height: 1,
            },
            pw::spa::utils::Rectangle {
                // Max
                width: 4096,
                height: 4096,
            }
        ),
        pw::spa::pod::property!(
            FormatProperties::VideoFramerate,
            Choice,
            Range,
            Fraction,
            pw::spa::utils::Fraction {
                num: options.fps,
                denom: 1
            },
            pw::spa::utils::Fraction { num: 0, denom: 1 },
            pw::spa::utils::Fraction {
                num: 1000,
                denom: 1
            }
        ),
    );

    let metas_obj = pw::spa::pod::object!(
        SpaTypes::ObjectParamMeta,
        ParamType::Meta,
        Property::new(
            SPA_PARAM_META_type,
            pw::spa::pod::Value::Id(pw::spa::utils::Id(SPA_META_Header))
        ),
        Property::new(
            SPA_PARAM_META_size,
            pw::spa::pod::Value::Int(size_of::<pw::spa::sys::spa_meta_header>() as i32)
        ),
    );

    let values: Vec<u8> = pw::spa::pod::serialize::PodSerializer::serialize(
        std::io::Cursor::new(Vec::new()),
        &pw::spa::pod::Value::Object(obj),
    )?
    .0
    .into_inner();
    let metas_values: Vec<u8> = pw::spa::pod::serialize::PodSerializer::serialize(
        std::io::Cursor::new(Vec::new()),
        &pw::spa::pod::Value::Object(metas_obj),
    )?
    .0
    .into_inner();

    let mut params = [
        pw::spa::pod::Pod::from_bytes(&values)
            .context("Not enough space in screen capture 'values' param.")?,
        pw::spa::pod::Pod::from_bytes(&metas_values)
            .context("Not enough space in screen capture 'metas_values' param.")?,
    ];

    stream.connect(
        Direction::Input,
        Some(stream_id),
        pw::stream::StreamFlags::AUTOCONNECT | pw::stream::StreamFlags::MAP_BUFFERS,
        &mut params,
    )?;

    Ok(mainloop)
}

// TODO: Format negotiation
fn pipewire_capturer(
    options: Options,
    tx: Sender<Result<Frame>>,
    ready_sender: &SyncSender<Result<()>>,
    stream_id: u32,
) {
    let mainloop = match start_pipewire_capturer(options, tx, stream_id) {
        Ok(mainloop) => {
            ready_sender.send(Ok(())).ok();
            mainloop
        }
        Err(err) => {
            ready_sender.send(Err(err)).ok();
            return;
        }
    };

    while CAPTURER_STATE.load(std::sync::atomic::Ordering::Relaxed) == 0 {
        std::thread::sleep(Duration::from_millis(10));
    }

    let pw_loop = mainloop.loop_();

    // User has called Capturer::start() and we start the main loop
    while CAPTURER_STATE.load(std::sync::atomic::Ordering::Relaxed) == 1
        && /* If the stream state got changed to `Error`, we exit. TODO: tell user that we exited */
          !STREAM_STATE_CHANGED_TO_ERROR.load(std::sync::atomic::Ordering::Relaxed)
    {
        pw_loop.iterate(Duration::from_millis(100));
    }
}

pub struct WaylandCapturer {
    capturer_join_handle: Option<JoinHandle<()>>,
    // The pipewire stream is deleted when the connection is dropped.
    // That's why we keep it alive
    _connection: dbus::blocking::Connection,
}

impl WaylandCapturer {
    // TODO: Error handling
    pub fn new(options: &Options, tx: Sender<Result<Frame>>) -> Result<Self> {
        let connection = dbus::blocking::Connection::new_session()
            .context("Failed to create dbus connection")?;
        let stream_id = ScreenCastPortal::new(&connection)
            .show_cursor(options.show_cursor)
            .context("Unsupported screen capture cursor display mode")?
            .create_stream()
            .context("Failed to get screen capture stream")?
            .pw_node_id();

        // TODO: Fix this hack
        let options = options.clone();
        let (ready_sender, ready_recv) = sync_channel(1);
        let capturer_join_handle =
            std::thread::spawn(move || pipewire_capturer(options, tx, &ready_sender, stream_id));

        match ready_recv.recv() {
            Ok(Ok(())) => {}
            Ok(Err(err)) => {
                return Err(anyhow!(err));
            }
            Err(RecvError) => {
                return Err(anyhow!(
                    "Wayland screen capture bug: stream unexpectedly dropped."
                ));
            }
        }

        Ok(Self {
            capturer_join_handle: Some(capturer_join_handle),
            _connection: connection,
        })
    }
}

impl LinuxCapturerImpl for WaylandCapturer {
    fn start_capture(&mut self) {
        CAPTURER_STATE.store(1, std::sync::atomic::Ordering::Relaxed);
    }

    fn stop_capture(&mut self) {
        CAPTURER_STATE.store(2, std::sync::atomic::Ordering::Relaxed);
        if let Some(handle) = self.capturer_join_handle.take() {
            match handle.join() {
                Ok(()) => {}
                Err(err) => log::error!("Failed to join Wayland screen capture thread: {:?}", err),
            }
        }
        CAPTURER_STATE.store(0, std::sync::atomic::Ordering::Relaxed);
        STREAM_STATE_CHANGED_TO_ERROR.store(false, std::sync::atomic::Ordering::Relaxed);
    }
}
