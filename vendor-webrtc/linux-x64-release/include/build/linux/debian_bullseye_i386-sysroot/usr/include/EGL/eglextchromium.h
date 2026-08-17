// Copyright (c) 2013 The Chromium Authors. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//    * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//    * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//    * Neither the name of Google Inc. nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

// This file contains Chromium-specific EGL extensions declarations.

#ifndef GPU_EGL_EGLEXTCHROMIUM_H_
#define GPU_EGL_EGLEXTCHROMIUM_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <EGL/eglplatform.h>

/* EGLSyncControlCHROMIUM requires 64-bit uint support */
#if KHRONOS_SUPPORT_INT64
#ifndef EGL_CHROMIUM_sync_control
#define EGL_CHROMIUM_sync_control 1
typedef khronos_uint64_t EGLuint64CHROMIUM;
#ifdef EGL_EGLEXT_PROTOTYPES
EGLAPI EGLBoolean EGLAPIENTRY eglGetSyncValuesCHROMIUM(
    EGLDisplay dpy, EGLSurface surface, EGLuint64CHROMIUM *ust,
    EGLuint64CHROMIUM *msc, EGLuint64CHROMIUM *sbc);
#endif /* EGL_EGLEXT_PROTOTYPES */
typedef EGLBoolean (EGLAPIENTRYP PFNEGLGETSYNCVALUESCHROMIUMPROC)
    (EGLDisplay dpy, EGLSurface surface, EGLuint64CHROMIUM *ust,
     EGLuint64CHROMIUM *msc, EGLuint64CHROMIUM *sbc);
#endif
#endif

#ifndef EGL_EXT_image_flush_external
#define EGL_EXT_image_flush_external 1
#define EGL_IMAGE_EXTERNAL_FLUSH_EXT 0x32A2
typedef EGLBoolean (EGLAPIENTRYP PFNEGLIMAGEFLUSHEXTERNALEXTPROC) (EGLDisplay dpy, EGLImageKHR image, const EGLAttrib *attrib_list);
typedef EGLBoolean (EGLAPIENTRYP PFNEGLIMAGEINVALIDATEEXTERNALEXTPROC) (EGLDisplay dpy, EGLImageKHR image, const EGLAttrib *attrib_list);
#ifdef EGL_EGLEXT_PROTOTYPES
EGLAPI EGLBoolean EGLAPIENTRY eglImageFlushExternalEXT (EGLDisplay dpy, EGLImageKHR image, const EGLAttrib *attrib_list);
EGLAPI EGLBoolean EGLAPIENTRY eglImageInvalidateExternalEXT (EGLDisplay dpy, EGLImageKHR image, const EGLAttrib *attrib_list);
#endif
#endif /* EGL_EXT_image_flush_external */

#ifdef __cplusplus
}
#endif

#endif // GPU_EGL_EGLEXTCHROMIUM_H_
