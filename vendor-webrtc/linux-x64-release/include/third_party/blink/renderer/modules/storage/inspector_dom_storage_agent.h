/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_INSPECTOR_DOM_STORAGE_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_INSPECTOR_DOM_STORAGE_AGENT_H_

#include "third_party/blink/renderer/core/inspector/inspector_base_agent.h"
#include "third_party/blink/renderer/core/inspector/protocol/dom_storage.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/storage/storage_area.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class BlinkStorageKey;
class InspectedFrames;

class MODULES_EXPORT InspectorDOMStorageAgent final
    : public InspectorBaseAgent<protocol::DOMStorage::Metainfo> {
 public:
  explicit InspectorDOMStorageAgent(InspectedFrames*);
  ~InspectorDOMStorageAgent() override;
  void Trace(Visitor*) const override;

  void DidDispatchDOMStorageEvent(const String& key,
                                  const String& old_value,
                                  const String& new_value,
                                  StorageArea::StorageType storage_type,
                                  const BlinkStorageKey& storage_key);

 private:
  void InnerEnable();

  // InspectorBaseAgent overrides.
  void Restore() override;

  // protocol::Dispatcher::DOMStorageCommandHandler overrides.
  protocol::Response enable() override;
  protocol::Response disable() override;
  protocol::Response clear(
      std::unique_ptr<protocol::DOMStorage::StorageId>) override;

  protocol::Response getDOMStorageItems(
      std::unique_ptr<protocol::DOMStorage::StorageId>,
      std::unique_ptr<protocol::Array<protocol::Array<String>>>* entries)
      override;
  protocol::Response setDOMStorageItem(
      std::unique_ptr<protocol::DOMStorage::StorageId>,
      const String& key,
      const String& value) override;
  protocol::Response removeDOMStorageItem(
      std::unique_ptr<protocol::DOMStorage::StorageId>,
      const String& key) override;

  protocol::Response FindStorageArea(
      std::unique_ptr<protocol::DOMStorage::StorageId>,
      StorageArea*&);
  std::unique_ptr<protocol::DOMStorage::StorageId> GetStorageId(
      const BlinkStorageKey& storage_key,
      bool is_local_storage);

  Member<InspectedFrames> inspected_frames_;
  InspectorAgentState::Boolean enabled_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_INSPECTOR_DOM_STORAGE_AGENT_H_
