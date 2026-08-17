// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_PUSH_PULL_FIFO_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_PUSH_PULL_FIFO_H_

#include "base/synchronization/lock.h"
#include "third_party/blink/renderer/platform/audio/audio_bus.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "third_party/blink/renderer/platform/wtf/threading.h"

namespace blink {

// A configuration data container for PushPullFIFO unit test.
struct PushPullFIFOStateForTest {
  const size_t fifo_length;
  const unsigned number_of_channels;
  const size_t frames_available;
  const size_t index_read;
  const size_t index_write;
  const unsigned overflow_count;
  const unsigned underflow_count;
};

// PushPullFIFO class is an intermediate audio sample storage between
// Blink-WebAudio and the renderer. The renderer's hardware callback buffer size
// varies on the platform, but the WebAudio always renders 128 frames (render
// quantum, RQ) thus FIFO is needed to handle the general case.
//
// Note that this object is concurrently accessed by two threads; WebAudio
// rendering thread (WebThread) in Blink and the audio device thread
// (AudioDeviceThread) from the media renderer. The push/pull operations touch
// most of variables in the class (index_write_, index_read_, frames_available_,
// and fifo_Bus_) so the thread safety must be handled with care.
//
// TODO(hongchan): add a unit test for multi-thread access.
class PLATFORM_EXPORT PushPullFIFO final {
  USING_FAST_MALLOC(PushPullFIFO);

 public:
  struct PullResult {
    uint32_t frames_provided = 0;
    size_t frames_to_render = 0;
  };

  // ||render_quantum_frames| is the render size used by the audio graph.  It
  // |defaults to 128, the original and default render size.
  explicit PushPullFIFO(unsigned number_of_channels,
                        uint32_t fifo_length,
                        unsigned render_quantum_frames = 128);
  PushPullFIFO(const PushPullFIFO&) = delete;
  PushPullFIFO& operator=(const PushPullFIFO&) = delete;
  ~PushPullFIFO();

  // Pushes the rendered frames by WebAudio engine.
  //  - The |input_bus| length has a length equal to |render_quantum_frames_|,
  //  fixed.
  //  - In case of overflow (FIFO full while push), the existing frames in FIFO
  //    will be overwritten and |index_read_| will be forcibly moved to
  //    |index_write_| to avoid reading overwritten frames.
  void Push(const AudioBus* input_bus);

  // Pulls |frames_requested| by the audio device thread and returns the actual
  // number of frames to be rendered by the source. (i.e. WebAudio graph)
  //  - If |frames_requested| is bigger than the length of |output_bus|, it
  //    violates SECURITY_CHECK().
  //  - If |frames_requested| is bigger than FIFO length, it violates
  //    SECURITY_CHECK().
  //  - In case of underflow (FIFO empty while pull), the remaining space in the
  //    requested output bus will be filled with silence. Thus it will fulfill
  //    the request from the consumer without causing error, but with a glitch.
  size_t Pull(AudioBus* output_bus, uint32_t frames_requested);

  // Pull and update `earmark_frames_` to make the dual thread rendering mode
  // (i.e. AudioWorklet) more smooth. (The single thread rendering does not need
  // this treatment.) Returns the number of frames which are pulled (guaranteed
  // to not exceed `frames_requested`) and the number of frames to be rendered
  // by the source (i.e. WebAudio graph).
  PullResult PullAndUpdateEarmark(AudioBus* output_bus,
                                  uint32_t frames_requested);

  void SetEarmarkFrames(size_t earmark_frames) {
    DCHECK(IsMainThread());
    base::AutoLock locker(lock_);
    earmark_frames_ = earmark_frames;
  }

  uint32_t length() const { return fifo_length_; }
  unsigned NumberOfChannels() const {
    lock_.AssertAcquired();
    return fifo_bus_->NumberOfChannels();
  }

  uint32_t GetFramesAvailable() {
    base::AutoLock locker(lock_);
    return frames_available_;
  }

  AudioBus* GetFIFOBusForTest() {
    base::AutoLock locker(lock_);
    return fifo_bus_.get();
  }

  size_t GetEarmarkFramesForTest() {
    base::AutoLock locker(lock_);
    return earmark_frames_;
  }

  // For single thread unit test only. Get the current configuration that
  // consists of FIFO length, number of channels, read/write index position and
  // under/overflow count.
  const PushPullFIFOStateForTest GetStateForTest();

 private:
  // The size of the FIFO.
  const uint32_t fifo_length_ = 0;

  // The render size used by the audio graph.
  const unsigned render_quantum_frames_;

  // For UMA reporting purpose.
  unsigned pull_count_ = 0;
  unsigned overflow_count_ = 0;
  unsigned underflow_count_ = 0;

  base::Lock lock_;

  // To adapt the unstable callback timing. Every buffer underrun from
  // PullAndUpdateEarmark() will increase this number.
  size_t earmark_frames_ GUARDED_BY(lock_) = 0;

  // The number of frames in the FIFO actually available for pulling.
  uint32_t frames_available_ GUARDED_BY(lock_) = 0;
  size_t index_read_ GUARDED_BY(lock_) = 0;
  size_t index_write_ GUARDED_BY(lock_) = 0;
  scoped_refptr<AudioBus> fifo_bus_ GUARDED_BY(lock_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_PUSH_PULL_FIFO_H_
