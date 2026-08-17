// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_SCOPED_RASTER_TIMER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_SCOPED_RASTER_TIMER_H_

#include <optional>

#include "base/timer/elapsed_timer.h"
#include "gpu/command_buffer/client/raster_interface.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class PLATFORM_EXPORT ScopedRasterTimer {
  STACK_ALLOCATED();

 public:
  class Host;
  // raster_interface: pass null if rasterization is not gpu-accelerated
  ScopedRasterTimer(gpu::raster::RasterInterface* raster_interface,
                    Host& host,
                    bool always_measure_for_testing);
  ~ScopedRasterTimer();

  // Histogram names.
  static constexpr const char* const kRasterDurationUnacceleratedHistogram =
      "Blink.Canvas.RasterDuration.Unaccelerated";
  static constexpr const char* const kRasterDurationAcceleratedCpuHistogram =
      "Blink.Canvas.RasterDuration.Accelerated.CPU";
  static constexpr const char* const kRasterDurationAcceleratedGpuHistogram =
      "Blink.Canvas.RasterDuration.Accelerated.GPU";
  static constexpr const char* const kRasterDurationAcceleratedTotalHistogram =
      "Blink.Canvas.RasterDuration.Accelerated.Total";

 private:
  class AsyncGpuRasterTimer {
   public:
    // At construction time: starts tracking commands issued to the gpu
    // interface.
    explicit AsyncGpuRasterTimer(
        gpu::raster::RasterInterface& raster_interface);

    // Stop tracking issue command
    void FinishedIssuingCommands(gpu::raster::RasterInterface& raster_interface,
                                 base::TimeDelta cpu_raster_duration);

    // Returns true if the timer is done (i.e. all commands issued before
    // call to FinishedIssuingCommands have been executed and timed on the
    // service side). Must wait for this method to return true before
    // destroying `this`, otherwise the measurement will be lost.
    bool CheckTimer(gpu::raster::RasterInterface& raster_interface);
    ~AsyncGpuRasterTimer() = default;

   private:
    GLuint done_ = 0;
    GLuint gl_query_id_ = 0u;
    base::TimeDelta cpu_raster_duration_;
  };

 public:
  // Classes with methods that use ScopedRasterTimer must inherit
  // ScopedRasterTimer::Host
  class Host {
   public:
    void CheckGpuTimers(gpu::raster::RasterInterface* raster_interface);
    void AddGpuTimer(std::unique_ptr<AsyncGpuRasterTimer>);

   private:
    WTF::Vector<std::unique_ptr<AsyncGpuRasterTimer>> gpu_timers_;
  };

 private:
  bool active_ = false;
  // Optional. nullptr indicates that raster work load is not GPU accelerated.
  gpu::raster::RasterInterface* const raster_interface_;
  std::optional<base::ElapsedTimer> timer_;
  std::unique_ptr<AsyncGpuRasterTimer> gpu_timer_;
  Host& host_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_SCOPED_RASTER_TIMER_H_
