// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_WORKER_RESOURCE_TIMING_NOTIFIER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_WORKER_RESOURCE_TIMING_NOTIFIER_IMPL_H_

#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/mojom/timing/resource_timing.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_persistent.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/loader/fetch/worker_resource_timing_notifier.h"

namespace blink {

class ExecutionContext;

// The implementation of WorkerResourceTimingNotifier that dispatches resource
// timing info to an execution context which is associated with the instance of
// this class. Thread safety: For the ctor and dtor, these must be called on the
// sequence of the execution context. For AddResourceTiming(), it can be called
// on a different sequence from the sequence of the execution context. In that
// case, this creates a copy of the given resource timing and passes it to the
// execution context's sequence via PostCrossThreadTask.
class CORE_EXPORT WorkerResourceTimingNotifierImpl final
    : public WorkerResourceTimingNotifier {
 public:
  static WorkerResourceTimingNotifierImpl* CreateForInsideResourceFetcher(
      ExecutionContext&);
  static WorkerResourceTimingNotifierImpl* CreateForOutsideResourceFetcher(
      ExecutionContext&);

  // Do not call this. Use static creation function instead. This is public
  // only for MakeGarbageCollected.
  explicit WorkerResourceTimingNotifierImpl(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);
  WorkerResourceTimingNotifierImpl(const WorkerResourceTimingNotifierImpl&) =
      delete;
  WorkerResourceTimingNotifierImpl& operator=(
      const WorkerResourceTimingNotifierImpl&) = delete;
  ~WorkerResourceTimingNotifierImpl() override = default;

  void AddResourceTiming(mojom::blink::ResourceTimingInfoPtr,
                         const AtomicString& initiator_type) override;

  void Trace(Visitor*) const override;

 private:
  void AddCrossThreadResourceTiming(mojom::blink::ResourceTimingInfoPtr,
                                    const String& initiator_type);

  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

  // Used when the execution context lives on the same sequence of this
  // notifier.
  // Note that using CrossThreadWeakPersistent should be fine to hold a
  // reference to an object that lives on the same sequence. Theoretically we
  // don't need to use Member<ExecutionContext> here, but we've seen
  // mysterious crashes when we do so.
  // TODO(crbug.com/959508): Merge |inside_execution_context_| and
  // |outside_execution_context_|.
  Member<ExecutionContext> inside_execution_context_;
  // Used when the execution context lives on a different sequence of this
  // notifier.
  CrossThreadWeakPersistent<ExecutionContext> outside_execution_context_;
};

// NullWorkerResourceTimingNotifier does nothing when AddResourceTiming() is
// called. This is used for toplevel shared/service worker script fetch.
class CORE_EXPORT NullWorkerResourceTimingNotifier final
    : public WorkerResourceTimingNotifier {
 public:
  NullWorkerResourceTimingNotifier() = default;
  NullWorkerResourceTimingNotifier(const NullWorkerResourceTimingNotifier&) =
      delete;
  NullWorkerResourceTimingNotifier& operator=(
      const NullWorkerResourceTimingNotifier&) = delete;
  ~NullWorkerResourceTimingNotifier() override = default;

  void AddResourceTiming(mojom::blink::ResourceTimingInfoPtr,
                         const AtomicString& initiator_type) override {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_WORKER_RESOURCE_TIMING_NOTIFIER_IMPL_H_
