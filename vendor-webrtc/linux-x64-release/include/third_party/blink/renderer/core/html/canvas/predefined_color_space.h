// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_PREDEFINED_COLOR_SPACE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_PREDEFINED_COLOR_SPACE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/graphics/predefined_color_space.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "ui/gfx/hdr_metadata.h"

namespace blink {

class CanvasHighDynamicRangeOptions;
class V8PredefinedColorSpace;

// Convert from a V8PredefinedColorSpace to a PredefinedColorSpace. Note that
// some values for PredefinedColorSpace are specified in the IDL but are
// supposed to be guarded behind the CanvasFloatingPoint and CanvasHDR features.
// This function will return false and throw an exception into `exception_state`
// if `color_space` is unsupported because its runtime flag is not enabled.
bool CORE_EXPORT
ValidateAndConvertColorSpace(const V8PredefinedColorSpace& v8_color_space,
                             PredefinedColorSpace& color_space,
                             ExceptionState& exception_state);

// Convert from a PredefinedColorSpace to a V8PredefinedColorSpace.
V8PredefinedColorSpace CORE_EXPORT
PredefinedColorSpaceToV8(PredefinedColorSpace color_space);

// Convert from CanvasHighDynamicRangeOptions to gfx::HDRMetadata.
void CORE_EXPORT
ParseCanvasHighDynamicRangeOptions(const CanvasHighDynamicRangeOptions* options,
                                   gfx::HDRMetadata& hdr_metadata);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_PREDEFINED_COLOR_SPACE_H_
