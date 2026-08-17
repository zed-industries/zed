/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_BASE_PSEUDO_TCP_H_
#define P2P_BASE_PSEUDO_TCP_H_

#include <stddef.h>
#include <stdint.h>

#include <list>
#include <memory>

#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/system/rtc_export.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

//////////////////////////////////////////////////////////////////////
// IPseudoTcpNotify
//////////////////////////////////////////////////////////////////////

class PseudoTcp;

class IPseudoTcpNotify {
 public:
  // Notification of tcp events
  virtual void OnTcpOpen(PseudoTcp* tcp) = 0;
  virtual void OnTcpReadable(PseudoTcp* tcp) = 0;
  virtual void OnTcpWriteable(PseudoTcp* tcp) = 0;
  virtual void OnTcpClosed(PseudoTcp* tcp, uint32_t error) = 0;

  // Write the packet onto the network
  enum WriteResult { WR_SUCCESS, WR_TOO_LARGE, WR_FAIL };
  virtual WriteResult TcpWritePacket(PseudoTcp* tcp,
                                     const char* buffer,
                                     size_t len) = 0;

 protected:
  virtual ~IPseudoTcpNotify() {}
};

//////////////////////////////////////////////////////////////////////
// PseudoTcp
//////////////////////////////////////////////////////////////////////

class RTC_EXPORT PseudoTcp {
 public:
  static uint32_t Now();

  PseudoTcp(IPseudoTcpNotify* notify, uint32_t conv);
  virtual ~PseudoTcp();

  int Connect();
  int Recv(char* buffer, size_t len);
  int Send(const char* buffer, size_t len);
  void Close(bool force);
  int GetError();

  enum TcpState {
    TCP_LISTEN,
    TCP_SYN_SENT,
    TCP_SYN_RECEIVED,
    TCP_ESTABLISHED,
    TCP_CLOSED
  };
  TcpState State() const { return m_state; }

  // Call this when the PMTU changes.
  void NotifyMTU(uint16_t mtu);

  // Call this based on timeout value returned from GetNextClock.
  // It's ok to call this too frequently.
  void NotifyClock(uint32_t now);

  // Call this whenever a packet arrives.
  // Returns true if the packet was processed successfully.
  bool NotifyPacket(const char* buffer, size_t len);

  // Call this to determine the next time NotifyClock should be called.
  // Returns false if the socket is ready to be destroyed.
  bool GetNextClock(uint32_t now, long& timeout);

  // Call these to get/set option values to tailor this PseudoTcp
  // instance's behaviour for the kind of data it will carry.
  // If an unrecognized option is set or got, an assertion will fire.
  //
  // Setting options for OPT_RCVBUF or OPT_SNDBUF after Connect() is called
  // will result in an assertion.
  enum Option {
    OPT_NODELAY,   // Whether to enable Nagle's algorithm (0 == off)
    OPT_ACKDELAY,  // The Delayed ACK timeout (0 == off).
    OPT_RCVBUF,    // Set the receive buffer size, in bytes.
    OPT_SNDBUF,    // Set the send buffer size, in bytes.
  };
  void GetOption(Option opt, int* value);
  void SetOption(Option opt, int value);

  // Returns current congestion window in bytes.
  uint32_t GetCongestionWindow() const;

  // Returns amount of data in bytes that has been sent, but haven't
  // been acknowledged.
  uint32_t GetBytesInFlight() const;

  // Returns number of bytes that were written in buffer and haven't
  // been sent.
  uint32_t GetBytesBufferedNotSent() const;

  // Returns current round-trip time estimate in milliseconds.
  uint32_t GetRoundTripTimeEstimateMs() const;

 protected:
  enum SendFlags { sfNone, sfDelayedAck, sfImmediateAck };

  struct Segment {
    uint32_t conv, seq, ack;
    uint8_t flags;
    uint16_t wnd;
    const char* data;
    uint32_t len;
    uint32_t tsval, tsecr;
  };

  struct SSegment {
    SSegment(uint32_t s, uint32_t l, bool c)
        : seq(s), len(l), /*tstamp(0),*/ xmit(0), bCtrl(c) {}
    uint32_t seq, len;
    // uint32_t tstamp;
    uint8_t xmit;
    bool bCtrl;
  };
  typedef std::list<SSegment> SList;

  struct RSegment {
    uint32_t seq, len;
  };

  uint32_t queue(const char* data, uint32_t len, bool bCtrl);

  // Creates a packet and submits it to the network. This method can either
  // send payload or just an ACK packet.
  //
  // `seq` is the sequence number of this packet.
  // `flags` is the flags for sending this packet.
  // `offset` is the offset to read from `m_sbuf`.
  // `len` is the number of bytes to read from `m_sbuf` as payload. If this
  // value is 0 then this is an ACK packet, otherwise this packet has payload.
  IPseudoTcpNotify::WriteResult packet(uint32_t seq,
                                       uint8_t flags,
                                       uint32_t offset,
                                       uint32_t len);
  bool parse(const uint8_t* buffer, uint32_t size);

  void attemptSend(SendFlags sflags = sfNone);

  void closedown(uint32_t err = 0);

  bool clock_check(uint32_t now, long& nTimeout);

  bool process(Segment& seg);
  bool transmit(const SList::iterator& seg, uint32_t now);

  void adjustMTU();

 protected:
  // This method is used in test only to query receive buffer state.
  bool isReceiveBufferFull() const;

  // This method is only used in tests, to disable window scaling
  // support for testing backward compatibility.
  void disableWindowScale();

 private:
  // Queue the connect message with TCP options.
  void queueConnectMessage();

  // Parse TCP options in the header.
  void parseOptions(const char* data, uint32_t len);

  // Apply a TCP option that has been read from the header.
  void applyOption(char kind, const char* data, uint32_t len);

  // Apply window scale option.
  void applyWindowScaleOption(uint8_t scale_factor);

  // Resize the send buffer with `new_size` in bytes.
  void resizeSendBuffer(uint32_t new_size);

  // Resize the receive buffer with `new_size` in bytes. This call adjusts
  // window scale factor `m_swnd_scale` accordingly.
  void resizeReceiveBuffer(uint32_t new_size);

  class LockedFifoBuffer final {
   public:
    explicit LockedFifoBuffer(size_t size);
    ~LockedFifoBuffer();

    size_t GetBuffered() const;
    bool SetCapacity(size_t size);
    bool ReadOffset(void* buffer,
                    size_t bytes,
                    size_t offset,
                    size_t* bytes_read);
    bool WriteOffset(const void* buffer,
                     size_t bytes,
                     size_t offset,
                     size_t* bytes_written);
    bool Read(void* buffer, size_t bytes, size_t* bytes_read);
    bool Write(const void* buffer, size_t bytes, size_t* bytes_written);
    void ConsumeReadData(size_t size);
    void ConsumeWriteBuffer(size_t size);
    bool GetWriteRemaining(size_t* size) const;

   private:
    bool ReadOffsetLocked(void* buffer,
                          size_t bytes,
                          size_t offset,
                          size_t* bytes_read)
        RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
    bool WriteOffsetLocked(const void* buffer,
                           size_t bytes,
                           size_t offset,
                           size_t* bytes_written)
        RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

    // the allocated buffer
    std::unique_ptr<char[]> buffer_ RTC_GUARDED_BY(mutex_);
    // size of the allocated buffer
    size_t buffer_length_ RTC_GUARDED_BY(mutex_);
    // amount of readable data in the buffer
    size_t data_length_ RTC_GUARDED_BY(mutex_);
    // offset to the readable data
    size_t read_position_ RTC_GUARDED_BY(mutex_);
    mutable Mutex mutex_;
  };

  IPseudoTcpNotify* m_notify;
  enum Shutdown { SD_NONE, SD_GRACEFUL, SD_FORCEFUL } m_shutdown;
  int m_error;

  // TCB data
  TcpState m_state;
  uint32_t m_conv;
  bool m_bReadEnable, m_bWriteEnable, m_bOutgoing;
  uint32_t m_lasttraffic;

  // Incoming data
  typedef std::list<RSegment> RList;
  RList m_rlist;
  uint32_t m_rbuf_len, m_rcv_nxt, m_rcv_wnd, m_lastrecv;
  uint8_t m_rwnd_scale;  // Window scale factor.
  LockedFifoBuffer m_rbuf;

  // Outgoing data
  SList m_slist;
  uint32_t m_sbuf_len, m_snd_nxt, m_snd_wnd, m_lastsend, m_snd_una;
  uint8_t m_swnd_scale;  // Window scale factor.
  LockedFifoBuffer m_sbuf;

  // Maximum segment size, estimated protocol level, largest segment sent
  uint32_t m_mss, m_msslevel, m_largest, m_mtu_advise;
  // Retransmit timer
  uint32_t m_rto_base;

  // Timestamp tracking
  uint32_t m_ts_recent, m_ts_lastack;

  // Round-trip calculation
  uint32_t m_rx_rttvar, m_rx_srtt, m_rx_rto;

  // Congestion avoidance, Fast retransmit/recovery, Delayed ACKs
  uint32_t m_ssthresh, m_cwnd;
  uint8_t m_dup_acks;
  uint32_t m_recover;
  uint32_t m_t_ack;

  // Configuration options
  bool m_use_nagling;
  uint32_t m_ack_delay;

  // This is used by unit tests to test backward compatibility of
  // PseudoTcp implementations that don't support window scaling.
  bool m_support_wnd_scale;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::IPseudoTcpNotify;
using ::webrtc::PseudoTcp;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_BASE_PSEUDO_TCP_H_
