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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_FONT_FACE_SET_LOAD_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_FONT_FACE_SET_LOAD_EVENT_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/css/font_face.h"
#include "third_party/blink/renderer/core/dom/events/event.h"

namespace blink {

class FontFaceSetLoadEventInit;

class FontFaceSetLoadEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static FontFaceSetLoadEvent* Create(
      const AtomicString& type,
      const FontFaceSetLoadEventInit* initializer) {
    return MakeGarbageCollected<FontFaceSetLoadEvent>(type, initializer);
  }

  static FontFaceSetLoadEvent* CreateForFontFaces(
      const AtomicString& type,
      const FontFaceArray& fontfaces = FontFaceArray()) {
    return MakeGarbageCollected<FontFaceSetLoadEvent>(type, fontfaces);
  }

  FontFaceSetLoadEvent(const AtomicString&, const FontFaceArray&);
  FontFaceSetLoadEvent(const AtomicString&, const FontFaceSetLoadEventInit*);
  ~FontFaceSetLoadEvent() override;

  FontFaceArray fontfaces() const { return fontfaces_; }

  const AtomicString& InterfaceName() const override;

  void Trace(Visitor*) const override;

 private:
  FontFaceArray fontfaces_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_FONT_FACE_SET_LOAD_EVENT_H_
