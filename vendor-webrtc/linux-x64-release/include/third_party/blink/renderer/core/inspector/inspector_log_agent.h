// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_LOG_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_LOG_AGENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/performance_monitor.h"
#include "third_party/blink/renderer/core/inspector/inspector_base_agent.h"
#include "third_party/blink/renderer/core/inspector/protocol/log.h"

namespace v8_inspector {
class V8InspectorSession;
}

namespace blink {

class ConsoleMessage;
class ConsoleMessageStorage;

class CORE_EXPORT InspectorLogAgent
    : public InspectorBaseAgent<protocol::Log::Metainfo>,
      public PerformanceMonitor::Client {
 public:
  InspectorLogAgent(ConsoleMessageStorage*,
                    PerformanceMonitor*,
                    v8_inspector::V8InspectorSession*);
  InspectorLogAgent(const InspectorLogAgent&) = delete;
  InspectorLogAgent& operator=(const InspectorLogAgent&) = delete;
  ~InspectorLogAgent() override;
  void Trace(Visitor*) const override;

  void Restore() override;

  // Called from InspectorInstrumentation.
  void ConsoleMessageAdded(ConsoleMessage*);

  // Protocol methods.
  protocol::Response enable() override;
  protocol::Response disable() override;
  protocol::Response clear() override;
  protocol::Response startViolationsReport(
      std::unique_ptr<protocol::Array<protocol::Log::ViolationSetting>>)
      override;
  protocol::Response stopViolationsReport() override;

 private:
  // PerformanceMonitor::Client implementation.
  void ReportLongLayout(base::TimeDelta duration) override;
  void ReportGenericViolation(PerformanceMonitor::Violation,
                              const String& text,
                              base::TimeDelta time,
                              SourceLocation*) override;
  void InnerEnable();

  Member<ConsoleMessageStorage> storage_;
  Member<PerformanceMonitor> performance_monitor_;
  v8_inspector::V8InspectorSession* v8_session_;
  InspectorAgentState::Boolean enabled_;
  InspectorAgentState::DoubleMap violation_thresholds_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_LOG_AGENT_H_
