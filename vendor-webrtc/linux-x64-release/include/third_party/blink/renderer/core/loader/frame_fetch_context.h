/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
n * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_FRAME_FETCH_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_FRAME_FETCH_CONTEXT_H_

#include <optional>

#include "base/task/single_thread_task_runner.h"
#include "base/types/optional_ref.h"
#include "services/network/public/mojom/web_client_hints_types.mojom-blink-forward.h"
#include "third_party/blink/public/common/subresource_load_metrics.h"
#include "third_party/blink/public/common/user_agent/user_agent_metadata.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/loader/content_security_notifier.mojom-blink.h"
#include "third_party/blink/public/mojom/service_worker/service_worker_object.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/loader/base_fetch_context.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/client_hints_preferences.h"
#include "third_party/blink/renderer/platform/loader/fetch/fetch_parameters.h"
#include "third_party/blink/renderer/platform/loader/fetch/loading_behavior_observer.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_fetcher.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_request.h"
#include "third_party/blink/renderer/platform/network/content_security_policy_parsers.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class ClientHintsPreferences;
class ContentSecurityPolicy;
class CoreProbeSink;
class Document;
class DocumentLoader;
class LocalFrame;
class LocalFrameClient;
class Settings;
class WebContentSettingsClient;

class CORE_EXPORT FrameFetchContext final : public BaseFetchContext,
                                            public LoadingBehaviorObserver {
 public:
  static ResourceFetcher* CreateFetcherForCommittedDocument(DocumentLoader&,
                                                            Document&);
  FrameFetchContext(DocumentLoader& document_loader,
                    Document& document,
                    const DetachableResourceFetcherProperties&);
  ~FrameFetchContext() override = default;

  std::optional<ResourceRequestBlockedReason> CanRequest(
      ResourceType type,
      const ResourceRequest& resource_request,
      const KURL& url,
      const ResourceLoaderOptions& options,
      ReportingDisposition reporting_disposition,
      base::optional_ref<const ResourceRequest::RedirectInfo> redirect_info)
      const override;
  mojom::FetchCacheMode ResourceRequestCachePolicy(
      const ResourceRequest&,
      ResourceType,
      FetchParameters::DeferOption) const override;
  void PrepareRequest(ResourceRequest&,
                      ResourceLoaderOptions&,
                      WebScopedVirtualTimePauser&,
                      ResourceType) override;

  void AddResourceTiming(mojom::blink::ResourceTimingInfoPtr,
                         const AtomicString& initiator_type) override;
  bool AllowImage() const override;

  void ModifyRequestForMixedContentUpgrade(ResourceRequest&) override;

  void PopulateResourceRequestBeforeCacheAccess(
      const ResourceLoaderOptions& options,
      ResourceRequest& request) override;

  void WillSendRequest(ResourceRequest& resource_request) override;

  void UpgradeResourceRequestForLoader(
      ResourceType,
      const std::optional<float> resource_width,
      ResourceRequest&,
      const ResourceLoaderOptions&) override;

  bool StartSpeculativeImageDecode(Resource* resource,
                                   base::OnceClosure callback) override;

  bool IsPrerendering() const override;

  bool DoesLCPPHaveAnyHintData() override;

  bool DoesLCPPHaveLcpElementLocatorHintData() override;

  // Exposed for testing.
  void AddClientHintsIfNecessary(const std::optional<float> resource_width,
                                 ResourceRequest&);

  void AddReducedAcceptLanguageIfNecessary(ResourceRequest&);

  FetchContext* Detach() override;

  void Trace(Visitor*) const override;

  bool CalculateIfAdSubresource(
      const ResourceRequestHead& resource_request,
      base::optional_ref<const KURL> alias_url,
      ResourceType type,
      const FetchInitiatorInfo& initiator_info) override;

  // LoadingBehaviorObserver overrides:
  void DidObserveLoadingBehavior(LoadingBehaviorFlag) override;

  std::unique_ptr<ResourceLoadInfoNotifierWrapper>
  CreateResourceLoadInfoNotifierWrapper() override;

  mojom::blink::ContentSecurityNotifier& GetContentSecurityNotifier() const;

  ExecutionContext* GetExecutionContext() const override;

  void UpdateSubresourceLoadMetrics(
      const SubresourceLoadMetrics& subresource_load_metrics) override;

  scoped_refptr<const SecurityOrigin> GetTopFrameOrigin() const override;

  const Vector<KURL>& GetPotentiallyUnusedPreloads() const override;

  void AddLcpPredictedCallback(base::OnceClosure callback) override;

 private:
  friend class FrameFetchContextTestBase;

  struct FrozenState;

  // Convenient accessors below can be used to transparently access the
  // relevant document loader or frame in either cases without null-checks.
  //
  // TODO(kinuko): Remove constness, these return non-const members.
  LocalFrame* GetFrame() const;
  LocalFrameClient* GetLocalFrameClient() const;

  // BaseFetchContext overrides:
  net::SiteForCookies GetSiteForCookies() const override;
  SubresourceFilter* GetSubresourceFilter() const override;
  bool AllowScript() const override;
  bool ShouldBlockRequestByInspector(const KURL&) const override;
  void DispatchDidBlockRequest(const ResourceRequest&,
                               const ResourceLoaderOptions&,
                               ResourceRequestBlockedReason,
                               ResourceType) const override;
  ContentSecurityPolicy* GetContentSecurityPolicyForWorld(
      const DOMWrapperWorld* world) const override;
  bool IsIsolatedSVGChromeClient() const override;
  void CountUsage(WebFeature) const override;
  void CountDeprecation(WebFeature) const override;
  bool ShouldBlockWebSocketByMixedContentCheck(const KURL&) const override;
  std::unique_ptr<WebSocketHandshakeThrottle> CreateWebSocketHandshakeThrottle()
      override;
  bool ShouldBlockFetchByMixedContentCheck(
      mojom::blink::RequestContextType request_context,
      network::mojom::blink::IPAddressSpace target_address_space,
      base::optional_ref<const ResourceRequest::RedirectInfo> redirect_info,
      const KURL& url,
      ReportingDisposition reporting_disposition,
      const String& devtools_id) const override;
  bool ShouldBlockFetchAsCredentialedSubresource(const ResourceRequest&,
                                                 const KURL&) const override;

  const KURL& Url() const override;
  ContentSecurityPolicy* GetContentSecurityPolicy() const override;

  WebContentSettingsClient* GetContentSettingsClient() const;
  Settings* GetSettings() const;
  String GetUserAgent() const;
  std::optional<UserAgentMetadata> GetUserAgentMetadata() const;
  const network::PermissionsPolicy* GetPermissionsPolicy() const override;
  const FeatureContext* GetFeatureContext() const override;
  HashSet<HashAlgorithm> CSPHashesToReport() const override;
  void AddCSPHashReport(
      const String& url,
      const HashMap<HashAlgorithm, String>& integrity_hashes) override;
  const ClientHintsPreferences GetClientHintsPreferences() const;
  float GetDevicePixelRatio() const;
  String GetReducedAcceptLanguage() const;

  enum class ClientHintsMode { kLegacy, kStandard };
  void SetFirstPartyCookie(ResourceRequest&);

  // Returns true if `resource_origin` is same as the origin of the top level
  // frame's main resource.
  bool IsFirstPartyOrigin(const SecurityOrigin* resource_origin) const;

  CoreProbeSink* Probe() const;

  // These are set on the constructor, and valid until Detach() is called.
  Member<DocumentLoader> document_loader_;
  Member<Document> document_;

  // Non-null only when detached.
  Member<FrozenState> frozen_state_;

  // Serializing the brand major version list is expensive, so it's cached.
  std::optional<UserAgentMetadata> last_ua_;
  std::optional<AtomicString> last_ua_serialized_brand_major_version_list_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_FRAME_FETCH_CONTEXT_H_
