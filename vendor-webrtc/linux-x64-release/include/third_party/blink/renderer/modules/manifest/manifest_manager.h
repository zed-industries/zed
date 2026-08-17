// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MANIFEST_MANIFEST_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MANIFEST_MANIFEST_MANAGER_H_

#include "base/functional/callback.h"
#include "mojo/public/cpp/bindings/receiver_set.h"
#include "third_party/blink/public/mojom/manifest/manifest.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/manifest/manifest_manager.mojom-blink.h"
#include "third_party/blink/public/web/web_manifest_manager.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver_set.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace blink {

class ManifestChangeNotifier;
class ManifestManagerTest;
class ManifestFetcher;
class ResourceResponse;

// The ManifestManager is a helper class that takes care of fetching and parsing
// the Manifest of the associated window. It uses the ManifestFetcher and
// the ManifestParser in order to do so.
//
// Consumers should use the mojo ManifestManager interface to use this class.
//
// Manifests returned from this class can only be empty if there is a network
// fetching error, parsing error, or frame/CORS/opaque origin related issue.
// Otherwise the manifest will always contain a `start_url`, `id`, and `scope`
// populated, as the parser will always default based on the document url if
// they are not specified in the json.
class MODULES_EXPORT ManifestManager
    : public GarbageCollected<ManifestManager>,
      public Supplement<LocalDOMWindow>,
      public mojom::blink::ManifestManager,
      public ExecutionContextLifecycleObserver {
 public:
  static const char kSupplementName[];

  static ManifestManager* From(LocalDOMWindow&);

  explicit ManifestManager(LocalDOMWindow&);

  ManifestManager(const ManifestManager&) = delete;
  ManifestManager& operator=(const ManifestManager&) = delete;

  ~ManifestManager() override;

  void DidChangeManifest();
  bool CanFetchManifest();

  KURL ManifestURL() const;
  bool ManifestUseCredentials() const;

  void RequestManifestForTesting(WebManifestManager::Callback callback);
  void SetManifestChangeNotifierForTest(ManifestChangeNotifier* notifier) {
    manifest_change_notifier_ = notifier;
  }

  // mojom::blink::ManifestManager implementation.
  void RequestManifest(RequestManifestCallback callback) override;
  void RequestManifestDebugInfo(
      RequestManifestDebugInfoCallback callback) override;
  void ParseManifestFromString(
      const KURL& document_url,
      const KURL& manifest_url,
      const String& manifest_contents,
      ParseManifestFromStringCallback callback) override;

  void Trace(Visitor*) const override;

 private:
  // Result of requesting the manifest. Storing as a class rather than
  // individual member fields makes it easier to cache the result between
  // requests and clear all that needs to be cleared when invalidating the
  // cache. Additionally this makes it more immediately obvious that a result
  // will never contain a null manifest or debug info, as we can enforce these
  // invariants in the API of this class.
  class Result {
   public:
    explicit Result(mojom::blink::ManifestRequestResult result,
                    KURL manifest_url = KURL(),
                    mojom::blink::ManifestPtr manifest = nullptr);
    Result(Result&&);
    Result& operator=(Result&&);

    mojom::blink::ManifestRequestResult result() const { return result_; }
    const KURL& manifest_url() const { return manifest_url_; }
    const mojom::blink::Manifest& manifest() const { return *manifest_; }
    const mojom::blink::ManifestDebugInfo& debug_info() const {
      return *debug_info_;
    }
    mojom::blink::ManifestDebugInfo& debug_info() { return *debug_info_; }

    void SetManifest(mojom::blink::ManifestPtr manifest);

   private:
    mojom::blink::ManifestRequestResult result_;
    KURL manifest_url_;
    mojom::blink::ManifestPtr manifest_;
    mojom::blink::ManifestDebugInfoPtr debug_info_;
  };

  using InternalRequestManifestCallback =
      base::OnceCallback<void(const Result&)>;

  // From ExecutionContextLifecycleObserver
  void ContextDestroyed() override;

  void RequestManifestImpl(InternalRequestManifestCallback callback);

  void FetchManifest();
  void OnManifestFetchComplete(const KURL& document_url,
                               const ResourceResponse& response,
                               const String& data);
  void ParseManifestFromPage(const KURL& document_url,
                             std::optional<KURL> manifest_url,
                             const String& data);
  void ResolveCallbacks(Result result);

  void BindReceiver(
      mojo::PendingReceiver<mojom::blink::ManifestManager> receiver);

  mojom::blink::ManifestPtr DefaultManifest();

  friend class ManifestManagerTest;

  Member<ManifestFetcher> fetcher_;
  Member<ManifestChangeNotifier> manifest_change_notifier_;

  // Contains the last RequestManifestImpl result as long as that result is
  // still valid for subsequent requests.
  std::optional<Result> cached_result_;

  Vector<InternalRequestManifestCallback> pending_callbacks_;

  HeapMojoReceiverSet<mojom::blink::ManifestManager, ManifestManager>
      receivers_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MANIFEST_MANIFEST_MANAGER_H_
