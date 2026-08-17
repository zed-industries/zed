// #if __LP64__ || NS_BUILD_32_LIKE_64
// This is the same as NSInteger/NSUInteger, so let's use the same definition
// as those (namely isize/usize).

/// [Apple's documentation](https://developer.apple.com/documentation/coreaudiotypes/avaudiointeger?language=objc)
pub type AVAudioInteger = isize;

/// [Apple's documentation](https://developer.apple.com/documentation/coreaudiotypes/avaudiouinteger?language=objc)
pub type AVAudioUInteger = usize;
