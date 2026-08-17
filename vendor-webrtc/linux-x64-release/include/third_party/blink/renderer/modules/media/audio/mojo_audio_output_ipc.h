// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_AUDIO_MOJO_AUDIO_OUTPUT_IPC_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_AUDIO_MOJO_AUDIO_OUTPUT_IPC_H_

#include <string>

#include "base/functional/callback_helpers.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "media/audio/audio_output_ipc.h"
#include "media/mojo/mojom/audio_data_pipe.mojom-blink.h"
#include "media/mojo/mojom/audio_output_stream.mojom-blink.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/public/mojom/media/renderer_audio_output_stream_factory.mojom-blink.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

// MojoAudioOutputIPC is a renderer-side class for handling creation,
// initialization and control of an output stream. May only be used on a single
// thread.
class MODULES_EXPORT MojoAudioOutputIPC
    : public media::AudioOutputIPC,
      public media::mojom::blink::AudioOutputStreamProviderClient {
 public:
  using FactoryAccessorCB = base::RepeatingCallback<
      blink::mojom::blink::RendererAudioOutputStreamFactory*()>;

  // |factory_accessor| is required to provide a
  // RendererAudioOutputStreamFactory* if IPC is possible.
  MojoAudioOutputIPC(
      FactoryAccessorCB factory_accessor,
      scoped_refptr<base::SingleThreadTaskRunner> io_task_runner);

  MojoAudioOutputIPC(const MojoAudioOutputIPC&) = delete;
  MojoAudioOutputIPC& operator=(const MojoAudioOutputIPC&) = delete;

  ~MojoAudioOutputIPC() override;

  // AudioOutputIPC implementation.
  void RequestDeviceAuthorization(media::AudioOutputIPCDelegate* delegate,
                                  const base::UnguessableToken& session_id,
                                  const std::string& device_id) override;
  void CreateStream(media::AudioOutputIPCDelegate* delegate,
                    const media::AudioParameters& params) override;
  void PlayStream() override;
  void PauseStream() override;
  void FlushStream() override;
  void CloseStream() override;
  void SetVolume(double volume) override;

  // media::mojom::AudioOutputStreamProviderClient implementation.
  void Created(
      mojo::PendingRemote<media::mojom::blink::AudioOutputStream> stream,
      media::mojom::blink::ReadWriteAudioDataPipePtr data_pipe) override;

 private:
  static constexpr double kDefaultVolume = 1.0;

  using AuthorizationCB = blink::mojom::blink::
      RendererAudioOutputStreamFactory::RequestDeviceAuthorizationCallback;

  bool AuthorizationRequested() const;
  bool StreamCreationRequested() const;

  void ProviderClientBindingDisconnected(uint32_t disconnect_reason,
                                         const std::string& description);

  mojo::PendingReceiver<media::mojom::blink::AudioOutputStreamProvider>
  MakeProviderReceiver();

  // Tries to acquire a RendererAudioOutputStreamFactory and requests device
  // authorization. On failure to acquire a factory, |callback| is destructed
  // asynchronously.
  void DoRequestDeviceAuthorization(const base::UnguessableToken& session_id,
                                    const std::string& device_id,
                                    AuthorizationCB callback);

  void ReceivedDeviceAuthorization(
      base::TimeTicks auth_start_time,
      media::mojom::blink::OutputDeviceStatus status,
      const media::AudioParameters& params,
      const String& device_id) const;

  const FactoryAccessorCB factory_accessor_;

  // This is the state that |delegate_| expects the stream to be in. It is
  // maintained for when the stream is created.
  enum { kPaused, kPlaying } expected_state_ = kPaused;
  std::optional<double> volume_;

  mojo::Receiver<media::mojom::blink::AudioOutputStreamProviderClient>
      receiver_{this};
  mojo::Remote<media::mojom::blink::AudioOutputStreamProvider> stream_provider_;
  mojo::Remote<media::mojom::blink::AudioOutputStream> stream_;
  raw_ptr<media::AudioOutputIPCDelegate> delegate_ = nullptr;
  scoped_refptr<base::SingleThreadTaskRunner> io_task_runner_;

  // To make sure we don't send an "authorization completed" callback for a
  // stream after it's closed, we use this weak factory.
  base::WeakPtrFactory<MojoAudioOutputIPC> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_AUDIO_MOJO_AUDIO_OUTPUT_IPC_H_
