/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_RANGE_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_RANGE_H_

#include "third_party/blink/public/platform/web_common.h"
#if INSIDE_BLINK
#include "third_party/blink/renderer/core/editing/forward.h"  // nogncheck
#endif

namespace blink {

#if INSIDE_BLINK
class LocalFrame;
#endif
class PlainTextRange;

class BLINK_EXPORT WebRange final {
 public:
  WebRange(int start, int length);
  WebRange();

  int StartOffset() const { return start_; }
  int EndOffset() const { return end_; }
  int length() const { return end_ - start_; }

  bool IsNull() const { return start_ == -1 && end_ == -1; }
  bool IsEmpty() const { return start_ == end_; }

#if INSIDE_BLINK
  WebRange(const EphemeralRange&);
  WebRange(const PlainTextRange&);

  EphemeralRange CreateEphemeralRange(LocalFrame*) const;
#endif

 private:
  // Note that this also matches the values for gfx::Range::InvalidRange
  // for easy conversion.
  int start_ = -1;
  int end_ = -1;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_RANGE_H_
