/*
 *  Copyright 2011 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef EXAMPLES_PEERCONNECTION_SERVER_PEER_CHANNEL_H_
#define EXAMPLES_PEERCONNECTION_SERVER_PEER_CHANNEL_H_

#include <time.h>

#include <queue>
#include <string>
#include <vector>

class DataSocket;

// Represents a single peer connected to the server.
class ChannelMember {
 public:
  explicit ChannelMember(DataSocket* socket);
  ~ChannelMember();

  bool connected() const { return connected_; }
  int id() const { return id_; }
  void set_disconnected() { connected_ = false; }
  bool is_wait_request(DataSocket* ds) const;
  const std::string& name() const { return name_; }

  bool TimedOut();

  std::string GetPeerIdHeader() const;

  bool NotifyOfOtherMember(const ChannelMember& other);

  // Returns a string in the form "name,id\n".
  std::string GetEntry() const;

  void ForwardRequestToPeer(DataSocket* ds, ChannelMember* peer);

  void OnClosing(DataSocket* ds);

  void QueueResponse(const std::string& status,
                     const std::string& content_type,
                     const std::string& extra_headers,
                     const std::string& data);

  void SetWaitingSocket(DataSocket* ds);

 protected:
  struct QueuedResponse {
    std::string status, content_type, extra_headers, data;
  };

  DataSocket* waiting_socket_;
  int id_;
  bool connected_;
  time_t timestamp_;
  std::string name_;
  std::queue<QueuedResponse> queue_;
  static int s_member_id_;
};

// Manages all currently connected peers.
class PeerChannel {
 public:
  typedef std::vector<ChannelMember*> Members;

  PeerChannel() {}

  ~PeerChannel() { DeleteAll(); }

  const Members& members() const { return members_; }

  // Returns true if the request should be treated as a new ChannelMember
  // request.  Otherwise the request is not peerconnection related.
  static bool IsPeerConnection(const DataSocket* ds);

  // Finds a connected peer that's associated with the `ds` socket.
  ChannelMember* Lookup(DataSocket* ds) const;

  // Checks if the request has a "peer_id" parameter and if so, looks up the
  // peer for which the request is targeted at.
  ChannelMember* IsTargetedRequest(const DataSocket* ds) const;

  // Adds a new ChannelMember instance to the list of connected peers and
  // associates it with the socket.
  bool AddMember(DataSocket* ds);

  // Closes all connections and sends a "shutting down" message to all
  // connected peers.
  void CloseAll();

  // Called when a socket was determined to be closing by the peer (or if the
  // connection went dead).
  void OnClosing(DataSocket* ds);

  void CheckForTimeout();

 protected:
  void DeleteAll();
  void BroadcastChangedState(const ChannelMember& member,
                             Members* delivery_failures);
  void HandleDeliveryFailures(Members* failures);

  // Builds a simple list of "name,id\n" entries for each member.
  std::string BuildResponseForNewMember(const ChannelMember& member,
                                        std::string* content_type);

 protected:
  Members members_;
};

#endif  // EXAMPLES_PEERCONNECTION_SERVER_PEER_CHANNEL_H_
