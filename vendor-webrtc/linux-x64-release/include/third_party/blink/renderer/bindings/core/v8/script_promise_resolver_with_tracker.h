// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_PROMISE_RESOLVER_WITH_TRACKER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_PROMISE_RESOLVER_WITH_TRACKER_H_

#include "base/metrics/histogram_functions.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"

namespace blink {

// ScriptPromiseResolverWithTracker is a wrapper around
// ScriptPromiseResolverBase which simplifies recording UMA metric and latency
// for APIs.

// Callers should ensure that the ResultEnumType has kOk and kTimedOut as
// values. This is a sample usage CL: http://crrev.com/c/4053546.
template <typename ResultEnumType, typename IDLResolvedType>
class CORE_EXPORT ScriptPromiseResolverWithTracker
    : public GarbageCollected<
          ScriptPromiseResolverWithTracker<ResultEnumType, IDLResolvedType>>,
      public ExecutionContextLifecycleObserver {
 public:
  // For a given metric |metric_name_prefix|, this class will record
  // "|metric_name_prefix|.Result" and "|metric_name_prefix|.Latency",
  // or "|metric_name_prefix|.|result_suffix_|" if a custom result-suffix
  // is specified.
  //
  // For example, if the targeted histograms are
  // "WebRTC.EnumerateDevices.Result" and "WebRTC.EnumerateDevices.Latency",
  // then |metric_name_prefix| should be provided as "WebRTC.EnumerateDevices".
  //
  // |timeout_interval| is the timeout limit after which a
  // ResultEnumType::kTimedOut response is recorded in the Result histogram.
  //
  // This creates/accesses the Latency histogram which has |n_buckets_| buckets
  // and the range of the buckets are from (min_latency_bucket,
  // max_latency_bucket).
  ScriptPromiseResolverWithTracker(
      ScriptState* script_state,
      std::string metric_name_prefix,
      base::TimeDelta timeout_interval,
      base::TimeDelta min_latency_bucket = base::Milliseconds(1),
      base::TimeDelta max_latency_bucket = base::Seconds(10),
      size_t n_buckets = 50)
      : ExecutionContextLifecycleObserver(nullptr),
        metric_name_prefix_(std::move(metric_name_prefix)),
        start_time_(base::TimeTicks::Now()),
        min_latency_bucket_(min_latency_bucket),
        max_latency_bucket_(max_latency_bucket),
        n_buckets_(n_buckets) {
    DCHECK(!metric_name_prefix_.empty());
    resolver_ = MakeGarbageCollected<ScriptPromiseResolver<IDLResolvedType>>(
        script_state);
    if (timeout_interval.is_positive()) {
      auto* execution_context = ExecutionContext::From(script_state);
      // We're goging to keep this class alive for the duration of timeout,
      // so observe execution context to detach the underlying resolver when
      // context is gone.
      SetExecutionContext(execution_context);
      execution_context->GetTaskRunner(TaskType::kInternalDefault)
          ->PostDelayedTask(
              FROM_HERE,
              WTF::BindOnce(&ScriptPromiseResolverWithTracker::RecordResult,
                            WrapPersistent(this), ResultEnumType::kTimedOut),
              timeout_interval);
    }
  }

  ScriptPromiseResolverWithTracker(const ScriptPromiseResolverWithTracker&) =
      delete;
  ScriptPromiseResolverWithTracker& operator=(
      const ScriptPromiseResolverWithTracker&) = delete;
  ~ScriptPromiseResolverWithTracker() override = default;

  template <typename T>
  void Resolve(T value, ResultEnumType result = ResultEnumType::kOk) {
    RecordResultAndLatency(result);
    resolver_->Resolve(value);
  }

  template <typename IDLRejectType, typename T>
  void Reject(T value, ResultEnumType result) {
    RecordResultAndLatency(result);
    resolver_->template Reject<IDLRejectType>(value);
  }

  void SetResultSuffix(std::string result_suffix) {
    CHECK(!result_suffix.empty());
    result_suffix_ = std::move(result_suffix);
  }

  void RecordAndThrowDOMException(ExceptionState& exception_state,
                                  DOMExceptionCode exception_code,
                                  const String& message,
                                  ResultEnumType result) {
    RecordResultAndLatency(result);
    exception_state.ThrowDOMException(exception_code, message);
    resolver_->Detach();
  }

  void RecordAndThrowTypeError(ExceptionState& exception_state,
                               const String& message,
                               ResultEnumType result) {
    RecordResultAndLatency(result);
    exception_state.ThrowTypeError(message);
    resolver_->Detach();
  }

  void RecordAndDetach(ResultEnumType result) {
    RecordResultAndLatency(result);
    resolver_->Detach();
  }

  void Resolve() { Resolve(ToV8UndefinedGenerator()); }

  void RecordResultAndLatency(ResultEnumType result) {
    RecordResult(result);
    RecordLatency();
  }

  void RecordResult(ResultEnumType result) {
    if (is_result_recorded_)
      return;

    is_result_recorded_ = true;
    base::UmaHistogramEnumeration(metric_name_prefix_ + "." + result_suffix_,
                                  result);
  }

  void RecordLatency() {
    if (is_latency_recorded_)
      return;

    is_latency_recorded_ = true;
    const base::TimeDelta elapsed = base::TimeTicks::Now() - start_time_;
    base::UmaHistogramCustomTimes(metric_name_prefix_ + ".Latency", elapsed,
                                  min_latency_bucket_, max_latency_bucket_,
                                  n_buckets_);
  }

  ScriptState* GetScriptState() const { return resolver_->GetScriptState(); }

  ScriptPromise<IDLResolvedType> Promise() { return resolver_->Promise(); }

  void Trace(Visitor* visitor) const override {
    ExecutionContextLifecycleObserver::Trace(visitor);
    visitor->Trace(resolver_);
  }

 private:
  void ContextDestroyed() override { resolver_->Detach(); }

  Member<ScriptPromiseResolver<IDLResolvedType>> resolver_;
  const std::string metric_name_prefix_;
  const base::TimeTicks start_time_;
  const base::TimeDelta min_latency_bucket_;
  const base::TimeDelta max_latency_bucket_;
  const size_t n_buckets_;
  std::string result_suffix_ = "Result";  // Mutable through SetResultSuffix().
  bool is_latency_recorded_ = false;
  bool is_result_recorded_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_PROMISE_RESOLVER_WITH_TRACKER_H_
