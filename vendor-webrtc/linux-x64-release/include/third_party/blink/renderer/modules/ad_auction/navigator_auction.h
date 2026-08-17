// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_AD_AUCTION_NAVIGATOR_AUCTION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_AD_AUCTION_NAVIGATOR_AUCTION_H_

#include <stdint.h>

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "base/uuid.h"
#include "mojo/public/cpp/base/big_buffer.h"
#include "third_party/blink/public/common/fenced_frame/redacted_fenced_frame_config.h"
#include "third_party/blink/public/common/interest_group/auction_config.h"
#include "third_party/blink/public/mojom/interest_group/ad_auction_service.mojom-blink.h"
#include "third_party/blink/public/mojom/interest_group/interest_group_types.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/frame/navigator.h"
#include "third_party/blink/renderer/modules/ad_auction/join_leave_queue.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/weborigin/security_origin.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {
class AdAuctionData;
class AdAuctionDataConfig;
class AdRequestConfig;
class Ads;
class AuctionAdInterestGroup;
class AuctionAdInterestGroupKey;
class AuctionAdConfig;
class ProtectedAudience;
class V8UnionFencedFrameConfigOrUSVString;

class MODULES_EXPORT NavigatorAuction final
    : public GarbageCollected<NavigatorAuction>,
      public Supplement<Navigator> {
 public:
  class AuctionHandle;
  static const char kSupplementName[];

  explicit NavigatorAuction(Navigator&);

  // Gets, or creates, NavigatorAuction supplement on Navigator.
  // See platform/Supplementable.h
  static NavigatorAuction& From(ExecutionContext*, Navigator&);

  // TODO(crbug.com/1441988): Make `const AuctionAdInterestGroup*` after rename.
  ScriptPromise<IDLUndefined> joinAdInterestGroup(ScriptState*,
                                                  AuctionAdInterestGroup*,
                                                  std::optional<double>,
                                                  ExceptionState&);
  static ScriptPromise<IDLUndefined> joinAdInterestGroup(
      ScriptState*,
      Navigator&,
      AuctionAdInterestGroup*,
      double,
      ExceptionState&);
  static ScriptPromise<IDLUndefined> joinAdInterestGroup(
      ScriptState*,
      Navigator&,
      AuctionAdInterestGroup*,
      ExceptionState&);
  ScriptPromise<IDLUndefined> leaveAdInterestGroup(
      ScriptState*,
      const AuctionAdInterestGroupKey*,
      ExceptionState&);
  static ScriptPromise<IDLUndefined> leaveAdInterestGroup(
      ScriptState*,
      Navigator&,
      const AuctionAdInterestGroupKey*,
      ExceptionState&);

  // Implicit leaveAdInterestGroup - only supported when called from within
  // a fenced frame showing FLEDGE ads.
  ScriptPromise<IDLUndefined> leaveAdInterestGroupForDocument(ScriptState*,
                                                              ExceptionState&);
  static ScriptPromise<IDLUndefined> leaveAdInterestGroup(ScriptState*,
                                                          Navigator&,
                                                          ExceptionState&);

  ScriptPromise<IDLUndefined> clearOriginJoinedAdInterestGroups(
      ScriptState*,
      const String&,
      Vector<String>,
      ExceptionState&);
  static ScriptPromise<IDLUndefined> clearOriginJoinedAdInterestGroups(
      ScriptState*,
      Navigator&,
      const String,
      ExceptionState&);
  static ScriptPromise<IDLUndefined> clearOriginJoinedAdInterestGroups(
      ScriptState*,
      Navigator&,
      const String,
      const Vector<String>,
      ExceptionState&);

  void updateAdInterestGroups();
  static void updateAdInterestGroups(ScriptState*, Navigator&, ExceptionState&);
  // TODO(crbug.com/1441988): Make `const AuctionAdConfig*` after rename.
  ScriptPromise<IDLNullable<V8UnionFencedFrameConfigOrUSVString>> runAdAuction(
      ScriptState*,
      AuctionAdConfig*,
      ExceptionState&,
      base::TimeTicks start_time = base::TimeTicks::Now());
  static ScriptPromise<IDLNullable<V8UnionFencedFrameConfigOrUSVString>>
  runAdAuction(ScriptState*, Navigator&, AuctionAdConfig*, ExceptionState&);

  ScriptPromise<IDLString> createAuctionNonce(ScriptState*, ExceptionState&);
  static ScriptPromise<IDLString> createAuctionNonce(ScriptState*,
                                                     Navigator&,
                                                     ExceptionState&);

  // If called from a FencedFrame that was navigated to the URN resulting from
  // an interest group ad auction, returns a Vector of ad component URNs
  // associated with the winning bid in that auction.
  //
  // `num_ad_components` is the number of ad component URNs to put in the
  // Vector. To avoid leaking data from the winning bidder worklet, the number
  // of ad components in the winning bid is not exposed. Instead, it's padded
  // with URNs to length kMaxAdAuctionAdComponents, and calling this method
  // returns the first `num_ad_components` URNs.
  //
  // Throws an exception if `num_ad_components` is greater than
  // kMaxAdAuctionAdComponents, or if called from a frame that was not navigated
  // to a URN representing the winner of an ad auction.
  static Vector<String> adAuctionComponents(ScriptState* script_state,
                                            Navigator& navigator,
                                            uint16_t num_ad_components,
                                            ExceptionState& exception_state);

  ScriptPromise<IDLUSVString> deprecatedURNToURL(
      ScriptState* script_state,
      const String& urn_uuid,
      bool send_reports,
      ExceptionState& exception_state);

  static ScriptPromise<IDLUSVString> deprecatedURNToURL(
      ScriptState* script_state,
      Navigator& navigator,
      const V8UnionFencedFrameConfigOrUSVString* urn_or_config,
      bool send_reports,
      ExceptionState& exception_state);

  ScriptPromise<IDLUndefined> deprecatedReplaceInURN(
      ScriptState* script_state,
      const String& urn_uuid,
      const Vector<std::pair<String, String>>& replacement,
      ExceptionState& exception_state);

  static ScriptPromise<IDLUndefined> deprecatedReplaceInURN(
      ScriptState* script_state,
      Navigator& navigator,
      const V8UnionFencedFrameConfigOrUSVString* urn_or_config,
      const Vector<std::pair<String, String>>& replacement,
      ExceptionState& exception_state);

  ScriptPromise<AdAuctionData> getInterestGroupAdAuctionData(
      ScriptState* script_state,
      const AdAuctionDataConfig* config,
      ExceptionState& exception_state,
      base::TimeTicks start_time = base::TimeTicks::Now());
  static ScriptPromise<AdAuctionData> getInterestGroupAdAuctionData(
      ScriptState* script_state,
      Navigator& navigator,
      const AdAuctionDataConfig* config,
      ExceptionState& exception_state);

  ScriptPromise<Ads> createAdRequest(ScriptState*,
                                     const AdRequestConfig*,
                                     ExceptionState&);
  static ScriptPromise<Ads> createAdRequest(ScriptState*,
                                            Navigator&,
                                            const AdRequestConfig*,
                                            ExceptionState&);
  ScriptPromise<IDLString> finalizeAd(ScriptState*,
                                      const Ads*,
                                      const AuctionAdConfig*,
                                      ExceptionState&);
  static ScriptPromise<IDLString> finalizeAd(ScriptState*,
                                             Navigator&,
                                             const Ads*,
                                             const AuctionAdConfig*,
                                             ExceptionState&);

  // Web-exposed API that returns whether an opaque-ads fenced frame would be
  // allowed to be created in the current active document of this node after
  // an ad auction is run.
  // Checks the following criteria:
  // - Not trying to load in a default mode fenced frame tree
  // - All of the sandbox/allow flags required to load a fenced frame are set
  //   in the embedder. See: blink::kFencedFrameMandatoryUnsandboxedFlags
  // - No CSP headers are in place that will stop the fenced frame from loading
  // - No CSPEE is applied to this or an ancestor frame
  bool canLoadAdAuctionFencedFrame(ScriptState*);
  static bool canLoadAdAuctionFencedFrame(ScriptState*, Navigator&);

  // Expose whether kFledgeEnforceKAnonymity feature is enabled or not.
  static bool deprecatedRunAdAuctionEnforcesKAnonymity(ScriptState*,
                                                       Navigator&);

  static ProtectedAudience* protectedAudience(ScriptState*,
                                              Navigator& navigator);

  void Trace(Visitor* visitor) const override {
    visitor->Trace(ad_auction_service_);
    visitor->Trace(protected_audience_);
    Supplement<Navigator>::Trace(visitor);
  }

 private:
  // Pending cross-site interest group joins and leaves. These may be added to a
  // queue before being passed to the browser process.

  struct PendingJoin {
    mojom::blink::InterestGroupPtr interest_group;
    mojom::blink::AdAuctionService::JoinInterestGroupCallback callback;
  };

  struct PendingLeave {
    scoped_refptr<const SecurityOrigin> owner;
    String name;
    mojom::blink::AdAuctionService::LeaveInterestGroupCallback callback;
  };

  struct PendingClear {
    scoped_refptr<const SecurityOrigin> owner;
    Vector<String> interest_groups_to_keep;
    mojom::blink::AdAuctionService::LeaveInterestGroupCallback callback;
  };

  // Tells the browser process to start `pending_join`. Its callback will be
  // invoked on completion.
  void StartJoin(PendingJoin&& pending_join);

  // Completion callback for joinInterestGroup() Mojo calls.
  void JoinComplete(bool is_cross_origin,
                    ScriptPromiseResolver<IDLUndefined>* resolver,
                    bool failed_well_known_check);

  // Tells the browser process to start `pending_leave`. Its callback will be
  // invoked on completion.
  void StartLeave(PendingLeave&& pending_leave);

  // Completion callback for clearOriginJoinedAdInterestGroups() Mojo calls.
  void LeaveComplete(bool is_cross_origin,
                     ScriptPromiseResolver<IDLUndefined>* resolver,
                     bool failed_well_known_check);

  // Tells the browser process to start `pending_clear`. Its callback will be
  // invoked on completion.
  void StartClear(PendingClear&& pending_clear);

  // Completion callback for leaveInterestGroup() Mojo calls.
  void ClearComplete(bool is_cross_origin,
                     ScriptPromiseResolver<IDLUndefined>* resolver,
                     bool failed_well_known_check);

  // Completion callback for createAdRequest() Mojo call.
  void AdsRequested(ScriptPromiseResolver<Ads>* resolver,
                    const WTF::String& ads_guid);
  // Completion callback for finalizeAd() Mojo call.
  void FinalizeAdComplete(ScriptPromiseResolver<IDLString>* resolver,
                          const std::optional<KURL>& creative_url);
  // Completion callback for Mojo call made by deprecatedURNToURL().
  void GetURLFromURNComplete(ScriptPromiseResolver<IDLUSVString>*,
                             const std::optional<KURL>&);
  // Completion callback for Mojo call made by deprecatedReplaceInURNComplete().
  void ReplaceInURNComplete(ScriptPromiseResolver<IDLUndefined>* resolver);

  void GetInterestGroupAdAuctionDataComplete(
      base::TimeTicks start_time,
      bool is_single_seller,
      ScriptPromiseResolver<AdAuctionData>* resolver,
      Vector<mojom::blink::AdAuctionPerSellerRequestPtr> requests,
      const std::optional<base::Uuid>& request_id);

  // Manage queues of cross-site join and leave operations that have yet to be
  // sent to the browser process.
  JoinLeaveQueue<PendingJoin> queued_cross_site_joins_;
  JoinLeaveQueue<PendingLeave> queued_cross_site_leaves_;
  JoinLeaveQueue<PendingClear> queued_cross_site_clears_;

  // The next available auction nonce suffix, used alongside the
  // `base_auction_nonce` provided by the Browser process to create unique
  // auction nonces when createAuctionNonce. Though this counter has 32 bits,
  // only the least significant 24 bits are used.
  uint32_t auction_nonce_counter_ = 0;

  HeapMojoRemote<mojom::blink::AdAuctionService> ad_auction_service_;
  Member<ProtectedAudience> protected_audience_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_AD_AUCTION_NAVIGATOR_AUCTION_H_
