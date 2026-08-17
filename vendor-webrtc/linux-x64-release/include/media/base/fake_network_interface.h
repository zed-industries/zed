/*
 *  Copyright (c) 2004 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MEDIA_BASE_FAKE_NETWORK_INTERFACE_H_
#define MEDIA_BASE_FAKE_NETWORK_INTERFACE_H_

#include <cstddef>
#include <cstdint>
#include <map>
#include <set>
#include <utility>
#include <vector>

#include "api/task_queue/pending_task_safety_flag.h"
#include "api/task_queue/task_queue_base.h"
#include "api/units/timestamp.h"
#include "media/base/media_channel.h"
#include "modules/rtp_rtcp/source/rtp_packet_received.h"
#include "modules/rtp_rtcp/source/rtp_util.h"
#include "rtc_base/async_packet_socket.h"
#include "rtc_base/byte_order.h"
#include "rtc_base/checks.h"
#include "rtc_base/copy_on_write_buffer.h"
#include "rtc_base/dscp.h"
#include "rtc_base/logging.h"
#include "rtc_base/socket.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"
#include "rtc_base/time_utils.h"

namespace webrtc {

// Fake NetworkInterface that sends/receives RTP/RTCP packets.
class FakeNetworkInterface : public MediaChannelNetworkInterface {
 public:
  FakeNetworkInterface()
      : thread_(Thread::Current()),
        dest_(NULL),
        conf_(false),
        sendbuf_size_(-1),
        recvbuf_size_(-1),
        dscp_(DSCP_NO_CHANGE) {}

  void SetDestination(MediaReceiveChannelInterface* dest) { dest_ = dest; }

  // Conference mode is a mode where instead of simply forwarding the packets,
  // the transport will send multiple copies of the packet with the specified
  // SSRCs. This allows us to simulate receiving media from multiple sources.
  void SetConferenceMode(bool conf, const std::vector<uint32_t>& ssrcs)
      RTC_LOCKS_EXCLUDED(mutex_) {
    MutexLock lock(&mutex_);
    conf_ = conf;
    conf_sent_ssrcs_ = ssrcs;
  }

  int NumRtpBytes() RTC_LOCKS_EXCLUDED(mutex_) {
    MutexLock lock(&mutex_);
    int bytes = 0;
    for (size_t i = 0; i < rtp_packets_.size(); ++i) {
      bytes += static_cast<int>(rtp_packets_[i].size());
    }
    return bytes;
  }

  int NumRtpBytes(uint32_t ssrc) RTC_LOCKS_EXCLUDED(mutex_) {
    MutexLock lock(&mutex_);
    int bytes = 0;
    GetNumRtpBytesAndPackets(ssrc, &bytes, NULL);
    return bytes;
  }

  int NumRtpPackets() RTC_LOCKS_EXCLUDED(mutex_) {
    MutexLock lock(&mutex_);
    return static_cast<int>(rtp_packets_.size());
  }

  int NumRtpPackets(uint32_t ssrc) RTC_LOCKS_EXCLUDED(mutex_) {
    MutexLock lock(&mutex_);
    int packets = 0;
    GetNumRtpBytesAndPackets(ssrc, NULL, &packets);
    return packets;
  }

  int NumSentSsrcs() RTC_LOCKS_EXCLUDED(mutex_) {
    MutexLock lock(&mutex_);
    return static_cast<int>(sent_ssrcs_.size());
  }

  CopyOnWriteBuffer GetRtpPacket(int index) RTC_LOCKS_EXCLUDED(mutex_) {
    MutexLock lock(&mutex_);
    if (index >= static_cast<int>(rtp_packets_.size())) {
      return {};
    }
    return rtp_packets_[index];
  }

  int NumRtcpPackets() RTC_LOCKS_EXCLUDED(mutex_) {
    MutexLock lock(&mutex_);
    return static_cast<int>(rtcp_packets_.size());
  }

  // Note: callers are responsible for deleting the returned buffer.
  const CopyOnWriteBuffer* GetRtcpPacket(int index) RTC_LOCKS_EXCLUDED(mutex_) {
    MutexLock lock(&mutex_);
    if (index >= static_cast<int>(rtcp_packets_.size())) {
      return NULL;
    }
    return new CopyOnWriteBuffer(rtcp_packets_[index]);
  }

  int sendbuf_size() const { return sendbuf_size_; }
  int recvbuf_size() const { return recvbuf_size_; }
  DiffServCodePoint dscp() const { return dscp_; }
  AsyncSocketPacketOptions options() const { return options_; }

 protected:
  virtual bool SendPacket(CopyOnWriteBuffer* packet,
                          const AsyncSocketPacketOptions& options)
      RTC_LOCKS_EXCLUDED(mutex_) {
    if (!webrtc::IsRtpPacket(*packet)) {
      return false;
    }

    MutexLock lock(&mutex_);
    sent_ssrcs_[webrtc::ParseRtpSsrc(*packet)]++;
    options_ = options;

    rtp_packets_.push_back(*packet);
    if (conf_) {
      for (size_t i = 0; i < conf_sent_ssrcs_.size(); ++i) {
        SetRtpSsrc(conf_sent_ssrcs_[i], *packet);
        PostPacket(*packet);
      }
    } else {
      PostPacket(*packet);
    }
    return true;
  }

  virtual bool SendRtcp(CopyOnWriteBuffer* packet,
                        const AsyncSocketPacketOptions& options)
      RTC_LOCKS_EXCLUDED(mutex_) {
    MutexLock lock(&mutex_);
    rtcp_packets_.push_back(*packet);
    options_ = options;
    if (!conf_) {
      // don't worry about RTCP in conf mode for now
      RTC_LOG(LS_VERBOSE) << "Dropping RTCP packet, they are not handled by "
                             "MediaChannel anymore.";
    }
    return true;
  }

  virtual int SetOption(SocketType /* type */, Socket::Option opt, int option) {
    if (opt == Socket::OPT_SNDBUF) {
      sendbuf_size_ = option;
    } else if (opt == Socket::OPT_RCVBUF) {
      recvbuf_size_ = option;
    } else if (opt == Socket::OPT_DSCP) {
      dscp_ = static_cast<DiffServCodePoint>(option);
    }
    return 0;
  }

  void PostPacket(CopyOnWriteBuffer packet) {
    thread_->PostTask(SafeTask(safety_.flag(), [this, packet = std::move(
                                                          packet)]() mutable {
      if (dest_) {
        RtpPacketReceived parsed_packet;
        if (parsed_packet.Parse(packet)) {
          parsed_packet.set_arrival_time(Timestamp::Micros(TimeMicros()));
          dest_->OnPacketReceived(std::move(parsed_packet));
        } else {
          RTC_DCHECK_NOTREACHED();
        }
      }
    }));
  }

 private:
  void SetRtpSsrc(uint32_t ssrc, CopyOnWriteBuffer& buffer) {
    RTC_CHECK_GE(buffer.size(), 12);
    webrtc::SetBE32(buffer.MutableData() + 8, ssrc);
  }

  void GetNumRtpBytesAndPackets(uint32_t ssrc, int* bytes, int* packets) {
    if (bytes) {
      *bytes = 0;
    }
    if (packets) {
      *packets = 0;
    }
    for (size_t i = 0; i < rtp_packets_.size(); ++i) {
      if (ssrc == webrtc::ParseRtpSsrc(rtp_packets_[i])) {
        if (bytes) {
          *bytes += static_cast<int>(rtp_packets_[i].size());
        }
        if (packets) {
          ++(*packets);
        }
      }
    }
  }

  TaskQueueBase* thread_;
  MediaReceiveChannelInterface* dest_;
  bool conf_;
  // The ssrcs used in sending out packets in conference mode.
  std::vector<uint32_t> conf_sent_ssrcs_;
  // Map to track counts of packets that have been sent per ssrc.
  // This includes packets that are dropped.
  std::map<uint32_t, uint32_t> sent_ssrcs_;
  // Map to track packet-number that needs to be dropped per ssrc.
  std::map<uint32_t, std::set<uint32_t> > drop_map_;
  Mutex mutex_;
  std::vector<CopyOnWriteBuffer> rtp_packets_;
  std::vector<CopyOnWriteBuffer> rtcp_packets_;
  int sendbuf_size_;
  int recvbuf_size_;
  DiffServCodePoint dscp_;
  // Options of the most recently sent packet.
  AsyncSocketPacketOptions options_;
  ScopedTaskSafety safety_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::FakeNetworkInterface;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // MEDIA_BASE_FAKE_NETWORK_INTERFACE_H_
