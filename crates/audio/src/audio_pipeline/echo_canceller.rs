#[cfg(not(any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd")))]
mod real_implementation {
    use anyhow::Context;
    use libwebrtc::native::apm;
    use parking_lot::Mutex;
    use std::sync::Arc;

    use crate::{CHANNEL_COUNT, SAMPLE_RATE};

    #[derive(Clone)]
    pub struct EchoCanceller(Arc<Mutex<apm::AudioProcessingModule>>);

    impl Default for EchoCanceller {
        fn default() -> Self {
            Self(Arc::new(Mutex::new(apm::AudioProcessingModule::new(
                true, false, false, false,
            ))))
        }
    }

    impl EchoCanceller {
        pub fn process_reverse_stream(&mut self, buf: &mut [i16]) {
            self.0
                .lock()
                .process_reverse_stream(buf, SAMPLE_RATE.get() as i32, CHANNEL_COUNT.get().into())
                .expect("Audio input and output threads should not panic");
        }

        pub fn process_stream(&mut self, buf: &mut [i16]) -> anyhow::Result<()> {
            self.0
                .lock()
                .process_stream(buf, SAMPLE_RATE.get() as i32, CHANNEL_COUNT.get() as i32)
                .context("livekit audio processor error")
        }
    }
}

#[cfg(any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd"))]
mod fake_implementation {
    #[derive(Clone, Default)]
    pub struct EchoCanceller;

    impl EchoCanceller {
        pub fn process_reverse_stream(&mut self, _buf: &mut [i16]) {}
        pub fn process_stream(&mut self, _buf: &mut [i16]) -> anyhow::Result<()> {
            Ok(())
        }
    }
}

#[cfg(any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd"))]
pub use fake_implementation::EchoCanceller;
#[cfg(not(any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd")))]
pub use real_implementation::EchoCanceller;
