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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_ERROR_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_ERROR_EVENT_H_

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/bindings/core/v8/world_safe_v8_reference.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event.h"
#include "third_party/blink/renderer/platform/bindings/dom_wrapper_world.h"
#include "third_party/blink/renderer/platform/bindings/source_location.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ErrorEventInit;

class CORE_EXPORT ErrorEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static ErrorEvent* Create(ScriptState* script_state) {
    return MakeGarbageCollected<ErrorEvent>(script_state);
  }
  static ErrorEvent* Create(const String& message,
                            std::unique_ptr<SourceLocation> location,
                            DOMWrapperWorld* world) {
    return MakeGarbageCollected<ErrorEvent>(message, std::move(location),
                                            ScriptValue(), world);
  }

  static ErrorEvent* Create(const String& message,
                            std::unique_ptr<SourceLocation> location,
                            ScriptValue error,
                            DOMWrapperWorld* world) {
    return MakeGarbageCollected<ErrorEvent>(message, std::move(location), error,
                                            world);
  }

  static ErrorEvent* Create(ScriptState* script_state,
                            const AtomicString& type,
                            const ErrorEventInit* initializer) {
    return MakeGarbageCollected<ErrorEvent>(script_state, type, initializer);
  }

  // Creates an error for a script whose errors are muted.
  static ErrorEvent* CreateSanitizedError(ScriptState* script_state);

  explicit ErrorEvent(ScriptState* script_state);
  ErrorEvent(const String& message,
             std::unique_ptr<SourceLocation>,
             ScriptValue error,
             DOMWrapperWorld*);
  ErrorEvent(ScriptState*, const AtomicString&, const ErrorEventInit*);
  ~ErrorEvent() override;

  // As |message| is exposed to JavaScript, never return |unsanitized_message_|.
  const String& message() const { return sanitized_message_; }
  const String& filename() const { return location_->Url(); }
  unsigned lineno() const { return location_->LineNumber(); }
  unsigned colno() const { return location_->ColumnNumber(); }
  ScriptValue error(ScriptState*) const;

  // Not exposed to JavaScript, prefers |unsanitized_message_|.
  const String& MessageForConsole() const {
    return !unsanitized_message_.empty() ? unsanitized_message_
                                         : sanitized_message_;
  }
  SourceLocation* Location() const { return location_.get(); }

  const AtomicString& InterfaceName() const override;
  bool CanBeDispatchedInWorld(const DOMWrapperWorld&) const override;
  bool IsErrorEvent() const override;

  DOMWrapperWorld* World() const { return world_.Get(); }

  void SetUnsanitizedMessage(const String&);

  void Trace(Visitor*) const override;

 private:
  String unsanitized_message_;
  String sanitized_message_;
  std::unique_ptr<SourceLocation> location_;
  WorldSafeV8Reference<v8::Value> error_;
  const Member<DOMWrapperWorld> world_;
};

template <>
struct DowncastTraits<ErrorEvent> {
  static bool AllowFrom(const Event& event) { return event.IsErrorEvent(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_ERROR_EVENT_H_
