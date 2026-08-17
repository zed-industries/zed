/*
 * Copyright (C) 2012 Google Inc. All Rights Reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_QUOTA_DOM_ERROR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_QUOTA_DOM_ERROR_H_

#include "third_party/blink/public/mojom/quota/quota_types.mojom-blink.h"
#include "third_party/blink/renderer/core/dom/dom_exception.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class MODULES_EXPORT DOMError : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static DOMError* Create(const String& name) {
    return MakeGarbageCollected<DOMError>(name);
  }
  static DOMError* Create(const String& name, const String& message) {
    return MakeGarbageCollected<DOMError>(name, message);
  }
  static DOMError* Create(DOMExceptionCode exception_code) {
    return MakeGarbageCollected<DOMError>(
        DOMException::GetErrorName(exception_code),
        DOMException::GetErrorMessage(exception_code));
  }
  static DOMError* Create(mojom::blink::QuotaStatusCode status_code);

  explicit DOMError(const String& name);
  DOMError(const String& name, const String& message);
  ~DOMError() override;

  const String& name() const { return name_; }
  const String& message() const { return message_; }

 private:
  const String name_;
  const String message_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_QUOTA_DOM_ERROR_H_
