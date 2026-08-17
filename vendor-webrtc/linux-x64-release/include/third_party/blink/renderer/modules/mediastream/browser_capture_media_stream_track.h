// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_BROWSER_CAPTURE_MEDIA_STREAM_TRACK_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_BROWSER_CAPTURE_MEDIA_STREAM_TRACK_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver_with_tracker.h"
#include "third_party/blink/renderer/modules/mediastream/crop_target.h"
#include "third_party/blink/renderer/modules/mediastream/media_stream_track_impl.h"
#include "third_party/blink/renderer/modules/mediastream/restriction_target.h"
#include "third_party/blink/renderer/modules/mediastream/sub_capture_target.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"

namespace blink {

class MODULES_EXPORT BrowserCaptureMediaStreamTrack
    : public MediaStreamTrackImpl {
  DEFINE_WRAPPERTYPEINFO();

 public:
  BrowserCaptureMediaStreamTrack(ExecutionContext* execution_context,
                                 MediaStreamComponent* component,
                                 base::OnceClosure callback);

  BrowserCaptureMediaStreamTrack(ExecutionContext* execution_context,
                                 MediaStreamComponent* component,
                                 MediaStreamSource::ReadyState ready_state,
                                 base::OnceClosure callback);

  ~BrowserCaptureMediaStreamTrack() override = default;

#if !BUILDFLAG(IS_ANDROID)
  void Trace(Visitor*) const override;

  // Allows tests to invoke OnSubCaptureTargetVersionObserved() directly, since
  // triggering it via mocks would be prohibitively difficult.
  void OnSubCaptureTargetVersionObservedForTesting(
      uint32_t sub_capture_target_version) {
    OnSubCaptureTargetVersionObserved(sub_capture_target_version);
  }
#endif

  ScriptPromise<IDLUndefined> cropTo(ScriptState*,
                                     CropTarget*,
                                     ExceptionState&);
  ScriptPromise<IDLUndefined> restrictTo(ScriptState*,
                                         RestrictionTarget*,
                                         ExceptionState&);

  BrowserCaptureMediaStreamTrack* clone(ExecutionContext*) override;

  // These values are persisted to logs. Entries should not be renumbered and
  // numeric values should never be reused.
  enum class ApplySubCaptureTargetResult {
    kOk = 0,
    kTimedOut = 1,
    kInvalidFormat = 2,
    kRejectedWithErrorGeneric = 3,
    kRejectedWithUnsupportedCaptureDevice = 4,
    kRejectedWithNotImplemented = 5,
    kNonIncreasingVersion = 6,
    kInvalidTarget = 7,
    kUnsupportedPlatform = 8,
    kMaxValue = kUnsupportedPlatform
  };

 private:
  // Helper function serving cropTo(), restrictTo(), and any potential
  // future function that takes a BCMST and mutates what it is capturing
  // to some subset of the original target, based on a target identified
  // using a SubCaptureTarget.
  ScriptPromise<IDLUndefined> ApplySubCaptureTarget(ScriptState*,
                                                    SubCaptureTarget::Type,
                                                    SubCaptureTarget*,
                                                    ExceptionState&);

#if !BUILDFLAG(IS_ANDROID)
  struct PromiseInfo : GarbageCollected<PromiseInfo> {
    explicit PromiseInfo(
        ScriptPromiseResolverWithTracker<ApplySubCaptureTargetResult,
                                         IDLUndefined>* promise_resolver)
        : promise_resolver(promise_resolver) {}

    void Trace(Visitor* visitor) const { visitor->Trace(promise_resolver); }

    const Member<ScriptPromiseResolverWithTracker<ApplySubCaptureTargetResult,
                                                  IDLUndefined>>
        promise_resolver;
    std::optional<media::mojom::ApplySubCaptureTargetResult> result;
    bool sub_capture_target_version_observed = false;
  };

  using SubCaptureTargetVersionToPromiseInfoMap =
      HeapHashMap<uint32_t,
                  Member<BrowserCaptureMediaStreamTrack::PromiseInfo>>;
  using PromiseMapIterator = SubCaptureTargetVersionToPromiseInfoMap::iterator;

  // Each cropTo() or restrictTo() call is associated with a unique
  // |sub_capture_target_version| which identifies this specific invocation.
  // When the browser process responds with the result of the invocation,
  // it triggers a call to OnResultFromBrowserProcess() with that
  // |sub_capture_target_version|.
  void OnResultFromBrowserProcess(
      uint32_t sub_capture_target_version,
      media::mojom::ApplySubCaptureTargetResult result);

  // OnSubCaptureTargetVersionObserved() is posted as a callback, bound to a
  // unique |sub_capture_target_version|. This callback be invoked when the
  // first frame is observed which is associated with that
  // |sub_capture_target_version|.
  // TODO(crbug.com/1266378): The Promise should also be resolved if a
  // a barrier event is observed. (That is, although no frame is delivered,
  // there is a guarantee that all future frames will be of this version
  // or later. This would happen if cropping a muted track, for instance.)
  void OnSubCaptureTargetVersionObserved(uint32_t sub_capture_target_version);

  // The Promise that cropTo() issued is resolved when both conditions
  // are fulfulled:
  // 1. OnResultFromBrowserProcess(kSuccess) called.
  // 2. OnSubCaptureTargetVersionObserved() called for the associated
  // |sub_capture_target_version|.
  //
  // The order of fulfillment does not matter.
  //
  // The Promise is rejected if OnResultFromBrowserProcess() is called with
  // an error value.
  void MaybeFinalizeCropPromise(PromiseMapIterator iter);

  // Each time cropTo() is called on a given track, its sub-capture-target
  // version increments. Associate each Promise with its sub-capture-target
  // version, so that Viz can easily stamp each frame. When we see the first
  // such frame, or an equivalent message, we can resolve the Promise. (An
  // "equivalent message" can be a notification of a dropped frame, or a
  // notification that a frame was not produced due to consisting of 0 pixels
  // after the crop was applied, or anything similar.)
  //
  // Note that frames before the first call to cropTo() will be associated
  // with a version of 0, both here and in Viz.
  HeapHashMap<uint32_t, Member<PromiseInfo>> pending_promises_;
#endif  // !BUILDFLAG(IS_ANDROID)
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_BROWSER_CAPTURE_MEDIA_STREAM_TRACK_H_
