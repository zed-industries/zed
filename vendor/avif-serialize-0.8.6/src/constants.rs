/// `Bt709` works for sRGB images.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ColorPrimaries {
    /// Rec.709 and sRGB
    Bt709 = 1,
    Unspecified = 2,
    /// ITU-R BT601-6 525
    Bt601 = 6,
    /// ITU-R BT2020
    Bt2020 = 9,
    /// SMPTE ST 431-2. NB: "P3" images use DisplayP3 instead.
    DciP3 = 11,
    /// SMPTE ST 432-1
    DisplayP3 = 12,
}

/// This controls how color data is interpreted (gamma).
///
/// If you don't know what to do with these, pick `Srgb`.
///
/// Reasonable options include `Bt709` (HDTV), `Bt2020_10` (Wide Gamut), `Smpte2084`, `Hlg` (HDR).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TransferCharacteristics {
    /// Rec.709. May be appropriate for conversions from video.
    Bt709 = 1,
    /// Don't use this for color channels.
    Unspecified = 2,
    /// Don't use this. Analog NTSC TV. BT.470 System M (historical)
    #[deprecated(note = "This is obsolete. Please don't proliferate legacy baggage.")]
    #[doc(hidden)]
    Bt470M = 4,
    /// Don't use this. Analog PAL TV. BT.470 System B, G (historical)
    #[deprecated(note = "This is obsolete. Please don't proliferate legacy baggage.")]
    #[doc(hidden)]
    Bt470BG = 5,
    /// ITU-R BT601-6 525. Not recommended, unless you're converting from unlabelled low-res video clips.
    /// See `Bt709` and `Srgb`.
    Bt601 = 6,
    /// Don't use this. SMPTE 240 M. It's just a worse Rec.709.
    Smpte240 = 7,
    /// "Linear transfer characteristics"
    Linear = 8,
    /// "Logarithmic transfer characteristic (100:1 range)"
    Log = 9,
    /// "Logarithmic transfer characteristic (100 * Sqrt(10) : 1 range)"
    LogSqrt = 10,
    /// IEC 61966-2-4
    Iec61966 = 11,
    /// Don't use this. Obsoleted BT.1361 extended color gamut system (historical)
    #[deprecated(note = "This is obsolete. Please don't proliferate legacy baggage.")]
    #[doc(hidden)]
    Bt1361 = 12,
    /// sRGB. This is the safe choice for encoding "standard" RGB images, especially 8-bit inputs.
    Srgb = 13,
    /// ITU-R BT2020 for 10-bit system. Reasonable for encoding wide gamut.
    Bt2020_10 = 14,
    /// ITU-R BT2020 for 12-bit system
    Bt2020_12 = 15,
    /// SMPTE ST 2084, ITU BT.2100 PQ
    Smpte2084 = 16,
    /// SMPTE ST 428. Not recommended. Overkill for images. Use `Bt2020_10` instead.
    Smpte428 = 17,
    /// BT.2100 HLG (Hybrid Log Gamma), ARIB STD-B67
    Hlg = 18,
}

/// This is the format of color channels.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MatrixCoefficients {
    /// GBR (sRGB). This isn't actually good for most RGB images. Use `Bt709` for lossy and `Ycgco` for lossless.
    Rgb = 0,
    /// ITU-R BT1361
    Bt709 = 1,
    Unspecified = 2,
    /// ITU-R BT601-6 525. This matches luma in JPEG's YCbCr when used with sRGB transfer characteristics, but is a bit off for chroma.
    Bt601 = 6,
    Ycgco = 8,
    /// ITU-R BT2020 non-constant luminance system
    Bt2020Ncl = 9,
    /// ITU-R BT2020 constant luminance system
    Bt2020Cl = 10,
}
