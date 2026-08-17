// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CAPABILITIES_MEDIA_CAPABILITIES_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CAPABILITIES_MEDIA_CAPABILITIES_H_

#include "base/gtest_prod_util.h"
#include "base/memory/raw_ptr.h"
#include "base/time/time.h"
#include "media/base/video_codecs.h"  // for media::VideoCodecProfile
#include "media/base/video_color_space.h"
#include "media/learning/mojo/public/cpp/mojo_learning_task_controller.h"
#include "media/learning/mojo/public/mojom/learning_task_controller.mojom-blink.h"
#include "media/mojo/mojom/video_decode_perf_history.mojom-blink.h"
#include "media/mojo/mojom/webrtc_video_perf.mojom-blink.h"
#include "third_party/blink/public/common/privacy_budget/identifiable_token.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_video_configuration.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/peerconnection/webrtc_decoding_info_handler.h"
#include "third_party/blink/renderer/platform/peerconnection/webrtc_encoding_info_handler.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class ExceptionState;
class ExecutionContext;
class MediaCapabilitiesDecodingInfo;
class MediaCapabilitiesInfo;
class MediaDecodingConfiguration;
class MediaEncodingConfiguration;
class MediaKeySystemAccess;
class NavigatorBase;
class ScriptState;

class MODULES_EXPORT MediaCapabilities final
    : public ScriptWrappable,
      public Supplement<NavigatorBase> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kLearningBadWindowThresholdParamName[];
  static const char kLearningNnrThresholdParamName[];
  static const char kWebrtcDecodeSmoothIfPowerEfficientParamName[];
  static const char kWebrtcEncodeSmoothIfPowerEfficientParamName[];

  static const char kSupplementName[];

  // Getter for navigator.mediaCapabilities
  static MediaCapabilities* mediaCapabilities(NavigatorBase&);

  explicit MediaCapabilities(NavigatorBase&);

  void Trace(blink::Visitor* visitor) const override;

  ScriptPromise<MediaCapabilitiesDecodingInfo> decodingInfo(
      ScriptState*,
      const MediaDecodingConfiguration*,
      ExceptionState&);
  ScriptPromise<MediaCapabilitiesInfo> encodingInfo(
      ScriptState*,
      const MediaEncodingConfiguration*,
      ExceptionState&);

 private:
  // Stores pending callback state from and intermediate prediction values while
  // we wait for all predictions to arrive.
  class PendingCallbackState : public GarbageCollected<PendingCallbackState> {
   public:
    PendingCallbackState(ScriptPromiseResolverBase* resolver,
                         MediaKeySystemAccess* access,
                         const base::TimeTicks& request_time,
                         std::optional<IdentifiableToken> input_token);
    virtual void Trace(blink::Visitor* visitor) const;

    Member<ScriptPromiseResolverBase> resolver;
    Member<MediaKeySystemAccess> key_system_access;
    std::optional<bool> is_supported;
    std::optional<bool> is_bad_window_prediction_smooth;
    std::optional<bool> is_nnr_prediction_smooth;
    std::optional<bool> db_is_smooth;
    std::optional<bool> db_is_power_efficient;
    std::optional<bool> is_gpu_factories_supported;
    std::optional<bool> is_builtin_video_codec;
    base::TimeTicks request_time;
    std::optional<IdentifiableToken> input_token;
  };

  FRIEND_TEST_ALL_PREFIXES(MediaCapabilitiesTests,
                           WebrtcDecodePowerEfficientIsSmooth);
  FRIEND_TEST_ALL_PREFIXES(MediaCapabilitiesTests,
                           WebrtcDecodeOverridePowerEfficientIsSmooth);
  FRIEND_TEST_ALL_PREFIXES(MediaCapabilitiesTests,
                           WebrtcEncodePowerEfficientIsSmooth);
  FRIEND_TEST_ALL_PREFIXES(MediaCapabilitiesTests,
                           WebrtcEncodeOverridePowerEfficientIsSmooth);

  // Lazily binds remote LearningTaskControllers for ML smoothness predictions
  // and returns whether binding succeeds. Returns true if it was already bound.
  bool EnsureLearningPredictors(ExecutionContext*);

  // Lazily binds to the VideoDecodePerfHistory service. Returns whether it was
  // successful. Returns true if it was already bound.
  bool EnsurePerfHistoryService(ExecutionContext*);

  // Lazily binds to the WebrtcVideoPerfHistory service. Returns whether it was
  // successful. Returns true if it was already bound.
  bool EnsureWebrtcPerfHistoryService(ExecutionContext* execution_context);

  ScriptPromise<MediaCapabilitiesDecodingInfo> GetEmeSupport(
      ScriptState*,
      media::VideoCodec,
      media::VideoCodecProfile,
      media::VideoColorSpace,
      const MediaDecodingConfiguration*,
      const base::TimeTicks& request_time,
      ExceptionState&);
  // Gets perf info from VideoDecodePerrHistory DB. Will optionally kick off
  // parallel request to GetPerfInfo_ML() when learning experiment is enabled.
  void GetPerfInfo(media::VideoCodec,
                   media::VideoCodecProfile,
                   media::VideoColorSpace,
                   const MediaDecodingConfiguration*,
                   const base::TimeTicks& request_time,
                   ScriptPromiseResolver<MediaCapabilitiesDecodingInfo>*,
                   MediaKeySystemAccess*);

  // Gets ML perf predictions from remote LearingTaskControllers.
  void GetPerfInfo_ML(ExecutionContext* execution_context,
                      int callback_id,
                      media::VideoCodec video_codec,
                      media::VideoCodecProfile video_profile,
                      int width,
                      double framerate);

  // Query media::GpuVideoAcceleratorFactories for support of hardware
  // accelerate decode. Only called when |UseGpuFactoriesForPowerEfficient()|
  // is true.
  void GetGpuFactoriesSupport(int callback_id,
                              media::VideoCodec video_codec,
                              media::VideoCodecProfile video_profile,
                              media::VideoColorSpace,
                              const MediaDecodingConfiguration*);

  // Callback for perf info from the VideoDecodePerfHistory service.
  void OnPerfHistoryInfo(int callback_id,
                         bool is_smooth,
                         bool is_power_efficient);

  // Callback for predictions from |bad_window_predictor_|.
  void OnBadWindowPrediction(
      int callback_id,
      const std::optional<::media::learning::TargetHistogram>& histogram);

  // Callback for predictions from |nnr_predictor_|.
  void OnNnrPrediction(
      int callback_id,
      const std::optional<::media::learning::TargetHistogram>& histogram);

  // Callback for GetGpuFactoriesSupport().
  void OnGpuFactoriesSupport(int callback_id,
                             bool is_supported,
                             media::VideoCodec video_codec);

  // Resolves the callback with associated |callback_id| and removes it from the
  // |pending_callback_map_|.
  void ResolveCallbackIfReady(int callback_id);

  enum class OperationType { kEncoding, kDecoding };
  void OnWebrtcSupportInfo(
      int callback_id,
      media::mojom::blink::WebrtcPredictionFeaturesPtr features,
      float frames_per_second,
      OperationType,
      bool is_supported,
      bool is_power_efficient);

  void OnWebrtcPerfHistoryInfo(int callback_id, OperationType, bool is_smooth);

  // Creates a new (incremented) callback ID from |last_callback_id_| for
  // mapping in |pending_cb_map_|.
  int CreateCallbackId();

  void set_webrtc_decoding_info_handler_for_test(
      WebrtcDecodingInfoHandler* handler) {
    webrtc_decoding_info_handler_for_test_ = handler;
  }

  void set_webrtc_encoding_info_handler_for_test(
      WebrtcEncodingInfoHandler* handler) {
    webrtc_encoding_info_handler_for_test_ = handler;
  }

  HeapMojoRemote<media::mojom::blink::VideoDecodePerfHistory>
      decode_history_service_;

  // Connection to a browser-process LearningTaskController for predicting the
  // number of consecutive "bad" dropped frame windows during a playback. See
  // media::SmoothnessHelper.
  HeapMojoRemote<media::learning::mojom::blink::LearningTaskController>
      bad_window_predictor_;

  // Connects to a browser-process LearningTaskController for predicting the
  // number of consecutive non-network re-buffers (NNRs). See
  // media::SmoothnessHelper.
  HeapMojoRemote<media::learning::mojom::blink::LearningTaskController>
      nnr_predictor_;

  HeapMojoRemote<media::mojom::blink::WebrtcVideoPerfHistory>
      webrtc_history_service_;

  // Holds the last key for callbacks in the map below. Incremented for each
  // usage.
  int last_callback_id_ = 0;

  // Maps a callback ID to state for pending callbacks.
  HeapHashMap<int, Member<PendingCallbackState>> pending_cb_map_;

  // Makes it possible to override the WebrtcDecodingInfoHandler in tests.
  raw_ptr<WebrtcDecodingInfoHandler> webrtc_decoding_info_handler_for_test_ =
      nullptr;

  // Makes it possible to override the WebrtcEncodingInfoHandler in tests.
  raw_ptr<WebrtcEncodingInfoHandler> webrtc_encoding_info_handler_for_test_ =
      nullptr;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CAPABILITIES_MEDIA_CAPABILITIES_H_
