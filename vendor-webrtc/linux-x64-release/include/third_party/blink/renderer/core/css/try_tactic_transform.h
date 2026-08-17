// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_TRY_TACTIC_TRANSFORM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_TRY_TACTIC_TRANSFORM_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/style/position_try_fallbacks.h"
#include "third_party/blink/renderer/platform/text/writing_direction_mode.h"
#include "third_party/blink/renderer/platform/text/writing_mode_utils.h"

namespace blink {

// Represents a transform via <try-tactic>.
//
// Considering an element at some initial position (0), there are only seven
// different possible transformations represented by <try-tactic> flips:
//
//
//        4             5
//
//     ┌───────────────────┐
//     │.                  │
//  0  │  .                │  2
//     │    .              │
//     │      .            │
//     │        .          │
//     │          .        │
//     │            .      │
//     │              .    │
//  1  │                .  │  3
//     │                  .│
//     └───────────────────┘
//
//        6             7
//
//
// Note that the dotted line represents the start-start to end-end diagonal
// that we mirror across for flip-start.
//
// In many cases, there are multiple ways to compose flips together to reach
// a certain point. For example, first flipping block, then flipping inline
// both end up at (3). A less obvious example is block-start-inline being
// equivalent to just start (4).
//
// Although the specified order of flips within the <try-tactic> matters, any
// tactic can be converted to another corresponding tactic with a *fixed* order.
// This class represents the "canonical" tactic, with the following fixed
// flipping order: block, inline, then start.
//
// The following table shows the canonical tactics (N) vs. the diagram above,
// and also shows how non-canonical are transformed into canonical ones (=>N)).
//
// block                  (1)
// inline                 (2)
// block inline           (3)
// start                  (4)
// block start            (5)
// inline start           (6)
// block inline start     (7)
//
// inline block           (=>3)
// block start inline     (=>4)
// inline start block     (=>4)
// start inline           (=>5)
// start block            (=>6)
// inline block start     (=>7)
// start block inline     (=>7)
// start inline block     (=>7)
//
class CORE_EXPORT TryTacticTransform {
  enum Mask {
    kBlock = 0b001,
    kInline = 0b010,
    kStart = 0b100,
  };

 public:
  TryTacticTransform() = default;

  explicit TryTacticTransform(const TryTacticList& tactic_list) {
    for (TryTactic tactic : tactic_list) {
      FlipTactic(tactic);
    }
  }

  bool operator==(const TryTacticTransform& o) const {
    return bits_ == o.bits_;
  }
  bool operator!=(const TryTacticTransform& o) const { return !(*this == o); }

  template <typename T>
  struct LogicalSides {
    T inline_start;
    T inline_end;
    T block_start;
    T block_end;
  };

  // Transforms side values. The results describes e.g. how arguments to
  // anchor() should be rewritten within a value.
  template <typename T>
  LogicalSides<T> Transform(LogicalSides<T> sides) const {
    LogicalSides<T> result = sides;
    // We must change each value to the corresponding flipped value.
    // For example, say that we begin with sides={IS, IE, BS, BE} and the
    // transform is "flip-block flip-start", then we should behave as follows:
    //
    // - Initial         : {IS, IE, BS, BE}
    // - After flip block: {IS, IE, BE, BS}, i.e. BE=>BS, BS=>BE.
    // - After flip start: {BS, BE, IE, IS}. i.e. I*=>B*, B*=>I*
    //
    // We can implement this as a series of swaps on the members of `result`,
    // in reverse order:
    //
    // - Initial            : result={IS, IE, BS, BE}
    // - Swap for flip-start: result={BS, BE, IS, IE}
    // - Swap for flip-block: result={BS, BE, IE, IS}
    //
    if (FlippedStart()) {
      std::swap(result.block_start, result.inline_start);
      std::swap(result.block_end, result.inline_end);
    }
    if (FlippedBlock()) {
      std::swap(result.block_start, result.block_end);
    }
    if (FlippedInline()) {
      std::swap(result.inline_start, result.inline_end);
    }
    return result;
  }

  template <typename T>
  LogicalToPhysical<T> Transform(LogicalSides<T> sides,
                                 WritingDirectionMode writing_direction) const {
    LogicalSides<T> transformed_sides = Transform(sides);
    return LogicalToPhysical<T>(
        writing_direction, transformed_sides.inline_start,
        transformed_sides.inline_end, transformed_sides.block_start,
        transformed_sides.block_end);
  }

  // The inverse transform takes to back to where you were before applying
  // this transform. For example (see class comment): if you start at (0)
  // and apply block-start, you end up at (5). The inverse is inline-start,
  // because that takes you back to (0) again.
  TryTacticTransform Inverse() const {
    // All the transforms are their own inverse transforms except for
    // block-start and inline-start.
    if (bits_ == (kBlock | kStart)) {
      return TryTacticTransform(kInline | kStart);
    }
    if (bits_ == (kInline | kStart)) {
      return TryTacticTransform(kBlock | kStart);
    }
    return *this;
  }

  bool FlippedBlock() const { return bits_ & kBlock; }
  bool FlippedInline() const { return bits_ & kInline; }
  bool FlippedStart() const { return bits_ & kStart; }

  // Returns an integer in the range [0,7] which uniquely identifies the
  // transform. Useful as a cache key, see TryValueFlips::cached_flip_sets_.
  unsigned CacheIndex() const { return bits_; }

 private:
  explicit TryTacticTransform(unsigned bits) : bits_(bits) {}

  void FlipTactic(TryTactic tactic) {
    switch (tactic) {
      case TryTactic::kNone:
        break;
      case TryTactic::kFlipBlock:
        FlipBlock();
        break;
      case TryTactic::kFlipInline:
        FlipInline();
        break;
      case TryTactic::kFlipStart:
        FlipStart();
        break;
    }
  }

  // These are used to build `bits_`, and called in the order provided
  // by the TryTacticList. When FlipStart is called before FlipBlock/Inline,
  // block/inline is flipped to produce the correct canonical transform.
  void FlipBlock() { bits_ = bits_ ^ (FlippedStart() ? kInline : kBlock); }
  void FlipInline() { bits_ = bits_ ^ (FlippedStart() ? kBlock : kInline); }
  void FlipStart() { bits_ = bits_ ^ kStart; }

  unsigned bits_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_TRY_TACTIC_TRANSFORM_H_
