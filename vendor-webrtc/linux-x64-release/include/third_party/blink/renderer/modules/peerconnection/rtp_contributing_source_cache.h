// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTP_CONTRIBUTING_SOURCE_CACHE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTP_CONTRIBUTING_SOURCE_CACHE_H_

#include <utility>
#include <vector>

#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/synchronization/waitable_event.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_rtc_rtp_contributing_source.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_rtc_rtp_synchronization_source.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_rtp_receiver.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_rtp_source.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class RTCPeerConnection;
class ScriptState;

// Implements RTCRtpReceiver.getSynchronizationSources/getContributingSources as
// well as a cache of the result. The cache serves two purposes:
// 1. According to spec, calling the getters multiple times inside the same task
//    execution cycle must return the same values. The cache is cleared in the
//    next microtask.
// 2. Getting the SSRC/CSRC values involves a block-invoke to the WebRTC worker
//    thread. This class uses a heuristic to maybe-update the cache for all
//    RTCRtpReceiver objects inside the same block-invoke, reducing the total
//    number of block-invokes if the getters are called on every RTCRtpReceiver.
class RtpContributingSourceCache {
 public:
  typedef Vector<std::unique_ptr<RTCRtpSource>> RTCRtpSources;

  RtpContributingSourceCache(
      RTCPeerConnection* pc,
      scoped_refptr<base::SingleThreadTaskRunner> worker_thread_runner);

  // When the owner of this object is Disposed(), this method must be called to
  // cancel any in-flight tasks.
  void Shutdown();

  HeapVector<Member<RTCRtpSynchronizationSource>> getSynchronizationSources(
      ScriptState* script_state,
      ExceptionState& exception_state,
      RTCRtpReceiver* receiver);
  HeapVector<Member<RTCRtpContributingSource>> getContributingSources(
      ScriptState* script_state,
      ExceptionState& exception_state,
      RTCRtpReceiver* receiver);

 private:
  // Ensures the cache for `requesting_receiver` is up-to-date. Based on a
  // heuristic, this method may update the cache for all other receivers while
  // it is at it.
  void MaybeUpdateRtpSources(ScriptState* script_state,
                             RTCRtpReceiver* requesting_receiver);
  void UpdateRtpSourcesOnWorkerThread(
      Vector<RTCRtpReceiverPlatform*>* receivers,
      HashMap<RTCRtpReceiverPlatform*, RTCRtpSources>*
          cached_sources_by_receiver,
      base::WaitableEvent* event);
  void ClearCache();
  const RTCRtpSources* GetRtpSources(RTCRtpReceiver* receiver) const;

  // Owner of all RTCRtpReceiver objects that this cache is concerned with.
  const WeakPersistent<RTCPeerConnection> pc_;
  const scoped_refptr<base::SingleThreadTaskRunner> worker_thread_runner_;
  // We cache audio and video receivers separately in case the app is only
  // interested in one of the kinds. Having a small fixed number of audio
  // receivers where audio levels are polled and a large number of video
  // receivers that are not being polled is a common setup.
  HashMap<RTCRtpReceiverPlatform*, RTCRtpSources>
      cached_sources_by_audio_receiver_;
  HashMap<RTCRtpReceiverPlatform*, RTCRtpSources>
      cached_sources_by_video_receiver_;

  base::WeakPtrFactory<RtpContributingSourceCache> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTP_CONTRIBUTING_SOURCE_CACHE_H_
