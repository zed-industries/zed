// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_ATTRIBUTION_SRC_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_ATTRIBUTION_SRC_LOADER_H_

#include <stdint.h>

#include <optional>
#include <utility>
#include <vector>

#include "components/attribution_reporting/data_host.mojom-blink-forward.h"
#include "components/attribution_reporting/registration_eligibility.mojom-blink-forward.h"
#include "mojo/public/cpp/bindings/shared_remote.h"
#include "services/network/public/mojom/attribution.mojom-forward.h"
#include "services/network/public/mojom/referrer_policy.mojom-blink-forward.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/forward.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace attribution_reporting {
class SuitableOrigin;
struct RegistrationInfo;
}  // namespace attribution_reporting

namespace blink {

class HTMLAnchorElementBase;
class HTMLElement;
class KURL;
class LocalFrame;
class ResourceRequest;
class ResourceResponse;
class WebString;

struct Impression;

// Handles source and trigger registration from blink.
// See
// https://github.com/WICG/attribution-reporting-api/blob/main/EVENT.md#registering-attribution-sources
// and
// https://github.com/WICG/attribution-reporting-api/blob/main/EVENT.md#triggering-attribution.
class CORE_EXPORT AttributionSrcLoader
    : public GarbageCollected<AttributionSrcLoader> {
 public:
  explicit AttributionSrcLoader(LocalFrame* frame);
  AttributionSrcLoader(const AttributionSrcLoader&) = delete;
  AttributionSrcLoader& operator=(const AttributionSrcLoader&) = delete;
  AttributionSrcLoader(AttributionSrcLoader&& other) = delete;
  AttributionSrcLoader& operator=(AttributionSrcLoader&& other) = delete;
  ~AttributionSrcLoader();

  // Registers zero or more attribution srcs by splitting `attribution_src` on
  // spaces and completing each token as a URL against the frame's document.
  // This method handles fetching each URL and notifying the browser process to
  // begin tracking it. It is a no-op if the frame is not attached.
  void Register(const AtomicString& attribution_src,
                HTMLElement* element,
                network::mojom::ReferrerPolicy);

  // Registers an attribution resource client for the given resource if
  // the request is eligible for attribution registration.
  // Returns whether a registration was successful.
  bool MaybeRegisterAttributionHeaders(const ResourceRequest& request,
                                       const ResourceResponse& response);

  // Splits `attribution_src` on spaces and completes each token as a URL
  // against the frame's document.
  //
  // For each URL eligible for Attribution Reporting, initiates a fetch for it,
  // and notifies the browser to begin tracking it.
  //
  // If at least one URL is eligible or `navigation_url` is, returns a
  // non-`std::nullopt` `Impression` to live alongside the navigation.
  [[nodiscard]] std::optional<Impression> RegisterNavigation(
      const KURL& navigation_url,
      const AtomicString& attribution_src,
      HTMLAnchorElementBase* element,
      bool has_transient_user_activation,
      network::mojom::ReferrerPolicy);

  // Same as the above, but uses an already-tokenized attribution src for use
  // with `window.open`.
  [[nodiscard]] std::optional<Impression> RegisterNavigation(
      const KURL& navigation_url,
      const std::vector<WebString>& attribution_srcs,
      bool has_transient_user_activation,
      network::mojom::ReferrerPolicy);

  [[nodiscard]] std::optional<Impression> PrepareContextMenuNavigation(
      const KURL& navigation_url,
      HTMLAnchorElementBase* anchor);

  // If the given impression is null, all data hosts are unbound.
  void RegisterFromContextMenuNavigation(const std::optional<Impression>&,
                                         HTMLAnchorElementBase* anchor);

  // Returns true if `url` can be used as an attributionsrc: its scheme is HTTP
  // or HTTPS, its origin is potentially trustworthy, the document's permission
  // policy supports Attribution Reporting, the window's context is secure, and
  // the Attribution Reporting runtime-enabled feature is enabled.
  //
  // Reports a DevTools issue using `element` otherwise, if `log_issues` is
  // true.
  [[nodiscard]] bool CanRegister(const KURL& url,
                                 HTMLElement* element,
                                 bool log_issues = true);

  void Trace(Visitor* visitor) const;

  network::mojom::AttributionSupport GetSupport() const;

  // Records whether the permission policy allows for Attribution support to
  // 'Conversions.AllowedByPermissionPolicy'.
  static void RecordAttributionFeatureAllowed(bool enabled);

 private:
  class ResourceClient;

  using DataHostSharedRemote =
      mojo::SharedRemote<attribution_reporting::mojom::blink::DataHost>;

  Vector<KURL> ParseAttributionSrc(const AtomicString& attribution_src,
                                   HTMLElement*);

  bool DoRegistration(const Vector<KURL>&,
                      std::optional<AttributionSrcToken>,
                      network::mojom::ReferrerPolicy,
                      DataHostSharedRemote);

  void PrepareNavigationDataHost(const Vector<KURL>&,
                                 AttributionSrcToken,
                                 DataHostSharedRemote&);

  [[nodiscard]] std::optional<Impression> RegisterNavigationInternal(
      const KURL& navigation_url,
      Vector<KURL> attribution_src_urls,
      HTMLAnchorElementBase*,
      bool has_transient_user_activation,
      network::mojom::ReferrerPolicy);

  // Returns the reporting origin corresponding to `url` if its protocol is in
  // the HTTP family, its origin is potentially trustworthy, and attribution is
  // allowed. Returns `std::nullopt` otherwise, and reports a DevTools issue
  // using `element` and `request_id if `log_issues` is true.
  std::optional<attribution_reporting::SuitableOrigin>
  ReportingOriginForUrlIfValid(const KURL& url,
                               HTMLElement* element,
                               const WTF::String& request_url,
                               std::optional<uint64_t> request_id,
                               bool log_issues = true);

  bool CreateAndSendRequests(Vector<KURL>,
                             std::optional<AttributionSrcToken>,
                             network::mojom::ReferrerPolicy,
                             DataHostSharedRemote);

  struct AttributionHeaders;

  void RegisterAttributionHeaders(
      attribution_reporting::mojom::blink::RegistrationEligibility,
      network::mojom::AttributionSupport,
      attribution_reporting::SuitableOrigin reporting_origin,
      const AttributionHeaders&,
      const attribution_reporting::RegistrationInfo&,
      bool was_fetched_via_service_worker);

  const Member<LocalFrame> local_frame_;

  using ContextMenuDataHostEntry =
      std::pair<AttributionSrcToken, DataHostSharedRemote>;

  Vector<ContextMenuDataHostEntry> context_menu_data_hosts_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_ATTRIBUTION_SRC_LOADER_H_
