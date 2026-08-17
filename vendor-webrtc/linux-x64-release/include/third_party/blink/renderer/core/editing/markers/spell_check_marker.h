// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_SPELL_CHECK_MARKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_SPELL_CHECK_MARKER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/markers/document_marker.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

// A subclass of DocumentMarker used to implement functionality shared between
// spelling and grammar markers. These two marker types both store a
// description string that can contain suggested replacements for a misspelling
// or grammar error.
class CORE_EXPORT SpellCheckMarker : public DocumentMarker {
 public:
  SpellCheckMarker(unsigned start_offset,
                   unsigned end_offset,
                   const String& description);
  SpellCheckMarker(const SpellCheckMarker&) = delete;
  SpellCheckMarker& operator=(const SpellCheckMarker&) = delete;

  const String& Description() const { return description_; }

 private:
  const String description_;
};

bool CORE_EXPORT IsSpellCheckMarker(const DocumentMarker&);

template <>
struct DowncastTraits<SpellCheckMarker> {
  static bool AllowFrom(const DocumentMarker& marker) {
    return IsSpellCheckMarker(marker);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_SPELL_CHECK_MARKER_H_
