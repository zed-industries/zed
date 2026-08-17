// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_COOKIE_JAR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_COOKIE_JAR_H_

#include <optional>

#include "mojo/public/cpp/base/shared_memory_version.h"
#include "services/network/public/mojom/restricted_cookie_manager.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {
class Document;
// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
//
// LINT.IfChange(FirstCookieRequest)
enum class FirstCookieRequest {
  kFirstOperationWasSet = 0,
  kFirstOperationWasGet = 1,
  kFirstOperationWasCookiesEnabled = 2,
  kMaxValue = kFirstOperationWasCookiesEnabled,
};
// LINT.ThenChange(//tools/metrics/histograms/metadata/blink/enums.xml:FirstCookieRequest)

class CORE_EXPORT CookieJar : public GarbageCollected<CookieJar> {
 public:
  explicit CookieJar(blink::Document* document);
  virtual ~CookieJar();
  void Trace(Visitor* visitor) const;

  void SetCookie(const String& value);
  String Cookies();
  bool CookiesEnabled();
  void SetCookieManager(
      mojo::PendingRemote<network::mojom::blink::RestrictedCookieManager>
          cookie_manager);

  // Invalidate cached string. To be called explicitly from Document. This is
  // used in cases where a Document action could change the ability for
  // CookieJar to return values to JS without changing the value of the cookies
  // themselves. For example changing storage access can stop the JS from being
  // able to access the document's Cookie without the value ever changing. In
  // that case it's faulty to treat a subsequent request as a cache hit so we
  // invalidate.
  void InvalidateCache();

 private:
  using CookiesResponsePtr = network::mojom::blink::CookiesResponsePtr;

  void OnSetCookieResponse(const KURL& cookie_url,
                           const bool apply_devtools_overrides,
                           CookiesResponsePtr response);

  void RequestRestrictedCookieManagerIfNeeded();
  void OnBackendDisconnect();

  // Returns true if last_cookies_ is not guaranteed to be up to date and an IPC
  // is needed to get the current cookie string.
  bool IPCNeeded(bool should_apply_devtools_overrides);

  // Updates the fake cookie cache after a
  // RestrictedCookieManager::GetCookiesString request returns.
  //
  // We want to evaluate the possible performance gain from having a cookie
  // cache. There is no real cache right now and this class just stores a hash
  // to determine if the current request could have been served from a real
  // cache.
  void UpdateCacheAfterGetRequest(const KURL& cookie_url,
                                  const String& cookie_string,
                                  uint64_t new_version);

  // This mechanism is designed to capture and isolate only the very first
  // request to cookie. We specifically focus on whether this initial action is
  // a GET or a SET operation or check to CookiesEnabled.

  // We want to evaluate the possible performance gain of returning the local
  // cache version and/or cookie string on SET. Especially, if SET is the first
  // request.
  void LogFirstCookieRequest(FirstCookieRequest first_cookie_request);

  // Checks with probe function if devtools is active. If so, devtools overrides
  // are applied to the cookie operation.
  bool ShouldApplyDevtoolsOverrides() const;

  HeapMojoRemote<network::mojom::blink::RestrictedCookieManager> backend_;
  Member<blink::Document> document_;

  // Hash used to determine if the value returned by a call to
  // RestrictedCookieManager::GetCookiesString is the same as a previous one.
  // Used to answer the question: "had we keep the last cookie_string around
  // would it have been possible to return that instead of making a new IPC?".
  // Combines hashes for the `cookie_string` returned by the call and the
  // `cookie_url` used as a parameter to the call.
  //
  // ATTENTION: Just use hashes for now to keep space overhead low, but more
  // importantly, because keeping cookies around is tricky from a security
  // perspective.
  std::optional<unsigned> last_cookies_hash_;
  // Whether the last operation performed on this jar was a set or get. Used
  // along with `last_cookies_hash_` when updating the histogram that tracks
  // cookie access results.
  bool last_operation_was_set_{false};

  std::optional<mojo::SharedMemoryVersionClient> shared_memory_version_client_;
  uint64_t last_version_ = mojo::shared_memory_version::kInvalidVersion;

  // Last decision of if devtools overrides needed to be applied. If the
  // decision changes, IPC is needed to get cookie with new devtools overrides
  bool last_devtools_overrides_were_applied = false;

  // Last received cookie string. Null if there is no last cached-version. Can
  // be empty since that is a valid cookie string.
  String last_cookies_;
  bool is_first_operation_ = true;

  // The number of required writes (via SetCookieFomString) that should be
  // observed by the network service in order to skip an IPC in Cookies().
  // This number increments when calling SetCookieFromString asynchronously
  // and it is compared with the committed_writes_count in shared memory.
  mojo::CountType required_committed_writes_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_COOKIE_JAR_H_
