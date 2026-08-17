pub const DEVPKEY_NFP_Capabilities: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0xfb3842cd_9e2a_4f83_8fcc_4b0761139ae9), pid: 2 };
pub const GUID_DEVINTERFACE_NFP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfb3842cd_9e2a_4f83_8fcc_4b0761139ae9);
pub const IOCTL_NFP_DISABLE: u32 = 5308492u32;
pub const IOCTL_NFP_ENABLE: u32 = 5308496u32;
pub const IOCTL_NFP_GET_KILO_BYTES_PER_SECOND: u32 = 5308548u32;
pub const IOCTL_NFP_GET_MAX_MESSAGE_BYTES: u32 = 5308544u32;
pub const IOCTL_NFP_GET_NEXT_SUBSCRIBED_MESSAGE: u32 = 5308480u32;
pub const IOCTL_NFP_GET_NEXT_TRANSMITTED_MESSAGE: u32 = 5308488u32;
pub const IOCTL_NFP_SET_PAYLOAD: u32 = 5308484u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SUBSCRIBED_MESSAGE {
    pub cbPayloadHint: u32,
    pub payload: [u8; 1],
}
impl Default for SUBSCRIBED_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
