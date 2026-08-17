// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_PERFORMANCE_TIMELINE_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_PERFORMANCE_TIMELINE_AGENT_H_

#include <memory>

#include "base/task/sequence_manager/task_time_observer.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/inspector/inspector_base_agent.h"
#include "third_party/blink/renderer/core/inspector/inspector_session_state.h"
#include "third_party/blink/renderer/core/inspector/protocol/performance_timeline.h"

namespace blink {

class ExecutionContext;
class InspectedFrames;
class PerformanceEntry;

class CORE_EXPORT InspectorPerformanceTimelineAgent final
    : public InspectorBaseAgent<protocol::PerformanceTimeline::Metainfo> {
 public:
  explicit InspectorPerformanceTimelineAgent(InspectedFrames*);
  InspectorPerformanceTimelineAgent(const InspectorPerformanceTimelineAgent&) =
      delete;
  InspectorPerformanceTimelineAgent& operator=(
      const InspectorPerformanceTimelineAgent&) = delete;
  ~InspectorPerformanceTimelineAgent() override;

  // PerformanceTimeline probes implementation.
  void PerformanceEntryAdded(ExecutionContext*, PerformanceEntry*);

  void Trace(Visitor*) const override;

 private:
  // Performance protocol domain implementation.
  protocol::Response enable(
      std::unique_ptr<protocol::Array<String>> event_types) override;
  protocol::Response disable() override;
  void Restore() override;

  void InnerEnable();
  bool IsEnabled() const;

  using EventsVector =
      protocol::Array<protocol::PerformanceTimeline::TimelineEvent>;
  void CollectEntries(AtomicString type, EventsVector* events);

  Member<InspectedFrames> inspected_frames_;
  InspectorAgentState::Integer enabled_types_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_PERFORMANCE_TIMELINE_AGENT_H_
