//
// Copyright 2018 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

#ifndef GRPC_TEST_CORE_TEST_UTIL_FAKE_UDP_AND_TCP_SERVER_H
#define GRPC_TEST_CORE_TEST_UTIL_FAKE_UDP_AND_TCP_SERVER_H

#include <grpc/support/port_platform.h>
#include <grpc/support/sync.h>

#include <functional>
#include <memory>
#include <string>
#include <thread>

namespace grpc_core {
namespace testing {

// This class is used to simulate a variety of network conditions in
// unit tests.
//
// Note that resulting server only listens on the IPv6 loopback
// address, "[::1]". This is expected to be OK as all known gRPC unit test
// environments have this address available.
//
// As examples, this can be used to (but is not limited to) exercise
// the following cases:
//
// 1) DNS resolver's UDP requests experience packet loss:
//
//     testing::FakeUdpAndTcpServer fake_dns_server(
//          testing::FakeUdpAndTcpServer::AcceptMode::
//             kWaitForClientToSendFirstBytes,
//     testing::FakeUdpAndTcpServer::CloseSocketUponCloseFromPeer);
//     auto server_uri = absl::StrFormat("dns:///[::]:%d/localhost:1234",
//         fake_dns_server.port());
//
// 2) Server gets stuck while setting up a security handshake and client's
//    security handshake times out (requires using secure channels):
//
//     testing::FakeUdpAndTcpServer fake_server(
//          testing::FakeUdpAndTcpServer::AcceptMode::
//             kWaitForClientToSendFirstBytes,
//     testing::FakeUdpAndTcpServer::CloseSocketUponCloseFromPeer);
//     auto server_uri = absl::StrFormat("[::1]:%d", fake_server.port());
//
// 3) Client connections are immediately closed after sending the first bytes
//    to an insecure server:
//
//     testing::FakeUdpAndTcpServer fake_server(
//          testing::FakeUdpAndTcpServer::AcceptMode::
//             kEagerlySendSettings,
//     testing::FakeUdpAndTcpServer::CloseSocketUponReceivingBytesFromPeer);
//     auto server_uri = absl::StrFormat("[::1]:%d", fake_server.port());
//
class FakeUdpAndTcpServer {
 public:
  enum class ProcessReadResult {
    kContinueReading = 0,
    kCloseSocket,
  };

  enum class AcceptMode {
    kWaitForClientToSendFirstBytes,  // useful for emulating ALTS based
                                     // grpc servers
    kEagerlySendSettings,  // useful for emulating insecure grpc servers (e.g.
                           // ALTS handshake servers)
  };

  explicit FakeUdpAndTcpServer(
      AcceptMode accept_mode,
      std::function<ProcessReadResult(int, int, int)> process_read_cb);

  ~FakeUdpAndTcpServer();

  const char* address() { return address_.c_str(); }

  int port() { return port_; };

  static ProcessReadResult CloseSocketUponReceivingBytesFromPeer(
      int bytes_received_size, int read_error, int s);

  static ProcessReadResult CloseSocketUponCloseFromPeer(int bytes_received_size,
                                                        int read_error, int s);

  static ProcessReadResult SendThreeAllZeroBytes(int bytes_received_size,
                                                 int read_error, int s);

  void ReadFromUdpSocket();

  // Run a loop that periodically, every 10 ms:
  //   1) Checks if there are any new TCP connections to accept.
  //   2) Checks if any data has arrived yet on established connections,
  //      and reads from them if so, processing the sockets as configured.
  void RunServerLoop();

 private:
  class FakeUdpAndTcpServerPeer {
   public:
    explicit FakeUdpAndTcpServerPeer(int fd);

    ~FakeUdpAndTcpServerPeer();

    void MaybeContinueSendingSettings();

    int fd() { return fd_; }

   private:
    int fd_;
    int total_bytes_sent_ = 0;
  };

  int accept_socket_;
  int udp_socket_;
  int port_;
  gpr_event stop_ev_;
  std::string address_;
  std::unique_ptr<std::thread> run_server_loop_thd_;
  const AcceptMode accept_mode_;
  std::function<ProcessReadResult(int, int, int)> process_read_cb_;
};

}  // namespace testing
}  // namespace grpc_core

#endif  // GRPC_TEST_CORE_TEST_UTIL_FAKE_UDP_AND_TCP_SERVER_H
