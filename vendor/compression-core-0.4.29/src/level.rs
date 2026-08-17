/// Level of compression data should be compressed with.
#[non_exhaustive]
#[derive(Default, Clone, Copy, Debug)]
pub enum Level {
    /// Fastest quality of compression, usually produces bigger size.
    Fastest,

    /// Best quality of compression, usually produces the smallest size.
    Best,

    /// Default quality of compression defined by the selected compression algorithm.
    #[default]
    Default,

    /// Precise quality based on the underlying compression algorithms' qualities. The
    /// interpretation of this depends on the algorithm chosen and the specific implementation
    /// backing it. Qualities are implicitly clamped to the algorithm's maximum.
    Precise(i32),
}
