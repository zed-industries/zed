// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_REPORTING_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_REPORTING_CONTEXT_H_

#include "third_party/blink/public/mojom/frame/reporting_observer.mojom-blink.h"
#include "third_party/blink/public/mojom/reporting/reporting.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_linked_hash_set.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver_set.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExecutionContext;
class Report;
class ReportingObserver;

// ReportingContext processes all reports for an ExecutionContext, and serves as
// a container for all active ReportingObservers on that ExecutionContext.
class CORE_EXPORT ReportingContext : public GarbageCollected<ReportingContext>,
                                     public mojom::blink::ReportingObserver,
                                     public Supplement<ExecutionContext> {
 public:
  static const char kSupplementName[];

  explicit ReportingContext(ExecutionContext&);

  // Returns the ReportingContext for an ExecutionContext. If one does not
  // already exist for the given context, one is created.
  static ReportingContext* From(ExecutionContext*);
  static ReportingContext* From(const ExecutionContext* context) {
    return ReportingContext::From(const_cast<ExecutionContext*>(context));
  }
  void Bind(mojo::PendingReceiver<mojom::blink::ReportingObserver> receiver);

  // Queues a report for the Reporting API and in all registered observers.
  virtual void QueueReport(Report*,
                           const Vector<String>& endpoints = {"default"});

  void RegisterObserver(blink::ReportingObserver*);
  void UnregisterObserver(blink::ReportingObserver*);

  // mojom::blink::ReportingObserver implementation.
  void Notify(mojom::blink::ReportPtr report) override;

  void Trace(Visitor*) const override;

 private:
  // Counts the use of a report type via UseCounter.
  void CountReport(Report*);

  const HeapMojoRemote<mojom::blink::ReportingServiceProxy>&
  GetReportingService() const;

  void NotifyInternal(Report* report);
  // Send |report| via the Reporting API to |endpoint|.
  void SendToReportingAPI(Report* report, const String& endpoint) const;

  HeapLinkedHashSet<Member<blink::ReportingObserver>> observers_;
  HeapHashMap<String, Member<GCedHeapLinkedHashSet<Member<Report>>>>
      report_buffer_;
  Member<ExecutionContext> execution_context_;

  // This is declared mutable so that the service endpoint can be cached by
  // const methods.
  mutable HeapMojoRemote<mojom::blink::ReportingServiceProxy>
      reporting_service_;

  // There might be up to two ReportingObservers stored here: one that is called
  // from the CrossOriginEmbedderPolicyReporter and one that is called from the
  // DocumentIsolationPolicyReporter.
  HeapMojoReceiverSet<mojom::blink::ReportingObserver, ReportingContext>
      receivers_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_REPORTING_CONTEXT_H_
