/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_PC_PEER_CONNECTION_H_
#define SDK_ANDROID_SRC_JNI_PC_PEER_CONNECTION_H_

#include <map>
#include <memory>
#include <vector>

#include "api/peer_connection_interface.h"
#include "pc/media_stream_observer.h"
#include "sdk/android/src/jni/jni_helpers.h"
#include "sdk/android/src/jni/pc/media_constraints.h"
#include "sdk/android/src/jni/pc/media_stream.h"
#include "sdk/android/src/jni/pc/rtp_receiver.h"
#include "sdk/android/src/jni/pc/rtp_transceiver.h"

namespace webrtc {
namespace jni {

void JavaToNativeRTCConfiguration(
    JNIEnv* jni,
    const JavaRef<jobject>& j_rtc_config,
    PeerConnectionInterface::RTCConfiguration* rtc_config);

KeyType GetRtcConfigKeyType(JNIEnv* env, const JavaRef<jobject>& j_rtc_config);

ScopedJavaLocalRef<jobject> NativeToJavaAdapterType(JNIEnv* env,
                                                    int adapterType);

// Adapter between the C++ PeerConnectionObserver interface and the Java
// PeerConnection.Observer interface.  Wraps an instance of the Java interface
// and dispatches C++ callbacks to Java.
class PeerConnectionObserverJni : public PeerConnectionObserver {
 public:
  PeerConnectionObserverJni(JNIEnv* jni, const JavaRef<jobject>& j_observer);
  ~PeerConnectionObserverJni() override;

  // Implementation of PeerConnectionObserver interface, which propagates
  // the callbacks to the Java observer.
  void OnIceCandidate(const IceCandidateInterface* candidate) override;
  void OnIceCandidateError(const std::string& address,
                           int port,
                           const std::string& url,
                           int error_code,
                           const std::string& error_text) override;

  void OnIceCandidatesRemoved(
      const std::vector<Candidate>& candidates) override;
  void OnSignalingChange(
      PeerConnectionInterface::SignalingState new_state) override;
  void OnIceConnectionChange(
      PeerConnectionInterface::IceConnectionState new_state) override;
  void OnStandardizedIceConnectionChange(
      PeerConnectionInterface::IceConnectionState new_state) override;
  void OnConnectionChange(
      PeerConnectionInterface::PeerConnectionState new_state) override;
  void OnIceConnectionReceivingChange(bool receiving) override;
  void OnIceGatheringChange(
      PeerConnectionInterface::IceGatheringState new_state) override;
  void OnIceSelectedCandidatePairChanged(
      const CandidatePairChangeEvent& event) override;
  void OnAddStream(scoped_refptr<MediaStreamInterface> stream) override;
  void OnRemoveStream(scoped_refptr<MediaStreamInterface> stream) override;
  void OnDataChannel(scoped_refptr<DataChannelInterface> channel) override;
  void OnRenegotiationNeeded() override;
  void OnAddTrack(
      scoped_refptr<RtpReceiverInterface> receiver,
      const std::vector<scoped_refptr<MediaStreamInterface>>& streams) override;
  void OnTrack(scoped_refptr<RtpTransceiverInterface> transceiver) override;
  void OnRemoveTrack(scoped_refptr<RtpReceiverInterface> receiver) override;

 private:
  typedef std::map<MediaStreamInterface*, JavaMediaStream>
      NativeToJavaStreamsMap;
  typedef std::map<MediaStreamTrackInterface*, RtpReceiverInterface*>
      NativeMediaStreamTrackToNativeRtpReceiver;

  // If the NativeToJavaStreamsMap contains the stream, return it.
  // Otherwise, create a new Java MediaStream. Returns a global jobject.
  JavaMediaStream& GetOrCreateJavaStream(
      JNIEnv* env,
      const scoped_refptr<MediaStreamInterface>& stream);

  // Converts array of streams, creating or re-using Java streams as necessary.
  ScopedJavaLocalRef<jobjectArray> NativeToJavaMediaStreamArray(
      JNIEnv* jni,
      const std::vector<scoped_refptr<MediaStreamInterface>>& streams);

  const ScopedJavaGlobalRef<jobject> j_observer_global_;

  // C++ -> Java remote streams.
  NativeToJavaStreamsMap remote_streams_;
  std::vector<JavaRtpReceiverGlobalOwner> rtp_receivers_;
  // Holds a reference to the Java transceivers given to the AddTrack
  // callback, so that the shared ownership by the Java object will be
  // properly disposed.
  std::vector<JavaRtpTransceiverGlobalOwner> rtp_transceivers_;
};

// PeerConnection doesn't take ownership of the observer. In Java API, we don't
// want the client to have to manually dispose the observer. To solve this, this
// wrapper class is used for object ownership.
//
// Also stores reference to the deprecated PeerConnection constraints for now.
class OwnedPeerConnection {
 public:
  OwnedPeerConnection(scoped_refptr<PeerConnectionInterface> peer_connection,
                      std::unique_ptr<PeerConnectionObserver> observer);
  // Deprecated. PC constraints are deprecated.
  OwnedPeerConnection(scoped_refptr<PeerConnectionInterface> peer_connection,
                      std::unique_ptr<PeerConnectionObserver> observer,
                      std::unique_ptr<MediaConstraints> constraints);
  ~OwnedPeerConnection();

  PeerConnectionInterface* pc() const { return peer_connection_.get(); }
  const MediaConstraints* constraints() const { return constraints_.get(); }

 private:
  scoped_refptr<PeerConnectionInterface> peer_connection_;
  std::unique_ptr<PeerConnectionObserver> observer_;
  std::unique_ptr<MediaConstraints> constraints_;
};

}  // namespace jni
}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_PC_PEER_CONNECTION_H_
