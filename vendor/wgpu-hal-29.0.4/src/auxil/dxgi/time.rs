#![allow(dead_code)] // IPresentationManager is unused currently

use windows::Win32::System::Performance::{QueryPerformanceCounter, QueryPerformanceFrequency};

pub enum PresentationTimer {
    /// DXGI uses [`QueryPerformanceCounter()`]
    Dxgi {
        /// How many ticks of QPC per second
        frequency: u64,
    },
    /// [`IPresentationManager`] uses [`QueryInterruptTimePrecise()`]
    ///
    /// [`IPresentationManager`]: https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Graphics/CompositionSwapchain/struct.IPresentationManager.html
    /// [`QueryInterruptTimePrecise()`]: https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/WindowsProgramming/fn.QueryInterruptTimePrecise.html
    #[allow(non_snake_case)]
    IPresentationManager {
        fnQueryInterruptTimePrecise: unsafe extern "system" fn(*mut u64),
    },
}

impl core::fmt::Debug for PresentationTimer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::Dxgi { frequency } => f
                .debug_struct("DXGI")
                .field("frequency", &frequency)
                .finish(),
            Self::IPresentationManager {
                fnQueryInterruptTimePrecise,
            } => f
                .debug_struct("IPresentationManager")
                .field(
                    "QueryInterruptTimePrecise",
                    &(fnQueryInterruptTimePrecise as usize),
                )
                .finish(),
        }
    }
}

impl PresentationTimer {
    /// Create a presentation timer using QueryPerformanceFrequency (what DXGI uses for presentation times)
    pub fn new_dxgi() -> Self {
        let mut frequency = 0;
        unsafe { QueryPerformanceFrequency(&mut frequency) }.unwrap();

        Self::Dxgi {
            frequency: frequency
                .try_into()
                .expect("Frequency should not be negative"),
        }
    }

    /// Create a presentation timer using QueryInterruptTimePrecise (what IPresentationManager uses for presentation times)
    ///
    /// Panics if QueryInterruptTimePrecise isn't found (below Win10)
    pub fn new_ipresentation_manager() -> Self {
        // We need to load this explicitly, as QueryInterruptTimePrecise is only available on Windows 10+
        //
        // Docs say it's in kernel32.dll, but it's actually in kernelbase.dll.
        // api-ms-win-core-realtime-l1-1-1.dll
        let kernelbase =
            libloading::os::windows::Library::open_already_loaded("kernelbase.dll").unwrap();
        // No concerns about lifetimes here as kernelbase is always there.
        let ptr = unsafe {
            kernelbase
                .get(c"QueryInterruptTimePrecise".to_bytes())
                .unwrap()
        };
        Self::IPresentationManager {
            fnQueryInterruptTimePrecise: *ptr,
        }
    }

    /// Gets the current time in nanoseconds.
    pub fn get_timestamp_ns(&self) -> u128 {
        // Always do u128 math _after_ hitting the timing function.
        match *self {
            PresentationTimer::Dxgi { frequency } => {
                let mut counter = 0;
                unsafe { QueryPerformanceCounter(&mut counter) }.unwrap();

                // counter * (1_000_000_000 / freq) but re-ordered to make more precise
                (counter as u128 * 1_000_000_000) / frequency as u128
            }
            PresentationTimer::IPresentationManager {
                fnQueryInterruptTimePrecise,
            } => {
                let mut counter = 0;
                unsafe { fnQueryInterruptTimePrecise(&mut counter) };

                // QueryInterruptTimePrecise uses units of 100ns for its tick.
                counter as u128 * 100
            }
        }
    }
}
