// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_FETCH_CLIENT_SETTINGS_OBJECT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_FETCH_CLIENT_SETTINGS_OBJECT_H_

#include "services/network/public/mojom/referrer_policy.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/security_context/insecure_request_policy.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/allowed_by_nosniff.h"
#include "third_party/blink/renderer/platform/loader/fetch/https_state.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/weborigin/security_origin.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_copier.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"

namespace blink {

// This is a partial interface of the "settings object" concept defined in the
// HTML spec:
// https://html.spec.whatwg.org/C/#settings-object
//
// This is also a partial interface of the "fetch client settings object" used
// in module script fetch. Other part of the "fetch client settings object" is
// currently implemented by ResourceFetcher and FetchContext, and this class is
// used together with them.
// https://html.spec.whatwg.org/C/#fetch-a-module-worker-script-tree
class PLATFORM_EXPORT FetchClientSettingsObject
    : public GarbageCollected<FetchClientSettingsObject> {
 public:
  virtual ~FetchClientSettingsObject() = default;

  // The URL of the environment settings object's global object.
  // https://html.spec.whatwg.org/#concept-settings-object-global
  //
  // Note: "global object's URL" is not explcitly defined in the spec.
  // This currently returns what ExecutionContext::Url() returns, i.e.
  // - Document's URL
  //   https://dom.spec.whatwg.org/#concept-document-url
  //   but can return a null URL in cases where "about:blank" should have
  //   been returned.
  // - WorkerGlobalScope's URL.
  //   https://html.spec.whatwg.org/#concept-workerglobalscope-url
  // - Worklet's parent Document's URL.
  // TODO(crbug.com/931532): Fix spec issues and make the implementation
  // spec-conformant.
  virtual const KURL& GlobalObjectUrl() const = 0;

  // "A URL used by APIs called by scripts that use this environment settings
  // object to parse URLs."
  // https://html.spec.whatwg.org/C/#api-base-url
  virtual const KURL& BaseUrl() const = 0;

  // "An origin used in security checks."
  // https://html.spec.whatwg.org/C/#concept-settings-object-origin
  virtual const SecurityOrigin* GetSecurityOrigin() const = 0;

  // "The default referrer policy for fetches performed using this environment
  // settings object as a request client."
  // https://html.spec.whatwg.org/C/#concept-settings-object-referrer-policy
  virtual network::mojom::ReferrerPolicy GetReferrerPolicy() const = 0;

  // "referrerURL" used in the "Determine request's Referrer" algorithm:
  // https://w3c.github.io/webappsec-referrer-policy/#determine-requests-referrer
  virtual const String GetOutgoingReferrer() const = 0;

  // https://html.spec.whatwg.org/C/#https-state
  virtual HttpsState GetHttpsState() const = 0;

  // Used for classic top-level scripts and importScripts().
  // TODO(crbug.com/794548): Remove this once we deprecate kLax.
  virtual AllowedByNosniff::MimeTypeCheck MimeTypeCheckForClassicWorkerScript()
      const = 0;

  // https://w3c.github.io/webappsec-upgrade-insecure-requests/#insecure-requests-policy
  virtual mojom::blink::InsecureRequestPolicy GetInsecureRequestsPolicy()
      const = 0;

  // https://w3c.github.io/webappsec-upgrade-insecure-requests/#upgrade-insecure-navigations-set
  using InsecureNavigationsSet = HashSet<unsigned, AlreadyHashedTraits>;
  virtual const InsecureNavigationsSet& GetUpgradeInsecureNavigationsSet()
      const = 0;

  virtual void Trace(Visitor*) const {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_FETCH_CLIENT_SETTINGS_OBJECT_H_
