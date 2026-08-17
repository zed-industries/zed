// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASESSION_MEDIA_SESSION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASESSION_MEDIA_SESSION_H_

#include "base/memory/raw_ptr.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/public/mojom/mediasession/media_session.mojom-blink.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_media_session_action.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/gc_plugin.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace base {
class TickClock;
}  // namespace base

namespace blink {

class ExceptionState;
class MediaMetadata;
class MediaPositionState;
class Navigator;
class V8MediaSessionActionHandler;
class V8MediaSessionPlaybackState;

class MODULES_EXPORT MediaSession final
    : public ScriptWrappable,
      public Supplement<Navigator>,
      public blink::mojom::blink::MediaSessionClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];
  static MediaSession* mediaSession(Navigator&);
  explicit MediaSession(Navigator&);

  void setPlaybackState(const V8MediaSessionPlaybackState&);
  V8MediaSessionPlaybackState playbackState();

  void setMetadata(MediaMetadata*);
  MediaMetadata* metadata() const;

  void setActionHandler(const V8MediaSessionAction& action,
                        V8MediaSessionActionHandler*,
                        ExceptionState&);

  void setPositionState(MediaPositionState*, ExceptionState&);

  void setMicrophoneActive(bool active);

  void setCameraActive(bool active);

  // Called by the MediaMetadata owned by |this| when it has updates. Also used
  // internally when a new MediaMetadata object is set.
  void OnMetadataChanged();

  void Trace(Visitor*) const override;

 private:
  friend class V8MediaSession;
  friend class MediaSessionTest;

  enum class ActionChangeType {
    kActionEnabled,
    kActionDisabled,
  };

  void NotifyActionChange(V8MediaSessionAction::Enum action, ActionChangeType);

  base::TimeDelta GetPositionNow() const;

  void RecalculatePositionState(bool was_set);

  // blink::mojom::blink::MediaSessionClient implementation.
  void DidReceiveAction(media_session::mojom::blink::MediaSessionAction,
                        mojom::blink::MediaSessionActionDetailsPtr) override;

  // Returns null if the associated window is detached.
  mojom::blink::MediaSessionService* GetService();

  raw_ptr<const base::TickClock, DanglingUntriaged> clock_ = nullptr;

  mojom::blink::MediaSessionPlaybackState playback_state_;
  media_session::mojom::blink::MediaPositionPtr position_state_;
  double declared_playback_rate_ = 0.0;
  Member<MediaMetadata> metadata_;
  HeapHashMap<V8MediaSessionAction::Enum, Member<V8MediaSessionActionHandler>>
      action_handlers_;
  HeapMojoRemote<mojom::blink::MediaSessionService> service_;
  HeapMojoReceiver<blink::mojom::blink::MediaSessionClient, MediaSession>
      client_receiver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASESSION_MEDIA_SESSION_H_
