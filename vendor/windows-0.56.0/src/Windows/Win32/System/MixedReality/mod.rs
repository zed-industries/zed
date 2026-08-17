pub const PERCEPTIONFIELD_StateStream_TimeStamps: windows_core::GUID = windows_core::GUID::from_u128(0xaa886119_f32f_49bf_92ca_f9ddf784d297);
#[repr(C)]
pub struct PERCEPTION_PAYLOAD_FIELD {
    pub FieldId: windows_core::GUID,
    pub OffsetInBytes: u32,
    pub SizeInBytes: u32,
}
impl Copy for PERCEPTION_PAYLOAD_FIELD {}
impl Clone for PERCEPTION_PAYLOAD_FIELD {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for PERCEPTION_PAYLOAD_FIELD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PERCEPTION_PAYLOAD_FIELD").field("FieldId", &self.FieldId).field("OffsetInBytes", &self.OffsetInBytes).field("SizeInBytes", &self.SizeInBytes).finish()
    }
}
impl windows_core::TypeKind for PERCEPTION_PAYLOAD_FIELD {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for PERCEPTION_PAYLOAD_FIELD {
    fn eq(&self, other: &Self) -> bool {
        self.FieldId == other.FieldId && self.OffsetInBytes == other.OffsetInBytes && self.SizeInBytes == other.SizeInBytes
    }
}
impl Eq for PERCEPTION_PAYLOAD_FIELD {}
impl Default for PERCEPTION_PAYLOAD_FIELD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct PERCEPTION_STATE_STREAM_TIMESTAMPS {
    pub InputTimestampInQpcCounts: i64,
    pub AvailableTimestampInQpcCounts: i64,
}
impl Copy for PERCEPTION_STATE_STREAM_TIMESTAMPS {}
impl Clone for PERCEPTION_STATE_STREAM_TIMESTAMPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for PERCEPTION_STATE_STREAM_TIMESTAMPS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PERCEPTION_STATE_STREAM_TIMESTAMPS").field("InputTimestampInQpcCounts", &self.InputTimestampInQpcCounts).field("AvailableTimestampInQpcCounts", &self.AvailableTimestampInQpcCounts).finish()
    }
}
impl windows_core::TypeKind for PERCEPTION_STATE_STREAM_TIMESTAMPS {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for PERCEPTION_STATE_STREAM_TIMESTAMPS {
    fn eq(&self, other: &Self) -> bool {
        self.InputTimestampInQpcCounts == other.InputTimestampInQpcCounts && self.AvailableTimestampInQpcCounts == other.AvailableTimestampInQpcCounts
    }
}
impl Eq for PERCEPTION_STATE_STREAM_TIMESTAMPS {}
impl Default for PERCEPTION_STATE_STREAM_TIMESTAMPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
