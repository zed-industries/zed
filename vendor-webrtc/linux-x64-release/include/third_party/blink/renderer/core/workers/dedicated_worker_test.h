// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_DEDICATED_WORKER_TEST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_DEDICATED_WORKER_TEST_H_

#include "third_party/blink/renderer/core/messaging/blink_transferable_message.h"
#include "third_party/blink/renderer/core/testing/page_test_base.h"
#include "third_party/blink/renderer/core/workers/global_scope_creation_params.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"

namespace blink {

class DedicatedWorker;
class DedicatedWorkerThreadForTest;
class DedicatedWorkerMessagingProxyForTest;

class DedicatedWorkerTest : public PageTestBase {
 public:
  DedicatedWorkerTest() = default;
  explicit DedicatedWorkerTest(
      base::test::TaskEnvironment::TimeSource time_source)
      : PageTestBase(time_source) {}

  void SetUp() override;
  void TearDown() override;

  DedicatedWorker* WorkerObject() { return worker_object_; }
  DedicatedWorkerMessagingProxyForTest* WorkerMessagingProxy();
  DedicatedWorkerThreadForTest* GetWorkerThread();

  void StartWorker(std::unique_ptr<GlobalScopeCreationParams> params = nullptr);
  void EvaluateClassicScript(const String& source_code);
  void WaitUntilWorkerIsRunning();

 protected:
  Event* CreateAndDispatchEvent(
      base::OnceCallback<void(BlinkTransferableMessage)> post_event_callback);

 private:
  Persistent<DedicatedWorker> worker_object_;
  Persistent<DedicatedWorkerMessagingProxyForTest> worker_messaging_proxy_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_DEDICATED_WORKER_TEST_H_
