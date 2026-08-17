// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_SCRIPT_PROCESSOR_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_SCRIPT_PROCESSOR_HANDLER_H_

#include "base/gtest_prod_util.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/synchronization/lock.h"
#include "base/synchronization/waitable_event.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"
#include "third_party/blink/renderer/platform/audio/audio_bus.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace base {
class SingleThreadTaskRunner;
class WaitableEvent;
}

namespace blink {

class AudioBuffer;
class BaseAudioContext;
class SharedAudioBuffer;

class ScriptProcessorHandler final : public AudioHandler {
 public:
  static scoped_refptr<ScriptProcessorHandler> Create(
      AudioNode&,
      float sample_rate,
      uint32_t buffer_size,
      uint32_t number_of_input_channels,
      uint32_t number_of_output_channels,
      const HeapVector<Member<AudioBuffer>>& input_buffers,
      const HeapVector<Member<AudioBuffer>>& output_buffers);
  ~ScriptProcessorHandler() override;

  // AudioHandler
  void Process(uint32_t frames_to_process) override;
  void Initialize() override;

  uint32_t BufferSize() const { return buffer_size_; }

  void SetChannelCount(uint32_t, ExceptionState&) override;
  void SetChannelCountMode(V8ChannelCountMode::Enum, ExceptionState&) override;

  uint32_t NumberOfOutputChannels() const override {
    return number_of_output_channels_;
  }

  base::Lock& GetBufferLock() LOCK_RETURNED(buffer_lock_) {
    return buffer_lock_;
  }

 private:
  ScriptProcessorHandler(AudioNode&,
                         float sample_rate,
                         uint32_t buffer_size,
                         uint32_t number_of_input_channels,
                         uint32_t number_of_output_channels,
                         const HeapVector<Member<AudioBuffer>>& input_buffers,
                         const HeapVector<Member<AudioBuffer>>& output_buffers);

  double TailTime() const override;
  double LatencyTime() const override;
  bool RequiresTailProcessing() const final;

  void FireProcessEvent(uint32_t);
  void FireProcessEventForOfflineAudioContext(uint32_t, base::WaitableEvent*);

  // Double buffering
  uint32_t DoubleBufferIndex() const { return double_buffer_index_; }
  void SwapBuffers() { double_buffer_index_ = 1 - double_buffer_index_; }
  uint32_t double_buffer_index_ = 0;

  mutable base::Lock buffer_lock_;
  WTF::Vector<std::unique_ptr<SharedAudioBuffer>> shared_input_buffers_
      GUARDED_BY(buffer_lock_);
  WTF::Vector<std::unique_ptr<SharedAudioBuffer>> shared_output_buffers_
      GUARDED_BY(buffer_lock_);

  uint32_t buffer_size_;
  uint32_t buffer_read_write_index_ = 0;
  uint32_t number_of_input_channels_;
  uint32_t number_of_output_channels_;

  scoped_refptr<AudioBus> internal_input_bus_;

  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

  base::WeakPtrFactory<ScriptProcessorHandler> weak_ptr_factory_{this};

  FRIEND_TEST_ALL_PREFIXES(ScriptProcessorNodeTest, BufferLifetime);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_SCRIPT_PROCESSOR_HANDLER_H_
