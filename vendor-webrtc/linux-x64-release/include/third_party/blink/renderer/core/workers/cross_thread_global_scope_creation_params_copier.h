// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_CROSS_THREAD_GLOBAL_SCOPE_CREATION_PARAMS_COPIER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_CROSS_THREAD_GLOBAL_SCOPE_CREATION_PARAMS_COPIER_H_

#include "services/network/public/mojom/content_security_policy.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/workers/global_scope_creation_params.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_copier.h"

namespace WTF {

template <>
struct CrossThreadCopier<
    Vector<network::mojom::blink::ContentSecurityPolicyPtr>> {
  STATIC_ONLY(CrossThreadCopier);
  using Type = Vector<network::mojom::blink::ContentSecurityPolicyPtr>;
  static Type Copy(const Type&);
};

template <>
struct CrossThreadCopier<std::unique_ptr<blink::GlobalScopeCreationParams>> {
  STATIC_ONLY(CrossThreadCopier);
  using Type = std::unique_ptr<blink::GlobalScopeCreationParams>;
  static Type Copy(Type);
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_CROSS_THREAD_GLOBAL_SCOPE_CREATION_PARAMS_COPIER_H_
