// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_HTMLCANVAS_CANVAS_CONTEXT_CREATION_ATTRIBUTES_HELPERS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_HTMLCANVAS_CANVAS_CONTEXT_CREATION_ATTRIBUTES_HELPERS_H_

#include "third_party/blink/renderer/platform/bindings/exception_state.h"

namespace blink {

class CanvasContextCreationAttributesCore;
class CanvasContextCreationAttributesModule;
class CanvasRenderingContext2DSettings;

bool ToCanvasContextCreationAttributes(
    const CanvasContextCreationAttributesModule*,
    CanvasContextCreationAttributesCore& result,
    ExceptionState& exception_state);

CanvasRenderingContext2DSettings* ToCanvasRenderingContext2DSettings(
    const CanvasContextCreationAttributesCore&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_HTMLCANVAS_CANVAS_CONTEXT_CREATION_ATTRIBUTES_HELPERS_H_
