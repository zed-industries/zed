use windows::Win32::{Foundation, Graphics::Dxgi};

pub(crate) trait HResult<O> {
    fn into_device_result(self, description: &str) -> Result<O, crate::DeviceError>;
}
impl<T> HResult<T> for windows::core::Result<T> {
    fn into_device_result(self, description: &str) -> Result<T, crate::DeviceError> {
        #![allow(unreachable_code)]

        self.map_err(|err| {
            log::error!("{description} failed: {err}");

            match err.code() {
                Foundation::E_OUTOFMEMORY => crate::DeviceError::OutOfMemory,
                Dxgi::DXGI_ERROR_DEVICE_RESET | Dxgi::DXGI_ERROR_DEVICE_REMOVED => {
                    #[cfg(feature = "device_lost_panic")]
                    panic!("{description} failed: Device lost ({err})");
                    crate::DeviceError::Lost
                }
                _ => {
                    #[cfg(feature = "internal_error_panic")]
                    panic!("{description} failed: {err}");
                    crate::DeviceError::Unexpected
                }
            }
        })
    }
}
