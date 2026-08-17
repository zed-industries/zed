/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_GRID_POSITION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_GRID_POSITION_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

enum GridPositionType {
  kAutoPosition,
  kExplicitPosition,      // [ <integer> || <string> ]
  kSpanPosition,          // span && [ <integer> || <string> ]
  kNamedGridAreaPosition  // <ident>
};

class GridPosition {
  DISALLOW_NEW();

 public:
  GridPosition() : type_(kAutoPosition), integer_position_(0) {}

  bool IsPositive() const { return IntegerPosition() > 0; }

  GridPositionType GetType() const { return type_; }
  bool IsAuto() const { return type_ == kAutoPosition; }
  bool IsSpan() const { return type_ == kSpanPosition; }
  bool IsNamedGridArea() const { return type_ == kNamedGridAreaPosition; }

  void SetExplicitPosition(int position, const AtomicString& named_grid_line) {
    type_ = kExplicitPosition;
    integer_position_ = position;
    named_grid_line_ = named_grid_line;
  }

  void SetAutoPosition() {
    type_ = kAutoPosition;
    integer_position_ = 0;
  }

  // 'span' values cannot be negative, yet we reuse the <integer> position which
  // can be. This means that we have to convert the span position to an integer,
  // losing some precision here. It shouldn't be an issue in practice though.
  void SetSpanPosition(int position, const AtomicString& named_grid_line) {
    type_ = kSpanPosition;
    integer_position_ = position;
    named_grid_line_ = named_grid_line;
  }

  void SetNamedGridArea(const AtomicString& named_grid_area) {
    type_ = kNamedGridAreaPosition;
    named_grid_line_ = named_grid_area;
  }

  int IntegerPosition() const {
    DCHECK_EQ(GetType(), kExplicitPosition);
    return integer_position_;
  }

  AtomicString NamedGridLine() const {
    DCHECK(GetType() == kExplicitPosition || GetType() == kSpanPosition ||
           GetType() == kNamedGridAreaPosition);
    return named_grid_line_;
  }

  int SpanPosition() const {
    DCHECK_EQ(GetType(), kSpanPosition);
    return integer_position_;
  }

  bool operator==(const GridPosition& other) const {
    return type_ == other.type_ &&
           integer_position_ == other.integer_position_ &&
           named_grid_line_ == other.named_grid_line_;
  }

  bool operator!=(const GridPosition& other) const { return !(*this == other); }

  bool ShouldBeResolvedAgainstOppositePosition() const {
    return IsAuto() || IsSpan();
  }

 private:
  GridPositionType type_;
  int integer_position_;
  AtomicString named_grid_line_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_GRID_POSITION_H_
