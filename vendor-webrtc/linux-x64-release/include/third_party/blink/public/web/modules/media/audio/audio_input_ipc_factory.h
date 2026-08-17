// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_MEDIA_AUDIO_AUDIO_INPUT_IPC_FACTORY_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_MEDIA_AUDIO_AUDIO_INPUT_IPC_FACTORY_H_

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/platform/web_common.h"

namespace base {
class SequencedTaskRunner;
}  // namespace base

namespace media {
class AudioInputIPC;
struct AudioSourceParameters;
}  // namespace media

namespace blink {

// This is a thread-safe factory for AudioInputIPC objects.
class BLINK_MODULES_EXPORT AudioInputIPCFactory {
 public:
  // This method can be called from any thread but requires a `frame_token`
  // and a `main_task_runner` so that the associated WebLocalFrame can be
  // found to asssocaite the audio channel with the frame.
  static std::unique_ptr<media::AudioInputIPC> CreateAudioInputIPC(
      const blink::LocalFrameToken& frame_token,
      scoped_refptr<base::SequencedTaskRunner> main_task_runner,
      const media::AudioSourceParameters& source_params);

  AudioInputIPCFactory() = delete;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_MEDIA_AUDIO_AUDIO_INPUT_IPC_FACTORY_H_
