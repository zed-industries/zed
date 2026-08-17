// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_FAKE_MOJO_BINDING_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_FAKE_MOJO_BINDING_CONTEXT_H_

#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/mojo/mojo_binding_context.h"

namespace blink {

// A simple MojoBindingContext implementation suitable for use in unit tests.
class FakeMojoBindingContext : public GarbageCollected<FakeMojoBindingContext>,
                               public MojoBindingContext {
  USING_PRE_FINALIZER(FakeMojoBindingContext, Dispose);

 public:
  explicit FakeMojoBindingContext(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);

  const BrowserInterfaceBrokerProxy& GetBrowserInterfaceBroker() const override;
  scoped_refptr<base::SingleThreadTaskRunner> GetTaskRunner(TaskType) override;

  void Dispose();

 private:
  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_FAKE_MOJO_BINDING_CONTEXT_H_
