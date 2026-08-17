/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_TOOLS_NETEQ_STATS_PLOTTER_H_
#define MODULES_AUDIO_CODING_NETEQ_TOOLS_NETEQ_STATS_PLOTTER_H_

#include <memory>
#include <string>

#include "absl/strings/string_view.h"
#include "modules/audio_coding/neteq/tools/neteq_delay_analyzer.h"
#include "modules/audio_coding/neteq/tools/neteq_stats_getter.h"
#include "modules/audio_coding/neteq/tools/neteq_test.h"

namespace webrtc {
namespace test {

class NetEqStatsPlotter : public NetEqSimulationEndedCallback {
 public:
  NetEqStatsPlotter(bool make_matlab_plot,
                    bool make_python_plot,
                    bool show_concealment_events,
                    absl::string_view base_file_name);

  void SimulationEnded(int64_t simulation_time_ms) override;

  NetEqStatsGetter* stats_getter() { return stats_getter_.get(); }

 private:
  std::unique_ptr<NetEqStatsGetter> stats_getter_;
  const bool make_matlab_plot_;
  const bool make_python_plot_;
  const bool show_concealment_events_;
  const std::string base_file_name_;
};

}  // namespace test
}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_NETEQ_TOOLS_NETEQ_STATS_PLOTTER_H_
