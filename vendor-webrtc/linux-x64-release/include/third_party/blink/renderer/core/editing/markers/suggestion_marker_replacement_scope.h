// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_SUGGESTION_MARKER_REPLACEMENT_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_SUGGESTION_MARKER_REPLACEMENT_SCOPE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class CORE_EXPORT SuggestionMarkerReplacementScope {
  STACK_ALLOCATED();

 public:
  SuggestionMarkerReplacementScope();
  SuggestionMarkerReplacementScope(const SuggestionMarkerReplacementScope&) =
      delete;
  SuggestionMarkerReplacementScope& operator=(
      const SuggestionMarkerReplacementScope&) = delete;
  ~SuggestionMarkerReplacementScope();

  static bool CurrentlyInScope();

 private:
  static bool currently_in_scope_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_SUGGESTION_MARKER_REPLACEMENT_SCOPE_H_
