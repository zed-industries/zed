/// The fill tessellator's result type.
pub type TessellationResult = Result<(), TessellationError>;

/// An error that can happen while generating geometry.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum GeometryBuilderError {
    InvalidVertex,
    TooManyVertices,
}

#[cfg(feature = "std")]
impl core::fmt::Display for GeometryBuilderError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GeometryBuilderError::InvalidVertex => {
                std::write!(f, "Invalid vertex")
            },
            GeometryBuilderError::TooManyVertices => {
                std::write!(f, "Too many vertices")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GeometryBuilderError {}

/// Describes an unexpected error happening during tessellation.
///
/// If you run into one of these, please consider
/// [filing an issue](https://github.com/nical/lyon/issues/new).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum InternalError {
    IncorrectActiveEdgeOrder(i16),
    InsufficientNumberOfSpans,
    InsufficientNumberOfEdges,
    MergeVertexOutside,
    InvalidNumberOfEdgesBelowVertex,
    ErrorCode(i16),
}

#[cfg(feature = "std")]
impl core::fmt::Display for InternalError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            InternalError::IncorrectActiveEdgeOrder(i) => {
                std::write!(f, "Incorrect active edge order ({i})")
            },
            InternalError::InsufficientNumberOfSpans => {
                std::write!(f, "Insufficient number of spans")
            },
            InternalError::InsufficientNumberOfEdges => {
                std::write!(f, "Insufficient number of edges")
            },
            InternalError::MergeVertexOutside => {
                std::write!(f, "Merge vertex is outside of the shape")
            },
            InternalError::InvalidNumberOfEdgesBelowVertex => {
                std::write!(f, "Unexpected number of edges below a vertex")
            },
            InternalError::ErrorCode(i) => {
                std::write!(f, "Error code: #{i}")
            },
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InternalError {}

/// The fill tessellator's error enumeration.
#[derive(Clone, Debug, PartialEq)]
pub enum TessellationError {
    // TODO Parameter typo
    UnsupportedParamater(UnsupportedParamater),
    GeometryBuilder(GeometryBuilderError),
    Internal(InternalError),
}

#[cfg(feature = "std")]
impl core::fmt::Display for TessellationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TessellationError::UnsupportedParamater(e) => {
                std::write!(f, "Unsupported parameter: {e}")
            },
            TessellationError::GeometryBuilder(e) => {
                std::write!(f, "Geometry builder error: {e}")
            },
            TessellationError::Internal(e) => {
                std::write!(f, "Internal error: {e}")
            },
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TessellationError {}

impl core::convert::From<GeometryBuilderError> for TessellationError {
    fn from(value: GeometryBuilderError) -> Self {
        Self::GeometryBuilder(value)
    }
}

impl core::convert::From<InternalError> for TessellationError {
    fn from(value: InternalError) -> Self {
        Self::Internal(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnsupportedParamater {
    PositionIsNaN,
    ToleranceIsNaN,
}

#[cfg(feature = "std")]
impl core::fmt::Display for UnsupportedParamater {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            UnsupportedParamater::PositionIsNaN => {
                std::write!(f, "Position is not a number")
            },
            UnsupportedParamater::ToleranceIsNaN => {
                std::write!(f, "Tolerance threshold is not a number")
            },
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for UnsupportedParamater {}
