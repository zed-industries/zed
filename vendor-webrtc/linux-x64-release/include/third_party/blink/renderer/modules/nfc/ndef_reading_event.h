// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_NFC_NDEF_READING_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_NFC_NDEF_READING_EVENT_H_

#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExceptionState;
class NDEFMessage;
class NDEFReadingEventInit;
class ScriptState;

class NDEFReadingEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static NDEFReadingEvent* Create(const ScriptState*,
                                  const AtomicString&,
                                  const NDEFReadingEventInit*,
                                  ExceptionState&);

  NDEFReadingEvent(const AtomicString&,
                   const NDEFReadingEventInit*,
                   NDEFMessage*);
  NDEFReadingEvent(const AtomicString&, const String&, NDEFMessage*);
  ~NDEFReadingEvent() override;

  const AtomicString& InterfaceName() const override;

  void Trace(Visitor*) const override;

  const String& serialNumber() const;
  NDEFMessage* message() const;

 private:
  String serial_number_;
  Member<NDEFMessage> message_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_NFC_NDEF_READING_EVENT_H_
