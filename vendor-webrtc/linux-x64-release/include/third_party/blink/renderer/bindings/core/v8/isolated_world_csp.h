// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_ISOLATED_WORLD_CSP_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_ISOLATED_WORLD_CSP_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/weborigin/security_origin.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ContentSecurityPolicy;
class LocalDOMWindow;

// A singleton storing content security policy for each isolated world.
class CORE_EXPORT IsolatedWorldCSP {
  USING_FAST_MALLOC(IsolatedWorldCSP);

 public:
  static IsolatedWorldCSP& Get();

  IsolatedWorldCSP(const IsolatedWorldCSP&) = delete;
  IsolatedWorldCSP& operator=(const IsolatedWorldCSP&) = delete;

  // Associated an isolated world with a Content Security Policy. Resources
  // embedded into the main world's DOM from script executed in an isolated
  // world should be restricted based on the isolated world's CSP, not the
  // main world's.
  //
  // Note: If |policy| is null, the PolicyInfo for |world_id| is cleared. If
  // |policy| is specified, |self_origin| must not be null.
  void SetContentSecurityPolicy(int32_t world_id,
                                const String& policy,
                                scoped_refptr<SecurityOrigin> self_origin);
  bool HasContentSecurityPolicy(int32_t world_id) const;

  // Creates a ContentSecurityPolicy instance for the given isolated |world_id|
  // and |window|. Returns null if no ContentSecurityPolicy is defined for the
  // given isolated |world_id|.
  ContentSecurityPolicy* CreateIsolatedWorldCSP(LocalDOMWindow& window,
                                                int32_t world_id);

 private:
  struct PolicyInfo {
    String policy;
    scoped_refptr<SecurityOrigin> self_origin;
  };

  IsolatedWorldCSP();

  // Map from the isolated world |world_id| to its PolicyInfo.
  HashMap<int, PolicyInfo> csp_map_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_ISOLATED_WORLD_CSP_H_
