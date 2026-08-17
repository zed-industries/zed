pub type CVReturn = i32;

pub const kCVReturnSuccess: CVReturn = 0;
pub const kCVReturnFirst: CVReturn = -6660;
pub const kCVReturnError: CVReturn = kCVReturnFirst;
pub const kCVReturnInvalidArgument: CVReturn = -6661;
pub const kCVReturnAllocationFailed: CVReturn = -6662;
pub const kCVReturnUnsupported: CVReturn = -6663;

// DisplayLink related errors
pub const kCVReturnInvalidDisplay: CVReturn = -6670;
pub const kCVReturnDisplayLinkAlreadyRunning: CVReturn = -6671;
pub const kCVReturnDisplayLinkNotRunning: CVReturn = -6672;
pub const kCVReturnDisplayLinkCallbacksNotSet: CVReturn = -6673;

// Buffer related errors
pub const kCVReturnInvalidPixelFormat: CVReturn = -6680;
pub const kCVReturnInvalidSize: CVReturn = -6681;
pub const kCVReturnInvalidPixelBufferAttributes: CVReturn = -6682;
pub const kCVReturnPixelBufferNotOpenGLCompatible: CVReturn = -6683;
pub const kCVReturnPixelBufferNotMetalCompatible: CVReturn = -6684;

// Buffer Pool related errors
pub const kCVReturnWouldExceedAllocationThreshold: CVReturn = -6689;
pub const kCVReturnPoolAllocationFailed: CVReturn = -6690;
pub const kCVReturnInvalidPoolAttributes: CVReturn = -6691;
pub const kCVReturnRetry: CVReturn = -6692;

pub const kCVReturnLast: CVReturn = -669;
