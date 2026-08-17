/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SUBSTRING_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SUBSTRING_UTIL_H_

#include <CoreFoundation/CoreFoundation.h>

#include <cstddef>

#include "base/apple/scoped_cftyperef.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace gfx {
class Point;
}  // namespace gfx

namespace blink {
class WebFrameWidgetImpl;
class LocalFrame;

class SubstringUtil {
 public:
  // Given a point inside a `WebFrameWidgetImpl`, determines the word underneath
  // that point and returns:
  //
  // - a `CFAttributedStringRef` of that word and
  // - the left baseline point of that word in `baseline_point`
  //
  // Returns nil on failure.
  CORE_EXPORT static base::apple::ScopedCFTypeRef<CFAttributedStringRef>
  AttributedWordAtPoint(WebFrameWidgetImpl*,
                        gfx::Point,
                        gfx::Point& baseline_point);

  // Given a range of a `LocalFrame`, determines the substring specified by that
  // range and returns:
  //
  // - a `CFAttributedStringRef` of that substring and
  // - the left baseline point of that substring in `baseline_point`
  //
  // Returns nil on failure.
  CORE_EXPORT static base::apple::ScopedCFTypeRef<CFAttributedStringRef>
  AttributedSubstringInRange(LocalFrame*,
                             wtf_size_t location,
                             wtf_size_t length,
                             gfx::Point& baseline_point);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SUBSTRING_UTIL_H_
