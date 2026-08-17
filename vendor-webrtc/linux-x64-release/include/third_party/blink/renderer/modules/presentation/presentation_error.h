// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_ERROR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_ERROR_H_

#include "third_party/blink/public/mojom/presentation/presentation.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/dom/dom_exception.h"

namespace blink {

// Creates a DOMException using the given PresentationError.
v8::Local<v8::Value> CreatePresentationError(
    v8::Isolate*,
    const mojom::blink::PresentationError&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_ERROR_H_
