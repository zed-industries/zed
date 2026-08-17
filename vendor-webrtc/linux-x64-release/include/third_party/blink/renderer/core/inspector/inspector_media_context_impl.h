// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_MEDIA_CONTEXT_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_MEDIA_CONTEXT_IMPL_H_

#include "build/build_config.h"
#include "third_party/blink/public/web/web_media_inspector.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"

namespace blink {

#if BUILDFLAG(IS_ANDROID)
// Players are cached per tab.
constexpr int kMaxCachedPlayerEvents = 128;
#else
constexpr int kMaxCachedPlayerEvents = 512;
#endif

class ExecutionContext;

struct MediaPlayer final : public GarbageCollected<MediaPlayer> {
  void Trace(Visitor*) const {}

  WebString player_id;
  Vector<InspectorPlayerError> errors;
  Vector<InspectorPlayerEvent> events;
  Vector<InspectorPlayerMessage> messages;
  HashMap<String, InspectorPlayerProperty> properties;
};

class CORE_EXPORT MediaInspectorContextImpl final
    : public GarbageCollected<MediaInspectorContextImpl>,
      public Supplement<ExecutionContext>,
      public MediaInspectorContext {
 public:
  static const char kSupplementName[];

  static MediaInspectorContextImpl* From(ExecutionContext&);

  explicit MediaInspectorContextImpl(ExecutionContext&);

  // MediaInspectorContext methods.
  WebString CreatePlayer() override;
  void DestroyPlayer(const WebString& playerId) override;

  void NotifyPlayerErrors(WebString playerId,
                          const InspectorPlayerErrors&) override;
  void NotifyPlayerEvents(WebString playerId,
                          const InspectorPlayerEvents&) override;
  void NotifyPlayerMessages(WebString playerId,
                            const InspectorPlayerMessages&) override;
  void SetPlayerProperties(WebString playerId,
                           const InspectorPlayerProperties&) override;

  // GarbageCollected methods.
  void Trace(Visitor*) const override;

  Vector<WebString> AllPlayerIdsAndMarkSent();
  const MediaPlayer& MediaPlayerFromId(const WebString&);

  HeapHashMap<String, Member<MediaPlayer>>* GetPlayersForTesting();
  int GetTotalEventCountForTesting() { return total_event_count_; }

 private:
  // When a player is added, its ID is stored in |unsent_players_| if no
  // connections are open. When an unsent player is destroyed, its ID is moved
  // to |dead_players_| and is first to be deleted if there is memory pressure.
  // If it has already been sent when it is destroyed, it gets moved to
  // |expendable_players_|, which is the second group of players to be deleted
  // on memory pressure.

  // If there are no dead or expendable players when it's time to start removing
  // players, then a player from |unsent_players_| will be removed. As a last
  // resort, remaining unended, already-sent players will be removed from
  // |players_| until the total event size is within the limit.

  // All events will be sent to any open clients regardless of players existing
  // because the clients can handle dead players and may have their own cache.
  void CullPlayers(const WebString& prefer_keep);
  void TrimPlayer(const WebString& playerId);
  void RemovePlayer(const WebString& playerId);

  HeapHashMap<String, Member<MediaPlayer>> players_;
  Vector<String> unsent_players_;
  Vector<String> dead_players_;
  Vector<String> expendable_players_;

  int total_event_count_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_MEDIA_CONTEXT_IMPL_H_
