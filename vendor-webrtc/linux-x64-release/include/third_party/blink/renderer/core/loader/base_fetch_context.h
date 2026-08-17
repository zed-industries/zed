// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_BASE_FETCH_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_BASE_FETCH_CONTEXT_H_

#include <optional>

#include "base/types/optional_ref.h"
#include "net/cookies/site_for_cookies.h"
#include "services/network/public/mojom/referrer_policy.mojom-blink-forward.h"
#include "services/network/public/mojom/web_client_hints_types.mojom-blink-forward.h"
#include "third_party/blink/public/common/user_agent/user_agent_metadata.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink-forward.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/csp/content_security_policy.h"
#include "third_party/blink/renderer/core/frame/web_feature_forward.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/fetch_client_settings_object.h"
#include "third_party/blink/renderer/platform/loader/fetch/fetch_context.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_request.h"

namespace blink {

class DetachableConsoleLogger;
class DOMWrapperWorld;
class DetachableResourceFetcherProperties;
class KURL;
class SubresourceFilter;
class WebSocketHandshakeThrottle;

// A core-level implementation of FetchContext that does not depend on
// Frame. This class provides basic default implementation for some methods.
class CORE_EXPORT BaseFetchContext : public FetchContext {
 public:
  std::optional<ResourceRequestBlockedReason> CanRequest(
      ResourceType,
      const ResourceRequest&,
      const KURL&,
      const ResourceLoaderOptions&,
      ReportingDisposition,
      base::optional_ref<const ResourceRequest::RedirectInfo>) const override;
  std::optional<ResourceRequestBlockedReason>
  CanRequestBasedOnSubresourceFilterOnly(
      ResourceType,
      const ResourceRequest&,
      const KURL&,
      const ResourceLoaderOptions&,
      ReportingDisposition,
      base::optional_ref<const ResourceRequest::RedirectInfo>) const override;
  std::optional<ResourceRequestBlockedReason> CheckCSPForRequest(
      mojom::blink::RequestContextType,
      network::mojom::RequestDestination request_destination,
      network::mojom::RequestMode request_mode,
      const KURL&,
      const ResourceLoaderOptions&,
      ReportingDisposition,
      const KURL& url_before_redirects,
      ResourceRequest::RedirectStatus) const override;
  std::optional<ResourceRequestBlockedReason> CheckAndEnforceCSPForRequest(
      mojom::blink::RequestContextType,
      network::mojom::RequestDestination request_destination,
      network::mojom::RequestMode request_mode,
      const KURL&,
      const ResourceLoaderOptions&,
      ReportingDisposition,
      const KURL& url_before_redirects,
      ResourceRequest::RedirectStatus) const override;

  void Trace(Visitor*) const override;

  const DetachableResourceFetcherProperties& GetResourceFetcherProperties()
      const {
    return *fetcher_properties_;
  }

  DetachableConsoleLogger& GetDetachableConsoleLogger() const {
    return *console_logger_;
  }

  virtual void CountUsage(mojom::WebFeature) const = 0;
  virtual void CountDeprecation(mojom::WebFeature) const = 0;
  virtual net::SiteForCookies GetSiteForCookies() const = 0;

  virtual SubresourceFilter* GetSubresourceFilter() const = 0;
  virtual bool ShouldBlockWebSocketByMixedContentCheck(const KURL&) const = 0;
  virtual std::unique_ptr<WebSocketHandshakeThrottle>
  CreateWebSocketHandshakeThrottle() = 0;

  // If the optional `alias_url` is non-null, it will be used to perform the
  // check in place of `resource_request.Url()`, e.g. in the case of DNS
  // aliases.
  bool CalculateIfAdSubresource(
      const ResourceRequestHead& resource_request,
      base::optional_ref<const KURL> alias_url,
      ResourceType type,
      const FetchInitiatorInfo& initiator_info) override;

 protected:
  BaseFetchContext(const DetachableResourceFetcherProperties& properties,
                   DetachableConsoleLogger* logger)
      : fetcher_properties_(properties), console_logger_(logger) {}

  // Used for security checks.
  virtual bool AllowScript() const = 0;

  // Note: subclasses are expected to override following methods.
  // Used in the default implementation for CanRequest, CanFollowRedirect
  // and AllowResponse.
  virtual bool ShouldBlockRequestByInspector(const KURL&) const = 0;
  virtual void DispatchDidBlockRequest(const ResourceRequest&,
                                       const ResourceLoaderOptions&,
                                       ResourceRequestBlockedReason,
                                       ResourceType) const = 0;
  virtual ContentSecurityPolicy* GetContentSecurityPolicyForWorld(
      const DOMWrapperWorld* world) const = 0;

  virtual bool IsIsolatedSVGChromeClient() const = 0;
  virtual bool ShouldBlockFetchByMixedContentCheck(
      mojom::blink::RequestContextType request_context,
      network::mojom::blink::IPAddressSpace target_address_space,
      base::optional_ref<const ResourceRequest::RedirectInfo> redirect_info,
      const KURL& url,
      ReportingDisposition reporting_disposition,
      const String& devtools_id) const = 0;
  virtual bool ShouldBlockFetchAsCredentialedSubresource(const ResourceRequest&,
                                                         const KURL&) const = 0;
  virtual const KURL& Url() const = 0;
  virtual ContentSecurityPolicy* GetContentSecurityPolicy() const = 0;

  virtual ExecutionContext* GetExecutionContext() const = 0;

 private:
  const Member<const DetachableResourceFetcherProperties> fetcher_properties_;

  const Member<DetachableConsoleLogger> console_logger_;

  void PrintAccessDeniedMessage(const KURL&) const;

  // Utility methods that are used in default implement for CanRequest,
  // CanFollowRedirect and AllowResponse.
  std::optional<ResourceRequestBlockedReason> CanRequestInternal(
      ResourceType,
      const ResourceRequest&,
      const KURL&,
      const ResourceLoaderOptions&,
      ReportingDisposition,
      base::optional_ref<const ResourceRequest::RedirectInfo> redirect_info)
      const;

  std::optional<ResourceRequestBlockedReason> CheckCSPForRequestInternal(
      mojom::blink::RequestContextType,
      network::mojom::RequestDestination request_destination,
      network::mojom::RequestMode request_mode,
      const KURL&,
      const ResourceLoaderOptions&,
      ReportingDisposition,
      const KURL& url_before_redirects,
      ResourceRequest::RedirectStatus redirect_status,
      ContentSecurityPolicy::CheckHeaderType) const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_BASE_FETCH_CONTEXT_H_
