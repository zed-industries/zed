/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_VERSION_CHANGE_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_VERSION_CHANGE_EVENT_H_

#include <optional>

#include "third_party/blink/public/mojom/indexeddb/indexeddb.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_idb_version_change_event_init.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/indexeddb/idb_any.h"
#include "third_party/blink/renderer/modules/indexeddb/idb_request.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class V8IDBDataLossAmount;

class IDBVersionChangeEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static IDBVersionChangeEvent* Create() {
    return MakeGarbageCollected<IDBVersionChangeEvent>();
  }
  static IDBVersionChangeEvent* Create(
      const AtomicString& event_type,
      const IDBVersionChangeEventInit* initializer) {
    return MakeGarbageCollected<IDBVersionChangeEvent>(event_type, initializer);
  }

  IDBVersionChangeEvent();
  IDBVersionChangeEvent(
      const AtomicString& event_type,
      uint64_t old_version,
      const std::optional<uint64_t>& new_version,
      mojom::blink::IDBDataLoss data_loss = mojom::blink::IDBDataLoss::None,
      const String& data_loss_message = String());
  IDBVersionChangeEvent(const AtomicString& event_type,
                        const IDBVersionChangeEventInit*);

  uint64_t oldVersion() const { return old_version_; }
  std::optional<uint64_t> newVersion() const { return new_version_; }

  V8IDBDataLossAmount dataLoss() const;
  const String& dataLossMessage() const { return data_loss_message_; }

  const AtomicString& InterfaceName() const override;

  void Trace(Visitor*) const override;

 private:
  uint64_t old_version_;
  std::optional<uint64_t> new_version_;
  mojom::blink::IDBDataLoss data_loss_;
  String data_loss_message_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_VERSION_CHANGE_EVENT_H_
