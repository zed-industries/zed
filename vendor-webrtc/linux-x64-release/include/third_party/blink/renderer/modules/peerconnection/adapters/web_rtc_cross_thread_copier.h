// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_ADAPTERS_WEB_RTC_CROSS_THREAD_COPIER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_ADAPTERS_WEB_RTC_CROSS_THREAD_COPIER_H_

// This file defines specializations for the CrossThreadCopier that allow WebRTC
// types to be passed across threads using their copy constructors.

#include <set>
#include <vector>

#include "base/unguessable_token.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_copier.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/webrtc/api/rtc_error.h"
#include "third_party/webrtc/api/scoped_refptr.h"
#include "third_party/webrtc/api/transport/network_types.h"
#include "third_party/webrtc/p2p/base/port_allocator.h"
#include "third_party/webrtc/p2p/base/transport_description.h"
#include "third_party/webrtc/rtc_base/socket_address.h"

namespace webrtc {
class DtlsTransportInformation;
class MediaStreamInterface;
class RtpReceiverInterface;
class SctpTransportInformation;
class VideoTrackInterface;
struct DataBuffer;
}  // namespace webrtc

namespace blink {

class MockWebRtcVideoTrack;
class MediaStreamVideoTrack;

}  // namespace blink

namespace WTF {

template <>
struct CrossThreadCopier<std::optional<base::UnguessableToken>>
    : public CrossThreadCopierPassThrough<
          std::optional<base::UnguessableToken>> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<webrtc::IceParameters>
    : public CrossThreadCopierPassThrough<webrtc::IceParameters> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<std::set<webrtc::SocketAddress>>
    : public CrossThreadCopierPassThrough<std::set<webrtc::SocketAddress>> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<std::vector<webrtc::RelayServerConfig>>
    : public CrossThreadCopierPassThrough<
          std::vector<webrtc::RelayServerConfig>> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<Vector<webrtc::Candidate>>
    : public CrossThreadCopierPassThrough<Vector<webrtc::Candidate>> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<webrtc::Candidate>
    : public CrossThreadCopierPassThrough<webrtc::Candidate> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<std::pair<webrtc::Candidate, webrtc::Candidate>>
    : public CrossThreadCopierPassThrough<
          std::pair<webrtc::Candidate, webrtc::Candidate>> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<webrtc::DtlsTransportInformation>
    : public CrossThreadCopierPassThrough<webrtc::DtlsTransportInformation> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<webrtc::SctpTransportInformation>
    : public CrossThreadCopierPassThrough<webrtc::SctpTransportInformation> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<webrtc::RTCError>
    : public CrossThreadCopierPassThrough<webrtc::RTCError> {
  STATIC_ONLY(CrossThreadCopier);
  using Type = webrtc::RTCError;
  static webrtc::RTCError Copy(webrtc::RTCError error) {
    return error;  // This is in fact a move.
  }
};

template <>
struct CrossThreadCopier<webrtc::scoped_refptr<webrtc::RtpReceiverInterface>>
    : public CrossThreadCopierPassThrough<
          webrtc::scoped_refptr<webrtc::RtpReceiverInterface>> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<
    std::vector<webrtc::scoped_refptr<webrtc::MediaStreamInterface>>>
    : public CrossThreadCopierPassThrough<
          std::vector<webrtc::scoped_refptr<webrtc::MediaStreamInterface>>> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<blink::MockWebRtcVideoTrack>
    : public CrossThreadCopierPassThrough<blink::MockWebRtcVideoTrack> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<blink::MediaStreamVideoTrack>
    : public CrossThreadCopierPassThrough<blink::MediaStreamVideoTrack> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<webrtc::scoped_refptr<webrtc::VideoTrackInterface>>
    : public CrossThreadCopierPassThrough<
          webrtc::scoped_refptr<webrtc::VideoTrackInterface>> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<webrtc::DataBuffer>
    : public CrossThreadCopierPassThrough<webrtc::DataBuffer> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<webrtc::TransportPacketsFeedback>
    : public CrossThreadCopierPassThrough<webrtc::TransportPacketsFeedback> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<webrtc::SentPacket>
    : public CrossThreadCopierPassThrough<webrtc::SentPacket> {
  STATIC_ONLY(CrossThreadCopier);
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_ADAPTERS_WEB_RTC_CROSS_THREAD_COPIER_H_
