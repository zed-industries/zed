// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_IO_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_IO_AGENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/inspector/inspector_base_agent.h"
#include "third_party/blink/renderer/core/inspector/protocol/io.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace v8 {
class Isolate;
}

namespace v8_inspector {
class V8InspectorSession;
}

namespace blink {

class CORE_EXPORT InspectorIOAgent final
    : public InspectorBaseAgent<protocol::IO::Metainfo> {
 public:
  InspectorIOAgent(v8::Isolate*, v8_inspector::V8InspectorSession*);
  InspectorIOAgent(const InspectorIOAgent&) = delete;
  InspectorIOAgent& operator=(const InspectorIOAgent&) = delete;
  ~InspectorIOAgent() override;

 private:
  void Restore() override {}

  // Called from the front-end.
  protocol::Response resolveBlob(const String& object_id,
                                 String* uuid) override;

  v8::Isolate* isolate_;
  v8_inspector::V8InspectorSession* v8_session_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_IO_AGENT_H_
