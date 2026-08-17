/*
 *  Copyright 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MEDIA_BASE_MEDIA_CHANNEL_IMPL_H_
#define MEDIA_BASE_MEDIA_CHANNEL_IMPL_H_

#include <stddef.h>
#include <stdint.h>

#include "api/array_view.h"
#include "api/call/transport.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "api/task_queue/task_queue_base.h"
#include "media/base/media_channel.h"
#include "rtc_base/async_packet_socket.h"
#include "rtc_base/copy_on_write_buffer.h"
#include "rtc_base/dscp.h"
#include "rtc_base/socket.h"
#include "rtc_base/thread_annotations.h"
// This file contains the base classes for classes that implement
// the channel interfaces.
// These implementation classes used to be the exposed interface names,
// but this is in the process of being changed.

namespace webrtc {

// The `MediaChannelUtil` class provides functionality that is used by
// multiple MediaChannel-like objects, of both sending and receiving
// types.
class MediaChannelUtil {
 public:
  explicit MediaChannelUtil(TaskQueueBase* network_thread,
                            bool enable_dscp = false);
  virtual ~MediaChannelUtil();
  // Returns the absolute sendtime extension id value from media channel.
  virtual int GetRtpSendTimeExtnId() const;

  Transport* transport() { return &transport_; }

  // Base methods to send packet using MediaChannelNetworkInterface.
  // These methods are used by some tests only.
  bool SendPacket(CopyOnWriteBuffer* packet,
                  const AsyncSocketPacketOptions& options);

  bool SendRtcp(CopyOnWriteBuffer* packet,
                const AsyncSocketPacketOptions& options);

  int SetOption(MediaChannelNetworkInterface::SocketType type,
                Socket::Option opt,
                int option);

  // Functions that form part of one or more interface classes.
  // Not marked override, since this class does not inherit from the
  // interfaces.

  // Corresponds to the SDP attribute extmap-allow-mixed, see RFC8285.
  // Set to true if it's allowed to mix one- and two-byte RTP header extensions
  // in the same stream. The setter and getter must only be called from
  // worker_thread.
  void SetExtmapAllowMixed(bool extmap_allow_mixed);
  bool ExtmapAllowMixed() const;

  void SetInterface(MediaChannelNetworkInterface* iface);
  // Returns `true` if a non-null MediaChannelNetworkInterface pointer is held.
  // Must be called on the network thread.
  bool HasNetworkInterface() const;

 protected:
  bool DscpEnabled() const;

  void SetPreferredDscp(DiffServCodePoint new_dscp);

 private:
  // Implementation of the webrtc::Transport interface required
  // by Call().
  class TransportForMediaChannels : public Transport {
   public:
    TransportForMediaChannels(TaskQueueBase* network_thread, bool enable_dscp);

    virtual ~TransportForMediaChannels();

    // Implementation of webrtc::Transport
    bool SendRtp(ArrayView<const uint8_t> packet,
                 const PacketOptions& options) override;
    bool SendRtcp(ArrayView<const uint8_t> packet) override;

    // Not implementation of webrtc::Transport
    void SetInterface(MediaChannelNetworkInterface* iface);

    int SetOption(MediaChannelNetworkInterface::SocketType type,
                  Socket::Option opt,
                  int option);

    bool DoSendPacket(CopyOnWriteBuffer* packet,
                      bool rtcp,
                      const AsyncSocketPacketOptions& options);

    bool HasNetworkInterface() const {
      RTC_DCHECK_RUN_ON(network_thread_);
      return network_interface_ != nullptr;
    }
    bool DscpEnabled() const { return enable_dscp_; }

    void SetPreferredDscp(DiffServCodePoint new_dscp);

   private:
    // This is the DSCP value used for both RTP and RTCP channels if DSCP is
    // enabled. It can be changed at any time via `SetPreferredDscp`.
    DiffServCodePoint PreferredDscp() const {
      RTC_DCHECK_RUN_ON(network_thread_);
      return preferred_dscp_;
    }

    // Apply the preferred DSCP setting to the underlying network interface RTP
    // and RTCP channels. If DSCP is disabled, then apply the default DSCP
    // value.
    void UpdateDscp() RTC_RUN_ON(network_thread_);

    int SetOptionLocked(MediaChannelNetworkInterface::SocketType type,
                        Socket::Option opt,
                        int option) RTC_RUN_ON(network_thread_);

    const scoped_refptr<PendingTaskSafetyFlag> network_safety_
        RTC_PT_GUARDED_BY(network_thread_);
    TaskQueueBase* const network_thread_;
    const bool enable_dscp_;
    MediaChannelNetworkInterface* network_interface_
        RTC_GUARDED_BY(network_thread_) = nullptr;
    DiffServCodePoint preferred_dscp_ RTC_GUARDED_BY(network_thread_) =
        webrtc::DSCP_DEFAULT;
  };

  bool extmap_allow_mixed_ = false;
  TransportForMediaChannels transport_;
};

}  // namespace webrtc

#endif  // MEDIA_BASE_MEDIA_CHANNEL_IMPL_H_
