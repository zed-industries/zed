/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MIXED_CONTENT_CHECKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MIXED_CONTENT_CHECKER_H_

#include <optional>

#include "base/gtest_prod_util.h"
#include "base/types/optional_ref.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/loader/content_security_notifier.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/loader/mixed_content.mojom-blink-forward.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/loader/fetch/https_state.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_request.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_response.h"
#include "third_party/blink/renderer/platform/loader/mixed_content.h"
#include "third_party/blink/renderer/platform/weborigin/reporting_disposition.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ConsoleMessage;
class ExecutionContext;
class FetchClientSettingsObject;
class Frame;
class FrameFetchContext;
class LocalFrame;
class KURL;
class ResourceResponse;
class SecurityOrigin;
class Settings;
class SourceLocation;
class WebContentSettingsClient;
class WorkerFetchContext;

// Checks resource loads for mixed content. If PlzNavigate is enabled then this
// class only checks for sub-resource loads while frame-level loads are
// delegated to the browser where they are checked by
// MixedContentNavigationThrottle. Changes to this class might need to be
// reflected on its browser counterpart.
//
// Current mixed content W3C draft that drives this implementation:
// https://w3c.github.io/webappsec-mixed-content/
class CORE_EXPORT MixedContentChecker final {
  DISALLOW_NEW();

 public:
  static bool ShouldBlockFetch(LocalFrame* frame,
                               mojom::blink::RequestContextType request_context,
                               network::mojom::blink::IPAddressSpace,
                               const KURL& url_before_redirects,
                               ResourceRequest::RedirectStatus redirect_status,
                               const KURL& url,
                               const String& devtools_id,
                               ReportingDisposition reporting_disposition,
                               mojom::blink::ContentSecurityNotifier& notifier);

  static bool ShouldBlockFetchOnWorker(WorkerFetchContext&,
                                       mojom::blink::RequestContextType,
                                       const KURL& url_before_redirects,
                                       ResourceRequest::RedirectStatus,
                                       const KURL&,
                                       ReportingDisposition,
                                       bool is_worklet_global_scope);

  static bool IsWebSocketAllowed(const FrameFetchContext&,
                                 LocalFrame*,
                                 const KURL&);
  static bool IsWebSocketAllowed(WorkerFetchContext&, const KURL&);

  static bool IsMixedContent(const SecurityOrigin*, const KURL&);
  static bool IsMixedContent(const String& origin_protocol, const KURL&);
  static bool IsMixedContent(const FetchClientSettingsObject&, const KURL&);
  static bool IsMixedFormAction(
      LocalFrame*,
      const KURL&,
      ReportingDisposition = ReportingDisposition::kReport);

  static bool ShouldAutoupgrade(
      const FetchClientSettingsObject* fetch_client_settings_object,
      mojom::blink::RequestContextType type,
      WebContentSettingsClient* settings_client,
      const ResourceRequest& resource_request,
      ExecutionContext* execution_context_for_logging);

  static mojom::blink::MixedContentContextType ContextTypeForInspector(
      LocalFrame*,
      const ResourceRequest&);

  static void HandleCertificateError(
      const ResourceResponse&,
      mojom::blink::RequestContextType,
      MixedContent::CheckModeForPlugin,
      mojom::blink::ContentSecurityNotifier& notifier);

  // Receive information about mixed content found externally.
  static void MixedContentFound(LocalFrame*,
                                const KURL& main_resource_url,
                                const KURL& mixed_content_url,
                                mojom::blink::RequestContextType,
                                bool was_allowed,
                                const KURL& url_before_redirects,
                                bool had_redirect,
                                std::unique_ptr<SourceLocation>);

  static ConsoleMessage* CreateConsoleMessageAboutFetchAutoupgrade(
      const KURL& main_resource_url,
      const KURL& mixed_content_url);

  static ConsoleMessage* CreateConsoleMessageAboutFetchIPAddressNoAutoupgrade(
      const KURL& main_resource_url,
      const KURL& mixed_content_url);

  // Upgrade the insecure requests.
  // https://w3c.github.io/webappsec-upgrade-insecure-requests/
  // Upgrading itself is done based on |fetch_client_settings_object|.
  // |execution_context_for_logging| is used only for logging, use counters,
  // UKM-related things.
  static void UpgradeInsecureRequest(
      ResourceRequest&,
      const FetchClientSettingsObject* fetch_client_settings_object,
      ExecutionContext* execution_context_for_logging,
      mojom::RequestContextFrameType,
      WebContentSettingsClient* settings_client);

  static MixedContent::CheckModeForPlugin DecideCheckModeForPlugin(Settings*);

  MixedContentChecker(const MixedContentChecker&) = delete;
  MixedContentChecker& operator=(const MixedContentChecker&) = delete;

 private:
  FRIEND_TEST_ALL_PREFIXES(MixedContentCheckerTest, HandleCertificateError);

  static Frame* InWhichFrameIsContentMixed(LocalFrame*, const KURL&);

  static ConsoleMessage* CreateConsoleMessageAboutFetch(
      const KURL&,
      const KURL&,
      mojom::blink::RequestContextType,
      bool allowed,
      std::unique_ptr<SourceLocation>);
  static ConsoleMessage* CreateConsoleMessageAboutWebSocket(const KURL&,
                                                            const KURL&,
                                                            bool allowed);
  static void Count(Frame*,
                    mojom::blink::RequestContextType,
                    const LocalFrame*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MIXED_CONTENT_CHECKER_H_
