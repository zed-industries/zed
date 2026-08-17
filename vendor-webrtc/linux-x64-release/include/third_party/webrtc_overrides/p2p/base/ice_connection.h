// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_WEBRTC_OVERRIDES_P2P_BASE_ICE_CONNECTION_H_
#define THIRD_PARTY_WEBRTC_OVERRIDES_P2P_BASE_ICE_CONNECTION_H_

#include <cstdint>
#include <ostream>
#include <string>
#include <vector>

#include "third_party/webrtc/api/array_view.h"
#include "third_party/webrtc/api/candidate.h"
#include "third_party/webrtc/p2p/base/connection.h"
#include "third_party/webrtc/rtc_base/system/rtc_export.h"

namespace blink {

// Represents an ICE connection comprising of a local candidate, a remote
// candidate, and some state information about the connection.
class RTC_EXPORT IceConnection {
 public:
  struct RttSample {
    int64_t timestamp;
    int value;
  };

  enum class WriteState {
    STATE_WRITABLE = 0,          // we have received ping responses recently
    STATE_WRITE_UNRELIABLE = 1,  // we have had a few ping failures
    STATE_WRITE_INIT = 2,        // we have yet to receive a ping response
    STATE_WRITE_TIMEOUT = 3,     // we have had a large number of ping failures
  };

  explicit IceConnection(const webrtc::Connection* connection);

  IceConnection(const IceConnection&) = default;

  ~IceConnection() = default;

  // The connection ID.
  uint32_t id() const { return id_; }
  // The local candidate for this connection.
  const webrtc::Candidate& local_candidate() const { return local_candidate_; }
  // The remote candidate for this connection.
  const webrtc::Candidate& remote_candidate() const {
    return remote_candidate_;
  }

  // Whether the connection is in a connected state.
  bool connected() const { return connected_; }
  // Whether the connection is currently active for the transport.
  bool selected() const { return selected_; }
  // Write state of the connection.
  WriteState write_state() const { return write_state_; }

  // Last time we sent a ping to the other side.
  int64_t last_ping_sent() const { return last_ping_sent_; }
  // Last time we received a ping from the other side.
  int64_t last_ping_received() const { return last_ping_received_; }
  // Last time we received date from the other side.
  int64_t last_data_received() const { return last_data_received_; }
  // Last time we received a response to a ping from the other side.
  int64_t last_ping_response_received() const {
    return last_ping_response_received_;
  }
  // The number of pings sent.
  int num_pings_sent() const { return num_pings_sent_; }
  // Samples of round trip times.
  const webrtc::ArrayView<const RttSample> rtt_samples() const {
    return rtt_samples_;
  }

  std::string ToString() const;
  // Pretty printing for unit test matchers.
  friend void PrintTo(const IceConnection& conn, std::ostream* os) {
    *os << conn.ToString();
  }

 private:
  uint32_t id_;
  webrtc::Candidate local_candidate_;
  webrtc::Candidate remote_candidate_;

  // Connection state information.

  bool connected_;
  bool selected_;
  WriteState write_state_;
  int64_t last_ping_sent_;
  int64_t last_ping_received_;
  int64_t last_data_received_;
  int64_t last_ping_response_received_;
  int num_pings_sent_;

  std::vector<RttSample> rtt_samples_;
};

}  // namespace blink

#endif  // THIRD_PARTY_WEBRTC_OVERRIDES_P2P_BASE_ICE_CONNECTION_H_
