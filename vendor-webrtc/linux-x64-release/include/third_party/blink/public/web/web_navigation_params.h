// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_NAVIGATION_PARAMS_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_NAVIGATION_PARAMS_H_

#include <memory>
#include <optional>

#include "base/containers/flat_map.h"
#include "base/containers/span.h"
#include "base/memory/raw_ptr.h"
#include "base/time/time.h"
#include "base/unguessable_token.h"
#include "base/uuid.h"
#include "net/storage_access_api/status.h"
#include "services/metrics/public/cpp/ukm_source_id.h"
#include "services/network/public/cpp/permissions_policy/permissions_policy_declaration.h"
#include "services/network/public/mojom/link_header.mojom-shared.h"
#include "services/network/public/mojom/referrer_policy.mojom-shared.h"
#include "services/network/public/mojom/url_loader_factory.mojom-shared.h"
#include "services/network/public/mojom/url_response_head.mojom-shared.h"
#include "services/network/public/mojom/web_client_hints_types.mojom-shared.h"
#include "third_party/blink/public/common/fenced_frame/redacted_fenced_frame_config.h"
#include "third_party/blink/public/common/frame/frame_policy.h"
#include "third_party/blink/public/common/frame/view_transition_state.h"
#include "third_party/blink/public/common/navigation/impression.h"
#include "third_party/blink/public/common/page/browsing_context_group_info.h"
#include "third_party/blink/public/common/storage_key/storage_key.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/blob/blob_url_store.mojom-shared.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-shared.h"
#include "third_party/blink/public/mojom/frame/policy_container.mojom-forward.h"
#include "third_party/blink/public/mojom/frame/triggering_event_info.mojom-shared.h"
#include "third_party/blink/public/mojom/navigation/navigation_params.mojom-shared.h"
#include "third_party/blink/public/mojom/navigation/renderer_content_settings.mojom.h"
#include "third_party/blink/public/mojom/runtime_feature_state/runtime_feature.mojom-shared.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_content_security_policy_struct.h"
#include "third_party/blink/public/platform/web_data.h"
#include "third_party/blink/public/platform/web_http_body.h"
#include "third_party/blink/public/platform/web_navigation_body_loader.h"
#include "third_party/blink/public/platform/web_policy_container.h"
#include "third_party/blink/public/platform/web_security_origin.h"
#include "third_party/blink/public/platform/web_source_location.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/public/platform/web_url.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/public/platform/web_url_response.h"
#include "third_party/blink/public/web/web_form_element.h"
#include "third_party/blink/public/web/web_frame_load_type.h"
#include "third_party/blink/public/web/web_history_item.h"
#include "third_party/blink/public/web/web_navigation_policy.h"
#include "third_party/blink/public/web/web_navigation_timings.h"
#include "third_party/blink/public/web/web_navigation_type.h"

#if INSIDE_BLINK
#include "base/memory/scoped_refptr.h"
#endif

namespace base {
class TickClock;
}

namespace blink {

class WebDocumentLoader;
class WebServiceWorkerNetworkProvider;

// This structure holds all information collected by Blink when
// navigation is being initiated.
struct BLINK_EXPORT WebNavigationInfo {
  WebNavigationInfo() = default;
  ~WebNavigationInfo() = default;

  // The main resource request.
  WebURLRequest url_request;

  // The base url of the requestor. Only used for about:srcdoc and about:blank
  // navigations.
  WebURL requestor_base_url;

  // The frame type. This must not be kNone. See RequestContextFrameType.
  // TODO(dgozman): enforce this is not kNone.
  mojom::RequestContextFrameType frame_type =
      mojom::RequestContextFrameType::kNone;

  // The navigation type. See WebNavigationType.
  WebNavigationType navigation_type = kWebNavigationTypeOther;

  // The navigation policy. See WebNavigationPolicy.
  WebNavigationPolicy navigation_policy = kWebNavigationPolicyCurrentTab;

  // Whether the frame had a transient user activation
  // at the time this request was issued.
  // TODO(mustaq): Combine this with had_transient_user_activation, or make it
  // less confusing which of the two we should use in various scenarios.
  bool has_transient_user_activation = false;

  // The load type. See WebFrameLoadType.
  WebFrameLoadType frame_load_type = WebFrameLoadType::kStandard;

  // If true, will override cases where a WebFrameLoadType::kStandard navigation
  // is implicitly converted to a kReplaceCurrentItem navigation.
  mojom::ForceHistoryPush force_history_push = mojom::ForceHistoryPush::kNo;

  // During a history load, a child frame can be initially navigated
  // to an url from the history state. This flag indicates it.
  bool is_history_navigation_in_new_child_frame = false;

  // Whether the navigation is a result of client redirect.
  bool is_client_redirect = false;

  // Whether the navigation initiator frame has the
  // |network::mojom::blink::WebSandboxFlags::kDownloads| bit set in its sandbox
  // flags set.
  bool initiator_frame_has_download_sandbox_flag = false;

  // Whether the navigation initiator frame is an ad frame.
  bool initiator_frame_is_ad = false;

  // Whether there is ad script in stack when the navigation is initiated. Note
  // that will also be true if the initiator frame is ad.
  bool is_ad_script_in_stack = false;

  // Whether this is a navigation in the opener frame initiated
  // by the window.open'd frame.
  bool is_opener_navigation = false;

  // True if the initiator explicitly asked for opener relationships to be
  // preserved, via rel="opener".
  bool has_rel_opener = false;

  // Whether this is a navigation to _unfencedTop, i.e. to the top-level frame
  // from a renderer process that does not get a handle to the frame.
  // The browser should ignore the specified target frame and pick (and
  // validate) the top-level frame instead.
  // TODO(crbug.com/1315802): Refactor _unfencedTop handling.
  bool is_unfenced_top_navigation = false;

  // Event information. See TriggeringEventInfo.
  blink::mojom::TriggeringEventInfo triggering_event_info =
      blink::mojom::TriggeringEventInfo::kUnknown;

  // If the navigation is a result of form submit, the form element is provided.
  WebFormElement form;

  // The location in the source which triggered the navigation.
  // Used to help web developers understand what caused the navigation.
  WebSourceLocation source_location;

  // The initiator of this navigation used by DevTools.
  WebString devtools_initiator_info;

  // Whether this navigation should check CSP.
  network::mojom::CSPDisposition
      should_check_main_world_content_security_policy =
          network::mojom::CSPDisposition::CHECK;

  // When navigating to a blob url, this token specifies the blob.
  CrossVariantMojoRemote<mojom::BlobURLTokenInterfaceBase> blob_url_token;

  // When navigation initiated from the user input, this tracks
  // the input start time.
  base::TimeTicks input_start;

  // An earlier and more accurate representation of the starting time of the
  // navigation (based on the creation of the FrameLoadRequest), compared to the
  // `navigation_start` that is passed through many of the navigation APIs
  // (which is delayed until after the renderer process runs beforeunload
  // handlers). The goal is to later use this in many navigation metrics to
  // better identify performance improvements and regressions, while excluding
  // just the time spent in beforeunload handlers. This value is currently only
  // used in trace events and limited metrics to gauge the impact the change.
  // TODO(crbug.com/385170155): Use this for most navigation metrics, with
  // better ways to exclude beforeunload time.
  base::TimeTicks actual_navigation_start;

  // Specifies whether or not a MHTML Archive can be used to load a subframe
  // resource instead of doing a network request.
  enum class ArchiveStatus { Absent, Present };
  ArchiveStatus archive_status = ArchiveStatus::Absent;

  // The origin trial features activated in the document initiating this
  // navigation that should be applied in the document being navigated to.
  std::vector<int> initiator_origin_trial_features;

  // The value of hrefTranslate attribute of a link, if this navigation was
  // inititated by clicking a link.
  WebString href_translate;

  // Optional impression associated with this navigation. This is attached when
  // a navigation results from a click on an anchor tag that has conversion
  // measurement attributes.
  std::optional<Impression> impression;

  // The frame policy specified by the frame owner element.
  // For top-level window with no opener, this is the default lax FramePolicy.
  // This attribute is used for the synchronous re-navigation to about:blank
  // only.
  FramePolicy frame_policy;

  // The frame token of the initiator Frame.
  std::optional<LocalFrameToken> initiator_frame_token;

  // A handle for keeping the initiator RenderFrameHost's
  // NavigationStateKeepAlive alive until we create the NavigationRequest.
  CrossVariantMojoRemote<mojom::NavigationStateKeepAliveHandleInterfaceBase>
      initiator_navigation_state_keep_alive_handle;

  // The initiator frame's LocalDOMWindow's Storage Access API status.
  net::StorageAccessApiStatus storage_access_api_status =
      net::StorageAccessApiStatus::kNone;

  // Whether this navigation was initiated by the container, e.g. iframe changed
  // src. Only container-initiated navigation report resource timing to the
  // parent.
  bool is_container_initiated = false;
};

// This structure holds all information provided by the embedder that is
// needed for blink to load a Document. This is hence different from
// WebDocumentLoader::ExtraData, which is an opaque structure stored in the
// DocumentLoader and used by the embedder.
struct BLINK_EXPORT WebNavigationParams {
  WebNavigationParams();
  ~WebNavigationParams();

  // Construct with a specific `document_token`, `devtools_navigation_token`,
  // and `base_auction_nonce` rather than randomly creating new ones.
  explicit WebNavigationParams(
      const blink::DocumentToken& document_token,
      const base::UnguessableToken& devtools_navigation_token,
      const base::Uuid& base_auction_nonce);

  // Shortcut for navigating based on WebNavigationInfo parameters.
  //
  // This is used only for navigations not driven by the browser process:
  // - the re-navigation to about:blank when creating a new subframe or window
  //   with initial location == about:blank (see https://crbug.com/778318)
  // - unit tests.
  static std::unique_ptr<WebNavigationParams> CreateFromInfo(
      const WebNavigationInfo&);

  // Shortcut for loading html with "text/html" mime type and "UTF8" encoding.
  static std::unique_ptr<WebNavigationParams> CreateWithEmptyHTMLForTesting(
      const WebURL& base_url);
  static std::unique_ptr<WebNavigationParams> CreateWithHTMLStringForTesting(
      base::span<const char> html,
      const WebURL& base_url);

  // Fills |body_loader| based on the provided static data.
  static void FillBodyLoader(WebNavigationParams*, base::span<const char> data);
  static void FillBodyLoader(WebNavigationParams*, WebData data);

  // Fills |response| and |body_loader| based on the provided static data.
  // |url| must be set already.
  static void FillStaticResponse(WebNavigationParams*,
                                 WebString mime_type,
                                 WebString text_encoding,
                                 base::span<const char> data);
#if INSIDE_BLINK
  static void FillStaticResponse(WebNavigationParams*,
                                 WebString mime_type,
                                 WebString text_encoding,
                                 SharedBuffer* data);
#endif

  // This block defines the request used to load the main resource
  // for this navigation.

  // This URL indicates the security origin and will be used as a base URL
  // to resolve links in the committed document.
  WebURL url;
  // The http method (if any) used to load the main resource.
  WebString http_method;
  // The referrer string and policy used to load the main resource.
  WebString referrer;
  network::mojom::ReferrerPolicy referrer_policy =
      network::mojom::ReferrerPolicy::kDefault;
  // The http body of the request used to load the main resource, if any.
  WebHTTPBody http_body;
  // The http content type of the request used to load the main resource, if
  // any.
  WebString http_content_type;
  // The http status code of the request used to load the main resource, if any.
  int http_status_code = 0;
  // The origin of the request used to load the main resource, specified at
  // https://fetch.spec.whatwg.org/#concept-request-origin. This is never null
  // for renderer-initiated navigations, but can be null for 1)
  // browser-initiated navigations and 2) loading of initial empty documents.
  // TODO(dgozman,nasko): we shouldn't need both this and |origin_to_commit|.
  WebSecurityOrigin requestor_origin;
  // If non-null, used as a URL which we weren't able to load. For example,
  // history item will contain this URL instead of request's URL.
  // This URL can be retrieved through WebDocumentLoader::UnreachableURL.
  WebURL unreachable_url;
  // If non-null, this gives the pre-redirect URL in case that we're committing
  // a failed navigation.
  WebURL pre_redirect_url_for_failed_navigations;

  // If `url` is about:srcdoc or about:blank, this is the default base URL to
  // use for the new document. It corresponds to the initiator's base URL
  // snapshotted when the navigation started.
  WebURL fallback_base_url;

  // The net error code for failed navigation. Must be non-zero when
  // |unreachable_url| is non-null.
  int error_code = 0;

  // This block defines the document content. The alternatives in the order
  // of precedence are:
  // 1. If |is_static_data| is false:
  //   1a. If url loads as an empty document (according to
  //       WebDocumentLoader::WillLoadUrlAsEmpty), the document will be empty.
  //   1b. If loading an iframe of mhtml archive, the document will be
  //       retrieved from the archive.
  // 2. Otherwise, provided redirects and response are used to construct
  //    the final response.
  //   2a. If body loader is present, it will be used to fetch the content.
  //   2b. If body loader is missing, but url is a data url, it will be
  //       decoded and used as response and document content.
  //   2c. If decoding data url fails, or url is not a data url, the
  //       navigation will fail.

  struct RedirectInfo {
    // New base url after redirect.
    WebURL new_url;
    // Http method used for redirect.
    WebString new_http_method;
    // New referrer string and policy used for redirect.
    WebString new_referrer;
    network::mojom::ReferrerPolicy new_referrer_policy;
    // Redirect response itself.
    // TODO(dgozman): we only use this response for navigation timings.
    // Perhaps, we can just get rid of it.
    WebURLResponse redirect_response;
    // When navigation is restarted due to a Critical-CH header this stores the
    // time at which the the restart was initiated.
    base::TimeTicks critical_ch_restart_time;
  };
  // Redirects which happened while fetching the main resource.
  // TODO(dgozman): we are only interested in the final values instead of
  // all information about redirects.
  std::vector<RedirectInfo> redirects;
  // The final response for the main resource. This will be used to determine
  // the type of resulting document.
  WebURLResponse response;
  // The body loader which allows to retrieve the response body when available.
  std::unique_ptr<WebNavigationBodyLoader> body_loader;
  // Whether |response| and |body_loader| represent static data. In this case
  // we skip some security checks and insist on loading this exact content.
  bool is_static_data = false;

  // This block defines the type of the navigation.

  // The load type. See WebFrameLoadType for definition.
  WebFrameLoadType frame_load_type = WebFrameLoadType::kStandard;
  // History item should be provided for back-forward load types.
  WebHistoryItem history_item;
  // Whether this navigation is a result of client redirect.
  bool is_client_redirect = false;
  // Cache mode to be used for subresources, instead of the one determined
  // by |frame_load_type|.
  std::optional<blink::mojom::FetchCacheMode> force_fetch_cache_mode;

  // Miscellaneous parameters.

  // The origin in which a navigation should commit. When provided, Blink
  // should use this origin directly and not compute locally the new document
  // origin. It is currently only specified on error document navigations, where
  // the origin should be an opaque origin based on the URL that failed to load.
  //
  // TODO(https://crbug.com/888079): Always provide origin_to_commit.
  WebSecurityOrigin origin_to_commit;

  // The storage key of the document that will be created by the navigation.
  // This is compatible with the origin that the browser calculates for this
  // navigation. Currently, the final origin used by a navigation is still
  // determined by the renderer, except when `origin_to_commit` above is set.
  // Until the browser is able to compute the origin accurately in all cases
  // (see https://crbug.com/888079), this is actually just a provisional
  // `storage_key`. The final storage key is computed by the document loader
  // taking into account the origin computed by the renderer.
  StorageKey storage_key;

  blink::DocumentToken document_token;
  // The devtools token for this navigation. See DocumentLoader
  // for details.
  base::UnguessableToken devtools_navigation_token;

  // Seed for all PAAPI Auction Nonces generated in this document.
  base::Uuid base_auction_nonce;

  // Known timings related to navigation. If the navigation has
  // started in another process, timings are propagated from there.
  WebNavigationTimings navigation_timings;
  // Indicates that the frame was previously discarded.
  // was_discarded is exposed on Document after discard, see:
  // https://github.com/WICG/web-lifecycle
  bool was_discarded = false;
  // Whether this navigation had a transient user activation
  // when inititated.
  bool had_transient_user_activation = false;
  // Whether this navigation has a sticky user activation flag.
  bool is_user_activated = false;
  // Whether the navigation should be allowed to invoke a text fragment anchor.
  // This is based on a user activation but is different from the above bit as
  // it can be propagated across redirects and is consumed on use.
  bool has_text_fragment_token = false;
  // Whether this navigation was browser initiated.
  bool is_browser_initiated = false;
  // Whether the document should be able to access local file:// resources.
  bool grant_load_local_resources = false;
  // The service worker network provider to be used in the new
  // document.
  std::unique_ptr<blink::WebServiceWorkerNetworkProvider>
      service_worker_network_provider;

  // This is `true` only for commit requests coming from
  // `RenderFrameImpl::SynchronouslyConmmitAboutBlankForBug778318`.
  bool is_synchronous_commit_for_bug_778318 = false;

  // This struct keeps the information about a prefetched signed exchange.
  struct BLINK_EXPORT PrefetchedSignedExchange {
    PrefetchedSignedExchange();
    PrefetchedSignedExchange(
        const WebURL& outer_url,
        const WebString& header_integrity,
        const WebURL& inner_url,
        const WebURLResponse& inner_response,
        CrossVariantMojoRemote<network::mojom::URLLoaderFactoryInterfaceBase>
            loader_factory);
    ~PrefetchedSignedExchange();

    WebURL outer_url;
    WebString header_integrity;
    WebURL inner_url;
    WebURLResponse inner_response;
    CrossVariantMojoRemote<network::mojom::URLLoaderFactoryInterfaceBase>
        loader_factory;
  };
  std::vector<std::unique_ptr<PrefetchedSignedExchange>>
      prefetched_signed_exchanges;
  // An optional tick clock to be used for document loader timing. This is used
  // for testing.
  raw_ptr<const base::TickClock> tick_clock = nullptr;
  // The origin trial features activated in the document initiating this
  // navigation that should be applied in the document being navigated to.
  std::vector<int> initiator_origin_trial_features;

  // UKM source id to be associated with the Document that will be installed
  // in the current frame.
  ukm::SourceId document_ukm_source_id = ukm::kInvalidSourceId;

  // The frame policy specified by the frame owner element.
  // Should be std::nullopt for top level navigations
  std::optional<FramePolicy> frame_policy;

  // A list of origin trial names to enable for the document being loaded.
  std::vector<WebString> force_enabled_origin_trials;

  // Whether the page is in an origin-keyed agent cluster.
  // https://html.spec.whatwg.org/C/#is-origin-keyed
  bool origin_agent_cluster = false;

  // Whether the decision to use origin-keyed or site-keyed agent clustering
  // (which itself is recorded in origin_agent_cluster, above) has been
  // made based on absent Origin-Agent-Cluster http header.
  bool origin_agent_cluster_left_as_default = true;

  // List of client hints enabled for top-level frame. These still need to be
  // checked against permissions policy before use.
  std::vector<network::mojom::WebClientHintsType> enabled_client_hints;

  // Whether the navigation is cross-site and swaps BrowsingContextGroups
  // (BrowsingInstances).
  bool is_cross_site_cross_browsing_context_group = false;

  // Whether the new document should start with sticky user activation, because
  // the previously committed document did, and the navigation was same-site.
  bool should_have_sticky_user_activation = false;

  // Blink's copy of the policy container containing security policies to be
  // enforced on the document created by this navigation.
  std::unique_ptr<WebPolicyContainer> policy_container;

  // Blink's copy of a permissions policy constructed in the browser that should
  // take precedence over any permissions policy constructed in blink. This is
  // useful for isolated applications, which use a different base permissions
  // policy than blink, which uses a fully permissive policy as its base.
  std::optional<network::ParsedPermissionsPolicy> permissions_policy_override;

  // These are used to construct a subset of the back/forward list for the
  // window.navigation API. They only have the attributes that are needed for
  // that API.
  std::vector<WebHistoryItem> navigation_api_back_entries;
  std::vector<WebHistoryItem> navigation_api_forward_entries;
  WebHistoryItem navigation_api_previous_entry;

  // List of URLs which are preloaded by HTTP Early Hints.
  // TODO(https://crbug.com/1317936): Pass information more than URL such as
  // request destination so that ResourceFetcher can provide more useful
  // console messages when Early Hints preloaded resources are not used.
  std::vector<WebURL> early_hints_preloaded_resources;

  // If this is a navigation to fenced frame from an interest group auction,
  // contains URNs mapped to the ad components returned by the winning bid.
  // Null, otherwise.
  std::optional<std::vector<WebURL>> ad_auction_components;

  // Whether the current context would be allowed to create an opaque-ads
  //  frame (based on the browser-side calculations). See
  // NavigatorAuction::canLoadAdAuctionFencedFrame for usage and
  // ::blink::mojom::CommitNavigationParams::ancestor_or_self_has_cspee for
  // where the value is coming from.
  bool ancestor_or_self_has_cspee = false;

  // Reduced Accept-Language negotiates the language when navigating to a new
  // document in the main frame, and the browser supplies the same negotiated
  // language to the main frame and all its subframes when committing a
  // navigation. This value will be used as the Accept-Language for subresource
  // requests made by the document committed by this navigation. For example,
  // when navigating to a URL with embedded image subresource request, the
  // language negotiation only happens in top-level document. It will store the
  // top-level document's negotiated language as `reduced_accept_language` here.
  // Whenever fetching image subresources, the HTTP Accept-Language header will
  // be set as the reduced accept language which was stored here. As we only do
  // language negotiation on the top-level document, all subresource requests
  // will inherit the Accept-Language header value of the top-level document.
  WebString reduced_accept_language;

  // Carries on the `navigational_delivery_type` in `NavigationParam` on the
  // renderer side.
  network::mojom::NavigationDeliveryType navigation_delivery_type =
      network::mojom::NavigationDeliveryType::kDefault;

  // Provides cached state from the previous Document that will be replaced by
  // this navigation for a ViewTransition.
  std::optional<ViewTransitionState> view_transition_state;

  // If this is a navigation to an "opaque-ads" fenced frame through an ad
  // auction, this stores the collection of properties that were loaded into a
  // fenced frame to specify its behavior. This is read into an inner
  // `FencedFrameConfig` object to give a fenced frame access to the
  // components associated with the winning bid in an auction.
  std::optional<FencedFrame::RedactedFencedFrameProperties>
      fenced_frame_properties;

  // Maps the blink runtime-enabled features modified in the browser process to
  // their new enabled/disabled status:
  // <enum_representing_runtime_enabled_feature, enabled/disabled>
  base::flat_map<::blink::mojom::RuntimeFeature, bool>
      modified_runtime_features;

  // The Storage Access API status that the document should be loaded with.
  net::StorageAccessApiStatus load_with_storage_access =
      net::StorageAccessApiStatus::kNone;

  // Indicates which browsing context group this frame belongs to. This starts
  // as nullopt and is only set when we commit a main frame in another browsing
  // context group. Same browsing context group navigations never set this
  // because no update is required. Subframes navigations never set this,
  // because they cannot change browsing context group.
  std::optional<BrowsingContextGroupInfo> browsing_context_group_info =
      std::nullopt;

  // For each document, the browser passes along state for each
  // renderer-enforced content setting.
  mojom::RendererContentSettingsPtr content_settings;

  // The cookie deprecation label for cookie deprecation facilitated testing.
  WebString cookie_deprecation_label;

  // The :visited link hashtable is stored in shared memory and contains salted
  // hashes for all visits. Each salt corresponds to a unique origin, and
  // renderer processes are only informed of salts that correspond to their
  // origins. As a result, any given renderer process can only
  // learn about visits relevant to origins for which it has the salt.
  //
  // Here we store the salt corresponding to this navigation's origin to
  // be committed. It will allow the renderer process that commits this
  // navigation to learn about visits hashed with this salt. If the :visited
  // link hashtable is not yet initialized (or the feature is disabled), the
  // salt value will not be set here. Instead, PartitionedVisitedLinkWriter will
  // send the salt values to the renderer (specifically to VisitedLinkReader via
  // the VisitedLinkNotificationSink interface) after the :visited link
  // hashtable is initialized.
  std::optional<uint64_t> visited_link_salt;

  // Map of permission statuses at commit time.
  // Note: the permission statues will be only used as initial states of
  // `CachedPermissionStatus` in renderer side.
  //  Could be std::nullopt for synchronous commit, same document navigations.
  std::optional<base::flat_map<mojom::PermissionName, mojom::PermissionStatus>>
      initial_permission_statuses;

  // When this is set to true, the navigation must create a new document
  // sequence number to avoid appearing as a same-document navigation, even if
  // the URL seems like a match. This matters for cross-origin navigations
  // (apart from error pages with the same precursor origin).
  bool force_new_document_sequence_number = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_NAVIGATION_PARAMS_H_
