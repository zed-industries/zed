// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HIGHLIGHT_CSS_HIGHLIGHT_REGISTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HIGHLIGHT_CSS_HIGHLIGHT_REGISTRY_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/highlight/highlight_registry.h"

namespace blink {

class HighlightRegistry;

class CORE_EXPORT CSSHighlightRegistry {
  STATIC_ONLY(CSSHighlightRegistry);

 public:
  static HighlightRegistry* highlights(ScriptState*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HIGHLIGHT_CSS_HIGHLIGHT_REGISTRY_H_
