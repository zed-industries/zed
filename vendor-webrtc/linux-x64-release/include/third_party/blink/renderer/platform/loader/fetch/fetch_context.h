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
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_FETCH_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_FETCH_CONTEXT_H_

#include <memory>
#include <optional>

#include "base/task/single_thread_task_runner.h"
#include "base/types/optional_ref.h"
#include "third_party/blink/public/common/subresource_load_metrics.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/loader/request_context_frame_type.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/timing/resource_timing.mojom-blink-forward.h"
#include "third_party/blink/public/platform/resource_load_info_notifier_wrapper.h"
#include "third_party/blink/public/platform/resource_request_blocked_reason.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/fetch_initiator_info.h"
#include "third_party/blink/renderer/platform/loader/fetch/fetch_parameters.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_load_priority.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_request.h"
#include "third_party/blink/renderer/platform/network/content_security_policy_parsers.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/weborigin/reporting_disposition.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace network {
class PermissionsPolicy;
}  // namespace network

namespace blink {

enum class ResourceType : uint8_t;
class KURL;
class Resource;
struct ResourceLoaderOptions;
class SecurityOrigin;
class WebScopedVirtualTimePauser;

// The FetchContext is an interface for performing context specific processing
// in response to events in the ResourceFetcher. The ResourceFetcher or its job
// class, ResourceLoader, may call the methods on a FetchContext.
//
// Any processing that depends on components outside platform/loader/fetch/
// should be implemented on a subclass of this interface, and then exposed to
// the ResourceFetcher via this interface.
class PLATFORM_EXPORT FetchContext : public GarbageCollected<FetchContext> {
 public:
  FetchContext() = default;
  FetchContext(const FetchContext&) = delete;
  FetchContext& operator=(const FetchContext&) = delete;

  static FetchContext& NullInstance() {
    return *MakeGarbageCollected<FetchContext>();
  }

  virtual ~FetchContext() = default;

  virtual void Trace(Visitor*) const {}

  virtual void AddAdditionalRequestHeaders(ResourceRequest&);

  // Returns the cache policy for the resource. ResourceRequest is not passed as
  // a const reference as a header needs to be added for doc.write blocking
  // intervention.
  virtual mojom::FetchCacheMode ResourceRequestCachePolicy(
      const ResourceRequest&,
      ResourceType,
      FetchParameters::DeferOption) const;

  // This internally dispatches WebLocalFrameClient::WillSendRequest and hooks
  // request interceptors like ServiceWorker and ApplicationCache.
  // This may modify the request and ResourceLoaderOptions.
  // |virtual_time_pauser| is an output parameter. PrepareRequest may
  // create a new WebScopedVirtualTimePauser and set it to
  // |virtual_time_pauser|.
  // This is called on initial and every redirect request.
  virtual void PrepareRequest(ResourceRequest&,
                              ResourceLoaderOptions&,
                              WebScopedVirtualTimePauser& virtual_time_pauser,
                              ResourceType);

  virtual void AddResourceTiming(mojom::blink::ResourceTimingInfoPtr,
                                 const AtomicString& initiator_type);
  virtual bool AllowImage() const { return false; }
  virtual std::optional<ResourceRequestBlockedReason> CanRequest(
      ResourceType,
      const ResourceRequest&,
      const KURL&,
      const ResourceLoaderOptions&,
      ReportingDisposition,
      base::optional_ref<const ResourceRequest::RedirectInfo> redirect_info)
      const {
    return ResourceRequestBlockedReason::kOther;
  }
  // In derived classes, performs *only* a SubresourceFilter check for whether
  // the request can go through or should be blocked.
  virtual std::optional<ResourceRequestBlockedReason>
  CanRequestBasedOnSubresourceFilterOnly(
      ResourceType,
      const ResourceRequest&,
      const KURL&,
      const ResourceLoaderOptions&,
      ReportingDisposition,
      base::optional_ref<const ResourceRequest::RedirectInfo> redirect_info)
      const {
    return ResourceRequestBlockedReason::kOther;
  }
  virtual std::optional<ResourceRequestBlockedReason> CheckCSPForRequest(
      mojom::blink::RequestContextType,
      network::mojom::RequestDestination request_destination,
      network::mojom::RequestMode request_mode,
      const KURL&,
      const ResourceLoaderOptions&,
      ReportingDisposition,
      const KURL& url_before_redirects,
      ResourceRequest::RedirectStatus) const {
    return ResourceRequestBlockedReason::kOther;
  }

  virtual std::optional<ResourceRequestBlockedReason>
  CheckAndEnforceCSPForRequest(
      mojom::blink::RequestContextType,
      network::mojom::RequestDestination request_destination,
      network::mojom::RequestMode request_mode,
      const KURL&,
      const ResourceLoaderOptions&,
      ReportingDisposition,
      const KURL& url_before_redirects,
      ResourceRequest::RedirectStatus) const {
    return ResourceRequestBlockedReason::kOther;
  }

  // Called from RequestResource() to upgrade insecure ResourceRequests if
  // necessary and prepare them for checking CSP. A mutable ResourceRequest is
  // passed as the URL may be modified. After this call returns, it is not
  // permitted to modify the URL of the ResourceRequest.
  virtual void ModifyRequestForMixedContentUpgrade(ResourceRequest&) {}

  // Populates the ResourceRequest with enough information for a cache lookup.
  // If the resource requires a load, then UpgradeResourceRequestForLoader() is
  // called.
  virtual void PopulateResourceRequestBeforeCacheAccess(
      const ResourceLoaderOptions& options,
      ResourceRequest& request) {}

  // Called after csp checks to potentially override the URL of the request.
  virtual void WillSendRequest(ResourceRequest& request) {}

  // Called if a resource request needs to be loaded (vs served from the cache).
  // This adds additional information to the ResourceRequest needed for
  // loading. This is called after PopulateResourceRequestBeforeCacheAccess().
  // This function may be called in some circumstances when still served from
  // the cache. For example, if the right probes are present, then this is
  // always called.
  virtual void UpgradeResourceRequestForLoader(
      ResourceType,
      const std::optional<float> resource_width,
      ResourceRequest&,
      const ResourceLoaderOptions&);

  virtual bool StartSpeculativeImageDecode(Resource* resource,
                                           base::OnceClosure callback);

  // Called when the underlying context is detached. Note that some
  // FetchContexts continue working after detached (e.g., for fetch() operations
  // with "keepalive" specified).
  // Returns a "detached" fetch context which cannot be null.
  virtual FetchContext* Detach() {
    return MakeGarbageCollected<FetchContext>();
  }

  virtual const network::PermissionsPolicy* GetPermissionsPolicy() const {
    return nullptr;
  }

  virtual const FeatureContext* GetFeatureContext() const { return nullptr; }

  // Determine if the request is on behalf of an advertisement. If so, return
  // true. Checks `resource_request.Url()` unless `alias_url` is non-null, in
  // which case it checks the latter.
  virtual bool CalculateIfAdSubresource(
      const ResourceRequestHead& resource_request,
      base::optional_ref<const KURL> alias_url,
      ResourceType type,
      const FetchInitiatorInfo& initiator_info) {
    return false;
  }

  // Returns a wrapper of ResourceLoadInfoNotifier to notify loading stats.
  virtual std::unique_ptr<ResourceLoadInfoNotifierWrapper>
  CreateResourceLoadInfoNotifierWrapper() {
    return nullptr;
  }

  // Returns if the request context is for prerendering or not.
  virtual bool IsPrerendering() const { return false; }

  // Update SubresourceLoad metrics.
  virtual void UpdateSubresourceLoadMetrics(
      const SubresourceLoadMetrics& subresource_load_metrics) {}

  // Returns true iff we have LCPP hint data for the fetch context.
  virtual bool DoesLCPPHaveAnyHintData() { return false; }

  // Returns true iff we have LCP element locator hint data for the fetch
  // context.
  virtual bool DoesLCPPHaveLcpElementLocatorHintData() { return false; }

  // Returns the origin of the top frame in the document or the dedicated
  // worker. This returns nullptr for Shared Workers and Service Workers.
  virtual scoped_refptr<const SecurityOrigin> GetTopFrameOrigin() const {
    return nullptr;
  }

  // Returns the list of potentially unused preload URLs flagged by the LCP
  // predcitor, which is attached to the frame. This returns an empty Vector for
  // Shared Workers and Service Workers.
  virtual const Vector<KURL>& GetPotentiallyUnusedPreloads() const {
    return empty_unused_preloads_;
  }

  virtual void AddLcpPredictedCallback(base::OnceClosure callback) {
    NOTIMPLEMENTED();
  }

  virtual HashSet<HashAlgorithm> CSPHashesToReport() const {
    return HashSet<HashAlgorithm>();
  }
  virtual void AddCSPHashReport(
      const String& url,
      const HashMap<HashAlgorithm, String>& integrity_hashes) {}

 protected:
  const Vector<KURL> empty_unused_preloads_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_FETCH_CONTEXT_H_
