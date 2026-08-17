/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef RTC_TOOLS_DATA_CHANNEL_BENCHMARK_GRPC_SIGNALING_H_
#define RTC_TOOLS_DATA_CHANNEL_BENCHMARK_GRPC_SIGNALING_H_

#include <memory>
#include <string>

#include "api/jsep.h"
#include "rtc_tools/data_channel_benchmark/signaling_interface.h"

namespace webrtc {

// This class defines a server enabling clients to perform a PeerConnection
// negotiation directly over gRPC.
// When a client connects, a callback is run to handle the request.
class GrpcSignalingServerInterface {
 public:
  virtual ~GrpcSignalingServerInterface() = default;

  // Start listening for connections.
  virtual void Start() = 0;

  // Wait for the gRPC server to terminate.
  virtual void Wait() = 0;

  // Stop the gRPC server instance.
  virtual void Stop() = 0;

  // The port the server is listening on.
  virtual int SelectedPort() = 0;

  // Create a gRPC server listening on |port| that will run |callback| on each
  // request. If |oneshot| is true, it will terminate after serving one request.
  static std::unique_ptr<GrpcSignalingServerInterface> Create(
      std::function<void(webrtc::SignalingInterface*)> callback,
      int port,
      bool oneshot);
};

// This class defines a client that can connect to a server and perform a
// PeerConnection negotiation directly over gRPC.
class GrpcSignalingClientInterface {
 public:
  virtual ~GrpcSignalingClientInterface() = default;

  // Connect the client to the gRPC server.
  virtual bool Start() = 0;
  virtual webrtc::SignalingInterface* signaling_client() = 0;

  // Create a client to connnect to a server at |server_address|.
  static std::unique_ptr<GrpcSignalingClientInterface> Create(
      const std::string& server_address);
};

}  // namespace webrtc
#endif  // RTC_TOOLS_DATA_CHANNEL_BENCHMARK_GRPC_SIGNALING_H_
