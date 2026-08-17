// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_WEBRTC_MEDIA_STREAM_TRACK_ADAPTER_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_WEBRTC_MEDIA_STREAM_TRACK_ADAPTER_MAP_H_

#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/peerconnection/webrtc_media_stream_track_adapter.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_component.h"
#include "third_party/blink/renderer/platform/peerconnection/two_keys_adapter_map.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"
#include "third_party/webrtc/api/media_stream_interface.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace blink {
class PeerConnectionDependencyFactory;

// A map and owner of |WebRtcMediaStreamTrackAdapter|s. It takes care of
// creating, initializing and disposing track adapters independently of media
// streams. Adapters are accessed via |AdapterRef|s, when all references to an
// adapter are destroyed it is disposed and removed from the map.
// Objects of this class must be constructed on the main thread, after which
// they may be accessed from any thread. The two exceptions to that are
// `GetOrCreateLocalTrackAdapter()` that must be called from the main thread and
// `GetOrCreateRemoteTrackAdapter()` which must not be called from the main
// thread.
class MODULES_EXPORT WebRtcMediaStreamTrackAdapterMap
    : public WTF::ThreadSafeRefCounted<WebRtcMediaStreamTrackAdapterMap> {
 public:
  // Acts as an accessor to adapter members without leaking a reference to the
  // adapter. When the last |AdapterRef| is destroyed, the corresponding adapter
  // is |Dispose|d and removed from the map.
  class MODULES_EXPORT AdapterRef {
   public:
    // Must be invoked on the main thread. If this was the last reference to the
    // adapter it will be disposed and removed from the map.
    ~AdapterRef();

    std::unique_ptr<AdapterRef> Copy() const;
    bool is_initialized() const { return adapter_->is_initialized(); }
    void InitializeOnMainThread();
    MediaStreamComponent* track() const { return adapter_->track(); }
    webrtc::scoped_refptr<webrtc::MediaStreamTrackInterface> webrtc_track()
        const {
      return adapter_->webrtc_track();
    }

    // Warning: Holding an external reference to the adapter will prevent
    // |~AdapterRef| from disposing the adapter.
    blink::WebRtcMediaStreamTrackAdapter* GetAdapterForTesting() const {
      return adapter_.get();
    }

   private:
    friend class WebRtcMediaStreamTrackAdapterMap;

    enum class Type { kLocal, kRemote };

    // Assumes map's |lock_| is held.
    AdapterRef(scoped_refptr<WebRtcMediaStreamTrackAdapterMap> map,
               Type type,
               scoped_refptr<blink::WebRtcMediaStreamTrackAdapter> adapter);

    scoped_refptr<WebRtcMediaStreamTrackAdapterMap> map_;
    Type type_;
    // A reference to the entry's adapter, ensures that |HasOneRef()| is false
    // as long as the |AdapterRef| is kept alive (the map entry has one
    // reference to it too).
    scoped_refptr<blink::WebRtcMediaStreamTrackAdapter> adapter_;
  };

  // Must be invoked on the main thread.
  WebRtcMediaStreamTrackAdapterMap(
      blink::PeerConnectionDependencyFactory* const factory,
      scoped_refptr<base::SingleThreadTaskRunner> main_thread);

  // Gets a new reference to the local track adapter, or null if no such adapter
  // was found. When all references are destroyed the adapter is disposed and
  // removed from the map. This method can be called from any thread, but
  // references must be destroyed on the main thread.
  // The adapter is a associated with a blink and webrtc track, lookup works by
  // either track.
  std::unique_ptr<AdapterRef> GetLocalTrackAdapter(
      MediaStreamComponent* component);
  std::unique_ptr<AdapterRef> GetLocalTrackAdapter(
      webrtc::MediaStreamTrackInterface* webrtc_track);
  // Invoke on the main thread. Gets a new reference to the local track adapter
  // for the web track. If no adapter exists for the track one is created and
  // initialized. When all references are destroyed the adapter is disposed and
  // removed from the map. References must be destroyed on the main thread.
  std::unique_ptr<AdapterRef> GetOrCreateLocalTrackAdapter(
      MediaStreamComponent* component);
  size_t GetLocalTrackCount() const;

  // Gets a new reference to the remote track adapter. When all references are
  // destroyed the adapter is disposed and removed from the map. This method can
  // be called from any thread, but references must be destroyed on the main
  // thread. The adapter is a associated with a blink and webrtc track, lookup
  // works by either track.
  // First variety: If an adapter exists it will already be initialized, if one
  // does not exist null is returned.
  std::unique_ptr<AdapterRef> GetRemoteTrackAdapter(
      MediaStreamComponent* component);
  // Second variety: If an adapter exists it may or may not be initialized, see
  // |AdapterRef::is_initialized|. If an adapter does not exist null is
  // returned.
  std::unique_ptr<AdapterRef> GetRemoteTrackAdapter(
      webrtc::MediaStreamTrackInterface* webrtc_track);
  // Invoke on the webrtc signaling thread. Gets a new reference to the remote
  // track adapter for the webrtc track. If no adapter exists for the track one
  // is created and initialization completes on the main thread in a post. When
  // all references are destroyed the adapter is disposed and removed from the
  // map. References must be destroyed on the main thread.
  std::unique_ptr<AdapterRef> GetOrCreateRemoteTrackAdapter(
      scoped_refptr<webrtc::MediaStreamTrackInterface> webrtc_track);
  size_t GetRemoteTrackCount() const;

 private:
  friend class WTF::ThreadSafeRefCounted<WebRtcMediaStreamTrackAdapterMap>;

  // "(MediaStreamComponent, webrtc::MediaStreamTrackInterface) ->
  // WebRtcMediaStreamTrackAdapter" maps. The primary key is based on the object
  // used to create the adapter. Local tracks are created from
  // |MediaStreamComponent|s, remote tracks are created from
  // |webrtc::MediaStreamTrackInterface|s.
  // The adapter keeps the |webrtc::MediaStreamTrackInterface| alive with ref
  // counting making it safe to use a raw pointer for key.
  using LocalTrackAdapterMap = blink::TwoKeysAdapterMap<
      int,  // MediaStreamComponent::UniqueId()
      webrtc::MediaStreamTrackInterface*,
      scoped_refptr<blink::WebRtcMediaStreamTrackAdapter>>;
  using RemoteTrackAdapterMap = blink::TwoKeysAdapterMap<
      webrtc::MediaStreamTrackInterface*,
      int,  // MediaStreamComponent::UniqueId()
      scoped_refptr<blink::WebRtcMediaStreamTrackAdapter>>;

  // Invoke on the main thread.
  virtual ~WebRtcMediaStreamTrackAdapterMap();

  // The adapter map is indirectly owned by `RTCPeerConnection`, which is
  // outlived by the `PeerConnectionDependencyFactory`, so `factory_` should
  // never be null (with the possible exception of the dtor).
  const CrossThreadWeakPersistent<PeerConnectionDependencyFactory> factory_;
  scoped_refptr<base::SingleThreadTaskRunner> main_thread_;

  mutable base::Lock lock_;
  LocalTrackAdapterMap local_track_adapters_;
  RemoteTrackAdapterMap remote_track_adapters_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_WEBRTC_MEDIA_STREAM_TRACK_ADAPTER_MAP_H_
