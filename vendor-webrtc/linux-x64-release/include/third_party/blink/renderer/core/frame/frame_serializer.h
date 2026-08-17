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
 * contributors may be used to endorse or promo te products derived from
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_SERIALIZER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_SERIALIZER_H_

#include "base/functional/callback.h"
#include "base/functional/function_ref.h"
#include "third_party/blink/public/web/web_frame_serializer.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class LocalFrame;
class Frame;

struct SerializedResource;

// Internal functionality exposed for unit testing.
namespace internal {
// Returns the result of replacing all case-insensitive occurrences of `from` in
// `source` with the result of `transform.Run(match)`.
CORE_EXPORT String
ReplaceAllCaseInsensitive(String source,
                          const String& from,
                          base::FunctionRef<String(const String&)> transform);
}  // namespace internal

// This class is used to serialize frame's contents to MHTML. It serializes
// frame's document and resources such as images and CSS stylesheets.
class CORE_EXPORT FrameSerializer {
 public:
  FrameSerializer() = delete;

  // Returns a Content-ID to be used for the given frame.
  // See rfc2557 - section 8.3 - "Use of the Content-ID header and CID URLs".
  // Format note - the returned string should be of the form "<foo@bar.com>"
  // (i.e. the strings should include the angle brackets).
  static String GetContentID(Frame* frame);

  // Serializes the frame. Writes output to the given deque of
  // SerializedResources and uses `web_delegate` for controlling some
  // serialization aspects. All serialized content and retrieved resources are
  // added to `resources`. The first resource is the frame's serialized content.
  // Subsequent resources are images, css, etc.
  static void SerializeFrame(
      WebFrameSerializer::MHTMLPartsGenerationDelegate& web_delegate,
      LocalFrame& frame,
      base::OnceCallback<void(Deque<SerializedResource>)> done_callback);

  static String MarkOfTheWebDeclaration(const KURL&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_SERIALIZER_H_
