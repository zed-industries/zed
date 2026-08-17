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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_BORDER_IMAGE_LENGTH_BOX_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_BORDER_IMAGE_LENGTH_BOX_H_

#include "third_party/blink/renderer/core/style/border_image_length.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Represents a computed border image width or outset.
//
// http://www.w3.org/TR/css3-background/#border-image-width
// http://www.w3.org/TR/css3-background/#border-image-outset
class BorderImageLengthBox {
  DISALLOW_NEW();

 public:
  BorderImageLengthBox(Length length)
      : left_(length), right_(length), top_(length), bottom_(length) {}

  BorderImageLengthBox(double number)
      : left_(number), right_(number), top_(number), bottom_(number) {}

  BorderImageLengthBox(const BorderImageLength& top,
                       const BorderImageLength& right,
                       const BorderImageLength& bottom,
                       const BorderImageLength& left)
      : left_(left), right_(right), top_(top), bottom_(bottom) {}

  const BorderImageLength& Left() const { return left_; }
  const BorderImageLength& Right() const { return right_; }
  const BorderImageLength& Top() const { return top_; }
  const BorderImageLength& Bottom() const { return bottom_; }

  bool operator==(const BorderImageLengthBox& other) const {
    return left_ == other.left_ && right_ == other.right_ &&
           top_ == other.top_ && bottom_ == other.bottom_;
  }

  bool operator!=(const BorderImageLengthBox& other) const {
    return !(*this == other);
  }

  bool NonZero() const {
    return !(left_.IsZero() && right_.IsZero() && top_.IsZero() &&
             bottom_.IsZero());
  }

 private:
  BorderImageLength left_;
  BorderImageLength right_;
  BorderImageLength top_;
  BorderImageLength bottom_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_BORDER_IMAGE_LENGTH_BOX_H_
