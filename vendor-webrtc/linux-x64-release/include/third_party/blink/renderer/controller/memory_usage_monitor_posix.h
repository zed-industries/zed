// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CONTROLLER_MEMORY_USAGE_MONITOR_POSIX_H_
#define THIRD_PARTY_BLINK_RENDERER_CONTROLLER_MEMORY_USAGE_MONITOR_POSIX_H_

#include "base/files/file.h"
#include "base/files/scoped_file.h"
#include "base/gtest_prod_util.h"
#include "build/build_config.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "third_party/blink/renderer/controller/controller_export.h"
#include "third_party/blink/renderer/controller/memory_usage_monitor.h"

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)
#include "third_party/blink/public/mojom/memory_usage_monitor_linux.mojom-blink.h"
#endif

namespace blink {

// MemoryUsageMonitor implementation for Android and Linux.
class CONTROLLER_EXPORT MemoryUsageMonitorPosix
    : public MemoryUsageMonitor
#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)
    ,
      public mojom::blink::MemoryUsageMonitorLinux
#endif
{
 public:
  MemoryUsageMonitorPosix() = default;

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)
  static void Bind(
      mojo::PendingReceiver<mojom::blink::MemoryUsageMonitorLinux> receiver);
#endif

 private:
  FRIEND_TEST_ALL_PREFIXES(MemoryUsageMonitorPosixTest,
                           CalculateProcessFootprint);

  friend class CrashMemoryMetricsReporterImpl;
  void GetProcessMemoryUsage(MemoryUsage&) override;
  static bool CalculateProcessMemoryFootprint(int statm_fd,
                                              int status_fd,
                                              uint64_t* private_footprint,
                                              uint64_t* swap_footprint,
                                              uint64_t* vm_size,
                                              uint64_t* vm_hwm_size);

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)
  // mojom::MemoryUsageMonitorLinux implementations:
  void SetProcFiles(base::File statm_file, base::File status_file) override;
#endif

#if BUILDFLAG(IS_ANDROID)
  void ResetFileDescriptors();
  // For Android, SetProcFiles is used only for testing.
  void SetProcFiles(base::File statm_file, base::File status_file);

  bool file_descriptors_reset_ = false;
#endif

  // The file descriptor to current process proc files. The files are kept open
  // for the whole lifetime of the renderer.
  base::ScopedFD statm_fd_;
  base::ScopedFD status_fd_;

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)
  mojo::Receiver<mojom::blink::MemoryUsageMonitorLinux> receiver_{this};
#endif
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CONTROLLER_MEMORY_USAGE_MONITOR_POSIX_H_
