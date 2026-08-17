// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_PUSHABLE_MEDIA_STREAM_AUDIO_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_PUSHABLE_MEDIA_STREAM_AUDIO_SOURCE_H_

#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/synchronization/lock.h"
#include "base/task/sequenced_task_runner.h"
#include "base/task/single_thread_task_runner.h"
#include "base/thread_annotations.h"
#include "media/base/audio_buffer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_audio_source.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"

namespace blink {

// Simplifies the creation of audio tracks.
class MODULES_EXPORT PushableMediaStreamAudioSource
    : public MediaStreamAudioSource {
 public:
  // Helper class that facilitates interacting with a
  // PushableMediaStreamAudioSource from multiple threads. This also includes
  // safely posting tasks to/from outside the main thread.
  // The public methods of this class can be called on any thread.
  class MODULES_EXPORT LOCKABLE Broker
      : public WTF::ThreadSafeRefCounted<Broker> {
   public:
    Broker(const Broker&) = delete;
    Broker& operator=(const Broker&) = delete;

    // Increases the count of connected clients.
    void OnClientStarted();
    // Decreases the count of connected clients. If the count reaches zero,
    // StopSource() is called.
    // OnClientStarted() should not be called after the number of clients
    // reaches zero and StopSource() has been been called. It would have
    // no effect.
    // In practice, the clients are WritableStream underlying sources that
    // use the same PushableMediaStreamAudioSource, such as the sources for
    // the transferred version of a stream (for example, in a Worker) and the
    // corresponding original stream (for example, in the Window scope).
    // During a transfer, a new client is created in the new realm, then the
    // old client disconnects.
    void OnClientStopped();
    bool IsRunning();
    void PushAudioData(scoped_refptr<media::AudioBuffer> data);
    void StopSource();
    void SetShouldDeliverAudioOnAudioTaskRunner(
        bool should_deliver_audio_on_audio_task_runner);
    bool ShouldDeliverAudioOnAudioTaskRunner();

   private:
    friend class PushableMediaStreamAudioSource;

    explicit Broker(PushableMediaStreamAudioSource* source,
                    scoped_refptr<base::SequencedTaskRunner> audio_task_runner);

    // These functions must be called on |main_task_runner_|.
    void OnSourceStarted();
    void OnSourceDestroyedOrStopped();
    void StopSourceOnMain();
    void AssertLockAcquired() const ASSERT_EXCLUSIVE_LOCK();

    base::Lock lock_;
    // Source can only change its value on |main_task_runner_|. We use |lock_|
    // to guard it for value changes and for reads outside |main_task_runner_|.
    // It is not necessary to guard it with |lock_| to read its value on
    // |main_task_runner_|. This helps avoid deadlocks in
    // Stop()/OnSourceDestroyedOrStopped() interactions.
    raw_ptr<PushableMediaStreamAudioSource> source_;
    // The same apples to |is_running_|, but since it does not have complex
    // interactions with owners, like |source_| does, we always guard it for
    // simplicity.
    bool is_running_ GUARDED_BY(lock_) = false;
    int num_clients_ GUARDED_BY(lock_) = 0;
    scoped_refptr<base::SingleThreadTaskRunner> main_task_runner_;

    int should_deliver_audio_on_audio_task_runner_ GUARDED_BY(lock_) = true;
    const scoped_refptr<base::SequencedTaskRunner> audio_task_runner_;
  };

  PushableMediaStreamAudioSource(
      scoped_refptr<base::SingleThreadTaskRunner> main_task_runner,
      scoped_refptr<base::SequencedTaskRunner> audio_task_runner);

  ~PushableMediaStreamAudioSource() override;

  // Push data to the audio tracks. Can be called from any thread.
  void PushAudioData(scoped_refptr<media::AudioBuffer> data);

  // These functions can be called on any thread.
  bool IsRunning() const { return broker_->IsRunning(); }
  scoped_refptr<Broker> GetBroker() const { return broker_; }

 private:
  friend class Broker;
  // Actually push data to the audio tracks. Can be called from any thread.
  void DeliverData(scoped_refptr<media::AudioBuffer> data);

  // MediaStreamAudioSource implementation.
  bool EnsureSourceIsStarted() final;
  void EnsureSourceIsStopped() final;

  int last_channels_ GUARDED_BY(broker_) = 0;
  int last_frames_ GUARDED_BY(broker_) = 0;
  int last_sample_rate_ GUARDED_BY(broker_) = 0;

  const scoped_refptr<Broker> broker_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_PUSHABLE_MEDIA_STREAM_AUDIO_SOURCE_H_
