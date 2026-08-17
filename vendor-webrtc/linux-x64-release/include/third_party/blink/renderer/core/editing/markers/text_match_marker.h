/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_TEXT_MATCH_MARKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_TEXT_MATCH_MARKER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/markers/document_marker.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

// A subclass of DocumentMarker used to store information specific to TextMatch
// markers. We store whether or not the match is active, a PhysicalRect used for
// rendering the marker, and whether or not the PhysicalRect is currently
// up-to-date.
class CORE_EXPORT TextMatchMarker final : public DocumentMarker {
 private:
  enum class LayoutStatus { kInvalid, kValidNull, kValidNotNull };

 public:
  enum class MatchStatus { kInactive, kActive };

  TextMatchMarker(unsigned start_offset, unsigned end_offset, MatchStatus);
  TextMatchMarker(const TextMatchMarker&) = delete;
  TextMatchMarker& operator=(const TextMatchMarker&) = delete;

  // DocumentMarker implementations
  MarkerType GetType() const final;

  // TextMatchMarker-specific
  bool IsActiveMatch() const;
  void SetIsActiveMatch(bool active);

  bool IsRendered() const;
  bool Contains(const PhysicalOffset&) const;
  void SetRect(const PhysicalRect&);
  const PhysicalRect& GetRect() const;

  void Invalidate();
  bool IsValid() const;

 private:
  MatchStatus match_status_;
  LayoutStatus layout_status_ = LayoutStatus::kInvalid;
  PhysicalRect rect_;
};

template <>
struct DowncastTraits<TextMatchMarker> {
  static bool AllowFrom(const DocumentMarker& marker) {
    return marker.GetType() == DocumentMarker::kTextMatch;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_TEXT_MATCH_MARKER_H_
