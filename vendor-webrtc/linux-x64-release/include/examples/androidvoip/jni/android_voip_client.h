/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef EXAMPLES_ANDROIDVOIP_JNI_ANDROID_VOIP_CLIENT_H_
#define EXAMPLES_ANDROIDVOIP_JNI_ANDROID_VOIP_CLIENT_H_

#include <jni.h>

#include <memory>
#include <string>
#include <vector>

#include "api/audio_codecs/audio_format.h"
#include "api/call/transport.h"
#include "api/voip/voip_base.h"
#include "api/voip/voip_engine.h"
#include "rtc_base/async_packet_socket.h"
#include "rtc_base/async_udp_socket.h"
#include "rtc_base/network/received_packet.h"
#include "rtc_base/socket_address.h"
#include "rtc_base/thread.h"
#include "sdk/android/native_api/jni/scoped_java_ref.h"

namespace webrtc_examples {

// AndroidVoipClient facilitates the use of the VoIP API defined in
// api/voip/voip_engine.h. One instance of AndroidVoipClient should
// suffice for most VoIP applications. AndroidVoipClient implements
// webrtc::Transport to send RTP/RTCP packets to the remote endpoint.
// It also creates methods (slots) for sockets to connect to in
// order to receive RTP/RTCP packets. AndroidVoipClient does all
// operations with webrtc::Thread (voip_thread_), this is to comply
// with consistent thread usage requirement with ProcessThread used
// within VoipEngine, as well as providing asynchronicity to the
// caller. AndroidVoipClient is meant to be used by Java through JNI.
class AndroidVoipClient : public webrtc::Transport {
 public:
  // Returns a pointer to an AndroidVoipClient object. Clients should
  // use this factory method to create AndroidVoipClient objects. The
  // method will return a nullptr in case of initialization errors.
  // It is the client's responsibility to delete the pointer when
  // they are done with it (this class provides a Delete() method).
  static AndroidVoipClient* Create(
      JNIEnv* env,
      const jni_zero::JavaParamRef<jobject>& application_context,
      const jni_zero::JavaParamRef<jobject>& j_voip_client);

  ~AndroidVoipClient() override;

  // Provides client with a Java List of Strings containing names of
  // the built-in supported codecs through callback.
  void GetSupportedCodecs(JNIEnv* env);

  // Provides client with a Java String of the default local IPv4 address
  // through callback. If IPv4 address is not found, provide the default
  // local IPv6 address. If IPv6 address is not found, provide an empty
  // string.
  void GetLocalIPAddress(JNIEnv* env);

  // Sets the encoder used by the VoIP API.
  void SetEncoder(JNIEnv* env,
                  const jni_zero::JavaParamRef<jstring>& j_encoder_string);

  // Sets the decoders used by the VoIP API.
  void SetDecoders(JNIEnv* env,
                   const jni_zero::JavaParamRef<jobject>& j_decoder_strings);

  // Sets two local/remote addresses, one for RTP packets, and another for
  // RTCP packets. The RTP address will have IP address j_ip_address_string
  // and port number j_port_number_int, the RTCP address will have IP address
  // j_ip_address_string and port number j_port_number_int+1.
  void SetLocalAddress(
      JNIEnv* env,
      const jni_zero::JavaParamRef<jstring>& j_ip_address_string,
      jint j_port_number_int);
  void SetRemoteAddress(
      JNIEnv* env,
      const jni_zero::JavaParamRef<jstring>& j_ip_address_string,
      jint j_port_number_int);

  // Starts a VoIP session, then calls a callback method with a boolean
  // value indicating if the session has started successfully. The VoIP
  // operations below can only be used after a session has already started.
  void StartSession(JNIEnv* env);

  // Stops the current session, then calls a callback method with a
  // boolean value indicating if the session has stopped successfully.
  void StopSession(JNIEnv* env);

  // Starts sending RTP/RTCP packets to the remote endpoint, then calls
  // a callback method with a boolean value indicating if sending
  // has started successfully.
  void StartSend(JNIEnv* env);

  // Stops sending RTP/RTCP packets to the remote endpoint, then calls
  // a callback method with a boolean value indicating if sending
  // has stopped successfully.
  void StopSend(JNIEnv* env);

  // Starts playing out the voice data received from the remote endpoint,
  // then calls a callback method with a boolean value indicating if
  // playout has started successfully.
  void StartPlayout(JNIEnv* env);

  // Stops playing out the voice data received from the remote endpoint,
  // then calls a callback method with a boolean value indicating if
  // playout has stopped successfully.
  void StopPlayout(JNIEnv* env);

  // Deletes this object. Used by client when they are done.
  void Delete(JNIEnv* env);

  // Implementation for Transport.
  bool SendRtp(webrtc::ArrayView<const uint8_t> packet,
               const webrtc::PacketOptions& options) override;
  bool SendRtcp(webrtc::ArrayView<const uint8_t> packet) override;

  void OnSignalReadRTPPacket(webrtc::AsyncPacketSocket* socket,
                             const webrtc::ReceivedIpPacket& packet);
  void OnSignalReadRTCPPacket(webrtc::AsyncPacketSocket* socket,
                              const webrtc::ReceivedIpPacket& packet);

 private:
  AndroidVoipClient(JNIEnv* env,
                    const jni_zero::JavaParamRef<jobject>& j_voip_client)
      : voip_thread_(webrtc::Thread::CreateWithSocketServer()),
        j_voip_client_(env, j_voip_client) {}

  void Init(JNIEnv* env,
            const jni_zero::JavaParamRef<jobject>& application_context);

  // Overloaded methods having native C++ variables as arguments.
  void SetEncoder(const std::string& encoder);
  void SetDecoders(const std::vector<std::string>& decoders);
  void SetLocalAddress(const std::string& ip_address, int port_number);
  void SetRemoteAddress(const std::string& ip_address, int port_number);

  // Methods to send and receive RTP/RTCP packets. Takes in a
  // copy of a packet as a vector to prolong the lifetime of
  // the packet as these methods will be called asynchronously.
  void SendRtpPacket(const std::vector<uint8_t>& packet_copy);
  void SendRtcpPacket(const std::vector<uint8_t>& packet_copy);
  void ReadRTPPacket(const std::vector<uint8_t>& packet_copy);
  void ReadRTCPPacket(const std::vector<uint8_t>& packet_copy);

  // Method to print out ChannelStatistics
  void LogChannelStatistics(JNIEnv* env);

  // Used to invoke operations and send/receive RTP/RTCP packets.
  std::unique_ptr<webrtc::Thread> voip_thread_;
  // Reference to the VoipClient java instance used to
  // invoke callbacks when operations are finished.
  jni_zero::ScopedJavaGlobalRef<jobject> j_voip_client_
      RTC_GUARDED_BY(voip_thread_);
  // A list of AudioCodecSpec supported by the built-in
  // encoder/decoder factories.
  std::vector<webrtc::AudioCodecSpec> supported_codecs_
      RTC_GUARDED_BY(voip_thread_);
  // A JNI context used by the voip_thread_.
  JNIEnv* env_ RTC_GUARDED_BY(voip_thread_);
  // The entry point to all VoIP APIs.
  std::unique_ptr<webrtc::VoipEngine> voip_engine_ RTC_GUARDED_BY(voip_thread_);
  // Used by the VoIP API to facilitate a VoIP session.
  std::optional<webrtc::ChannelId> channel_ RTC_GUARDED_BY(voip_thread_);
  // Members below are used for network related operations.
  std::unique_ptr<webrtc::AsyncUDPSocket> rtp_socket_
      RTC_GUARDED_BY(voip_thread_);
  std::unique_ptr<webrtc::AsyncUDPSocket> rtcp_socket_
      RTC_GUARDED_BY(voip_thread_);
  webrtc::SocketAddress rtp_local_address_ RTC_GUARDED_BY(voip_thread_);
  webrtc::SocketAddress rtcp_local_address_ RTC_GUARDED_BY(voip_thread_);
  webrtc::SocketAddress rtp_remote_address_ RTC_GUARDED_BY(voip_thread_);
  webrtc::SocketAddress rtcp_remote_address_ RTC_GUARDED_BY(voip_thread_);
};

}  // namespace webrtc_examples

#endif  // EXAMPLES_ANDROIDVOIP_JNI_ANDROID_VOIP_CLIENT_H_
