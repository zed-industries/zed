/*
 * Copyright (c) 2013, Opera Software ASA. All rights reserved.
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
 *     * Neither the name of Opera Software ASA nor the names of its
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_BORDER_IMAGE_LENGTH_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_BORDER_IMAGE_LENGTH_H_

#include "third_party/blink/renderer/platform/geometry/length.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Represents an individual computed border image width or outset.
//
// http://www.w3.org/TR/css3-background/#border-image-width
// http://www.w3.org/TR/css3-background/#border-image-outset
class BorderImageLength {
  DISALLOW_NEW();

 public:
  BorderImageLength(double number) : number_(number), type_(kNumberType) {}

  BorderImageLength(const Length& length)
      : length_(length), number_(0), type_(kLengthType) {}

  bool IsNumber() const { return type_ == kNumberType; }
  bool IsLength() const { return type_ == kLengthType; }

  const Length& length() const {
    DCHECK(IsLength());
    return length_;
  }
  Length& length() {
    DCHECK(IsLength());
    return length_;
  }

  double Number() const {
    DCHECK(IsNumber());
    return number_;
  }

  bool operator==(const BorderImageLength& other) const {
    return type_ == other.type_ && length_ == other.length_ &&
           number_ == other.number_;
  }

  bool IsZero() const {
    if (IsLength()) {
      return length_.IsZero();
    }

    DCHECK(IsNumber());
    return !number_;
  }

 private:
  // Ideally we would put the 2 following fields in a union, but Length has a
  // constructor, a destructor and a copy assignment which isn't allowed.
  Length length_;
  double number_;
  enum { kLengthType, kNumberType } type_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_BORDER_IMAGE_LENGTH_H_
