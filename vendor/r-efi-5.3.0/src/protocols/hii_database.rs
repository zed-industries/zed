//! Human Interface Infrastructure (HII) Protocol
//!
//! Database manager for HII-related data structures.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xef9fc172,
    0xa1b2,
    0x4693,
    0xb3,
    0x27,
    &[0x6d, 0x32, 0xfc, 0x41, 0x60, 0x42],
);

pub const SET_KEYBOARD_LAYOUT_EVENT_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x14982a4f,
    0xb0ed,
    0x45b8,
    0xa8,
    0x11,
    &[0x5a, 0x7a, 0x9b, 0xc2, 0x32, 0xdf],
);

pub type ProtocolNewPackageList = eficall! {fn(
    *const Protocol,
    *const crate::hii::PackageListHeader,
    crate::base::Handle,
    *mut crate::hii::Handle,
) -> crate::base::Status};

pub type ProtocolRemovePackageList = eficall! {fn(
    *const Protocol,
    crate::hii::Handle,
) -> crate::base::Status};

pub type ProtocolUpdatePackageList = eficall! {fn(
    *const Protocol,
    crate::hii::Handle,
    *const crate::hii::PackageListHeader,
) -> crate::base::Status};

pub type ProtocolListPackageLists = eficall! {fn(
    *const Protocol,
    u8,
    *const crate::base::Guid,
    *mut usize,
    *mut crate::hii::Handle,
) -> crate::base::Status};

pub type ProtocolExportPackageLists = eficall! {fn(
    *const Protocol,
    crate::hii::Handle,
    *mut usize,
    *mut crate::hii::PackageListHeader,
) -> crate::base::Status};

pub type ProtocolRegisterPackageNotify = eficall! {fn(
    *const Protocol,
    u8,
    *const crate::base::Guid,
    Notify,
    NotifyType,
    *mut crate::base::Handle,
) -> crate::base::Status};

pub type ProtocolUnregisterPackageNotify = eficall! {fn(
    *const Protocol,
    crate::base::Handle,
) -> crate::base::Status};

pub type ProtocolFindKeyboardLayouts = eficall! {fn(
    *const Protocol,
    *mut u16,
    *mut crate::base::Guid,
) -> crate::base::Status};

pub type ProtocolGetKeyboardLayout = eficall! {fn(
    *const Protocol,
    *const crate::base::Guid,
    *mut u16,
    *mut KeyboardLayout,
) -> crate::base::Status};

pub type ProtocolSetKeyboardLayout = eficall! {fn(
    *const Protocol,
    *mut crate::base::Guid,
) -> crate::base::Status};

pub type ProtocolGetPackageListHandle = eficall! {fn(
    *const Protocol,
    crate::hii::Handle,
    *mut crate::base::Handle,
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub new_package_list: ProtocolNewPackageList,
    pub remove_package_list: ProtocolRemovePackageList,
    pub update_package_list: ProtocolUpdatePackageList,
    pub list_package_lists: ProtocolListPackageLists,
    pub export_package_lists: ProtocolExportPackageLists,
    pub register_package_notify: ProtocolRegisterPackageNotify,
    pub unregister_package_notify: ProtocolUnregisterPackageNotify,
    pub find_keyboard_layouts: ProtocolFindKeyboardLayouts,
    pub get_keyboard_layout: ProtocolGetKeyboardLayout,
    pub set_keyboard_layout: ProtocolSetKeyboardLayout,
    pub get_package_list_handle: ProtocolGetPackageListHandle,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct KeyboardLayout<const N: usize = 0> {
    pub layout_length: u16,
    pub guid: crate::base::Guid,
    pub layout_descriptor_string_offset: u32,
    pub descriptor_count: u8,
    pub descriptors: [KeyDescriptor; N],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct KeyDescriptor {
    pub key: Key,
    pub unicode: crate::base::Char16,
    pub shifted_unicode: crate::base::Char16,
    pub alt_gr_unicode: crate::base::Char16,
    pub shifted_alt_gr_unicode: crate::base::Char16,
    pub modifier: u16,
    pub affected_attribute: u16,
}

pub const AFFECTED_BY_STANDARD_SHIFT: u16 = 0x0001;
pub const AFFECTED_BY_CAPS_LOCK: u16 = 0x0002;
pub const AFFECTED_BY_NUM_LOCK: u16 = 0x0004;

pub type Key = u32;

pub const EFI_KEY_LCTRL: Key = 0x00000000;
pub const EFI_KEY_A0: Key = 0x00000001;
pub const EFI_KEY_LALT: Key = 0x00000002;
pub const EFI_KEY_SPACE_BAR: Key = 0x00000003;
pub const EFI_KEY_A2: Key = 0x00000004;
pub const EFI_KEY_A3: Key = 0x00000005;
pub const EFI_KEY_A4: Key = 0x00000006;
pub const EFI_KEY_RCTRL: Key = 0x00000007;
pub const EFI_KEY_LEFT_ARROW: Key = 0x00000008;
pub const EFI_KEY_DOWN_ARROW: Key = 0x00000009;
pub const EFI_KEY_RIGHT_ARROW: Key = 0x0000000a;
pub const EFI_KEY_ZERO: Key = 0x0000000b;
pub const EFI_KEY_PERIOD: Key = 0x0000000c;
pub const EFI_KEY_ENTER: Key = 0x0000000d;
pub const EFI_KEY_LSHIFT: Key = 0x0000000e;
pub const EFI_KEY_B0: Key = 0x0000000f;
pub const EFI_KEY_B1: Key = 0x00000010;
pub const EFI_KEY_B2: Key = 0x00000011;
pub const EFI_KEY_B3: Key = 0x00000012;
pub const EFI_KEY_B4: Key = 0x00000013;
pub const EFI_KEY_B5: Key = 0x00000014;
pub const EFI_KEY_B6: Key = 0x00000015;
pub const EFI_KEY_B7: Key = 0x00000016;
pub const EFI_KEY_B8: Key = 0x00000017;
pub const EFI_KEY_B9: Key = 0x00000018;
pub const EFI_KEY_B10: Key = 0x00000019;
pub const EFI_KEY_RSHIFT: Key = 0x0000001a;
pub const EFI_KEY_UP_ARROW: Key = 0x0000001b;
pub const EFI_KEY_ONE: Key = 0x0000001c;
pub const EFI_KEY_TWO: Key = 0x0000001d;
pub const EFI_KEY_THREE: Key = 0x0000001e;
pub const EFI_KEY_CAPS_LOCK: Key = 0x0000001f;
pub const EFI_KEY_C1: Key = 0x00000020;
pub const EFI_KEY_C2: Key = 0x00000021;
pub const EFI_KEY_C3: Key = 0x00000022;
pub const EFI_KEY_C4: Key = 0x00000023;
pub const EFI_KEY_C5: Key = 0x00000024;
pub const EFI_KEY_C6: Key = 0x00000025;
pub const EFI_KEY_C7: Key = 0x00000026;
pub const EFI_KEY_C8: Key = 0x00000027;
pub const EFI_KEY_C9: Key = 0x00000028;
pub const EFI_KEY_C10: Key = 0x00000029;
pub const EFI_KEY_C11: Key = 0x0000002a;
pub const EFI_KEY_C12: Key = 0x0000002b;
pub const EFI_KEY_FOUR: Key = 0x0000002c;
pub const EFI_KEY_FIVE: Key = 0x0000002d;
pub const EFI_KEY_SIX: Key = 0x0000002e;
pub const EFI_KEY_PLUS: Key = 0x0000002f;
pub const EFI_KEY_TAB: Key = 0x00000030;
pub const EFI_KEY_D1: Key = 0x00000031;
pub const EFI_KEY_D2: Key = 0x00000032;
pub const EFI_KEY_D3: Key = 0x00000033;
pub const EFI_KEY_D4: Key = 0x00000034;
pub const EFI_KEY_D5: Key = 0x00000035;
pub const EFI_KEY_D6: Key = 0x00000036;
pub const EFI_KEY_D7: Key = 0x00000037;
pub const EFI_KEY_D8: Key = 0x00000038;
pub const EFI_KEY_D9: Key = 0x00000039;
pub const EFI_KEY_D10: Key = 0x0000003a;
pub const EFI_KEY_D11: Key = 0x0000003b;
pub const EFI_KEY_D12: Key = 0x0000003c;
pub const EFI_KEY_D13: Key = 0x0000003d;
pub const EFI_KEY_DEL: Key = 0x0000003e;
pub const EFI_KEY_END: Key = 0x0000003f;
pub const EFI_KEY_PGDN: Key = 0x00000040;
pub const EFI_KEY_SEVEN: Key = 0x00000041;
pub const EFI_KEY_EIGHT: Key = 0x00000042;
pub const EFI_KEY_NINE: Key = 0x00000043;
pub const EFI_KEY_E0: Key = 0x00000044;
pub const EFI_KEY_E1: Key = 0x00000045;
pub const EFI_KEY_E2: Key = 0x00000046;
pub const EFI_KEY_E3: Key = 0x00000047;
pub const EFI_KEY_E4: Key = 0x00000048;
pub const EFI_KEY_E5: Key = 0x00000049;
pub const EFI_KEY_E6: Key = 0x0000004a;
pub const EFI_KEY_E7: Key = 0x0000004b;
pub const EFI_KEY_E8: Key = 0x0000004c;
pub const EFI_KEY_E9: Key = 0x0000004d;
pub const EFI_KEY_E10: Key = 0x0000004e;
pub const EFI_KEY_E11: Key = 0x0000004f;
pub const EFI_KEY_E12: Key = 0x00000050;
pub const EFI_KEY_BACK_SPACE: Key = 0x00000051;
pub const EFI_KEY_INS: Key = 0x00000052;
pub const EFI_KEY_HOME: Key = 0x00000053;
pub const EFI_KEY_PGUP: Key = 0x00000054;
pub const EFI_KEY_NLCK: Key = 0x00000055;
pub const EFI_KEY_SLASH: Key = 0x00000056;
pub const EFI_KEY_ASTERISK: Key = 0x00000057;
pub const EFI_KEY_MINUS: Key = 0x00000058;
pub const EFI_KEY_ESC: Key = 0x00000059;
pub const EFI_KEY_F1: Key = 0x0000005a;
pub const EFI_KEY_F2: Key = 0x0000005b;
pub const EFI_KEY_F3: Key = 0x0000005c;
pub const EFI_KEY_F4: Key = 0x0000005d;
pub const EFI_KEY_F5: Key = 0x0000005e;
pub const EFI_KEY_F6: Key = 0x0000005f;
pub const EFI_KEY_F7: Key = 0x00000060;
pub const EFI_KEY_F8: Key = 0x00000061;
pub const EFI_KEY_F9: Key = 0x00000062;
pub const EFI_KEY_F10: Key = 0x00000063;
pub const EFI_KEY_F11: Key = 0x00000064;
pub const EFI_KEY_F12: Key = 0x00000065;
pub const EFI_KEY_PRINT: Key = 0x00000066;
pub const EFI_KEY_SLCK: Key = 0x00000067;
pub const EFI_KEY_PAUSE: Key = 0x00000068;

pub const NULL_MODIFIER: u16 = 0x0000;
pub const LEFT_CONTROL_MODIFIER: u16 = 0x0001;
pub const RIGHT_CONTROL_MODIFIER: u16 = 0x0002;
pub const LEFT_ALT_MODIFIER: u16 = 0x0003;
pub const RIGHT_ALT_MODIFIER: u16 = 0x0004;
pub const ALT_GR_MODIFIER: u16 = 0x0005;
pub const INSERT_MODIFIER: u16 = 0x0006;
pub const DELETE_MODIFIER: u16 = 0x0007;
pub const PAGE_DOWN_MODIFIER: u16 = 0x0008;
pub const PAGE_UP_MODIFIER: u16 = 0x0009;
pub const HOME_MODIFIER: u16 = 0x000A;
pub const END_MODIFIER: u16 = 0x000B;
pub const LEFT_SHIFT_MODIFIER: u16 = 0x000C;
pub const RIGHT_SHIFT_MODIFIER: u16 = 0x000D;
pub const CAPS_LOCK_MODIFIER: u16 = 0x000E;
pub const NUM_LOCK_MODIFIER: u16 = 0x000F;
pub const LEFT_ARROW_MODIFIER: u16 = 0x0010;
pub const RIGHT_ARROW_MODIFIER: u16 = 0x0011;
pub const DOWN_ARROW_MODIFIER: u16 = 0x0012;
pub const UP_ARROW_MODIFIER: u16 = 0x0013;
pub const NS_KEY_MODIFIER: u16 = 0x0014;
pub const NS_KEY_DEPENDENCY_MODIFIER: u16 = 0x0015;
pub const FUNCTION_KEY_ONE_MODIFIER: u16 = 0x0016;
pub const FUNCTION_KEY_TWO_MODIFIER: u16 = 0x0017;
pub const FUNCTION_KEY_THREE_MODIFIER: u16 = 0x0018;
pub const FUNCTION_KEY_FOUR_MODIFIER: u16 = 0x0019;
pub const FUNCTION_KEY_FIVE_MODIFIER: u16 = 0x001A;
pub const FUNCTION_KEY_SIX_MODIFIER: u16 = 0x001B;
pub const FUNCTION_KEY_SEVEN_MODIFIER: u16 = 0x001C;
pub const FUNCTION_KEY_EIGHT_MODIFIER: u16 = 0x001D;
pub const FUNCTION_KEY_NINE_MODIFIER: u16 = 0x001E;
pub const FUNCTION_KEY_TEN_MODIFIER: u16 = 0x001F;
pub const FUNCTION_KEY_ELEVEN_MODIFIER: u16 = 0x0020;
pub const FUNCTION_KEY_TWELVE_MODIFIER: u16 = 0x0021;
pub const PRINT_MODIFIER: u16 = 0x0022;
pub const SYS_REQUEST_MODIFIER: u16 = 0x0023;
pub const SCROLL_LOCK_MODIFIER: u16 = 0x0024;
pub const PAUSE_MODIFIER: u16 = 0x0025;
pub const BREAK_MODIFIER: u16 = 0x0026;
pub const LEFT_LOGO_MODIFIER: u16 = 0x0027;
pub const RIGHT_LOGO_MODIFIER: u16 = 0x0028;
pub const MENU_MODIFIER: u16 = 0x0029;

pub type Notify = eficall! {fn(
    u8,
    *const crate::base::Guid,
    *const crate::hii::PackageHeader,
    crate::hii::Handle,
    NotifyType,
) -> crate::base::Status};

pub type NotifyType = usize;

pub const NOTIFY_NEW_PACK: NotifyType = 0x00000001;
pub const NOTIFY_REMOVE_PACK: NotifyType = 0x00000002;
pub const NOTIFY_EXPORT_PACK: NotifyType = 0x00000004;
pub const NOTIFY_ADD_PACK: NotifyType = 0x00000008;
