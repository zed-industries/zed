/*
 * Copyright (C) 2007, 2008 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOM_EXCEPTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOM_EXCEPTION_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/exception_code.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8-exception.h"

namespace blink {
class ExceptionContext;

class CORE_EXPORT DOMException : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Constructor exposed to script. Only for use by the bindings code.
  static DOMException* Create(const String& message, const String& name);

  // V8ThrowDOMException::CreateOrEmpty() should be used to create DOMException
  // objects as it correctly attaches the stack property to the JavaScript
  // object. However this constructor has many existing callers. This
  // constructor can also be legitimately used by subclasses, in which case
  // V8ThrowDOMException::AttachStackProperty() should be called after this
  // constructor to attach the stack property. Strings are passed by value to
  // avoid reference count churn when they are constructed from a temporary.
  // TODO(https://crbug.com/40639312): Replace DOMException constructor calls.
  DOMException(DOMExceptionCode,
               String sanitized_message,
               String unsanitized_message = String());

  // V8ThrowDOMException::CreateOrEmpty() should be used to create DOMException
  // objects, however many callers call the DOMException constructor with
  // literal strings. This constructor reduces code size for those callsites.
  // TODO(https://crbug.com/40639312): Replace DOMException constructor calls.
  explicit DOMException(DOMExceptionCode,
                        const char* sanitized_message = nullptr,
                        const char* unsanitized_message = nullptr);

  // For use by Create() only.
  DOMException(uint16_t legacy_code,
               const String& name,
               const String& sanitized_message,
               const String& unsanitized_message);

  static String GetErrorName(DOMExceptionCode);
  static String GetErrorMessage(DOMExceptionCode);

  uint16_t code() const { return legacy_code_; }
  const String& name() const { return name_; }

  // This is the message that's exposed to JavaScript: never return unsanitized
  // data.
  const String& message() const { return sanitized_message_; }

  // This is the message that's exposed to the console: if an unsanitized
  // message is present, we prefer it.
  String ToStringForConsole() const;

  void AddContextToMessages(v8::ExceptionContext type,
                            const char* class_name,
                            const String& property_name);

 private:
  uint16_t legacy_code_;
  String name_;
  String sanitized_message_;
  String unsanitized_message_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOM_EXCEPTION_H_
