//! Types that are useful for FFI bindings to `wgpu`.

use crate::{command::IdReferences, id};

pub type TexelCopyBufferInfo = wgt::TexelCopyBufferInfo<id::BufferId>;
pub type TexelCopyTextureInfo = wgt::TexelCopyTextureInfo<id::TextureId>;
pub type CopyExternalImageDestInfo = wgt::CopyExternalImageDestInfo<id::TextureId>;

pub type Command = super::Command<IdReferences>;
