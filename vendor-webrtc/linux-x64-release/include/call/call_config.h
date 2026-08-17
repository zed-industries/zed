/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef CALL_CALL_CONFIG_H_
#define CALL_CALL_CONFIG_H_

#include <memory>
#include <optional>

#include "api/environment/environment.h"
#include "api/fec_controller.h"
#include "api/metronome/metronome.h"
#include "api/neteq/neteq_factory.h"
#include "api/network_state_predictor.h"
#include "api/scoped_refptr.h"
#include "api/task_queue/task_queue_base.h"
#include "api/transport/bitrate_settings.h"
#include "api/transport/network_control.h"
#include "api/units/time_delta.h"
#include "call/audio_state.h"
#include "call/rtp_transport_config.h"
#include "call/rtp_transport_controller_send_factory_interface.h"

namespace webrtc {

class AudioProcessing;

struct CallConfig {
  // If `network_task_queue` is set to nullptr, Call will assume that network
  // related callbacks will be made on the same TQ as the Call instance was
  // constructed on.
  explicit CallConfig(const Environment& env,
                      TaskQueueBase* network_task_queue = nullptr);

  // Move-only.
  CallConfig(CallConfig&&) = default;
  CallConfig& operator=(CallConfig&& other) = default;

  ~CallConfig();

  RtpTransportConfig ExtractTransportConfig() const;

  Environment env;

  // Bitrate config used until valid bitrate estimates are calculated. Also
  // used to cap total bitrate used. This comes from the remote connection.
  BitrateConstraints bitrate_config;

  // AudioState which is possibly shared between multiple calls.
  scoped_refptr<AudioState> audio_state;

  // Audio Processing Module to be used in this call.
  AudioProcessing* audio_processing = nullptr;

  // FecController to use for this call.
  FecControllerFactoryInterface* fec_controller_factory = nullptr;

  // NetworkStatePredictor to use for this call.
  NetworkStatePredictorFactoryInterface* network_state_predictor_factory =
      nullptr;

  // Call-specific Network controller factory to use. If this is set, it
  // takes precedence over network_controller_factory.
  std::unique_ptr<NetworkControllerFactoryInterface>
      per_call_network_controller_factory;
  // Network controller factory to use for this call if
  // per_call_network_controller_factory is null.
  NetworkControllerFactoryInterface* network_controller_factory = nullptr;

  // NetEq factory to use for this call.
  NetEqFactory* neteq_factory = nullptr;

  TaskQueueBase* network_task_queue_ = nullptr;
  // RtpTransportControllerSend to use for this call.
  RtpTransportControllerSendFactoryInterface*
      rtp_transport_controller_send_factory = nullptr;

  Metronome* decode_metronome = nullptr;
  Metronome* encode_metronome = nullptr;

  // The burst interval of the pacer, see TaskQueuePacedSender constructor.
  std::optional<TimeDelta> pacer_burst_interval;

  // Enables send packet batching from the egress RTP sender.
  bool enable_send_packet_batching = false;
};

}  // namespace webrtc

#endif  // CALL_CALL_CONFIG_H_
