// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROCESS_CURRENT_PROCESS_H_
#define BASE_PROCESS_CURRENT_PROCESS_H_

#include <atomic>
#include <string>

#include "base/base_export.h"
#include "base/memory/raw_ptr.h"
#include "base/no_destructor.h"
#include "base/process/process_handle.h"
#include "base/synchronization/lock.h"
#include "base/trace_event/base_tracing.h"
#include "build/buildflag.h"

namespace tracing {
class TraceEventDataSource;
class CustomEventRecorder;
class TrackNameRecorder;
}  // namespace tracing

namespace mojo::core {
class Channel;
}

namespace network {
class ContentDecodingInterceptor;
}  // namespace network

namespace base {
namespace test {
class CurrentProcessForTest;
}  // namespace test

using CurrentProcessType =
    perfetto::protos::pbzero::ChromeProcessDescriptor::ProcessType;

// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
// Use coalesced service process for recording histograms.
enum class ShortProcessType {
  kUnspecified = 0,
  kBrowser = 1,
  kRenderer = 2,
  kUtility = 3,
  kZygote = 4,
  kSandboxHelper = 5,
  kGpu = 6,
  kPpapiPlugin = 7,
  kPpapiBroker = 8,
  kServiceNetwork = 9,
  kServiceStorage = 10,
  kService = 11,
  kRendererExtension = 12,
  kMaxValue = kRendererExtension,
};

// CurrentProcess class provides access to set of current process properties
// which are accessible only from the process itself (e.g. ProcessType,
// ProcessName).
// See base::CurrentThread for access to properties of the running
// thread and base::Process::Current for the properties which are known both
// from within and without the process (e.g. pid).
class BASE_EXPORT CurrentProcess {
 public:
  static CurrentProcess& GetInstance();

  CurrentProcess(const CurrentProcess&) = delete;
  CurrentProcess& operator=(const CurrentProcess&) = delete;
  ~CurrentProcess();

  bool operator==(const CurrentProcess& other) const;

  class TypeKey {
   private:
    TypeKey() = default;
    friend class ::base::test::CurrentProcessForTest;
    friend class ::tracing::TraceEventDataSource;
    friend class ::tracing::CustomEventRecorder;
    friend class ::tracing::TrackNameRecorder;
    friend class ::mojo::core::Channel;
    friend class ::network::ContentDecodingInterceptor;
  };
  // Returns an enum corresponding to the type of the current process (e.g.
  // browser / renderer / utility / etc). It can be used in metrics or tracing
  // code — for example, to split a number of low-level events with
  // process-type-agnostic implementation (e.g. number of posted tasks) by
  // process type for diagnostic purposes.
  // To avoid layering violations (i.e. //base or other low-level code modifying
  // its behaviour based on the //chrome or //content-level concepts like a
  // "browser" or "renderer" process), the access to this function is controlled
  // by an explicit list.
  CurrentProcessType GetType(TypeKey key) {
    return process_type_.load(std::memory_order_relaxed);
  }

  ShortProcessType GetShortType(TypeKey key);

  class NameKey {
   private:
    NameKey() = default;
    friend class ::base::test::CurrentProcessForTest;
    friend class ::tracing::TraceEventDataSource;
    friend class ::tracing::TrackNameRecorder;
  };
  std::string GetName(NameKey key) {
    AutoLock lock(lock_);
    return process_name_;
  }

  class BASE_EXPORT Delegate {
   public:
    // Called on the main thread of the process whose name is changing,
    // immediately after the name is set.
    virtual void OnProcessNameChanged(const std::string& process_name,
                                      CurrentProcessType process_type) = 0;

   protected:
    ~Delegate() = default;
  };

  // Sets the name and type of the process for the metrics and tracing. This
  // function should be called as early as possible in the process's lifetime
  // before starting any threads, typically in *Main() function. Provide
  // process_name as an argument if it can't be trivially derived from the
  // process type.
  void SetProcessType(CurrentProcessType process_type);

  // `delegate` might racily be invoked after resetting, thus its lifetime must
  // match `CurrentProcess`.
  void SetDelegate(Delegate* delegate, NameKey key);

  bool IsProcessNameEmpty() const {
    AutoLock lock(lock_);
    return process_name_.empty();
  }

 private:
  friend class base::NoDestructor<CurrentProcess>;

  CurrentProcess() = default;

  void SetProcessNameAndType(const std::string& process_name,
                             CurrentProcessType process_type);

  mutable Lock lock_;
  std::string process_name_;
  // The process_type_ is set at the startup before processes start running.
  // However, since it runs in multi-threaded environment and if has to be
  // changed later, we would want well-defined behaviour even if one thread
  // writes while another reads. There are some processes (e.g. Service process)
  // where we don't have a guarantee that it will be called early enough in the
  // process's lifetime, thus we use std::atomic here.
  std::atomic<CurrentProcessType> process_type_;

  raw_ptr<Delegate> delegate_;
};

}  // namespace base

#endif  // BASE_PROCESS_CURRENT_PROCESS_H_
