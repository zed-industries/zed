// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_CONTEXT_ATTRIBUTE_HELPERS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_CONTEXT_ATTRIBUTE_HELPERS_H_

#include "third_party/blink/public/platform/platform.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_webgl_context_attributes.h"
#include "third_party/blink/renderer/core/html/canvas/canvas_context_creation_attributes_core.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace gl {
enum class GpuPreference;
}

namespace blink {

WebGLContextAttributes* ToWebGLContextAttributes(
    const CanvasContextCreationAttributesCore&);

// Set up the attributes that can be used to create a GL context via the
// Platform API.
Platform::ContextAttributes ToPlatformContextAttributes(
    const CanvasContextCreationAttributesCore&,
    Platform::ContextType context_type);

// Turns the powerPreference context creation attribute into the
// gl::GpuPreference enum which is sent along with GPU switching
// notifications. This must not return the kDefault constant, but
// choose either the low-power or high-performance GPU.
gl::GpuPreference PowerPreferenceToGpuPreference(
    CanvasContextCreationAttributesCore::PowerPreference power_preference);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_CONTEXT_ATTRIBUTE_HELPERS_H_
