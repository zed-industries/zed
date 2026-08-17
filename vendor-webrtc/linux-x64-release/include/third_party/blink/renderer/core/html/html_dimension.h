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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_DIMENSION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_DIMENSION_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

// This class corresponds to a dimension as described in HTML5 by the
// "rules for parsing a list of dimensions" (section 2.4.4.6).
class HTMLDimension {
  DISALLOW_NEW();

 public:
  enum HTMLDimensionType { kRelative, kPercentage, kAbsolute };

  HTMLDimension() : type_(kAbsolute), value_(0) {}

  HTMLDimension(double value, HTMLDimensionType type)
      : type_(type), value_(value) {}

  HTMLDimensionType GetType() const { return type_; }

  bool IsRelative() const { return type_ == kRelative; }
  bool IsPercentage() const { return type_ == kPercentage; }
  bool IsAbsolute() const { return type_ == kAbsolute; }

  double Value() const { return value_; }

  bool operator==(const HTMLDimension& other) const {
    return type_ == other.type_ && value_ == other.value_;
  }
  bool operator!=(const HTMLDimension& other) const {
    return !(*this == other);
  }

 private:
  HTMLDimensionType type_;
  double value_;
};

CORE_EXPORT Vector<HTMLDimension> ParseListOfDimensions(const WTF::String&);
CORE_EXPORT bool ParseDimensionValue(const WTF::String&, HTMLDimension&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_DIMENSION_H_
