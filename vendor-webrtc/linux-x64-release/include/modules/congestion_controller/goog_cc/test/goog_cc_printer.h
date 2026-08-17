/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_CONGESTION_CONTROLLER_GOOG_CC_TEST_GOOG_CC_PRINTER_H_
#define MODULES_CONGESTION_CONTROLLER_GOOG_CC_TEST_GOOG_CC_PRINTER_H_

#include <deque>
#include <memory>
#include <string>

#include "api/rtc_event_log_output.h"
#include "api/transport/goog_cc_factory.h"
#include "api/transport/network_control.h"
#include "api/transport/network_types.h"
#include "api/units/data_size.h"
#include "api/units/timestamp.h"
#include "modules/congestion_controller/goog_cc/goog_cc_network_control.h"

namespace webrtc {

class FieldLogger {
 public:
  virtual ~FieldLogger() = default;
  virtual const std::string& name() const = 0;
  virtual void WriteValue(RtcEventLogOutput* out) = 0;
};

class GoogCcStatePrinter {
 public:
  GoogCcStatePrinter();
  GoogCcStatePrinter(const GoogCcStatePrinter&) = delete;
  GoogCcStatePrinter& operator=(const GoogCcStatePrinter&) = delete;
  ~GoogCcStatePrinter();

  void PrintHeaders(RtcEventLogOutput* log);
  void PrintState(RtcEventLogOutput* log,
                  GoogCcNetworkController* controller,
                  Timestamp at_time);

 private:
  std::deque<FieldLogger*> CreateLoggers();
  std::deque<std::unique_ptr<FieldLogger>> loggers_;

  GoogCcNetworkController* controller_ = nullptr;
  TargetTransferRate target_;
  PacerConfig pacing_;
  DataSize congestion_window_ = DataSize::PlusInfinity();
  NetworkStateEstimate est_;
};

class GoogCcDebugFactory : public GoogCcNetworkControllerFactory {
 public:
  GoogCcDebugFactory();
  explicit GoogCcDebugFactory(GoogCcFactoryConfig config);
  std::unique_ptr<NetworkControllerInterface> Create(
      NetworkControllerConfig config) override;

  void PrintState(Timestamp at_time);

  void AttachWriter(std::unique_ptr<RtcEventLogOutput> log_writer);

 private:
  GoogCcStatePrinter printer_;
  GoogCcNetworkController* controller_ = nullptr;
  std::unique_ptr<RtcEventLogOutput> log_writer_;
};
}  // namespace webrtc

#endif  // MODULES_CONGESTION_CONTROLLER_GOOG_CC_TEST_GOOG_CC_PRINTER_H_
