// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::guiddef::GUID;
use shared::minwindef::{ULONG, USHORT};
use shared::ntdef::{LONGLONG, ULONGLONG};
STRUCT!{struct SDP_LARGE_INTEGER_16 {
    LowPart: ULONGLONG,
    HighPart: LONGLONG,
}}
STRUCT!{struct SDP_ULARGE_INTEGER_16 {
    LowPart: ULONGLONG,
    HighPart: ULONGLONG,
}}
pub type PSDP_ULARGE_INTEGER_16 = *mut SDP_ULARGE_INTEGER_16;
pub type LPSDP_ULARGE_INTEGER_16 = *mut SDP_ULARGE_INTEGER_16;
pub type PSDP_LARGE_INTEGER_16 = *mut SDP_LARGE_INTEGER_16;
pub type LPSDP_LARGE_INTEGER_16 = *mut SDP_LARGE_INTEGER_16;
ENUM!{enum NodeContainerType {
    NodeContainerTypeSequence,
    NodeContainerTypeAlternative,
}}
pub type SDP_ERROR = USHORT;
pub type PSDP_ERROR = *mut USHORT;
ENUM!{enum SDP_TYPE {
    SDP_TYPE_NIL = 0x00,
    SDP_TYPE_UINT = 0x01,
    SDP_TYPE_INT = 0x02,
    SDP_TYPE_UUID = 0x03,
    SDP_TYPE_STRING = 0x04,
    SDP_TYPE_BOOLEAN = 0x05,
    SDP_TYPE_SEQUENCE = 0x06,
    SDP_TYPE_ALTERNATIVE = 0x07,
    SDP_TYPE_URL = 0x08,
    SDP_TYPE_CONTAINER = 0x20,
}}
ENUM!{enum SDP_SPECIFICTYPE {
    SDP_ST_NONE = 0x0000,
    SDP_ST_UINT8 = 0x0010,
    SDP_ST_UINT16 = 0x0110,
    SDP_ST_UINT32 = 0x0210,
    SDP_ST_UINT64 = 0x0310,
    SDP_ST_UINT128 = 0x0410,
    SDP_ST_INT8 = 0x0020,
    SDP_ST_INT16 = 0x0120,
    SDP_ST_INT32 = 0x0220,
    SDP_ST_INT64 = 0x0320,
    SDP_ST_INT128 = 0x0420,
    SDP_ST_UUID16 = 0x0130,
    SDP_ST_UUID32 = 0x0220,
    SDP_ST_UUID128 = 0x0430,
}}
STRUCT!{struct SdpAttributeRange {
    minAttribute: USHORT,
    maxAttribute: USHORT,
}}
UNION!{union SdpQueryUuidUnion {
    [u32; 4],
    uuid128 uuid128_mut: GUID,
    uuid32 uuid32_mut: ULONG,
    uuid16 uuid16_mut: USHORT,
}}
STRUCT!{struct SdpQueryUuid {
    u: SdpQueryUuidUnion,
    uuidType: USHORT,
}}
