#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechRecognitionUIStatus(pub i32);
impl SpeechRecognitionUIStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const Busy: Self = Self(1i32);
    pub const Cancelled: Self = Self(2i32);
    pub const Preempted: Self = Self(3i32);
    pub const PrivacyPolicyDeclined: Self = Self(4i32);
}
impl windows_core::TypeKind for SpeechRecognitionUIStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechRecognitionUIStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechRecognitionUIStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SpeechRecognitionUIStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Speech.Recognition.SpeechRecognitionUIStatus;i4)");
}
