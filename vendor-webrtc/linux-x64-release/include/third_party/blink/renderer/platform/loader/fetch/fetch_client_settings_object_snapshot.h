// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_FETCH_CLIENT_SETTINGS_OBJECT_SNAPSHOT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_FETCH_CLIENT_SETTINGS_OBJECT_SNAPSHOT_H_

#include "services/network/public/mojom/referrer_policy.mojom-blink.h"
#include "third_party/blink/public/mojom/security_context/insecure_request_policy.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/loader/fetch/fetch_client_settings_object.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/weborigin/security_origin.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_copier.h"

namespace blink {

// This class is needed to copy a FetchClientSettingsObjectSnapshot across
// threads, because it has some members which cannot be transferred across
// threads (SecurityOrigin for example).
// There are some rules / restrictions:
//   - This struct cannot contain an object that cannot be transferred across
//     threads.
//   - Non-thread-safe members need explicit copying rather than the copy
//     constructor or the assignment operator.
//   - This struct cannot contain any garbage-collected object because this
//     data can be constructed on a thread which runs without Oilpan.
struct CrossThreadFetchClientSettingsObjectData {
  USING_FAST_MALLOC(CrossThreadFetchClientSettingsObjectData);

 public:
  CrossThreadFetchClientSettingsObjectData(
      KURL global_object_url,
      KURL base_url,
      scoped_refptr<const SecurityOrigin> security_origin,
      network::mojom::ReferrerPolicy referrer_policy,
      String outgoing_referrer,
      HttpsState https_state,
      AllowedByNosniff::MimeTypeCheck mime_type_check_for_classic_worker_script,
      mojom::blink::InsecureRequestPolicy insecure_requests_policy,
      FetchClientSettingsObject::InsecureNavigationsSet
          insecure_navigations_set)
      : global_object_url(std::move(global_object_url)),
        base_url(std::move(base_url)),
        security_origin(std::move(security_origin)),
        referrer_policy(referrer_policy),
        outgoing_referrer(std::move(outgoing_referrer)),
        https_state(https_state),
        mime_type_check_for_classic_worker_script(
            mime_type_check_for_classic_worker_script),
        insecure_requests_policy(insecure_requests_policy),
        insecure_navigations_set(std::move(insecure_navigations_set)) {}
  CrossThreadFetchClientSettingsObjectData(
      const CrossThreadFetchClientSettingsObjectData&) = delete;
  CrossThreadFetchClientSettingsObjectData& operator=(
      const CrossThreadFetchClientSettingsObjectData&) = delete;

  const KURL global_object_url;
  const KURL base_url;
  const scoped_refptr<const SecurityOrigin> security_origin;
  const network::mojom::ReferrerPolicy referrer_policy;
  const String outgoing_referrer;
  const HttpsState https_state;
  const AllowedByNosniff::MimeTypeCheck
      mime_type_check_for_classic_worker_script;
  const mojom::blink::InsecureRequestPolicy insecure_requests_policy;
  const FetchClientSettingsObject::InsecureNavigationsSet
      insecure_navigations_set;
};

// This takes a partial snapshot of the execution context's states so that an
// instance of this class can be passed to another thread without cross-thread
// synchronization. Don't keep this object persistently, instead create a new
// instance per each "fetch a module script graph" algorithm:
// https://html.spec.whatwg.org/C/#fetch-a-module-script-tree
// https://html.spec.whatwg.org/C/#fetch-a-module-worker-script-tree
//
// This class should be used only for main worker (worklet) script loading. For
// other resources, FetchClientSettingsObjectImpl should be used. See the class
// level comments on that class.
class PLATFORM_EXPORT FetchClientSettingsObjectSnapshot final
    : public FetchClientSettingsObject {
 public:
  explicit FetchClientSettingsObjectSnapshot(const FetchClientSettingsObject&);
  explicit FetchClientSettingsObjectSnapshot(
      std::unique_ptr<CrossThreadFetchClientSettingsObjectData>);
  FetchClientSettingsObjectSnapshot(
      const KURL& global_object_url,
      const KURL& base_url,
      const scoped_refptr<const SecurityOrigin> security_origin,
      network::mojom::ReferrerPolicy referrer_policy,
      const String& outgoing_referrer,
      HttpsState https_state,
      AllowedByNosniff::MimeTypeCheck,
      mojom::blink::InsecureRequestPolicy,
      InsecureNavigationsSet);

  ~FetchClientSettingsObjectSnapshot() override = default;

  const KURL& GlobalObjectUrl() const override { return global_object_url_; }
  const KURL& BaseUrl() const override { return base_url_; }
  const SecurityOrigin* GetSecurityOrigin() const override {
    return security_origin_.get();
  }
  network::mojom::ReferrerPolicy GetReferrerPolicy() const override {
    return referrer_policy_;
  }
  const String GetOutgoingReferrer() const override {
    return outgoing_referrer_;
  }
  HttpsState GetHttpsState() const override { return https_state_; }

  mojom::blink::InsecureRequestPolicy GetInsecureRequestsPolicy()
      const override {
    return insecure_requests_policy_;
  }

  const InsecureNavigationsSet& GetUpgradeInsecureNavigationsSet()
      const override {
    return insecure_navigations_set_;
  }

  AllowedByNosniff::MimeTypeCheck MimeTypeCheckForClassicWorkerScript()
      const override {
    return mime_type_check_for_classic_worker_script_;
  }

  // Gets a copy of the data suitable for passing to another thread.
  std::unique_ptr<CrossThreadFetchClientSettingsObjectData> CopyData() const {
    return std::make_unique<CrossThreadFetchClientSettingsObjectData>(
        global_object_url_, base_url_, security_origin_->IsolatedCopy(),
        referrer_policy_, outgoing_referrer_, https_state_,
        mime_type_check_for_classic_worker_script_, insecure_requests_policy_,
        insecure_navigations_set_);
  }

 private:
  const KURL global_object_url_;
  const KURL base_url_;
  const scoped_refptr<const SecurityOrigin> security_origin_;
  const network::mojom::ReferrerPolicy referrer_policy_;
  const String outgoing_referrer_;
  const HttpsState https_state_;
  const AllowedByNosniff::MimeTypeCheck
      mime_type_check_for_classic_worker_script_;

  const mojom::blink::InsecureRequestPolicy insecure_requests_policy_;
  const InsecureNavigationsSet insecure_navigations_set_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_FETCH_CLIENT_SETTINGS_OBJECT_SNAPSHOT_H_
