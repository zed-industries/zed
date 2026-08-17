/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SURROUNDING_TEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SURROUNDING_TEXT_H_

#include "third_party/blink/renderer/core/core_export.h"

#if INSIDE_BLINK
#include "third_party/blink/renderer/core/editing/forward.h"  // nogncheck
#endif
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class LocalFrame;

// SurroundingText is a Blink API that gives access to the SurroundingText
// API. It allows caller to know the text surrounding a point or a range.
class CORE_EXPORT SurroundingText {
 public:
  // Initializes the object with the current selection in a given frame.
  // The maximum length of the contents retrieved is defined by max_length.
  // It does not include the text inside the range.
  SurroundingText(LocalFrame*, wtf_size_t max_length);

#if INSIDE_BLINK
  SurroundingText(const EphemeralRange&, wtf_size_t max_length);
#endif

  bool IsEmpty() const;

  // Surrounding text content retrieved.
  String TextContent() const;

  // Start offset of the initial text in the text content.
  wtf_size_t StartOffsetInTextContent() const;

  // End offset of the initial text in the text content.
  wtf_size_t EndOffsetInTextContent() const;

 private:
  String text_content_;
  wtf_size_t start_offset_in_text_content_;
  wtf_size_t end_offset_in_text_content_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SURROUNDING_TEXT_H_
