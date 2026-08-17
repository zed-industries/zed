// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ENCODED_AUDIO_UNDERLYING_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ENCODED_AUDIO_UNDERLYING_SOURCE_H_

#include "base/gtest_prod_util.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "base/unguessable_token.h"
#include "third_party/blink/renderer/core/streams/underlying_source_base.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"

namespace webrtc {
class TransformableAudioFrameInterface;
}  // namespace webrtc

namespace blink {

class MODULES_EXPORT RTCEncodedAudioUnderlyingSource
    : public UnderlyingSourceBase {
 public:
  // If |controller_override| is provided, it won't work as an instance of
  // |UnderlyingSourceBase| so shouldn't be used directly, only with
  // RTCEncodedUnderlyingSourceWrapper.
  explicit RTCEncodedAudioUnderlyingSource(
      ScriptState*,
      WTF::CrossThreadOnceClosure disconnect_callback);
  explicit RTCEncodedAudioUnderlyingSource(
      ScriptState*,
      WTF::CrossThreadOnceClosure disconnect_callback,
      bool enable_frame_restrictions,
      base::UnguessableToken owner_id,
      ReadableStreamDefaultControllerWithScriptScope* controller_override =
          nullptr);

  // UnderlyingSourceBase
  ScriptPromise<IDLUndefined> Pull(ScriptState*, ExceptionState&) override;
  ScriptPromise<IDLUndefined> Cancel(ScriptState*,
                                     ScriptValue reason,
                                     ExceptionState&) override;

  void OnFrameFromSource(
      std::unique_ptr<webrtc::TransformableAudioFrameInterface>);
  void Close();

  // Called on any thread to indicate the source is being transferred to an
  // UnderlyingSource on a different thread to this.
  void OnSourceTransferStarted();

  void Trace(Visitor*) const override;

 private:
  // Implements the handling of this stream being transferred to another
  // context, called on the thread upon which the instance was created.
  void OnSourceTransferStartedOnTaskRunner();

  // In case there is controller override, this one is returned. If not,
  // Controller() from the underlying source base will be returned.
  ReadableStreamDefaultControllerWithScriptScope* GetController();

  FRIEND_TEST_ALL_PREFIXES(RTCEncodedAudioUnderlyingSourceTest,
                           QueuedFramesAreDroppedWhenOverflow);
  static const int kMinQueueDesiredSize;

  const Member<ScriptState> script_state_;
  WTF::CrossThreadOnceClosure disconnect_callback_;
  Member<ReadableStreamDefaultControllerWithScriptScope> override_controller_;
  // Count of frames dropped due to the queue being full, for logging.
  int dropped_frames_ = 0;
  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
  const bool enable_frame_restrictions_;
  const base::UnguessableToken owner_id_;
  int64_t last_enqueued_frame_counter_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ENCODED_AUDIO_UNDERLYING_SOURCE_H_
