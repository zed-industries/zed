// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ENCODED_UNDERLYING_SOURCE_WRAPPER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ENCODED_UNDERLYING_SOURCE_WRAPPER_H_

#include "base/gtest_prod_util.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "base/unguessable_token.h"
#include "third_party/blink/renderer/core/streams/underlying_source_base.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_encoded_audio_underlying_source.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_encoded_video_underlying_source.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_persistent.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"

namespace blink {

// This wrapper acts as an underlying source and is used when it's still unknown
// if it is going to be video or audio. Once it's known, one of the
// members should be initialized.
class MODULES_EXPORT RTCEncodedUnderlyingSourceWrapper
    : public UnderlyingSourceBase {
 public:
  explicit RTCEncodedUnderlyingSourceWrapper(
      ScriptState*,
      WTF::CrossThreadOnceClosure disconnect_callback =
          WTF::CrossThreadOnceClosure());

  // UnderlyingSourceBase
  ScriptPromise<IDLUndefined> Pull(ScriptState*, ExceptionState&) override;
  ScriptPromise<IDLUndefined> Cancel(ScriptState*,
                                     ScriptValue reason,
                                     ExceptionState&) override;
  void Close();

  void CreateAudioUnderlyingSource(
      WTF::CrossThreadOnceClosure disconnect_callback_source,
      base::UnguessableToken owner_id);
  void CreateVideoUnderlyingSource(
      WTF::CrossThreadOnceClosure disconnect_callback_source,
      base::UnguessableToken owner_id);

  using VideoTransformer = WTF::CrossThreadRepeatingFunction<void(
      std::unique_ptr<webrtc::TransformableVideoFrameInterface>)>;
  using AudioTransformer = WTF::CrossThreadRepeatingFunction<void(
      std::unique_ptr<webrtc::TransformableAudioFrameInterface>)>;

  VideoTransformer GetVideoTransformer();
  AudioTransformer GetAudioTransformer();

  void Clear();
  void Trace(Visitor*) const override;

 private:
  Member<RTCEncodedAudioUnderlyingSource> audio_from_encoder_underlying_source_;
  Member<RTCEncodedVideoUnderlyingSource> video_from_encoder_underlying_source_;
  Member<ScriptState> script_state_;

  // This sequence checker is for checking that all the methods are called on
  // the same thread.
  SEQUENCE_CHECKER(sequence_checker_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ENCODED_UNDERLYING_SOURCE_WRAPPER_H_
