// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_EVENTSOURCE_EVENT_SOURCE_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_EVENTSOURCE_EVENT_SOURCE_PARSER_H_

#include <memory>
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/text_codec.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class MODULES_EXPORT EventSourceParser final
    : public GarbageCollected<EventSourceParser> {
 public:
  class MODULES_EXPORT Client : public GarbageCollectedMixin {
   public:
    virtual ~Client() = default;
    virtual void OnMessageEvent(const AtomicString& type,
                                const String& data,
                                const AtomicString& last_event_id) = 0;
    virtual void OnReconnectionTimeSet(uint64_t reconnection_time) = 0;
    void Trace(Visitor* visitor) const override {}
  };

  EventSourceParser(const AtomicString& last_event_id, Client*);

  void AddBytes(base::span<const char>);
  const AtomicString& LastEventId() const { return last_event_id_; }
  // Stop parsing. This can be called from Client::onMessageEvent.
  void Stop() { is_stopped_ = true; }
  void Trace(Visitor*) const;

 private:
  void ParseLine();
  String FromUTF8(base::span<const char> bytes);

  Vector<char> line_;

  AtomicString event_type_;
  Vector<char> data_;
  // This variable corresponds to "last event ID buffer" in the spec. The
  // value can be discarded when a connection is disconnected while
  // parsing an event.
  AtomicString id_;
  // This variable corresponds to "last event ID string" in the spec.
  AtomicString last_event_id_;

  Member<Client> client_;
  std::unique_ptr<TextCodec> codec_;

  bool is_recognizing_crlf_ = false;
  bool is_recognizing_bom_ = true;
  bool is_stopped_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_EVENTSOURCE_EVENT_SOURCE_PARSER_H_
