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
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_VALIDATION_MESSAGE_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_VALIDATION_MESSAGE_CLIENT_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class Document;
class Element;
class GraphicsContext;

class ValidationMessageClient : public GarbageCollectedMixin {
 public:
  virtual ~ValidationMessageClient() = default;

  // Show validation message for the specified anchor element. An
  // implementation of this function may hide the message automatically after
  // some period.
  virtual void ShowValidationMessage(Element& anchor,
                                     const String& main_message,
                                     TextDirection,
                                     const String& sub_message,
                                     TextDirection) = 0;

  // Hide validation message for the specified anchor if the message for the
  // anchor is already visible.
  virtual void HideValidationMessage(const Element& anchor) = 0;

  // Returns true if the validation message for the specified anchor element
  // is visible.
  virtual bool IsValidationMessageVisible(const Element& anchor) = 0;

  virtual void DocumentDetached(const Document&) = 0;
  virtual void DidChangeFocusTo(const Element* new_element) = 0;

  virtual void WillBeDestroyed() = 0;

  virtual void ServiceScriptedAnimations(base::TimeTicks) {}
  virtual void LayoutOverlay() {}
  virtual void UpdatePrePaint() {}
  virtual void PaintOverlay(GraphicsContext&) {}

  void Trace(Visitor* visitor) const override {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_VALIDATION_MESSAGE_CLIENT_H_
