// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_TRACING_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_TRACING_HELPER_H_

#include <string>

#include "base/compiler_specific.h"
#include "base/memory/raw_ptr.h"
#include "base/time/time.h"
#include "base/trace_event/trace_event.h"
#include "base/trace_event/typed_macros.h"
#include "third_party/blink/public/platform/task_type.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/perfetto/include/perfetto/tracing/event_context.h"
#include "third_party/perfetto/include/perfetto/tracing/traced_value.h"
#include "third_party/perfetto/include/perfetto/tracing/track.h"
#include "third_party/perfetto/protos/perfetto/trace/track_event/track_event.pbzero.h"

namespace blink {
namespace scheduler {

PLATFORM_EXPORT perfetto::StaticString YesNoStateToString(bool is_yes);

PLATFORM_EXPORT
perfetto::protos::pbzero::RendererMainThreadTaskExecution::TaskType
TaskTypeToProto(TaskType task_type);

class TraceableVariable;

PLATFORM_EXPORT perfetto::NamedTrack MakeNamedTrack(
    perfetto::StaticString name,
    const void* ptr,
    perfetto::Track parent = perfetto::ThreadTrack::Current());
PLATFORM_EXPORT perfetto::CounterTrack MakeCounterTrack(
    perfetto::StaticString name,
    const void* ptr,
    perfetto::Track parent = perfetto::ThreadTrack::Current());

// Unfortunately, using |base::trace_event::TraceLog::EnabledStateObserver|
// wouldn't be helpful in our case because removing one takes linear time
// and tracers may be created and disposed frequently.
class PLATFORM_EXPORT TraceableVariableController {
  DISALLOW_NEW();

 public:
  TraceableVariableController();
  ~TraceableVariableController();

  // Not thread safe.
  void RegisterTraceableVariable(TraceableVariable* traceable_variable);
  void DeregisterTraceableVariable(TraceableVariable* traceable_variable);

  void OnTraceLogEnabled();

 private:
  HashSet<TraceableVariable*> traceable_variables_;
};

// A wrapper for string literal used in template arguments.
template <size_t N>
class TracingCategory {
 public:
  constexpr TracingCategory(const char (&str)[N]) {
    std::copy_n(str, N, value);
  }

  char value[N];
};

class TraceableVariable {
 public:
  TraceableVariable(TraceableVariableController* controller)
      : controller_(controller) {
    controller_->RegisterTraceableVariable(this);
  }

  virtual ~TraceableVariable() {
    controller_->DeregisterTraceableVariable(this);
  }

  virtual void OnTraceLogEnabled() = 0;

 private:
  const raw_ptr<TraceableVariableController> controller_;  // Not owned.
};

// TODO(kraynov): Rename to something less generic and reflecting
// the enum nature of such variables.
template <typename T, TracingCategory category>
class TraceableState : public TraceableVariable {
 public:
  // Converter must return compile-time defined const strings because tracing
  // will not make a copy of them.
  using ConverterFuncPtr = perfetto::StaticString (*)(T);

  TraceableState(T initial_state,
                 perfetto::NamedTrack track,
                 TraceableVariableController* controller,
                 ConverterFuncPtr converter)
      : TraceableVariable(controller),
        converter_(converter),
        state_(initial_state),
        track_(track) {
    Trace();
  }

  TraceableState(const TraceableState&) = delete;

  ~TraceableState() override {
    if (slice_is_open_) {
      TRACE_EVENT_END(category.value, track_);
    }
  }

  TraceableState& operator=(const T& value) {
    Assign(value);
    return *this;
  }
  TraceableState& operator=(const TraceableState& another) {
    Assign(another.state_);
    return *this;
  }

  operator T() const { return state_; }
  const T& get() const { return state_; }

  void OnTraceLogEnabled() final { Trace(); }

  // TraceableState<T> is serialisable into trace iff T is serialisable.
  template <typename V = T>
  typename perfetto::check_traced_value_support<V>::type WriteIntoTrace(
      perfetto::TracedValue context) const {
    perfetto::WriteIntoTracedValue(std::move(context), get());
  }

 protected:
  void Assign(T new_state) {
    if (state_ != new_state) {
      state_ = new_state;
      Trace();
    }
  }

 private:
  void Trace() {
    if (!TRACE_EVENT_CATEGORY_ENABLED(category.value)) {
      return;
    }
    perfetto::StaticString state_str = converter_(state_);
    if (slice_is_open_) {
      TRACE_EVENT_END(category.value, track_);
    }
    slice_is_open_ = !!state_str;
    if (state_str) {
      TRACE_EVENT_BEGIN(category.value, state_str, track_);
    }
  }

  const ConverterFuncPtr converter_;

  T state_;
  perfetto::NamedTrack track_;

  // We have to track whether slice is open to avoid confusion since assignment,
  // "absent" state and OnTraceLogEnabled can happen anytime.
  bool slice_is_open_ = false;
};

template <typename T, TracingCategory category>
class TraceableCounter : public TraceableVariable {
 public:
  using ConverterFuncPtr = double (*)(const T&);

  TraceableCounter(T initial_value,
                   perfetto::CounterTrack track,
                   TraceableVariableController* controller,
                   ConverterFuncPtr converter)
      : TraceableVariable(controller),
        converter_(converter),
        value_(initial_value),
        track_(track) {
    Trace();
  }

  TraceableCounter(T initial_value,
                   perfetto::CounterTrack track,
                   TraceableVariableController* controller)
      : TraceableCounter(initial_value, track, controller, [](const T& value) {
          return static_cast<double>(value);
        }) {}

  TraceableCounter(const TraceableCounter&) = delete;

  TraceableCounter& operator=(const T& value) {
    value_ = value;
    Trace();
    return *this;
  }
  TraceableCounter& operator=(const TraceableCounter& another) {
    value_ = another.value_;
    Trace();
    return *this;
  }

  TraceableCounter& operator+=(const T& value) {
    value_ += value;
    Trace();
    return *this;
  }
  TraceableCounter& operator-=(const T& value) {
    value_ -= value;
    Trace();
    return *this;
  }

  const T& value() const { return value_; }
  const T* operator->() const { return &value_; }
  operator T() const { return value_; }

  void OnTraceLogEnabled() final { Trace(); }

  void Trace() const {
    TRACE_COUNTER(category.value, track_, converter_(value_));
  }

 private:
  const ConverterFuncPtr converter_;

  T value_;
  perfetto::CounterTrack track_;
};

// Add operators when it's needed.

template <typename T, TracingCategory category>
constexpr T operator-(const TraceableCounter<T, category>& counter) {
  return -counter.value();
}

template <typename T, TracingCategory category>
constexpr T operator/(const TraceableCounter<T, category>& lhs, const T& rhs) {
  return lhs.value() / rhs;
}

template <typename T, TracingCategory category>
constexpr bool operator>(const TraceableCounter<T, category>& lhs,
                         const T& rhs) {
  return lhs.value() > rhs;
}

template <typename T, TracingCategory category>
constexpr bool operator<(const TraceableCounter<T, category>& lhs,
                         const T& rhs) {
  return lhs.value() < rhs;
}

template <typename T, TracingCategory category>
constexpr bool operator!=(const TraceableCounter<T, category>& lhs,
                          const T& rhs) {
  return lhs.value() != rhs;
}

template <typename T, TracingCategory category>
constexpr T operator++(TraceableCounter<T, category>& counter) {
  counter = counter.value() + 1;
  return counter.value();
}

template <typename T, TracingCategory category>
constexpr T operator--(TraceableCounter<T, category>& counter) {
  counter = counter.value() - 1;
  return counter.value();
}

template <typename T, TracingCategory category>
constexpr T operator++(TraceableCounter<T, category>& counter, int) {
  T value = counter.value();
  counter = value + 1;
  return value;
}

template <typename T, TracingCategory category>
constexpr T operator--(TraceableCounter<T, category>& counter, int) {
  T value = counter.value();
  counter = value - 1;
  return value;
}

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_TRACING_HELPER_H_
