use quick_error::quick_error;

#[derive(Debug)]
#[doc(hidden)]
pub struct EncodingErrorDetail; // maybe later

quick_error! {
    /// Failures enum
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Error {
        /// Slices given to `encode_raw_planes` must be `width * height` large.
        TooFewPixels {
            display("Provided buffer is smaller than width * height")
        }
        Unsupported(msg: &'static str) {
            display("Not supported: {}", msg)
        }
        EncodingError(e: EncodingErrorDetail) {
            display("Encoding error reported by rav1e")
            from(_e: rav1e::InvalidConfig) -> (EncodingErrorDetail)
            from(_e: rav1e::EncoderStatus) -> (EncodingErrorDetail)
        }
    }
}
