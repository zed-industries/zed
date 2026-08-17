// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_FETCH_CLIENT_SETTINGS_OBJECT_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_FETCH_CLIENT_SETTINGS_OBJECT_IMPL_H_

#include "services/network/public/mojom/referrer_policy.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/security_context/insecure_request_policy.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/loader/fetch/fetch_client_settings_object.h"
#include "third_party/blink/renderer/platform/loader/fetch/https_state.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/weborigin/security_origin.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_copier.h"

namespace blink {

class ExecutionContext;

// This is an implementation of FetchClientSettingsObject. As opposed to
// FetchClientSettingsObjectSnapshot, this refers to up-to-date values of the
// settings object.
//
// This class should be used for resource loading other than main worker
// (worklet) scripts. For the main scripts, FetchClientSettingsObjectSnapshot
// should be used. See the class level comments on that class.
class CORE_EXPORT FetchClientSettingsObjectImpl final
    : public FetchClientSettingsObject {
 public:
  explicit FetchClientSettingsObjectImpl(ExecutionContext&);
  ~FetchClientSettingsObjectImpl() override = default;

  const KURL& GlobalObjectUrl() const override;
  const KURL& BaseUrl() const override;
  const SecurityOrigin* GetSecurityOrigin() const override;
  network::mojom::ReferrerPolicy GetReferrerPolicy() const override;

  const String GetOutgoingReferrer() const override;

  HttpsState GetHttpsState() const override;

  AllowedByNosniff::MimeTypeCheck MimeTypeCheckForClassicWorkerScript()
      const override;

  mojom::blink::InsecureRequestPolicy GetInsecureRequestsPolicy()
      const override;
  const InsecureNavigationsSet& GetUpgradeInsecureNavigationsSet()
      const override;

  void Trace(Visitor* visitor) const override;

 private:
  Member<ExecutionContext> execution_context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_FETCH_CLIENT_SETTINGS_OBJECT_IMPL_H_
