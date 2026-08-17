// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::hidusage::{PUSAGE, USAGE};
use shared::minwindef::{PUCHAR, PULONG, PUSHORT, UCHAR, ULONG, USHORT};
use shared::ntdef::NTSTATUS;
use shared::ntstatus::FACILITY_HID_ERROR_CODE;
use um::winnt::{BOOLEAN, LONG, PCHAR, PLONG, PVOID};
pub const HIDP_LINK_COLLECTION_ROOT: USHORT = -1i16 as u16;
pub const HIDP_LINK_COLLECTION_UNSPECIFIED: USHORT = 0;
ENUM!{enum HIDP_REPORT_TYPE {
    HidP_Input,
    HidP_Output,
    HidP_Feature,
}}
STRUCT!{struct USAGE_AND_PAGE {
    Usage: USAGE,
    UsagePage: USAGE,
}}
pub type PUSAGE_AND_PAGE = *mut USAGE_AND_PAGE;
// HidP_IsSameUsageAndPage
STRUCT!{struct HIDP_CAPS_Range {
    UsageMin: USAGE,
    UsageMax: USAGE,
    StringMin: USHORT,
    StringMax: USHORT,
    DesignatorMin: USHORT,
    DesignatorMax: USHORT,
    DataIndexMin: USHORT,
    DataIndexMax: USHORT,
}}
STRUCT!{struct HIDP_CAPS_NotRange {
    Usage: USAGE,
    Reserved1: USAGE,
    StringIndex: USHORT,
    Reserved2: USHORT,
    DesignatorIndex: USHORT,
    Reserved3: USHORT,
    DataIndex: USHORT,
    Reserved4: USHORT,
}}
UNION!{union HIDP_CAPS_u {
    [u16; 8],
    Range Range_mut: HIDP_CAPS_Range,
    NotRange NotRange_mut: HIDP_CAPS_NotRange,
}}
STRUCT!{struct HIDP_BUTTON_CAPS {
    UsagePage: USAGE,
    ReportID: UCHAR,
    IsAlias: BOOLEAN,
    BitField: USHORT,
    LinkCollection: USHORT,
    LinkUsage: USAGE,
    LinkUsagePage: USAGE,
    IsRange: BOOLEAN,
    IsStringRange: BOOLEAN,
    IsDesignatorRange: BOOLEAN,
    IsAbsolute: BOOLEAN,
    Reserved: [ULONG; 10],
    u: HIDP_CAPS_u,
}}
pub type PHIDP_BUTTON_CAPS = *mut HIDP_BUTTON_CAPS;
STRUCT!{struct HIDP_VALUE_CAPS {
    UsagePage: USAGE,
    ReportID: UCHAR,
    IsAlias: BOOLEAN,
    BitField: USHORT,
    LinkCollection: USHORT,
    LinkUsage: USAGE,
    LinkUsagePage: USAGE,
    IsRange: BOOLEAN,
    IsStringRange: BOOLEAN,
    IsDesignatorRange: BOOLEAN,
    IsAbsolute: BOOLEAN,
    HasNull: BOOLEAN,
    Reserved: UCHAR,
    BitSize: USHORT,
    ReportCount: USHORT,
    Reserved2: [USHORT; 5],
    UnitsExp: ULONG,
    Units: ULONG,
    LogicalMin: LONG,
    LogicalMax: LONG,
    PhysicalMin: LONG,
    PhysicalMax: LONG,
    u: HIDP_CAPS_u,
}}
pub type PHIDP_VALUE_CAPS = *mut HIDP_VALUE_CAPS;
STRUCT!{struct HIDP_LINK_COLLECTION_NODE {
    LinkUsage: USAGE,
    LinkUsagePage: USAGE,
    Parent: USHORT,
    NumberOfChildren: USHORT,
    NextSibling: USHORT,
    FirstChild: USHORT,
    bit_fields: ULONG,
    UserContext: PVOID,
}}
BITFIELD!{HIDP_LINK_COLLECTION_NODE bit_fields: ULONG [
    CollectionType set_CollectionType[0..8],
    IsAlias set_IsAlias[8..9],
]}
pub type PHIDP_LINK_COLLECTION_NODE = *mut HIDP_LINK_COLLECTION_NODE;
pub type PHIDP_REPORT_DESCRIPTOR = PUCHAR;
pub enum HIDP_PREPARSED_DATA {}
pub type PHIDP_PREPARSED_DATA = *mut HIDP_PREPARSED_DATA;
STRUCT!{struct HIDP_CAPS {
    Usage: USAGE,
    UsagePage: USAGE,
    InputReportByteLength: USHORT,
    OutputReportByteLength: USHORT,
    FeatureReportByteLength: USHORT,
    Reserved: [USHORT; 17],
    NumberLinkCollectionNodes: USHORT,
    NumberInputButtonCaps: USHORT,
    NumberInputValueCaps: USHORT,
    NumberInputDataIndices: USHORT,
    NumberOutputButtonCaps: USHORT,
    NumberOutputValueCaps: USHORT,
    NumberOutputDataIndices: USHORT,
    NumberFeatureButtonCaps: USHORT,
    NumberFeatureValueCaps: USHORT,
    NumberFeatureDataIndices: USHORT,
}}
pub type PHIDP_CAPS = *mut HIDP_CAPS;
UNION!{union HIDP_DATA_u {
    [u32; 1],
    RawValue RawValue_mut: ULONG,
    On On_mut: BOOLEAN,
}}
STRUCT!{struct HIDP_DATA {
    DataIndex: USHORT,
    Reserved: USHORT,
    u: HIDP_DATA_u,
}}
pub type PHIDP_DATA = *mut HIDP_DATA;
STRUCT!{struct HIDP_UNKNOWN_TOKEN {
    Token: UCHAR,
    Reserved: [UCHAR; 3],
    BitField: ULONG,
}}
pub type PHIDP_UNKNOWN_TOKEN = *mut HIDP_UNKNOWN_TOKEN;
STRUCT!{struct HIDP_EXTENDED_ATTRIBUTES {
    NumGlobalUnknowns: UCHAR,
    Reserved: [UCHAR; 3],
    GlobalUnknowns: PHIDP_UNKNOWN_TOKEN,
    Data: [ULONG; 1],
}}
pub type PHIDP_EXTENDED_ATTRIBUTES = *mut HIDP_EXTENDED_ATTRIBUTES;
extern "system" {
    pub fn HidP_GetCaps(
        PreparsedData: PHIDP_PREPARSED_DATA,
        Capabilities: PHIDP_CAPS,
    ) -> NTSTATUS;
    pub fn HidP_GetLinkCollectionNodes(
        LinkCollectionNodes: PHIDP_LINK_COLLECTION_NODE,
        LinkCollectionNodesLength: PULONG,
        PreparsedData: PHIDP_PREPARSED_DATA,
    ) -> NTSTATUS;
    pub fn HidP_GetSpecificButtonCaps(
        ReportType: HIDP_REPORT_TYPE,
        UsagePage: USAGE,
        LinkCollection: USHORT,
        Usage: USAGE,
        ButtonCaps: PHIDP_BUTTON_CAPS,
        ButtonCapsLength: PUSHORT,
        PreparsedData: PHIDP_PREPARSED_DATA,
    ) -> NTSTATUS;
    pub fn HidP_GetButtonCaps(
        ReportType: HIDP_REPORT_TYPE,
        ButtonCaps: PHIDP_BUTTON_CAPS,
        ButtonCapsLength: PUSHORT,
        PreparsedData: PHIDP_PREPARSED_DATA,
    ) -> NTSTATUS;
    pub fn HidP_GetSpecificValueCaps(
        ReportType: HIDP_REPORT_TYPE,
        UsagePage: USAGE,
        LinkCollection: USHORT,
        Usage: USAGE,
        ValueCaps: PHIDP_VALUE_CAPS,
        ValueCapsLength: PUSHORT,
        PreparsedData: PHIDP_PREPARSED_DATA,
    ) -> NTSTATUS;
    pub fn HidP_GetValueCaps(
        ReportType: HIDP_REPORT_TYPE,
        ValueCaps: PHIDP_VALUE_CAPS,
        ValueCapsLength: PUSHORT,
        PreparsedData: PHIDP_PREPARSED_DATA,
    ) -> NTSTATUS;
    pub fn HidP_GetExtendedAttributes(
        ReportType: HIDP_REPORT_TYPE,
        DataIndex: USHORT,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Attributes: PHIDP_EXTENDED_ATTRIBUTES,
        LengthAttributes: PULONG,
    ) -> NTSTATUS;
    pub fn HidP_InitializeReportForID(
        ReportType: HIDP_REPORT_TYPE,
        ReportID: UCHAR,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_SetData(
        ReportType: HIDP_REPORT_TYPE,
        DataList: PHIDP_DATA,
        DataLength: PULONG,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_GetData(
        ReportType: HIDP_REPORT_TYPE,
        DataList: PHIDP_DATA,
        DataLength: PULONG,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_MaxDataListLength(
        ReportType: HIDP_REPORT_TYPE,
        PreparsedData: PHIDP_PREPARSED_DATA,
    ) -> ULONG;
    pub fn HidP_SetUsages(
        ReportType: HIDP_REPORT_TYPE,
        UsagePage: USAGE,
        LinkCollection: USHORT,
        UsageList: PUSAGE,
        UsageLength: PULONG,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_UnsetUsages(
        ReportType: HIDP_REPORT_TYPE,
        UsagePage: USAGE,
        LinkCollection: USHORT,
        UsageList: PUSAGE,
        UsageLength: PULONG,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_GetUsages(
        ReportType: HIDP_REPORT_TYPE,
        UsagePage: USAGE,
        LinkCollection: USHORT,
        UsageList: PUSAGE,
        UsageLength: PULONG,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_GetUsagesEx(
        ReportType: HIDP_REPORT_TYPE,
        LinkCollection: USHORT,
        ButtonList: PUSAGE_AND_PAGE,
        UsageLength: *mut ULONG,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_MaxUsageListLength(
        ReportType: HIDP_REPORT_TYPE,
        UsagePage: USAGE,
        PreparsedData: PHIDP_PREPARSED_DATA,
    ) -> ULONG;
    pub fn HidP_SetUsageValue(
        ReportType: HIDP_REPORT_TYPE,
        UsagePage: USAGE,
        LinkCollection: USHORT,
        Usage: USAGE,
        UsageValue: ULONG,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_SetScaledUsageValue(
        ReportType: HIDP_REPORT_TYPE,
        UsagePage: USAGE,
        LinkCollection: USHORT,
        Usage: USAGE,
        UsageValue: LONG,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_SetUsageValueArray(
        ReportType: HIDP_REPORT_TYPE,
        UsagePage: USAGE,
        LinkCollection: USHORT,
        Usage: USAGE,
        UsageValue: PCHAR,
        UsageValueByteLength: USHORT,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_GetUsageValue(
        ReportType: HIDP_REPORT_TYPE,
        UsagePage: USAGE,
        LinkCollection: USHORT,
        Usage: USAGE,
        UsageValue: PULONG,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_GetScaledUsageValue(
        ReportType: HIDP_REPORT_TYPE,
        UsagePage: USAGE,
        LinkCollection: USHORT,
        Usage: USAGE,
        UsageValue: PLONG,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_GetUsageValueArray(
        ReportType: HIDP_REPORT_TYPE,
        UsagePage: USAGE,
        LinkCollection: USHORT,
        Usage: USAGE,
        UsageValue: PCHAR,
        UsageValueByteLength: USHORT,
        PreparsedData: PHIDP_PREPARSED_DATA,
        Report: PCHAR,
        ReportLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_UsageListDifference(
        PreviousUsageList: PUSAGE,
        CurrentUsageList: PUSAGE,
        BreakUsageList: PUSAGE,
        MakeUsageList: PUSAGE,
        UsageListLength: ULONG,
    ) -> NTSTATUS;
    pub fn HidP_TranslateUsagesToI8042ScanCodes(
        ChangedUsageList: PUSAGE,
        UsageListLength: ULONG,
        KeyAction: HIDP_KEYBOARD_DIRECTION,
        ModifierState: PHIDP_KEYBOARD_MODIFIER_STATE,
        InsertCodesProcedure: PHIDP_INSERT_SCANCODES,
        InsertCodesContext: PVOID,
    ) -> NTSTATUS;
}
ENUM!{enum HIDP_KEYBOARD_DIRECTION {
    HidP_Keyboard_Break,
    HidP_Keyboard_Make,
}}
STRUCT!{struct HIDP_KEYBOARD_MODIFIER_STATE {
    ul: ULONG,
}}
BITFIELD!{HIDP_KEYBOARD_MODIFIER_STATE ul: ULONG [
    LeftControl set_LeftControl[0..1],
    LeftShift set_LeftShift[1..2],
    LeftAlt set_LeftAlt[2..3],
    LeftGUI set_LeftGUI[3..4],
    RightControl set_RightControl[4..5],
    RightShift set_RightShift[5..6],
    RightAlt set_RightAlt[6..7],
    RigthGUI set_RigthGUI[7..8],
    CapsLock set_CapsLock[8..9],
    ScollLock set_ScollLock[9..10],
    NumLock set_NumLock[10..11],
]}
pub type PHIDP_KEYBOARD_MODIFIER_STATE = *mut HIDP_KEYBOARD_MODIFIER_STATE;
FN!{stdcall PHIDP_INSERT_SCANCODES(
    Context: PVOID,
    NewScanCodes: PCHAR,
    Length: ULONG,
) -> BOOLEAN}
pub const HIDP_STATUS_SUCCESS: NTSTATUS = HIDP_ERROR_CODES!(0x0, 0);
pub const HIDP_STATUS_NULL: NTSTATUS = HIDP_ERROR_CODES!(0x8, 1);
pub const HIDP_STATUS_INVALID_PREPARSED_DATA: NTSTATUS = HIDP_ERROR_CODES!(0xC, 1);
pub const HIDP_STATUS_INVALID_REPORT_TYPE: NTSTATUS = HIDP_ERROR_CODES!(0xC, 2);
pub const HIDP_STATUS_INVALID_REPORT_LENGTH: NTSTATUS = HIDP_ERROR_CODES!(0xC, 3);
pub const HIDP_STATUS_USAGE_NOT_FOUND: NTSTATUS = HIDP_ERROR_CODES!(0xC, 4);
pub const HIDP_STATUS_VALUE_OUT_OF_RANGE: NTSTATUS = HIDP_ERROR_CODES!(0xC, 5);
pub const HIDP_STATUS_BAD_LOG_PHY_VALUES: NTSTATUS = HIDP_ERROR_CODES!(0xC, 6);
pub const HIDP_STATUS_BUFFER_TOO_SMALL: NTSTATUS = HIDP_ERROR_CODES!(0xC, 7);
pub const HIDP_STATUS_INTERNAL_ERROR: NTSTATUS = HIDP_ERROR_CODES!(0xC, 8);
pub const HIDP_STATUS_I8042_TRANS_UNKNOWN: NTSTATUS = HIDP_ERROR_CODES!(0xC, 9);
pub const HIDP_STATUS_INCOMPATIBLE_REPORT_ID: NTSTATUS = HIDP_ERROR_CODES!(0xC, 0xA);
pub const HIDP_STATUS_NOT_VALUE_ARRAY: NTSTATUS = HIDP_ERROR_CODES!(0xC, 0xB);
pub const HIDP_STATUS_IS_VALUE_ARRAY: NTSTATUS = HIDP_ERROR_CODES!(0xC, 0xC);
pub const HIDP_STATUS_DATA_INDEX_NOT_FOUND: NTSTATUS = HIDP_ERROR_CODES!(0xC, 0xD);
pub const HIDP_STATUS_DATA_INDEX_OUT_OF_RANGE: NTSTATUS = HIDP_ERROR_CODES!(0xC, 0xE);
pub const HIDP_STATUS_BUTTON_NOT_PRESSED: NTSTATUS = HIDP_ERROR_CODES!(0xC, 0xF);
pub const HIDP_STATUS_REPORT_DOES_NOT_EXIST: NTSTATUS = HIDP_ERROR_CODES!(0xC, 0x10);
pub const HIDP_STATUS_NOT_IMPLEMENTED: NTSTATUS = HIDP_ERROR_CODES!(0xC, 0x20);
pub const HIDP_STATUS_I8242_TRANS_UNKNOWN: NTSTATUS = HIDP_STATUS_I8042_TRANS_UNKNOWN;
