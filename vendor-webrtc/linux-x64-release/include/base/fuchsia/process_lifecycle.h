// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FUCHSIA_PROCESS_LIFECYCLE_H_
#define BASE_FUCHSIA_PROCESS_LIFECYCLE_H_

#include <fidl/fuchsia.process.lifecycle/cpp/fidl.h>
#include <lib/fidl/cpp/binding.h>
#include <lib/fidl/cpp/wire/channel.h>

#include <optional>

#include "base/base_export.h"
#include "base/functional/callback.h"

namespace base {

// Registers a fuchsia.process.lifecycle.Lifecycle protocol implementation to
// receive graceful termination requests from the Component Framework v2
// ELF executable runner.
//
// The implementation consumes the PA_LIFECYCLE handle, which the ELF runner
// will provide only if the Component manifest contains a lifecycle/stop_event
// registration.
class BASE_EXPORT ProcessLifecycle final
    : public fidl::Server<fuchsia_process_lifecycle::Lifecycle> {
 public:
  explicit ProcessLifecycle(base::OnceClosure on_stop);
  ~ProcessLifecycle() override;

  ProcessLifecycle(const ProcessLifecycle&) = delete;
  ProcessLifecycle& operator=(const ProcessLifecycle&) = delete;

  // fuchsia_process_lifecycle::Lifecycle implementation.
  void Stop(StopCompleter::Sync& completer) override;

 private:
  base::OnceClosure on_stop_;

  std::optional<fidl::ServerBinding<fuchsia_process_lifecycle::Lifecycle>>
      binding_;
};

}  // namespace base

#endif  // BASE_FUCHSIA_PROCESS_LIFECYCLE_H_
