#![warn(missing_docs, missing_debug_implementations)]
#![forbid(improper_ctypes, unsafe_op_in_unsafe_fn)]

//! Wayland cursor utilities
//!
//! This crate aims to re-implement the functionality of the `libwayland-cursor` library in Rust.
//!
//! It allows you to load cursors from the system and display them correctly.
//!
//! First of all, you need to create a [`CursorTheme`], which represents the full cursor theme.
//!
//! From this theme, using the [`get_cursor()`][CursorTheme::get_cursor()] method, you can load a
//! specific [`Cursor`], which can contain several images if the cursor is animated. It also provides
//! you with the means of querying which frame of the animation should be displayed at what time, as
//! well as handles to the buffers containing these frames, to attach them to a wayland surface.
//!
//! # Example
//!
//! ```
//! use wayland_cursor::CursorTheme;
//! # use std::ops::Deref;
//! # use std::thread::sleep;
//! # use std::time::{Instant, Duration};
//! # fn test(connection: &wayland_client::Connection, cursor_surface: &wayland_client::protocol::wl_surface::WlSurface, shm: wayland_client::protocol::wl_shm::WlShm) {
//! // Load the default cursor theme.
//! let mut cursor_theme = CursorTheme::load(&connection, shm, 32)
//!     .expect("Could not load cursor theme");
//! let cursor = cursor_theme.get_cursor("wait")
//!     .expect("Cursor not provided by theme");
//!
//! let start_time = Instant::now();
//! loop {
//!     // Obtain which frame we should show, and for how long.
//!     let millis = start_time.elapsed().as_millis();
//!     let fr_info = cursor.frame_and_duration(millis as u32);
//!
//!     // Here, we obtain the right cursor frame...
//!     let buffer = &cursor[fr_info.frame_index];
//!     // and attach it to a wl_surface.
//!     cursor_surface.attach(Some(&buffer), 0, 0);
//!     cursor_surface.commit();
//!
//!     sleep(Duration::from_millis(fr_info.frame_duration as u64));
//! }
//! # }
//! ```

use std::borrow::Cow;
use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::{Error as IoError, Read, Result as IoResult, Seek, SeekFrom, Write};
use std::ops::{Deref, Index};
use std::os::unix::io::{AsFd, OwnedFd};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use rustix::fs::Mode;
#[cfg(any(target_os = "linux", target_os = "android"))]
use rustix::fs::{memfd_create, MemfdFlags};
use rustix::io::Errno;
use rustix::shm;
#[cfg(any(target_os = "linux", target_os = "android"))]
use std::ffi::CStr;

use wayland_client::backend::{InvalidId, ObjectData, WeakBackend};
use wayland_client::protocol::wl_buffer::WlBuffer;
use wayland_client::protocol::wl_shm::{self, Format, WlShm};
use wayland_client::protocol::wl_shm_pool::{self, WlShmPool};
use wayland_client::{Connection, Proxy, WEnum};

use xcursor::parser as xparser;
use xcursor::CursorTheme as XCursorTheme;
use xparser::Image as XCursorImage;

/// Represents a cursor theme loaded from the system.
#[derive(Debug)]
pub struct CursorTheme {
    name: String,
    cursors: Vec<Cursor>,
    size: u32,
    pool: WlShmPool,
    pool_size: i32,
    file: File,
    backend: WeakBackend,
    fallback: Option<FallBack>,
}

type FallBackInner = Box<dyn Fn(&str, u32) -> Option<Cow<'static, [u8]>> + Send + Sync>;

struct FallBack(FallBackInner);

impl FallBack {
    fn new<F>(fallback: F) -> Self
    where
        F: Fn(&str, u32) -> Option<Cow<'static, [u8]>> + Send + Sync + 'static,
    {
        Self(Box::new(fallback))
    }
}

impl Debug for FallBack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("fallback function")
    }
}

impl CursorTheme {
    /// Load a cursor theme from system defaults.
    ///
    /// Same as calling the following:
    /// ```
    /// # use wayland_cursor::CursorTheme;
    /// # use wayland_client::{Connection, backend::InvalidId, protocol::wl_shm};
    /// # fn example(conn: &Connection, shm: wl_shm::WlShm, size: u32) -> Result<CursorTheme, InvalidId> {
    /// CursorTheme::load_or(conn, shm, "default", size)
    /// # }
    /// ```
    pub fn load(conn: &Connection, shm: WlShm, size: u32) -> Result<Self, InvalidId> {
        Self::load_or(conn, shm, "default", size)
    }

    /// Load a cursor theme, using `name` as fallback.
    ///
    /// The theme name and cursor size are read from the `XCURSOR_THEME` and
    /// `XCURSOR_SIZE` environment variables, respectively, or from the provided variables
    /// if those are invalid.
    pub fn load_or(
        conn: &Connection,
        shm: WlShm,
        name: &str,
        mut size: u32,
    ) -> Result<Self, InvalidId> {
        let name_string = String::from(name);
        let name = &env::var("XCURSOR_THEME").unwrap_or(name_string);

        if let Ok(var) = env::var("XCURSOR_SIZE") {
            if let Ok(int) = var.parse() {
                size = int;
            }
        }

        Self::load_from_name(conn, shm, name, size)
    }

    /// Create a new cursor theme, ignoring the system defaults.
    pub fn load_from_name(
        conn: &Connection,
        shm: WlShm,
        name: &str,
        size: u32,
    ) -> Result<Self, InvalidId> {
        // Set some minimal cursor size to hold it. We're not using `size` argument for that,
        // because the actual size that we'll use depends on theme sizes available on a system.
        // The minimal size covers most common minimal theme size, which is 16.
        const INITIAL_POOL_SIZE: i32 = 16 * 16 * 4;

        //  Create shm.
        let mem_fd = create_shm_fd().expect("Shm fd allocation failed");
        let mut file = File::from(mem_fd);
        file.set_len(INITIAL_POOL_SIZE as u64).expect("Failed to set buffer length");

        // Ensure that we have the same we requested.
        file.write_all(&[0; INITIAL_POOL_SIZE as usize]).expect("Write to shm fd failed");
        // Flush to ensure the compositor has access to the buffer when it tries to map it.
        file.flush().expect("Flush on shm fd failed");

        let pool_id = conn.send_request(
            &shm,
            wl_shm::Request::CreatePool { fd: file.as_fd(), size: INITIAL_POOL_SIZE },
            Some(Arc::new(IgnoreObjectData)),
        )?;
        let pool = WlShmPool::from_id(conn, pool_id)?;

        let name = String::from(name);

        Ok(Self {
            name,
            file,
            size,
            pool,
            pool_size: INITIAL_POOL_SIZE,
            cursors: Vec::new(),
            backend: conn.backend().downgrade(),
            fallback: None,
        })
    }

    /// Retrieve a cursor from the theme.
    ///
    /// This method returns [`None`] if this cursor is not provided either by the theme, or by one of its parents.
    ///
    /// If a [fallback is set], it will use the data returned by the fallback.
    ///
    /// [fallback is set]: Self::set_fallback()
    pub fn get_cursor(&mut self, name: &str) -> Option<&Cursor> {
        match self.cursors.iter().position(|cursor| cursor.name == name) {
            Some(i) => Some(&self.cursors[i]),
            None => {
                let cursor = match self.load_cursor(name, self.size) {
                    None => {
                        let fallback = self.fallback.as_ref()?;
                        let data = fallback.0(name, self.size)?;
                        let images = xparser::parse_xcursor(&data)?;
                        let conn = Connection::from_backend(self.backend.upgrade()?);
                        Cursor::new(&conn, name, self, &images, self.size)
                    }
                    Some(cursor) => cursor,
                };
                self.cursors.push(cursor);
                self.cursors.iter().last()
            }
        }
    }

    /// Set a fallback to load the cursor data, in case the system theme is missing a cursor that you need.
    ///
    /// Your fallback will be invoked with the name and size of the requested cursor and should return a byte
    /// array with the contents of an `xcursor` file, or [`None`] if you don't provide a fallback for this cursor.
    ///
    /// For example, this defines a generic fallback cursor image and uses it for all missing cursors:
    /// ```ignore
    /// use wayland_cursor::CursorTheme;
    /// use wayland_client::{Connection, backend::InvalidId, protocol::wl_shm};
    /// fn example(conn: &Connection, shm: wl_shm::WlShm, size: u32) -> Result<CursorTheme, InvalidId> {
    ///   let mut theme = CursorTheme::load_or(conn, shm, "default", size)?;
    ///   theme.set_fallback(|name, size| {
    ///       include_bytes!("./icons/default")
    ///   });
    ///   Ok(theme)
    /// }
    /// ```
    pub fn set_fallback<F>(&mut self, fallback: F)
    where
        F: Fn(&str, u32) -> Option<Cow<'static, [u8]>> + Send + Sync + 'static,
    {
        self.fallback = Some(FallBack::new(fallback))
    }

    /// This function loads a cursor, parses it and pushes the images onto the shm pool.
    ///
    /// Keep in mind that if the cursor is already loaded, the function will make a duplicate.
    fn load_cursor(&mut self, name: &str, size: u32) -> Option<Cursor> {
        let conn = Connection::from_backend(self.backend.upgrade()?);
        let icon_path = XCursorTheme::load(&self.name).load_icon(name)?;
        let mut icon_file = File::open(icon_path).ok()?;

        let mut buf = Vec::new();
        let images = {
            icon_file.read_to_end(&mut buf).ok()?;
            xparser::parse_xcursor(&buf)?
        };

        Some(Cursor::new(&conn, name, self, &images, size))
    }

    /// Grow the wl_shm_pool this theme is stored on.
    ///
    /// This method does nothing if the provided size is smaller or equal to the pool's current size.
    fn grow(&mut self, size: i32) {
        if size > self.pool_size {
            self.file.set_len(size as u64).expect("Failed to set new buffer length");
            self.pool.resize(size);
            self.pool_size = size;
        }
    }
}

/// A cursor from a theme. Can contain several images if animated.
#[derive(Debug, Clone)]
pub struct Cursor {
    name: String,
    images: Vec<CursorImageBuffer>,
    total_duration: u32,
}

impl Cursor {
    /// Construct a new Cursor.
    ///
    /// Each of the provided images will be written into `theme`.
    /// This will also grow `theme.pool` if necessary.
    fn new(
        conn: &Connection,
        name: &str,
        theme: &mut CursorTheme,
        images: &[XCursorImage],
        size: u32,
    ) -> Self {
        let mut total_duration = 0;
        let images: Vec<CursorImageBuffer> = Self::nearest_images(size, images)
            .map(|image| {
                let buffer = CursorImageBuffer::new(conn, theme, image);
                total_duration += buffer.delay;

                buffer
            })
            .collect();

        Self { total_duration, name: String::from(name), images }
    }

    fn nearest_images(size: u32, images: &[XCursorImage]) -> impl Iterator<Item = &XCursorImage> {
        // Follow the nominal size of the cursor to choose the nearest
        let nearest_image =
            images.iter().min_by_key(|image| (size as i32 - image.size as i32).abs()).unwrap();

        images.iter().filter(move |image| {
            image.width == nearest_image.width && image.height == nearest_image.height
        })
    }

    /// Given a time, calculate which frame to show, and how much time remains until the next frame.
    ///
    /// Time will wrap, so if for instance the cursor has an animation lasting 100ms,
    /// then calling this function with 5ms and 105ms as input gives the same output.
    pub fn frame_and_duration(&self, mut millis: u32) -> FrameAndDuration {
        millis %= self.total_duration;

        let mut res = 0;
        for (i, img) in self.images.iter().enumerate() {
            if millis < img.delay {
                res = i;
                break;
            }
            millis -= img.delay;
        }

        FrameAndDuration { frame_index: res, frame_duration: millis }
    }

    /// Total number of images forming this cursor animation
    pub fn image_count(&self) -> usize {
        self.images.len()
    }
}

impl Index<usize> for Cursor {
    type Output = CursorImageBuffer;

    fn index(&self, index: usize) -> &Self::Output {
        &self.images[index]
    }
}

/// A buffer containing a cursor image.
///
/// You can access the `WlBuffer` via `Deref`.
///
/// Note that this buffer is internally managed by wayland-cursor, as such you should
/// not try to act on it beyond assigning it to `wl_surface`s.
#[derive(Debug, Clone)]
pub struct CursorImageBuffer {
    buffer: WlBuffer,
    delay: u32,
    xhot: u32,
    yhot: u32,
    width: u32,
    height: u32,
}

impl CursorImageBuffer {
    /// Construct a new CursorImageBuffer
    ///
    /// This function appends the pixels of the image to the provided file,
    /// and constructs a wl_buffer on that data.
    fn new(conn: &Connection, theme: &mut CursorTheme, image: &XCursorImage) -> Self {
        let buf = &image.pixels_rgba;
        let offset = theme.file.seek(SeekFrom::End(0)).unwrap();

        // Resize memory before writing to it to handle shm correctly.
        let new_size = offset + buf.len() as u64;
        theme.grow(new_size as i32);

        theme.file.write_all(buf).unwrap();

        let buffer_id = conn
            .send_request(
                &theme.pool,
                wl_shm_pool::Request::CreateBuffer {
                    offset: offset as i32,
                    width: image.width as i32,
                    height: image.height as i32,
                    stride: (image.width * 4) as i32,
                    format: WEnum::Value(Format::Argb8888),
                },
                Some(Arc::new(IgnoreObjectData)),
            )
            .unwrap();

        let buffer = WlBuffer::from_id(conn, buffer_id).unwrap();

        Self {
            buffer,
            delay: image.delay,
            xhot: image.xhot,
            yhot: image.yhot,
            width: image.width,
            height: image.height,
        }
    }

    /// Dimensions of this image
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Location of the pointer hotspot in this image
    pub fn hotspot(&self) -> (u32, u32) {
        (self.xhot, self.yhot)
    }

    /// Time (in milliseconds) for which this image should be displayed
    pub fn delay(&self) -> u32 {
        self.delay
    }
}

impl Deref for CursorImageBuffer {
    type Target = WlBuffer;

    fn deref(&self) -> &WlBuffer {
        &self.buffer
    }
}

/// Which frame to show, and for how long.
///
/// This struct is output by `Cursor::frame_and_duration`
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FrameAndDuration {
    /// The index of the frame which should be shown.
    pub frame_index: usize,
    /// The duration that the frame should be shown for (in milliseconds).
    pub frame_duration: u32,
}

/// Create a shared file descriptor in memory.
fn create_shm_fd() -> IoResult<OwnedFd> {
    // Only try memfd on systems that provide it, (like Linux, Android)
    #[cfg(any(target_os = "linux", target_os = "android"))]
    loop {
        match memfd_create(
            CStr::from_bytes_with_nul(b"wayland-cursor-rs\0").unwrap(),
            MemfdFlags::CLOEXEC,
        ) {
            Ok(fd) => return Ok(fd),
            Err(Errno::INTR) => continue,
            Err(Errno::NOSYS) => break,
            Err(errno) => return Err(errno.into()),
        }
    }

    // Fallback to using shm_open.
    let sys_time = SystemTime::now();
    let mut mem_file_handle = format!(
        "/wayland-cursor-rs-{}",
        sys_time.duration_since(UNIX_EPOCH).unwrap().subsec_nanos()
    );
    loop {
        match shm::open(
            mem_file_handle.as_str(),
            shm::OFlags::CREATE | shm::OFlags::EXCL | shm::OFlags::RDWR,
            Mode::RUSR | Mode::WUSR,
        ) {
            Ok(fd) => match shm::unlink(mem_file_handle.as_str()) {
                Ok(_) => return Ok(fd),
                Err(errno) => return Err(IoError::from(errno)),
            },
            Err(Errno::EXIST) => {
                // If a file with that handle exists then change the handle
                mem_file_handle = format!(
                    "/wayland-cursor-rs-{}",
                    sys_time.duration_since(UNIX_EPOCH).unwrap().subsec_nanos()
                );
                continue;
            }
            Err(Errno::INTR) => continue,
            Err(errno) => return Err(IoError::from(errno)),
        }
    }
}

struct IgnoreObjectData;

impl ObjectData for IgnoreObjectData {
    fn event(
        self: Arc<Self>,
        _: &wayland_client::backend::Backend,
        _: wayland_client::backend::protocol::Message<wayland_client::backend::ObjectId, OwnedFd>,
    ) -> Option<Arc<dyn ObjectData>> {
        None
    }
    fn destroyed(&self, _: wayland_client::backend::ObjectId) {}
}
