#[cfg(feature = "Win32_Devices_AllJoyn")]
pub mod AllJoyn;
#[cfg(feature = "Win32_Devices_Beep")]
pub mod Beep;
#[cfg(feature = "Win32_Devices_BiometricFramework")]
pub mod BiometricFramework;
#[cfg(feature = "Win32_Devices_Bluetooth")]
pub mod Bluetooth;
#[cfg(feature = "Win32_Devices_Cdrom")]
pub mod Cdrom;
#[cfg(feature = "Win32_Devices_Communication")]
pub mod Communication;
#[cfg(feature = "Win32_Devices_DeviceAndDriverInstallation")]
pub mod DeviceAndDriverInstallation;
#[cfg(feature = "Win32_Devices_DeviceQuery")]
pub mod DeviceQuery;
#[cfg(feature = "Win32_Devices_Display")]
pub mod Display;
#[cfg(feature = "Win32_Devices_Dvd")]
pub mod Dvd;
#[cfg(feature = "Win32_Devices_Enumeration")]
pub mod Enumeration;
#[cfg(feature = "Win32_Devices_Fax")]
pub mod Fax;
#[cfg(feature = "Win32_Devices_HumanInterfaceDevice")]
pub mod HumanInterfaceDevice;
#[cfg(feature = "Win32_Devices_Nfc")]
pub mod Nfc;
#[cfg(feature = "Win32_Devices_Nfp")]
pub mod Nfp;
#[cfg(feature = "Win32_Devices_PortableDevices")]
pub mod PortableDevices;
#[cfg(feature = "Win32_Devices_Properties")]
pub mod Properties;
#[cfg(feature = "Win32_Devices_Pwm")]
pub mod Pwm;
#[cfg(feature = "Win32_Devices_Sensors")]
pub mod Sensors;
#[cfg(feature = "Win32_Devices_SerialCommunication")]
pub mod SerialCommunication;
#[cfg(feature = "Win32_Devices_Tapi")]
pub mod Tapi;
#[cfg(feature = "Win32_Devices_Usb")]
pub mod Usb;
#[cfg(feature = "Win32_Devices_WebServicesOnDevices")]
pub mod WebServicesOnDevices;
pub const BUS1394_LOCAL_HOST_INSTANCE_KEY: windows_sys::core::PCWSTR = windows_sys::core::w!("LOCAL HOST EUI64");
pub const BUS1394_VIRTUAL_DEVICE_LIST_KEY: windows_sys::core::PCWSTR = windows_sys::core::w!("Virtual Device List");
pub const IEEE1394API_ACCESS_EXCLUSIVE: u32 = 64u32;
pub const IEEE1394API_ACCESS_SHARED_READ: u32 = 16u32;
pub const IEEE1394API_ACCESS_SHARED_WRITE: u32 = 32u32;
pub const IEEE1394API_BUS_RESET_LOCAL_NODE_INITIATED: u32 = 4u32;
pub const IEEE1394API_BUS_RESET_LOCAL_NODE_IS_IRM: u32 = 2u32;
pub const IEEE1394API_BUS_RESET_LOCAL_NODE_IS_ROOT: u32 = 1u32;
pub const IEEE1394API_DEVICE_OWNERSHIP_LOCAL_NODE: u32 = 1u32;
pub const IEEE1394API_DEVICE_OWNERSHIP_REMOTE_NODE: u32 = 4u32;
pub const IEEE1394API_NOTIFICATION_BUS_RESET: u32 = 2u32;
pub const IEEE1394API_NOTIFICATION_DEVICE_ACCESS: u32 = 1u32;
pub const IEEE1394API_REMOTE_ACCESS_TRANSFER_REQUEST: u32 = 1u32;
pub const IEEE1394API_RESOURCE_OWNERSHIP_LOCAL_NODE: u32 = 2u32;
pub const IEEE1394API_RESOURCE_OWNERSHIP_REMOTE_NODE: u32 = 8u32;
pub const IEEE1394_API_ADD_VIRTUAL_DEVICE: u32 = 1u32;
pub const IEEE1394_API_DEVICE_ACCESS_TRANSFER: u32 = 3u32;
pub const IEEE1394_API_REMOVE_VIRTUAL_DEVICE: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IEEE1394_API_REQUEST {
    pub RequestNumber: u32,
    pub Flags: u32,
    pub u: IEEE1394_API_REQUEST_0,
}
impl Default for IEEE1394_API_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IEEE1394_API_REQUEST_0 {
    pub AddVirtualDevice: IEEE1394_VDEV_PNP_REQUEST,
    pub RemoveVirtualDevice: IEEE1394_VDEV_PNP_REQUEST,
}
impl Default for IEEE1394_API_REQUEST_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IEEE1394_API_SET_LOCAL_NODE_PROPERTIES: u32 = 4u32;
pub const IEEE1394_REQUEST_FLAG_PERSISTENT: u32 = 2u32;
pub const IEEE1394_REQUEST_FLAG_UNICODE: u32 = 1u32;
pub const IEEE1394_REQUEST_FLAG_USE_LOCAL_HOST_EUI: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IEEE1394_VDEV_PNP_REQUEST {
    pub fulFlags: u32,
    pub Reserved: u32,
    pub InstanceId: u64,
    pub DeviceId: u8,
}
pub const IOCTL_IEEE1394_API_REQUEST: u32 = 2229248u32;
