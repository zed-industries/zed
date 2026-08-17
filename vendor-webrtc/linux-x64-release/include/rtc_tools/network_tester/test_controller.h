/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_TOOLS_NETWORK_TESTER_TEST_CONTROLLER_H_
#define RTC_TOOLS_NETWORK_TESTER_TEST_CONTROLLER_H_

#include <stddef.h>
#include <stdint.h>

#include <array>
#include <memory>
#include <optional>
#include <string>

#include "api/sequence_checker.h"
#include "p2p/base/basic_packet_socket_factory.h"
#include "rtc_base/async_packet_socket.h"
#include "rtc_base/network/received_packet.h"
#include "rtc_base/socket_address.h"
#include "rtc_base/socket_server.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"
#include "rtc_tools/network_tester/packet_logger.h"
#include "rtc_tools/network_tester/packet_sender.h"

#ifdef WEBRTC_NETWORK_TESTER_PROTO
#include "rtc_tools/network_tester/network_tester_packet.pb.h"
using webrtc::network_tester::packet::NetworkTesterPacket;
#else
class NetworkTesterPacket;
#endif  // WEBRTC_NETWORK_TESTER_PROTO

namespace webrtc {

constexpr size_t kEthernetMtu = 1500;

class TestController {
 public:
  TestController(int min_port,
                 int max_port,
                 const std::string& config_file_path,
                 const std::string& log_file_path);
  ~TestController();

  TestController(const TestController&) = delete;
  TestController& operator=(const TestController&) = delete;

  void SendConnectTo(const std::string& hostname, int port);

  void SendData(const NetworkTesterPacket& packet,
                std::optional<size_t> data_size);

  void OnTestDone();

  bool IsTestDone();

 private:
  void OnReadPacket(AsyncPacketSocket* socket,
                    const ReceivedIpPacket& received_packet);
  RTC_NO_UNIQUE_ADDRESS SequenceChecker test_controller_thread_checker_;
  std::unique_ptr<SocketServer> socket_server_;
  std::unique_ptr<Thread> packet_sender_thread_;
  BasicPacketSocketFactory socket_factory_
      RTC_GUARDED_BY(packet_sender_thread_);
  const std::string config_file_path_;
  PacketLogger packet_logger_ RTC_GUARDED_BY(packet_sender_thread_);
  Mutex test_done_lock_ RTC_GUARDED_BY(test_controller_thread_checker_);
  bool local_test_done_ RTC_GUARDED_BY(test_done_lock_);
  bool remote_test_done_ RTC_GUARDED_BY(test_done_lock_);
  std::array<char, kEthernetMtu> send_data_
      RTC_GUARDED_BY(packet_sender_thread_);
  std::unique_ptr<AsyncPacketSocket> udp_socket_
      RTC_GUARDED_BY(packet_sender_thread_);
  SocketAddress remote_address_;
  std::unique_ptr<PacketSender> packet_sender_
      RTC_GUARDED_BY(packet_sender_thread_);
  scoped_refptr<webrtc::PendingTaskSafetyFlag> task_safety_flag_;
};

}  // namespace webrtc

#endif  // RTC_TOOLS_NETWORK_TESTER_TEST_CONTROLLER_H_
