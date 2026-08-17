//! Platform-specific items.
//!
//! This module also contains the implementation of the platform's dynamically dispatched [`Host`]
//! type and its associated [`Device`], [`Stream`] and other associated types. These
//! types are useful in the case that users require switching between audio host APIs at runtime.

#[doc(inline)]
pub use self::platform_impl::*;

#[cfg(feature = "custom")]
pub use crate::host::custom::{Device as CustomDevice, Host as CustomHost, Stream as CustomStream};

/// A macro to assist with implementing a platform's dynamically dispatched [`Host`] type.
///
/// These dynamically dispatched types are necessary to allow for users to switch between hosts at
/// runtime.
///
/// For example the invocation `impl_platform_host(Wasapi wasapi "WASAPI", Asio asio "ASIO")`,
/// this macro should expand to:
///
// This sample code block is marked as text because it's not a valid test,
// it's just illustrative. (see rust issue #96573)
/// ```text
/// pub enum HostId {
///     Wasapi,
///     Asio,
/// }
///
/// pub enum Host {
///     Wasapi(crate::host::wasapi::Host),
///     Asio(crate::host::asio::Host),
/// }
/// ```
///
/// And so on for Device, Devices, Host, Stream, SupportedInputConfigs,
/// SupportedOutputConfigs and all their necessary trait implementations.
///
macro_rules! impl_platform_host {
    ($($(#[cfg($feat: meta)])? $HostVariant:ident => $Host:ty),* $(,)?) => {
        /// All hosts supported by CPAL on this platform.
        pub const ALL_HOSTS: &'static [HostId] = &[
            $(
                $(#[cfg($feat)])?
                HostId::$HostVariant,
            )*
        ];

        /// The platform's dynamically dispatched `Host` type.
        ///
        /// An instance of this `Host` type may represent one of the `Host`s available
        /// on the platform.
        ///
        /// Use this type if you require switching between available hosts at runtime.
        ///
        /// This type may be constructed via the [`host_from_id`] function. [`HostId`]s may
        /// be acquired via the [`ALL_HOSTS`] const, and the [`available_hosts`] function.
        pub struct Host(HostInner);

        /// The `Device` implementation associated with the platform's dynamically dispatched
        /// [`Host`] type.
        #[derive(Clone)]
        pub struct Device(DeviceInner);

        /// The `Devices` iterator associated with the platform's dynamically dispatched [`Host`]
        /// type.
        pub struct Devices(DevicesInner);

        /// The `Stream` implementation associated with the platform's dynamically dispatched
        /// [`Host`] type.
        #[must_use = "If the stream is not stored it will not play."]
        pub struct Stream(StreamInner);

        /// The `SupportedInputConfigs` iterator associated with the platform's dynamically
        /// dispatched [`Host`] type.
        #[derive(Clone)]
        pub struct SupportedInputConfigs(SupportedInputConfigsInner);

        /// The `SupportedOutputConfigs` iterator associated with the platform's dynamically
        /// dispatched [`Host`] type.
        #[derive(Clone)]
        pub struct SupportedOutputConfigs(SupportedOutputConfigsInner);

        /// Unique identifier for available hosts on the platform.
        ///
        /// Only the hosts supported by the current platform are available as enum variants.
        /// For cross-platform code that needs to handle hosts from other platforms,
        /// use the string representation via [`std::fmt::Display`]/[`std::str::FromStr`].
        ///
        /// # Available Host Strings
        ///
        /// For cross-platform matching, these host strings are available:
        ///
        /// - `"aaudio"` - Android Audio
        /// - `"alsa"` - Advanced Linux Sound Architecture
        /// - `"asio"` - ASIO
        /// - `"coreaudio"` - CoreAudio
        /// - `"custom"` - Custom host (requires `custom` feature)
        /// - `"emscripten"` - Emscripten
        /// - `"jack"` - JACK Audio Connection Kit
        /// - `"null"` - Null host
        /// - `"wasapi"` - Windows Audio Session API
        /// - `"webaudio"` - Web Audio API
        /// - `"audioworklet"` - Audio Worklet
        ///
        /// # Cross-Platform Example
        ///
        /// ```
        /// use cpal::HostId;
        /// use std::str::FromStr;
        ///
        /// fn handle_host_string(host_string: &str) {
        ///     // String matching works on all platforms
        ///     match host_string {
        ///         "alsa" => println!("ALSA host"),
        ///         "coreaudio" => println!("CoreAudio host"),
        ///         "jack" => println!("JACK host"),
        ///         "wasapi" => println!("WASAPI host"),
        ///         "asio" => println!("ASIO host"),
        ///         "aaudio" => println!("AAudio host"),
        ///         _ => println!("Other host"),
        ///     }
        ///
        ///     // Parse host string (may fail if host is not available on this platform)
        ///     if let Ok(host_id) = HostId::from_str(host_string) {
        ///         println!("Successfully parsed: {}", host_id);
        ///     }
        /// }
        /// ```
        #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
        pub enum HostId {
            $(
                $(#[cfg($feat)])?
                $(#[cfg_attr(docsrs, doc(cfg($feat)))])?
                $HostVariant,
            )*
        }

        /// Contains a platform specific [`Device`] implementation.
        #[derive(Clone)]
        pub enum DeviceInner {
            $(
                $(#[cfg($feat)])?
                $HostVariant(<$Host as crate::traits::HostTrait>::Device),
            )*
        }

        /// Contains a platform specific [`Devices`] implementation.
        pub enum DevicesInner {
            $(
                $(#[cfg($feat)])?
                $HostVariant(<$Host as crate::traits::HostTrait>::Devices),
            )*
        }

        /// Contains a platform specific [`Host`] implementation.
        pub enum HostInner {
            $(
                $(#[cfg($feat)])?
                $HostVariant($Host),
            )*
        }

        /// Contains a platform specific [`Stream`] implementation.
        pub enum StreamInner {
            $(
                $(#[cfg($feat)])?
                $HostVariant(<<$Host as crate::traits::HostTrait>::Device as crate::traits::DeviceTrait>::Stream),
            )*
        }

        #[derive(Clone)]
        enum SupportedInputConfigsInner {
            $(
                $(#[cfg($feat)])?
                $HostVariant(<<$Host as crate::traits::HostTrait>::Device as crate::traits::DeviceTrait>::SupportedInputConfigs),
            )*
        }

        #[derive(Clone)]
        enum SupportedOutputConfigsInner {
            $(
                $(#[cfg($feat)])?
                $HostVariant(<<$Host as crate::traits::HostTrait>::Device as crate::traits::DeviceTrait>::SupportedOutputConfigs),
            )*
        }

        impl HostId {
            pub fn name(&self) -> &'static str {
                match self {
                    $(
                        $(#[cfg($feat)])?
                        HostId::$HostVariant => stringify!($HostVariant),
                    )*
                }
            }
        }

        impl std::fmt::Display for HostId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.name().to_lowercase())
            }
        }

        impl std::str::FromStr for HostId {
            type Err = crate::HostUnavailable;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $(
                    $(#[cfg($feat)])?
                    if stringify!($HostVariant).eq_ignore_ascii_case(s) {
                        return Ok(HostId::$HostVariant);
                    }
                )*
                Err(crate::HostUnavailable)
            }
        }

        impl Devices {
            /// Returns a reference to the underlying platform specific implementation of this
            /// `Devices`.
            pub fn as_inner(&self) -> &DevicesInner {
                &self.0
            }

            /// Returns a mutable reference to the underlying platform specific implementation of
            /// this `Devices`.
            pub fn as_inner_mut(&mut self) -> &mut DevicesInner {
                &mut self.0
            }

            /// Returns the underlying platform specific implementation of this `Devices`.
            pub fn into_inner(self) -> DevicesInner {
                self.0
            }
        }

        impl Device {
            /// Returns a reference to the underlying platform specific implementation of this
            /// `Device`.
            pub fn as_inner(&self) -> &DeviceInner {
                &self.0
            }

            /// Returns a mutable reference to the underlying platform specific implementation of
            /// this `Device`.
            pub fn as_inner_mut(&mut self) -> &mut DeviceInner {
                &mut self.0
            }

            /// Returns the underlying platform specific implementation of this `Device`.
            pub fn into_inner(self) -> DeviceInner {
                self.0
            }
        }

        impl Host {
            /// The unique identifier associated with this `Host`.
            pub fn id(&self) -> HostId {
                match self.0 {
                    $(
                        $(#[cfg($feat)])?
                        HostInner::$HostVariant(_) => HostId::$HostVariant,
                    )*
                }
            }

            /// Returns a reference to the underlying platform specific implementation of this
            /// `Host`.
            pub fn as_inner(&self) -> &HostInner {
                &self.0
            }

            /// Returns a mutable reference to the underlying platform specific implementation of
            /// this `Host`.
            pub fn as_inner_mut(&mut self) -> &mut HostInner {
                &mut self.0
            }

            /// Returns the underlying platform specific implementation of this `Host`.
            pub fn into_inner(self) -> HostInner {
                self.0
            }
        }

        impl Stream {
            /// Returns a reference to the underlying platform specific implementation of this
            /// `Stream`.
            pub fn as_inner(&self) -> &StreamInner {
                &self.0
            }

            /// Returns a mutable reference to the underlying platform specific implementation of
            /// this `Stream`.
            pub fn as_inner_mut(&mut self) -> &mut StreamInner {
                &mut self.0
            }

            /// Returns the underlying platform specific implementation of this `Stream`.
            pub fn into_inner(self) -> StreamInner {
                self.0
            }
        }

        impl Iterator for Devices {
            type Item = Device;

            fn next(&mut self) -> Option<Self::Item> {
                match self.0 {
                    $(
                        $(#[cfg($feat)])?
                        DevicesInner::$HostVariant(ref mut d) => {
                            d.next().map(DeviceInner::$HostVariant).map(Device::from)
                        }
                    )*
                }
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                match self.0 {
                    $(
                        $(#[cfg($feat)])?
                        DevicesInner::$HostVariant(ref d) => d.size_hint(),
                    )*
                }
            }
        }

        impl Iterator for SupportedInputConfigs {
            type Item = crate::SupportedStreamConfigRange;

            fn next(&mut self) -> Option<Self::Item> {
                match self.0 {
                    $(
                        $(#[cfg($feat)])?
                        SupportedInputConfigsInner::$HostVariant(ref mut s) => s.next(),
                    )*
                }
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                match self.0 {
                    $(
                        $(#[cfg($feat)])?
                        SupportedInputConfigsInner::$HostVariant(ref d) => d.size_hint(),
                    )*
                }
            }
        }

        impl Iterator for SupportedOutputConfigs {
            type Item = crate::SupportedStreamConfigRange;

            fn next(&mut self) -> Option<Self::Item> {
                match self.0 {
                    $(
                        $(#[cfg($feat)])?
                        SupportedOutputConfigsInner::$HostVariant(ref mut s) => s.next(),
                    )*
                }
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                match self.0 {
                    $(
                        $(#[cfg($feat)])?
                        SupportedOutputConfigsInner::$HostVariant(ref d) => d.size_hint(),
                    )*
                }
            }
        }

        impl crate::traits::DeviceTrait for Device {
            type SupportedInputConfigs = SupportedInputConfigs;
            type SupportedOutputConfigs = SupportedOutputConfigs;
            type Stream = Stream;

            #[allow(deprecated)]
            fn name(&self) -> Result<String, crate::DeviceNameError> {
                match self.0 {
                    $(
                        $(#[cfg($feat)])?
                        DeviceInner::$HostVariant(ref d) => d.name(),
                    )*
                }
            }

            fn description(&self) -> Result<crate::DeviceDescription, crate::DeviceNameError> {
                match self.0 {
                    $(
                        $(#[cfg($feat)])?
                        DeviceInner::$HostVariant(ref d) => d.description(),
                    )*
                }
            }

            fn id(&self) -> Result<crate::DeviceId, crate::DeviceIdError> {
                match self.0 {
                    $(
                        $(#[cfg($feat)])?
                        DeviceInner::$HostVariant(ref d) => d.id(),
                    )*
                }
            }

            fn supports_input(&self) -> bool {
                match self.0 {
                    $(
                        $(#[cfg($feat)])?
                        DeviceInner::$HostVariant(ref d) => d.supports_input(),
                    )*
                }
            }

            fn supports_output(&self) -> bool {
                match self.0 {
                    $(
                        $(#[cfg($feat)])?
                        DeviceInner::$HostVariant(ref d) => d.supports_output(),
                    )*
                }
            }

            fn supported_input_configs(&self) -> Result<Self::SupportedInputConfigs, crate::SupportedStreamConfigsError> {
                match self.0 {
                    $(
                        $(#[cfg($feat)])?
                        DeviceInner::$HostVariant(ref d) => {
                            d.supported_input_configs()
                                .map(SupportedInputConfigsInner::$HostVariant)
                                .map(SupportedInputConfigs)
                        }
                    )*
                }
            }

            fn supported_output_configs(&self) -> Result<Self::SupportedOutputConfigs, crate::SupportedStreamConfigsError> {
                match self.0 {
                    $(
                        $(#[cfg($feat)])?
                        DeviceInner::$HostVariant(ref d) => {
                            d.supported_output_configs()
                                .map(SupportedOutputConfigsInner::$HostVariant)
                                .map(SupportedOutputConfigs)
                        }
                    )*
                }
            }

            fn default_input_config(&self) -> Result<crate::SupportedStreamConfig, crate::DefaultStreamConfigError> {
                match self.0 {
                    $(
                        $(#[cfg($feat)])?
                        DeviceInner::$HostVariant(ref d) => d.default_input_config(),
                    )*
                }
            }

            fn default_output_config(&self) -> Result<crate::SupportedStreamConfig, crate::DefaultStreamConfigError> {
                match self.0 {
                    $(
                        $(#[cfg($feat)])?
                        DeviceInner::$HostVariant(ref d) => d.default_output_config(),
                    )*
                }
            }

            fn build_input_stream_raw<D, E>(
                &self,
                config: &crate::StreamConfig,
                sample_format: crate::SampleFormat,
                data_callback: D,
                error_callback: E,
                timeout: Option<std::time::Duration>,
            ) -> Result<Self::Stream, crate::BuildStreamError>
            where
                D: FnMut(&crate::Data, &crate::InputCallbackInfo) + Send + 'static,
                E: FnMut(crate::StreamError) + Send + 'static,
            {
                match self.0 {
                    $(
                        $(#[cfg($feat)])?
                        DeviceInner::$HostVariant(ref d) => d
                            .build_input_stream_raw(
                                config,
                                sample_format,
                                data_callback,
                                error_callback,
                                timeout,
                            )
                            .map(StreamInner::$HostVariant)
                            .map(Stream::from),
                    )*
                }
            }

            fn build_output_stream_raw<D, E>(
                &self,
                config: &crate::StreamConfig,
                sample_format: crate::SampleFormat,
                data_callback: D,
                error_callback: E,
                timeout: Option<std::time::Duration>,
            ) -> Result<Self::Stream, crate::BuildStreamError>
            where
                D: FnMut(&mut crate::Data, &crate::OutputCallbackInfo) + Send + 'static,
                E: FnMut(crate::StreamError) + Send + 'static,
            {
                match self.0 {
                    $(
                        $(#[cfg($feat)])?
                        DeviceInner::$HostVariant(ref d) => d
                            .build_output_stream_raw(
                                config,
                                sample_format,
                                data_callback,
                                error_callback,
                                timeout,
                            )
                            .map(StreamInner::$HostVariant)
                            .map(Stream::from),
                    )*
                }
            }
        }

        impl crate::traits::HostTrait for Host {
            type Devices = Devices;
            type Device = Device;

            fn is_available() -> bool {
                $(
                    $(#[cfg($feat)])?
                    if <$Host>::is_available() { return true; }
                )*
                false
            }

            fn devices(&self) -> Result<Self::Devices, crate::DevicesError> {
                match self.0 {
                    $(
                        $(#[cfg($feat)])?
                        HostInner::$HostVariant(ref h) => {
                            h.devices().map(DevicesInner::$HostVariant).map(Devices::from)
                        }
                    )*
                }
            }

            fn default_input_device(&self) -> Option<Self::Device> {
                match self.0 {
                    $(
                        $(#[cfg($feat)])?
                        HostInner::$HostVariant(ref h) => {
                            h.default_input_device().map(DeviceInner::$HostVariant).map(Device::from)
                        }
                    )*
                }
            }

            fn default_output_device(&self) -> Option<Self::Device> {
                match self.0 {
                    $(
                        $(#[cfg($feat)])?
                        HostInner::$HostVariant(ref h) => {
                            h.default_output_device().map(DeviceInner::$HostVariant).map(Device::from)
                        }
                    )*
                }
            }
        }

        impl crate::traits::StreamTrait for Stream {
            fn play(&self) -> Result<(), crate::PlayStreamError> {
                match self.0 {
                    $(
                        $(#[cfg($feat)])?
                        StreamInner::$HostVariant(ref s) => {
                            s.play()
                        }
                    )*
                }
            }

            fn pause(&self) -> Result<(), crate::PauseStreamError> {
                match self.0 {
                    $(
                        $(#[cfg($feat)])?
                        StreamInner::$HostVariant(ref s) => {
                            s.pause()
                        }
                    )*
                }
            }
        }

        impl From<DeviceInner> for Device {
            fn from(d: DeviceInner) -> Self {
                Device(d)
            }
        }

        impl From<DevicesInner> for Devices {
            fn from(d: DevicesInner) -> Self {
                Devices(d)
            }
        }

        impl From<HostInner> for Host {
            fn from(h: HostInner) -> Self {
                Host(h)
            }
        }

        impl From<StreamInner> for Stream {
            fn from(s: StreamInner) -> Self {
                Stream(s)
            }
        }

        $(
            $(#[cfg($feat)])?
            impl From<<$Host as crate::traits::HostTrait>::Device> for Device {
                fn from(h: <$Host as crate::traits::HostTrait>::Device) -> Self {
                    DeviceInner::$HostVariant(h).into()
                }
            }

            $(#[cfg($feat)])?
            impl From<<$Host as crate::traits::HostTrait>::Devices> for Devices {
                fn from(h: <$Host as crate::traits::HostTrait>::Devices) -> Self {
                    DevicesInner::$HostVariant(h).into()
                }
            }

            $(#[cfg($feat)])?
            impl From<$Host> for Host {
                fn from(h: $Host) -> Self {
                    HostInner::$HostVariant(h).into()
                }
            }

            $(#[cfg($feat)])?
            impl From<<<$Host as crate::traits::HostTrait>::Device as crate::traits::DeviceTrait>::Stream> for Stream {
                fn from(h: <<$Host as crate::traits::HostTrait>::Device as crate::traits::DeviceTrait>::Stream) -> Self {
                    StreamInner::$HostVariant(h).into()
                }
            }
        )*

        /// Produces a list of hosts that are currently available on the system.
        pub fn available_hosts() -> Vec<HostId> {
            let mut host_ids = vec![];
            $(
                $(#[cfg($feat)])?
                if <$Host as crate::traits::HostTrait>::is_available() {
                    host_ids.push(HostId::$HostVariant);
                }
            )*
            host_ids
        }

        /// Given a unique host identifier, initialise and produce the host if it is available.
        pub fn host_from_id(id: HostId) -> Result<Host, crate::HostUnavailable> {
            match id {
                $(
                    $(#[cfg($feat)])?
                    HostId::$HostVariant => {
                        <$Host>::new()
                            .map(HostInner::$HostVariant)
                            .map(Host::from)
                    }
                )*
            }
        }

        impl Default for Host {
            fn default() -> Host {
                default_host()
            }
        }
    };
}

// TODO: Add pulseaudio here eventually.
#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd"
))]
mod platform_impl {
    #[cfg_attr(
        docsrs,
        doc(cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd"
        )))
    )]
    pub use crate::host::alsa::Host as AlsaHost;
    #[cfg(feature = "jack")]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(
            any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd"
            ),
            feature = "jack"
        )))
    )]
    pub use crate::host::jack::Host as JackHost;

    impl_platform_host!(
        #[cfg(feature = "jack")] Jack => JackHost,
        Alsa => AlsaHost,
        #[cfg(feature = "custom")] Custom => super::CustomHost
    );

    /// The default host for the current compilation target platform.
    pub fn default_host() -> Host {
        AlsaHost::new()
            .expect("the default host should always be available")
            .into()
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod platform_impl {
    #[cfg_attr(docsrs, doc(cfg(any(target_os = "macos", target_os = "ios"))))]
    pub use crate::host::coreaudio::Host as CoreAudioHost;
    #[cfg(all(feature = "jack", target_os = "macos"))]
    #[cfg_attr(docsrs, doc(cfg(all(feature = "jack", target_os = "macos"))))]
    pub use crate::host::jack::Host as JackHost;

    impl_platform_host!(
        CoreAudio => CoreAudioHost,
        #[cfg(all(feature = "jack", target_os = "macos"))] Jack => JackHost,
        #[cfg(feature = "custom")] Custom => super::CustomHost
    );

    /// The default host for the current compilation target platform.
    pub fn default_host() -> Host {
        CoreAudioHost::new()
            .expect("the default host should always be available")
            .into()
    }
}

#[cfg(target_os = "emscripten")]
mod platform_impl {
    #[cfg_attr(docsrs, doc(cfg(target_os = "emscripten")))]
    pub use crate::host::emscripten::Host as EmscriptenHost;
    impl_platform_host!(
        Emscripten => EmscriptenHost,
        #[cfg(feature = "custom")] Custom => super::CustomHost
    );

    /// The default host for the current compilation target platform.
    pub fn default_host() -> Host {
        EmscriptenHost::new()
            .expect("the default host should always be available")
            .into()
    }
}

#[cfg(all(target_arch = "wasm32", feature = "wasm-bindgen"))]
mod platform_impl {
    #[cfg_attr(
        docsrs,
        doc(cfg(all(target_arch = "wasm32", feature = "wasm-bindgen")))
    )]
    pub use crate::host::webaudio::Host as WebAudioHost;

    #[cfg(feature = "audioworklet")]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(
            target_arch = "wasm32",
            feature = "wasm-bindgen",
            feature = "audioworklet"
        )))
    )]
    pub use crate::host::audioworklet::Host as AudioWorkletHost;

    impl_platform_host!(
        WebAudio => WebAudioHost,
        #[cfg(feature = "audioworklet")] AudioWorklet => AudioWorkletHost,
        #[cfg(feature = "custom")] Custom => super::CustomHost
    );

    /// The default host for the current compilation target platform.
    pub fn default_host() -> Host {
        WebAudioHost::new()
            .expect("the default host should always be available")
            .into()
    }
}

#[cfg(windows)]
mod platform_impl {
    #[cfg(feature = "asio")]
    #[cfg_attr(docsrs, doc(cfg(all(windows, feature = "asio"))))]
    pub use crate::host::asio::Host as AsioHost;
    #[cfg(feature = "jack")]
    #[cfg_attr(docsrs, doc(cfg(all(windows, feature = "jack"))))]
    pub use crate::host::jack::Host as JackHost;
    #[cfg_attr(docsrs, doc(cfg(windows)))]
    pub use crate::host::wasapi::Host as WasapiHost;

    impl_platform_host!(
        #[cfg(feature = "asio")] Asio => AsioHost,
        Wasapi => WasapiHost,
        #[cfg(feature = "jack")] Jack => JackHost,
        #[cfg(feature = "custom")] Custom => super::CustomHost,
    );

    /// The default host for the current compilation target platform.
    pub fn default_host() -> Host {
        WasapiHost::new()
            .expect("the default host should always be available")
            .into()
    }
}

#[cfg(target_os = "android")]
mod platform_impl {
    #[cfg_attr(docsrs, doc(cfg(target_os = "android")))]
    pub use crate::host::aaudio::Host as AAudioHost;
    impl_platform_host!(
        AAudio => AAudioHost,
        #[cfg(feature = "custom")] Custom => super::CustomHost
    );

    /// The default host for the current compilation target platform.
    pub fn default_host() -> Host {
        AAudioHost::new()
            .expect("the default host should always be available")
            .into()
    }
}

#[cfg(not(any(
    windows,
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "macos",
    target_os = "ios",
    target_os = "emscripten",
    target_os = "android",
    all(target_arch = "wasm32", feature = "wasm-bindgen"),
)))]
mod platform_impl {
    #[cfg_attr(
        docsrs,
        doc(cfg(not(any(
            windows,
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "macos",
            target_os = "ios",
            target_os = "emscripten",
            target_os = "android",
            all(target_arch = "wasm32", feature = "wasm-bindgen")
        ))))
    )]
    pub use crate::host::null::Host as NullHost;

    impl_platform_host!(
        Null => NullHost,
        #[cfg(feature = "custom")] Custom => super::CustomHost,
    );

    /// The default host for the current compilation target platform.
    pub fn default_host() -> Host {
        NullHost::new()
            .expect("the default host should always be available")
            .into()
    }
}
