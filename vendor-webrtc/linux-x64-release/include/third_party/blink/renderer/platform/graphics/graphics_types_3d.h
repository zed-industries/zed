/*
 * Copyright (C) 2011 Google Inc.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GRAPHICS_TYPES_3D_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GRAPHICS_TYPES_3D_H_

#ifndef __glext_h_
#include "third_party/khronos/GLES2/gl2ext.h"
#include "third_party/khronos/GLES3/gl3.h"
#endif

#include <stdint.h>
#include "third_party/blink/renderer/platform/wtf/forward.h"

// WebGL-specific enums
const unsigned GC3D_UNPACK_FLIP_Y_WEBGL = 0x9240;
const unsigned GC3D_UNPACK_PREMULTIPLY_ALPHA_WEBGL = 0x9241;
const unsigned GC3D_CONTEXT_LOST_WEBGL = 0x9242;
const unsigned GC3D_UNPACK_COLORSPACE_CONVERSION_WEBGL = 0x9243;
const unsigned GC3D_BROWSER_DEFAULT_WEBGL = 0x9244;
const unsigned GC3D_MAX_CLIENT_WAIT_TIMEOUT_WEBGL = 0x9247;

// GL_ARB_texture_rectangle
const unsigned GC3D_TEXTURE_RECTANGLE_ARB = 0x84F5;

// GL_AMD_compressed_ATC_texture
const unsigned GC3D_COMPRESSED_ATC_RGB_AMD = 0x8C92;
const unsigned GC3D_COMPRESSED_ATC_RGBA_EXPLICIT_ALPHA_AMD = 0x8C93;
const unsigned GC3D_COMPRESSED_ATC_RGBA_INTERPOLATED_ALPHA_AMD = 0x87EE;

namespace blink {

enum SourceDrawingBuffer { kFrontBuffer, kBackBuffer };

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GRAPHICS_TYPES_3D_H_
