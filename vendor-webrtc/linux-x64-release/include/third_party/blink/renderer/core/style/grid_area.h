/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_GRID_AREA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_GRID_AREA_H_

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/style/grid_enums.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_functions.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/math_extras.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// Legacy grid expands out auto-repeaters, so it has a lower cap than GridNG.
// Note that this actually allows a [-999, 999] range.
constexpr int kLegacyGridMaxTracks = 1000;
constexpr int kGridMaxTracks = 10000000;

// A span of grid tracks in a single direction (either rows or columns).
// Note that `start_line` and `end_line` are grid lines' indexes; despite line
// numbers in the spec start at 1, the indexes here start at 0.
struct GridSpan {
  USING_FAST_MALLOC(GridSpan);

 public:
  static GridSpan UntranslatedDefiniteGridSpan(int start_line, int end_line) {
    return GridSpan(start_line, end_line, kUntranslatedDefinite);
  }

  static GridSpan TranslatedDefiniteGridSpan(wtf_size_t start_line,
                                             wtf_size_t end_line) {
    return GridSpan(start_line, end_line, kTranslatedDefinite);
  }

  static GridSpan IndefiniteGridSpan(wtf_size_t span_size = 1) {
    return GridSpan(0, span_size, kIndefinite);
  }

  bool operator==(const GridSpan& o) const {
    return type_ == o.type_ && start_line_ == o.start_line_ &&
           end_line_ == o.end_line_;
  }

  bool operator<(const GridSpan& o) const {
    DCHECK(IsTranslatedDefinite());
    return start_line_ < o.start_line_ ||
           (start_line_ == o.start_line_ && end_line_ < o.end_line_);
  }

  bool operator<=(const GridSpan& o) const {
    DCHECK(IsTranslatedDefinite());
    return *this < o || *this == o;
  }

  bool Contains(wtf_size_t line) const {
    DCHECK(IsTranslatedDefinite());
    DCHECK_GE(start_line_, 0);
    DCHECK_LT(start_line_, end_line_);
    return line >= static_cast<wtf_size_t>(start_line_) &&
           line <= static_cast<wtf_size_t>(end_line_);
  }

  unsigned GetHash() const {
    // In general, a negative `end_line_` will reduce collisions of indefinite
    // spans since it represents the range `[-end_line_, 0]`, which can never
    // occur in definite spans that ensure `start_line_ < end_line_`.
    return WTF::HashInts(start_line_, IsIndefinite() ? -end_line_ : end_line_);
  }

  bool Intersects(GridSpan span) const {
    DCHECK(IsTranslatedDefinite());
    DCHECK(span.IsTranslatedDefinite());
    DCHECK_GE(start_line_, 0);
    DCHECK_LT(start_line_, end_line_);
    DCHECK_GE(span.start_line_, 0);
    DCHECK_LT(span.start_line_, span.end_line_);

    return start_line_ < span.end_line_ && end_line_ >= span.start_line_;
  }

  wtf_size_t IntegerSpan() const {
    DCHECK(IsTranslatedDefinite());
    DCHECK_LT(start_line_, end_line_);
    return end_line_ - start_line_;
  }

  wtf_size_t IndefiniteSpanSize() const {
    DCHECK(IsIndefinite());
    DCHECK_EQ(start_line_, 0);
    DCHECK_GT(end_line_, 0);
    return end_line_;
  }

  int UntranslatedStartLine() const {
    DCHECK_EQ(type_, kUntranslatedDefinite);
    return start_line_;
  }

  int UntranslatedEndLine() const {
    DCHECK_EQ(type_, kUntranslatedDefinite);
    return end_line_;
  }

  wtf_size_t StartLine() const {
    DCHECK(IsTranslatedDefinite());
    DCHECK_GE(start_line_, 0);
    return start_line_;
  }

  wtf_size_t EndLine() const {
    DCHECK(IsTranslatedDefinite());
    DCHECK_GT(end_line_, 0);
    return end_line_;
  }

  bool IsUntranslatedDefinite() const { return type_ == kUntranslatedDefinite; }
  bool IsTranslatedDefinite() const { return type_ == kTranslatedDefinite; }
  bool IsIndefinite() const { return type_ == kIndefinite; }

  void Translate(wtf_size_t offset) {
    DCHECK_NE(type_, kIndefinite);
    *this =
        GridSpan(start_line_ + offset, end_line_ + offset, kTranslatedDefinite);
  }

  void SetStart(int start_line) {
    DCHECK_NE(type_, kIndefinite);
    *this = GridSpan(start_line, end_line_, kTranslatedDefinite);
  }

  void SetEnd(int end_line) {
    DCHECK_NE(type_, kIndefinite);
    *this = GridSpan(start_line_, end_line, kTranslatedDefinite);
  }

  void Intersect(int start_line, int end_line) {
    DCHECK_NE(type_, kIndefinite);
    *this = GridSpan(std::max(start_line_, start_line),
                     std::min(end_line_, end_line), kTranslatedDefinite);
  }

 private:
  enum GridSpanType { kIndefinite, kTranslatedDefinite, kUntranslatedDefinite };

  GridSpan(int start_line, int end_line, GridSpanType type) : type_(type) {
    if (type_ == kIndefinite) {
      DCHECK_EQ(start_line, 0);
      end_line_ = ClampTo(end_line, 1, kGridMaxTracks);
    } else {
      start_line_ = ClampTo(start_line, -kGridMaxTracks, kGridMaxTracks - 1);
      end_line_ = ClampTo(end_line, start_line_ + 1, kGridMaxTracks);
#if DCHECK_IS_ON()
      if (type == kTranslatedDefinite) {
        DCHECK_GE(start_line_, 0);
      }
#endif
    }
  }

  int start_line_{0};
  int end_line_{0};
  GridSpanType type_{kIndefinite};
};

// This represents a grid area that spans in both rows' and columns' direction.
struct GridArea {
  USING_FAST_MALLOC(GridArea);

 public:
  // HashMap requires a default constuctor.
  GridArea()
      : columns(GridSpan::IndefiniteGridSpan()),
        rows(GridSpan::IndefiniteGridSpan()) {}

  GridArea(const GridSpan& r, const GridSpan& c) : columns(c), rows(r) {}

  const GridSpan& Span(GridTrackSizingDirection track_direction) const {
    return (track_direction == kForColumns) ? columns : rows;
  }

  void SetSpan(const GridSpan& span, GridTrackSizingDirection track_direction) {
    if (track_direction == kForColumns) {
      columns = span;
    } else {
      rows = span;
    }
  }

  wtf_size_t StartLine(GridTrackSizingDirection track_direction) const {
    return Span(track_direction).StartLine();
  }

  wtf_size_t EndLine(GridTrackSizingDirection track_direction) const {
    return Span(track_direction).EndLine();
  }

  wtf_size_t SpanSize(GridTrackSizingDirection track_direction) const {
    return Span(track_direction).IntegerSpan();
  }

  void Transpose() { std::swap(columns, rows); }

  bool operator==(const GridArea& o) const {
    return columns == o.columns && rows == o.rows;
  }

  bool operator!=(const GridArea& o) const { return !(*this == o); }

  GridSpan columns;
  GridSpan rows;
};

typedef HashMap<String, GridArea> NamedGridAreaMap;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_GRID_AREA_H_
