// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_MEDIA_AUDIO_AUDIO_OUTPUT_IPC_FACTORY_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_MEDIA_AUDIO_AUDIO_OUTPUT_IPC_FACTORY_H_

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "base/unguessable_token.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/platform/web_common.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace media {
class AudioOutputIPC;
}

namespace blink {
class BrowserInterfaceBrokerProxy;

// This is a factory for AudioOutputIPC objects. It is threadsafe. This class
// is designed to be leaked at shutdown, as it posts tasks to itself using
// base::Unretained and also hands out references to itself in the
// AudioOutputIPCs it creates, but in the case where the owner is sure that
// there are no outstanding references (such as in a unit test), the class can
// be destructed.
// TODO(maxmorin): Registering the factories for each frame will become
// unnecessary when https://crbug.com/668275 is fixed. When that is done, this
// class can be greatly simplified.
class BLINK_MODULES_EXPORT AudioOutputIPCFactory {
 public:
  explicit AudioOutputIPCFactory(
      scoped_refptr<base::SingleThreadTaskRunner> io_task_runner);
  AudioOutputIPCFactory(const AudioOutputIPCFactory&) = delete;
  AudioOutputIPCFactory& operator=(const AudioOutputIPCFactory&) = delete;
  ~AudioOutputIPCFactory();

  static AudioOutputIPCFactory& GetInstance();

  const scoped_refptr<base::SingleThreadTaskRunner>& io_task_runner() const;

  // Enables |this| to create MojoAudioOutputIPCs for the specified frame.
  // Does nothing if not using mojo factories.
  void RegisterRemoteFactory(
      const blink::LocalFrameToken& frame_token,
      const blink::BrowserInterfaceBrokerProxy& interface_broker);

  // Every call to the above method must be matched by a call to this one when
  // the frame is destroyed. Does nothing if not using mojo factories.
  void MaybeDeregisterRemoteFactory(const blink::LocalFrameToken& frame_token);

  // The returned object may only be used on |io_task_runner()|.
  std::unique_ptr<media::AudioOutputIPC> CreateAudioOutputIPC(
      const blink::LocalFrameToken& frame_token) const;

 private:
  // TODO(https://crbug.com/787252): When this header gets moved out of the
  // Blink public API layer, move this Pimpl class back to its outer class.
  class Impl;
  std::unique_ptr<Impl> impl_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_MEDIA_AUDIO_AUDIO_OUTPUT_IPC_FACTORY_H_
