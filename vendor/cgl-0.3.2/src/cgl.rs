// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Mac-specific OpenGL bindings.

#![allow(non_upper_case_globals)]

use libc::{c_void, c_int, c_char};
use std::os::raw;

pub type GLenum = raw::c_uint;
pub type GLint = raw::c_int;
pub type GLsizei = raw::c_int;
pub type GLuint = raw::c_uint;

pub type CGLPixelFormatAttribute = c_int;
pub type CGLContextParameter = c_int;
pub type CGLContextEnable = c_int;
pub type CGLGlobalOption = c_int;
pub type CGLError = c_int;

pub type CGLPixelFormatObj = *mut c_void;
pub type CGLContextObj = *mut c_void;
pub type CGLShareGroupObj = *mut c_void;
pub type IOSurfaceRef = *mut c_void;

pub const kCGLNoError: CGLError = 0;

pub const kCGLPFAAllRenderers: CGLPixelFormatAttribute = 1;
pub const kCGLPFADoubleBuffer: CGLPixelFormatAttribute = 5;
pub const kCGLPFAStereo: CGLPixelFormatAttribute = 6;
pub const kCGLPFAAuxBuffers: CGLPixelFormatAttribute = 7;
pub const kCGLPFAColorSize: CGLPixelFormatAttribute = 8;
pub const kCGLPFAAlphaSize: CGLPixelFormatAttribute = 11;
pub const kCGLPFADepthSize: CGLPixelFormatAttribute = 12;
pub const kCGLPFAStencilSize: CGLPixelFormatAttribute = 13;
pub const kCGLPFAAccumSize: CGLPixelFormatAttribute = 14;
pub const kCGLPFAMinimumPolicy: CGLPixelFormatAttribute = 51;
pub const kCGLPFAMaximumPolicy: CGLPixelFormatAttribute = 52;
pub const kCGLPFAOffScreen: CGLPixelFormatAttribute = 53;
pub const kCGLPFAFullScreen: CGLPixelFormatAttribute = 54;
pub const kCGLPFASampleBuffers: CGLPixelFormatAttribute = 55;
pub const kCGLPFASamples: CGLPixelFormatAttribute = 56;
pub const kCGLPFAAuxDepthStencil: CGLPixelFormatAttribute = 57;
pub const kCGLPFAColorFloat: CGLPixelFormatAttribute = 58;
pub const kCGLPFAMultisample: CGLPixelFormatAttribute = 59;
pub const kCGLPFASupersample: CGLPixelFormatAttribute = 60;
pub const kCGLPFASampleAlpha: CGLPixelFormatAttribute = 61;
pub const kCGLPFARendererID: CGLPixelFormatAttribute = 70;
pub const kCGLPFASingleRenderer: CGLPixelFormatAttribute = 71;
pub const kCGLPFANoRecovery: CGLPixelFormatAttribute = 72;
pub const kCGLPFAAccelerated: CGLPixelFormatAttribute = 73;
pub const kCGLPFAClosestPolicy: CGLPixelFormatAttribute = 74;
pub const kCGLPFARobust: CGLPixelFormatAttribute = 75;
pub const kCGLPFABackingStore: CGLPixelFormatAttribute = 76;
pub const kCGLPFAMPSafe: CGLPixelFormatAttribute = 78;
pub const kCGLPFAWindow: CGLPixelFormatAttribute = 80;
pub const kCGLPFAMultiScreen: CGLPixelFormatAttribute = 81;
pub const kCGLPFACompliant: CGLPixelFormatAttribute = 83;
pub const kCGLPFADisplayMask: CGLPixelFormatAttribute = 84;
pub const kCGLPFAPBuffer: CGLPixelFormatAttribute = 90;
pub const kCGLPFARemotePBuffer: CGLPixelFormatAttribute = 91;
pub const kCGLPFAAllowOfflineRenderers: CGLPixelFormatAttribute = 96;
pub const kCGLPFAAcceleratedCompute: CGLPixelFormatAttribute = 97;
pub const kCGLPFAOpenGLProfile: CGLPixelFormatAttribute = 99;
pub const kCGLPFAVirtualScreenCount: CGLPixelFormatAttribute = 128;

pub const kCGLCESwapRectangle: CGLContextEnable = 201;
pub const kCGLCESwapLimit: CGLContextEnable = 203;
pub const kCGLCERasterization: CGLContextEnable = 221;
pub const kCGLCEStateValidation: CGLContextEnable = 301;
pub const kCGLCESurfaceBackingSize: CGLContextEnable = 305;
pub const kCGLCEDisplayListOptimization: CGLContextEnable = 307;
pub const kCGLCEMPEngine: CGLContextEnable = 313;
pub const kCGLCECrashOnRemovedFunctions: CGLContextEnable = 316;

pub const kCGLCPSwapRectangle: CGLContextParameter = 200;
pub const kCGLCPSwapInterval: CGLContextParameter = 222;
pub const kCGLCPDispatchTableSize: CGLContextParameter = 224;
pub const kCGLCPClientStorage: CGLContextParameter = 226;
pub const kCGLCPSurfaceTexture: CGLContextParameter = 228;
pub const kCGLCPSurfaceOrder: CGLContextParameter = 235;
pub const kCGLCPSurfaceOpacity: CGLContextParameter = 236;
pub const kCGLCPSurfaceBackingSize: CGLContextParameter = 304;
pub const kCGLCPSurfaceSurfaceVolatile: CGLContextParameter = 306;
pub const kCGLCPReclaimResources: CGLContextParameter = 308;
pub const kCGLCPCurrentRendererID: CGLContextParameter = 309;
pub const kCGLCPGPUVertexProcessing: CGLContextParameter = 310;
pub const kCGLCPGPUFragmentProcessing: CGLContextParameter = 311;
pub const kCGLCPHasDrawable: CGLContextParameter = 314;
pub const kCGLCPMPSwapsInFlight: CGLContextParameter = 315;

pub const kCGLGOFormatCacheSize: CGLGlobalOption = 501;
pub const kCGLGOClearFormatCache: CGLGlobalOption = 502;
pub const kCGLGORetainRenderers: CGLGlobalOption = 503;
pub const kCGLGOResetLibrary: CGLGlobalOption = 504;
pub const kCGLGOUseErrorHandler: CGLGlobalOption = 505;
pub const kCGLGOUseBuildCache: CGLGlobalOption = 506;

pub const CORE_BOOLEAN_ATTRIBUTES: &'static [CGLPixelFormatAttribute] =
    &[kCGLPFAAllRenderers,
      kCGLPFADoubleBuffer,
      kCGLPFAStereo,
      kCGLPFAAuxBuffers,
      kCGLPFAMinimumPolicy,
      kCGLPFAMaximumPolicy,
      kCGLPFAOffScreen,
      kCGLPFAFullScreen,
      kCGLPFAAuxDepthStencil,
      kCGLPFAColorFloat,
      kCGLPFAMultisample,
      kCGLPFASupersample,
      kCGLPFASampleAlpha,
      kCGLPFASingleRenderer,
      kCGLPFANoRecovery,
      kCGLPFAAccelerated,
      kCGLPFAClosestPolicy,
      kCGLPFARobust,
      kCGLPFABackingStore,
      kCGLPFAMPSafe,
      kCGLPFAWindow,
      kCGLPFAMultiScreen,
      kCGLPFACompliant,
      kCGLPFAPBuffer,
      kCGLPFARemotePBuffer,
      kCGLPFAAllowOfflineRenderers,
      kCGLPFAAcceleratedCompute];

pub const CORE_INTEGER_ATTRIBUTES: &'static [CGLPixelFormatAttribute] =
    &[kCGLPFAColorSize,
      kCGLPFAAlphaSize,
      kCGLPFADepthSize,
      kCGLPFAStencilSize,
      kCGLPFAAccumSize,
      kCGLPFASampleBuffers,
      kCGLPFASamples,
      kCGLPFARendererID,
      kCGLPFADisplayMask,
      kCGLPFAOpenGLProfile,
      kCGLPFAVirtualScreenCount];

#[link(name = "OpenGL", kind = "framework")]
extern {
    // CGLCurrent.h

    pub fn CGLSetCurrentContext(ctx: CGLContextObj) -> CGLError;
    pub fn CGLGetCurrentContext() -> CGLContextObj;
    pub fn CGLGetShareGroup(context: CGLContextObj) -> CGLShareGroupObj;

    // OpenGL.h

    // Pixel format functions
    pub fn CGLChoosePixelFormat(attribs: *const CGLPixelFormatAttribute,
                                pix: *mut CGLPixelFormatObj,
                                npix: *mut GLint) -> CGLError;
    pub fn CGLDescribePixelFormat(pix: CGLPixelFormatObj,
                                  pix_num: GLint,
                                  attrib: CGLPixelFormatAttribute,
                                  value: *mut GLint) -> CGLError;
    pub fn CGLDestroyPixelFormat(pix: CGLPixelFormatObj) -> CGLError;
    pub fn CGLReleasePixelFormat(pix: CGLPixelFormatObj);
    pub fn CGLRetainPixelFormat(pix: CGLPixelFormatObj) -> CGLPixelFormatObj;
    pub fn CGLGetPixelFormatRetainCount(pix: CGLPixelFormatObj) -> GLuint;

    // Context functions
    pub fn CGLCreateContext(pix: CGLPixelFormatObj, share: CGLContextObj, ctx: *mut CGLContextObj) ->
                            CGLError;
    pub fn CGLDestroyContext(ctx: CGLContextObj) -> CGLError;
    pub fn CGLGetPixelFormat(ctx: CGLContextObj) -> CGLPixelFormatObj;

    // Getting and Setting Context Options
    pub fn CGLEnable(ctx: CGLContextObj, pname: CGLContextEnable) -> CGLError;
    pub fn CGLDisable(ctx: CGLContextObj, pname: CGLContextEnable) -> CGLError;
    pub fn CGLIsEnabled(ctx: CGLContextObj, pname: CGLContextEnable, enable: &mut GLint) -> CGLError;
    pub fn CGLSetParameter(ctx: CGLContextObj, pname: CGLContextParameter, params: &GLint) -> CGLError;
    pub fn CGLGetParameter(ctx: CGLContextObj, pname: CGLContextParameter, params: &mut GLint) -> CGLError;

    // Locking functions
    pub fn CGLLockContext(ctx: CGLContextObj) -> CGLError;
    pub fn CGLUnlockContext(ctx: CGLContextObj) -> CGLError;

    // Getting and Setting Global Information
    pub fn CGLSetOption(pname: CGLGlobalOption, param: &GLint) -> CGLError;
    pub fn CGLGetOption(pname: CGLGlobalOption, param: &mut GLint) -> CGLError;
    pub fn CGLSetGlobalOption(pname: CGLGlobalOption, param: &GLint) -> CGLError;
    pub fn CGLGetGlobalOption(pname: CGLGlobalOption, param: &mut GLint) -> CGLError;
    pub fn CGLGetVersion (major: &mut GLint, minor: &mut GLint) -> CGLError;

    // CGLIOSurface.h

    pub fn CGLTexImageIOSurface2D(ctx: CGLContextObj, target: GLenum, internal_format: GLenum,
                                  width: GLsizei, height: GLsizei, format: GLenum, ty: GLenum,
                                  ioSurface: IOSurfaceRef, plane: GLuint) -> CGLError;

    // https://developer.apple.com/library/mac/documentation/GraphicsImaging/Reference/CGL_OpenGL/#//apple_ref/c/func/CGLErrorString

    pub fn CGLErrorString(error: CGLError) -> *const c_char;
}
