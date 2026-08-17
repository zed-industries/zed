// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use block::Block;

use super::NSUInteger;

type DrawablePresentedHandler<'a> = Block<(&'a DrawableRef,), ()>;
type CFTimeInterval = f64;

/// See <https://developer.apple.com/documentation/metal/mtldrawable>
pub enum MTLDrawable {}

foreign_obj_type! {
    type CType = MTLDrawable;
    pub struct Drawable;
}

impl DrawableRef {
    pub fn present(&self) {
        unsafe { msg_send![self, present] }
    }

    pub fn drawable_id(&self) -> NSUInteger {
        unsafe { msg_send![self, drawableID] }
    }

    pub fn add_presented_handler(&self, block: &DrawablePresentedHandler) {
        unsafe { msg_send![self, addPresentedHandler: block] }
    }

    pub fn presented_time(&self) -> CFTimeInterval {
        unsafe { msg_send![self, presentedTime] }
    }
}
