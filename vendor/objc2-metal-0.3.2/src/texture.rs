#![allow(non_upper_case_globals)]
use crate::{MTLTextureSwizzle, MTLTextureSwizzleChannels};

pub const MTLTextureSwizzleChannelsDefault: MTLTextureSwizzleChannels = MTLTextureSwizzleChannels {
    red: MTLTextureSwizzle::Red,
    green: MTLTextureSwizzle::Green,
    blue: MTLTextureSwizzle::Blue,
    alpha: MTLTextureSwizzle::Alpha,
};
