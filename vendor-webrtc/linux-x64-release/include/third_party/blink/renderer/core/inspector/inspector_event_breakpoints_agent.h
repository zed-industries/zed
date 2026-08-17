// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_EVENT_BREAKPOINTS_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_EVENT_BREAKPOINTS_AGENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/inspector/inspector_base_agent.h"
#include "third_party/blink/renderer/core/inspector/inspector_session_state.h"
#include "third_party/blink/renderer/core/inspector/protocol/event_breakpoints.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8-inspector.h"

namespace blink {

namespace probe {
class ExecuteScript;
class UserCallback;
}  // namespace probe

namespace protocol {
class DictionaryValue;
}  // namespace protocol

class CORE_EXPORT InspectorEventBreakpointsAgent final
    : public InspectorBaseAgent<protocol::EventBreakpoints::Metainfo> {
 public:
  explicit InspectorEventBreakpointsAgent(v8_inspector::V8InspectorSession*);
  ~InspectorEventBreakpointsAgent() override;

  // Instrumentation probe methods.
  void DidCreateCanvasContext();
  void DidCreateOffscreenCanvasContext();
  void DidFireWebGLError(const String& error_name);
  void DidFireWebGLWarning();
  void ScriptExecutionBlockedByCSP(const String& directive_text);
  void DidFireWebGLErrorOrWarning(const String& message);
  void Will(const probe::ExecuteScript&);
  void Did(const probe::ExecuteScript&);
  void Will(const probe::UserCallback&);
  void Did(const probe::UserCallback&);
  void BreakableLocation(const char* name);
  void DidCreateAudioContext();
  void DidCloseAudioContext();
  void DidResumeAudioContext();
  void DidSuspendAudioContext();

 private:
  // InspectorBaseAgent overrides.
  protocol::Response disable() override;
  void Restore() override;

  // Protocol methods implementation.
  protocol::Response setInstrumentationBreakpoint(
      const String& event_name) override;
  protocol::Response removeInstrumentationBreakpoint(
      const String& event_name) override;

  bool IsEnabled() const;
  std::unique_ptr<protocol::DictionaryValue> MaybeBuildBreakpointData(
      const String& name);

  void TriggerSyncBreakpoint(const protocol::DictionaryValue& breakpoint_data);
  void ScheduleAsyncBreakpoint(
      const protocol::DictionaryValue& breakpoint_data);
  void UnscheduleAsyncBreakpoint();

  v8_inspector::V8InspectorSession* const v8_session_;
  InspectorAgentState::BooleanMap event_listener_breakpoints_{
      &agent_state_, /*default_value=*/false};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_EVENT_BREAKPOINTS_AGENT_H_
