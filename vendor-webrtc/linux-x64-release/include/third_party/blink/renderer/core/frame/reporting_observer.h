// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_REPORTING_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_REPORTING_OBSERVER_H_

#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_reporting_observer_callback.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_reporting_observer_options.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/frame/report.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ExecutionContext;
class Report;
class V8ReportingObserverCallback;

class CORE_EXPORT ReportingObserver final
    : public ScriptWrappable,
      public ActiveScriptWrappable<ReportingObserver>,
      public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static ReportingObserver* Create(ExecutionContext*,
                                   V8ReportingObserverCallback*,
                                   ReportingObserverOptions*);

  explicit ReportingObserver(ExecutionContext*,
                             V8ReportingObserverCallback*,
                             ReportingObserverOptions*);

  // ActiveScriptWrappable
  bool HasPendingActivity() const final;

  // Call the callback with all reports in |report_queue_|.
  void ReportToCallback();

  // Queues a report to be reported via callback soon (possibly in a batch).
  void QueueReport(Report* report);

  // Returns whether this ReportingObserver observes reports of the type |type|,
  // based on the |types| option.
  bool ObservedType(const String& type);

  // Returns the state of the |buffered| option.
  bool Buffered();

  // Sets the |buffered| option to false. This should be called after queueing
  // all buffered reports, so that they are not reported multiple times.
  void ClearBuffered();

  void observe();
  void disconnect();
  HeapVector<Member<Report>> takeRecords();

  void Trace(Visitor*) const override;

 private:
  Member<ExecutionContext> execution_context_;
  Member<V8ReportingObserverCallback> callback_;
  Member<ReportingObserverOptions> options_;
  HeapVector<Member<Report>> report_queue_;
  bool registered_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_REPORTING_OBSERVER_H_
