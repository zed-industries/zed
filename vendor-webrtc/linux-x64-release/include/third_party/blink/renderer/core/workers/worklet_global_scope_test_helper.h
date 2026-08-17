// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKLET_GLOBAL_SCOPE_TEST_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKLET_GLOBAL_SCOPE_TEST_HELPER_H_

#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/renderer/core/workers/worklet_global_scope.h"

namespace blink {

class FakeWorkletGlobalScope final : public WorkletGlobalScope {
  // Inherit the constructor from WorkletGlobalScope.
  using WorkletGlobalScope::WorkletGlobalScope;

  WorkletToken GetWorkletToken() const final { return token_; }
  ExecutionContextToken GetExecutionContextToken() const final {
    return token_;
  }

 private:
  network::mojom::RequestDestination GetDestination() const override {
    return network::mojom::RequestDestination::kScript;
  }

  // A fake token identifying this worker. This is default constructed to a
  // valid token.
  const AnimationWorkletToken token_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKLET_GLOBAL_SCOPE_TEST_HELPER_H_
