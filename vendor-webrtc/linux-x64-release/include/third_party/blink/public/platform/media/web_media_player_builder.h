// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MEDIA_WEB_MEDIA_PLAYER_BUILDER_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MEDIA_WEB_MEDIA_PLAYER_BUILDER_H_

#include <stdint.h>

#include <memory>

#include "base/functional/callback_forward.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "cc/layers/surface_layer.h"
#include "media/base/media_player_logging_id.h"
#include "media/base/routing_token_callback.h"
#include "media/mojo/mojom/media_metrics_provider.mojom-forward.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_media_player.h"

namespace base {
class SingleThreadTaskRunner;
class SequencedTaskRunner;
class TaskRunner;
}  // namespace base

namespace media {
class Demuxer;
class MediaLog;
class MediaObserver;
class RendererFactorySelector;
class SwitchableAudioRendererSink;
}  // namespace media

namespace viz {
class RasterContextProvider;
}

namespace blink {

class ResourceFetchContext;
class ThreadSafeBrowserInterfaceBrokerProxy;
class UrlIndex;
class WebContentDecryptionModule;
class WebLocalFrame;
class WebMediaPlayerClient;
class WebMediaPlayerEncryptedMediaClient;
class WebMediaPlayerDelegate;
class WebSurfaceLayerBridge;
class WebSurfaceLayerBridgeObserver;
class WebVideoFrameSubmitter;

using CreateSurfaceLayerBridgeCB =
    base::OnceCallback<std::unique_ptr<WebSurfaceLayerBridge>(
        WebSurfaceLayerBridgeObserver*,
        cc::UpdateSubmissionStateCB)>;

class BLINK_PLATFORM_EXPORT WebMediaPlayerBuilder {
 public:
  // Returns true if load will deferred. False if it will run immediately.
  using DeferLoadCB = base::RepeatingCallback<bool(base::OnceClosure)>;

  WebMediaPlayerBuilder(
      WebLocalFrame& frame,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);
  WebMediaPlayerBuilder(const WebMediaPlayerBuilder&) = delete;
  WebMediaPlayerBuilder& operator=(const WebMediaPlayerBuilder&) = delete;
  ~WebMediaPlayerBuilder();

  std::unique_ptr<WebMediaPlayer> Build(
      WebLocalFrame* frame,
      WebMediaPlayerClient* client,
      WebMediaPlayerEncryptedMediaClient* encrypted_client,
      WebMediaPlayerDelegate* delegate,
      std::unique_ptr<media::RendererFactorySelector> renderer_factory_selector,
      std::unique_ptr<WebVideoFrameSubmitter> video_frame_submitter,
      std::unique_ptr<media::MediaLog> media_log,
      media::MediaPlayerLoggingID player_id,
      DeferLoadCB defer_load_cb,
      scoped_refptr<media::SwitchableAudioRendererSink> audio_renderer_sink,
      scoped_refptr<base::SequencedTaskRunner> media_task_runner,
      scoped_refptr<base::TaskRunner> worker_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner> compositor_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner>
          video_frame_compositor_task_runner,
      WebContentDecryptionModule* initial_cdm,
      media::RequestRoutingTokenCallback request_routing_token_cb,
      base::WeakPtr<media::MediaObserver> media_observer,
      bool enable_instant_source_buffer_gc,
      bool embedded_media_experience_enabled,
      mojo::PendingRemote<media::mojom::MediaMetricsProvider> metrics_provider,
      CreateSurfaceLayerBridgeCB create_bridge_callback,
      scoped_refptr<viz::RasterContextProvider> raster_context_provider,
      bool use_surface_layer_for_video,
      bool is_background_suspend_enabled,
      bool is_background_video_playback_enabled,
      bool is_background_video_track_optimization_supported,
      std::unique_ptr<media::Demuxer> demuxer_override,
      scoped_refptr<ThreadSafeBrowserInterfaceBrokerProxy> remote_interfaces);

 private:
  // Media resource cache.
  std::unique_ptr<ResourceFetchContext> fetch_context_;
  std::unique_ptr<UrlIndex> url_index_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MEDIA_WEB_MEDIA_PLAYER_BUILDER_H_
