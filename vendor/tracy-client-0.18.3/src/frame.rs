use crate::Client;

/// A non-continuous frame region.
///
/// Create with the [`Client::non_continuous_frame`] function.
pub struct Frame(Client, FrameName);

/// A name for secondary and non-continuous frames.
///
/// Create with the [`frame_name!`](crate::frame_name) macro.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FrameName(pub(crate) &'static str);

impl FrameName {
    /// Construct a `FrameName` dynamically, leaking the provided String.
    ///
    /// You should call this function once for a given name, and store the returned `FrameName` for
    /// continued use, to avoid rapid memory use growth. Whenever possible, prefer the
    /// [`frame_name!`](crate::frame_name) macro, which takes a literal name and doesn't leak
    /// memory.
    ///
    /// The resulting value may be used as an argument for the the [`Client::secondary_frame_mark`]
    /// and [`Client::non_continuous_frame`] methods.
    #[must_use]
    pub fn new_leak(name: String) -> Self {
        #[cfg(feature = "enable")]
        {
            // Ensure the name is null-terminated.
            let mut name = name;
            name.push('\0');
            // Drop excess capacity by converting into a boxed str, then leak.
            let name = Box::leak(name.into_boxed_str());
            Self(name)
        }
        #[cfg(not(feature = "enable"))]
        {
            drop(name);
            Self("\0")
        }
    }
}

/// Instrumentation for global frame indicators.
impl Client {
    /// Indicate that rendering of a continuous frame has ended.
    ///
    /// # Examples
    ///
    /// In a traditional rendering scenarios a frame mark should be inserted after a buffer swap.
    ///
    /// ```
    /// use tracy_client::Client;
    /// # fn swap_buffers() {}
    /// # let client = tracy_client::Client::start();
    /// // loop {
    /// //     ...
    ///        swap_buffers();
    ///        Client::running().expect("client must be running").frame_mark();
    /// // }
    /// ```
    pub fn frame_mark(&self) {
        #[cfg(feature = "enable")]
        unsafe {
            let () = sys::___tracy_emit_frame_mark(std::ptr::null());
        }
    }

    /// Indicate that rendering of a secondary (named) continuous frame has ended.
    ///
    /// # Examples
    ///
    /// Much like with the primary frame mark, the secondary (named) frame mark should be inserted
    /// after some continuously repeating operation finishes one iteration of its processing.
    ///
    /// ```
    /// use tracy_client::frame_name;
    /// # fn physics_tick() {}
    /// # let client = tracy_client::Client::start();
    /// // loop {
    /// //     ...
    ///        physics_tick();
    ///        tracy_client::Client::running()
    ///            .expect("client must be running")
    ///            .secondary_frame_mark(frame_name!("physics"));
    /// // }
    /// ```
    pub fn secondary_frame_mark(&self, name: FrameName) {
        #[cfg(feature = "enable")]
        unsafe {
            // SAFE: We ensured that the name would be null-terminated.
            let () = sys::___tracy_emit_frame_mark(name.0.as_ptr().cast());
        }
    }

    /// Indicate that a processing of a non-continuous frame has begun.
    ///
    /// Dropping the returned [`Frame`] will terminate the non-continuous frame.
    ///
    /// # Examples
    ///
    /// ```
    /// use tracy_client::frame_name;
    /// # let client = tracy_client::Client::start();
    /// let _guard = tracy_client::Client::running()
    ///     .expect("client must be running")
    ///     .non_continuous_frame(frame_name!("a frame"));
    /// ```
    #[must_use]
    pub fn non_continuous_frame(&self, name: FrameName) -> Frame {
        #[cfg(feature = "enable")]
        unsafe {
            // SAFE: We ensure that the name would be null-terminated.
            let () = sys::___tracy_emit_frame_mark_start(name.0.as_ptr().cast());
        }
        Frame(self.clone(), name)
    }

    /// Emits an image of a frame.
    ///
    /// The following restrictions apply:
    /// - The image must be in RGBA format.
    /// - The image width and height must be divisible by four.
    /// - The image data must be less than 256 KB, and should be as small as
    ///   possible.
    ///
    /// The offset indicates how many frames in the past the image was captured.
    /// For example, an offset of 1 would associate the image with the previous
    /// call to [`frame_mark`](Client::frame_mark).
    ///
    /// The `flip` parameter indicates that the image should be flipped before
    /// displaying, for example if the image is an OpenGL texture.
    pub fn frame_image(&self, image: &[u8], width: u16, height: u16, offset: u8, flip: bool) {
        #[cfg(feature = "enable")]
        unsafe {
            // SAFE: Tracy copies the data before returning.
            let ptr = image.as_ptr();

            let () = sys::___tracy_emit_frame_image(ptr.cast(), width, height, offset, flip as i32);
        }
    }
}

/// Construct a [`FrameName`].
///
/// The resulting value may be used as an argument for the the [`Client::secondary_frame_mark`] and
/// [`Client::non_continuous_frame`] methods. The macro can be used in a `const` context.
#[macro_export]
macro_rules! frame_name {
    ($name: literal) => {{
        unsafe { $crate::internal::create_frame_name(concat!($name, "\0")) }
    }};
}

impl Drop for Frame {
    fn drop(&mut self) {
        #[cfg(feature = "enable")]
        unsafe {
            // SAFE: We ensure that thena me would be null-terminated. We also still have an owned
            // Client handle.
            let () = sys::___tracy_emit_frame_mark_end(self.1 .0.as_ptr().cast());
            std::convert::identity(&self.0);
        }
    }
}

/// Convenience shortcut for [`Client::frame_mark`] on the current client.
///
/// # Panics
///
/// - If a `Client` isn't currently running.
pub fn frame_mark() {
    Client::running()
        .expect("frame_mark! without a running Client")
        .frame_mark();
}

/// Convenience shortcut for [`Client::frame_image`] on the current client.
pub fn frame_image(image: &[u8], width: u16, height: u16, offset: u8, flip: bool) {
    Client::running()
        .expect("frame_image without a running Client")
        .frame_image(image, width, height, offset, flip);
}

/// Convenience macro for [`Client::secondary_frame_mark`] on the current client.
///
/// # Panics
///
/// - If a `Client` isn't currently running.
#[macro_export]
macro_rules! secondary_frame_mark {
    ($name: literal) => {{
        $crate::Client::running()
            .expect("secondary_frame_mark! without a running Client")
            .secondary_frame_mark($crate::frame_name!($name))
    }};
}

/// Convenience macro for [`Client::non_continuous_frame`] on the current client.
///
/// # Panics
///
/// - If a `Client` isn't currently running.
#[macro_export]
macro_rules! non_continuous_frame {
    ($name: literal) => {{
        $crate::Client::running()
            .expect("non_continuous_frame! without a running Client")
            .non_continuous_frame($crate::frame_name!($name))
    }};
}
