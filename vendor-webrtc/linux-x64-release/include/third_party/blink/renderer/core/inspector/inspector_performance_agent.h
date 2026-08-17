// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_PERFORMANCE_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_PERFORMANCE_AGENT_H_

#include <memory>

#include "base/task/sequence_manager/task_time_observer.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/inspector/inspector_base_agent.h"
#include "third_party/blink/renderer/core/inspector/protocol/performance.h"

namespace blink {

class InspectedFrames;

namespace probe {
class CallFunction;
class ExecuteScript;
class RecalculateStyle;
class UpdateLayout;
class V8Compile;
}  // namespace probe

class CORE_EXPORT InspectorPerformanceAgent final
    : public InspectorBaseAgent<protocol::Performance::Metainfo>,
      public base::sequence_manager::TaskTimeObserver {
 public:
  void Trace(Visitor*) const override;

  explicit InspectorPerformanceAgent(InspectedFrames*);
  InspectorPerformanceAgent(const InspectorPerformanceAgent&) = delete;
  InspectorPerformanceAgent& operator=(const InspectorPerformanceAgent&) =
      delete;
  ~InspectorPerformanceAgent() override;

  void Restore() override;

  // Performance protocol domain implementation.
  protocol::Response enable(std::optional<String> time_domain) override;
  protocol::Response disable() override;
  protocol::Response setTimeDomain(const String& time_domain) override;
  protocol::Response getMetrics(
      std::unique_ptr<protocol::Array<protocol::Performance::Metric>>*
          out_result) override;

  // PerformanceMetrics probes implementation.
  void ConsoleTimeStamp(v8::Isolate* isolate, v8::Local<v8::String> label);
  void Will(const probe::CallFunction&);
  void Did(const probe::CallFunction&);
  void Will(const probe::ExecuteScript&);
  void Did(const probe::ExecuteScript&);
  void Will(const probe::RecalculateStyle&);
  void Did(const probe::RecalculateStyle&);
  void Will(const probe::UpdateLayout&);
  void Did(const probe::UpdateLayout&);
  void Will(const probe::V8Compile&);
  void Did(const probe::V8Compile&);
  void WillStartDebuggerTask();
  void DidFinishDebuggerTask();

  // TaskTimeObserver implementation.
  void WillProcessTask(base::TimeTicks start_time) override;
  void DidProcessTask(base::TimeTicks start_time,
                      base::TimeTicks end_time) override;

 private:
  void ScriptStarts();
  void ScriptEnds();
  void InnerEnable();
  base::TimeTicks GetTimeTicksNow();
  base::TimeTicks GetThreadTimeNow();
  bool HasTimeDomain(const String& time_domain);
  protocol::Response InnerSetTimeDomain(const String& time_domain);

  Member<InspectedFrames> inspected_frames_;
  base::TimeDelta layout_duration_;
  base::TimeTicks layout_start_ticks_;
  base::TimeDelta recalc_style_duration_;
  base::TimeTicks recalc_style_start_ticks_;
  base::TimeDelta script_duration_;
  base::TimeTicks script_start_ticks_;
  base::TimeDelta task_duration_;
  base::TimeTicks task_start_ticks_;
  base::TimeDelta v8compile_duration_;
  base::TimeTicks v8compile_start_ticks_;
  base::TimeDelta devtools_command_duration_;
  base::TimeTicks devtools_command_start_ticks_;
  base::TimeTicks thread_time_origin_;
  uint64_t layout_count_ = 0;
  uint64_t recalc_style_count_ = 0;
  int script_call_depth_ = 0;
  int layout_depth_ = 0;
  InspectorAgentState::Boolean enabled_;
  InspectorAgentState::Boolean use_thread_ticks_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_PERFORMANCE_AGENT_H_
