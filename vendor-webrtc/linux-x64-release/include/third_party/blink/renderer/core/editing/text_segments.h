// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_TEXT_SEGMENTS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_TEXT_SEGMENTS_H_

#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

// Provides functions for finding boundary of text segments.
class TextSegments final {
  STATIC_ONLY(TextSegments);

 public:
  // The interface to find boundary of text segment.
  class Finder {
   public:
    class Position final {
      STACK_ALLOCATED();

     public:
      Position();

      static Position Before(unsigned offset);
      static Position After(unsigned offset);

      bool IsAfter() const { return type_ == kAfter; }
      bool IsBefore() const { return type_ == kBefore; }
      bool IsNone() const { return type_ == kNone; }

      unsigned Offset() const;

     private:
      enum Type { kNone, kBefore, kAfter };

      Position(unsigned value, Type type);

      const unsigned offset_ = 0;
      const Type type_ = kNone;
    };

    // Returns a text segment boundary position in |text| from |offset|.
    // Note: |text| must contains character 16.
    // Note: Since implementations can have state, |Find()| function isn't
    // marked |const| intentionally.
    virtual Position Find(const WTF::String text, unsigned offset) = 0;
  };

  // Returns a boundary position found by |finder| followed by |position|
  // (inclusive). |finder| can be stateful or stateless.
  static PositionInFlatTree FindBoundaryForward(
      const PositionInFlatTree& position,
      Finder* finder);

  static PositionInFlatTree FindBoundaryBackward(
      const PositionInFlatTree& position,
      Finder* finder);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_TEXT_SEGMENTS_H_
