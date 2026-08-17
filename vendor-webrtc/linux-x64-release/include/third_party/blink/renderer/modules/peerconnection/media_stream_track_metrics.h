// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_MEDIA_STREAM_TRACK_METRICS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_MEDIA_STREAM_TRACK_METRICS_H_

#include <stdint.h>

#include <memory>

#include "base/threading/thread_checker.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/public/mojom/mediastream/media_stream.mojom-blink.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/webrtc/api/peer_connection_interface.h"

namespace blink {

class MediaStreamTrackMetricsObserver;
class RTCPeerConnectionHandler;

// Responsible for observing the connected lifetimes of tracks going
// over a PeerConnection, and sending messages to the browser process
// about lifetime events.
//
// There should be exactly one of these objects owned by each
// RTCPeerConnectionHandler, and its lifetime should match the
// lifetime of its owner.
class MODULES_EXPORT MediaStreamTrackMetrics {
 public:
  explicit MediaStreamTrackMetrics();

  MediaStreamTrackMetrics(const MediaStreamTrackMetrics&) = delete;
  MediaStreamTrackMetrics& operator=(const MediaStreamTrackMetrics&) = delete;

  ~MediaStreamTrackMetrics();

  enum class Direction { kSend, kReceive };
  enum class Kind { kAudio, kVideo };
  enum class LifetimeEvent { kConnected, kDisconnected };

  // Starts tracking the lifetime of the track until |RemoveTrack| is called
  // or this object's lifetime ends.
  void AddTrack(Direction direction, Kind kind, const std::string& track_id);

  // Stops tracking the lifetime of the track.
  void RemoveTrack(Direction direction, Kind kind, const std::string& track_id);

  // Called to indicate changes in the ICE connection state for the
  // PeerConnection this object is associated with. Used to generate
  // the connected/disconnected lifetime events for these tracks.
  void IceConnectionChange(
      webrtc::PeerConnectionInterface::IceConnectionState new_state);

  // Send a lifetime message to the browser process. Virtual so that
  // it can be overridden in unit tests.
  //
  // |track_id| is the ID of the track that just got connected or
  // disconnected.
  //
  // |is_audio| is true for an audio track, false for a video track.
  //
  // |start_lifetime| is true to indicate that it just got connected,
  // false to indicate it is no longer connected.
  //
  // |is_remote| is true for remote streams (received over a
  // PeerConnection), false for local streams (sent over a
  // PeerConnection).
  virtual void SendLifetimeMessage(const std::string& track_id,
                                   Kind kind,
                                   LifetimeEvent lifetime_event,
                                   Direction direction);

 protected:
  // Calls SendLifetimeMessage for |observer| depending on |ice_state_|.
  void SendLifeTimeMessageDependingOnIceState(
      MediaStreamTrackMetricsObserver* observer);

  // Implements MakeUniqueId. |pc_id| is a cast of this object's
  // |this| pointer to a 64-bit integer, which is usable as a unique
  // ID for the PeerConnection this object is attached to (since there
  // is a one-to-one relationship).
  uint64_t MakeUniqueIdImpl(uint64_t pc_id,
                            const std::string& track,
                            Direction direction);

 private:
  // Make a unique ID for the given track, that is valid while the
  // track object and the PeerConnection it is attached to both exist.
  uint64_t MakeUniqueId(const std::string& track_id, Direction direction);

  mojo::Remote<blink::mojom::blink::MediaStreamTrackMetricsHost>&
  GetMediaStreamTrackMetricsHost();

  mojo::Remote<blink::mojom::blink::MediaStreamTrackMetricsHost>
      track_metrics_host_;

  Vector<std::unique_ptr<MediaStreamTrackMetricsObserver>> observers_;

  webrtc::PeerConnectionInterface::IceConnectionState ice_state_;

  THREAD_CHECKER(thread_checker_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_MEDIA_STREAM_TRACK_METRICS_H_
