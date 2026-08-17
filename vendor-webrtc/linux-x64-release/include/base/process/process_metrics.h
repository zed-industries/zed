// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file contains routines for gathering resource statistics for processes
// running on the system.

#ifndef BASE_PROCESS_PROCESS_METRICS_H_
#define BASE_PROCESS_PROCESS_METRICS_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <string_view>

#include "base/base_export.h"
#include "base/gtest_prod_util.h"
#include "base/memory/raw_ptr.h"
#include "base/process/process_handle.h"
#include "base/time/time.h"
#include "base/types/expected.h"
#include "base/values.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_APPLE)
#include <mach/mach.h>

#include "base/process/port_provider_mac.h"

#if !BUILDFLAG(IS_IOS)
#include <mach/mach_vm.h>
#endif
#endif

#if BUILDFLAG(IS_WIN)
#include "base/win/scoped_handle.h"
#include "base/win/windows_types.h"
#endif

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_ANDROID) || \
    BUILDFLAG(IS_AIX)
#include <string>
#include <utility>
#include <vector>

#include "base/threading/platform_thread.h"
#endif

namespace base {

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_ANDROID)
// Minor and major page fault counts since the process creation.
// Both counts are process-wide, and exclude child processes.
//
// minor: Number of page faults that didn't require disk IO.
// major: Number of page faults that required disk IO.
struct PageFaultCounts {
  int64_t minor;
  int64_t major;
};
#endif  // BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) ||
        // BUILDFLAG(IS_ANDROID)

// Convert a POSIX timeval to microseconds.
BASE_EXPORT int64_t TimeValToMicroseconds(const struct timeval& tv);

enum class ProcessUsageError {
  // The OS returned an error while measuring the CPU usage. The possible causes
  // vary by platform.
  kSystemError,

  // Process CPU usage couldn't be measured because the process wasn't running.
  // Some platforms may return kSystemError instead in this situation.
  kProcessNotFound,
};

using ProcessCPUUsageError = ProcessUsageError;

struct ProcessMemoryInfo {
  uint64_t resident_set_bytes = 0;

#if BUILDFLAG(IS_APPLE)
  uint64_t physical_footprint_bytes = 0;
  uint64_t internal_bytes = 0;
  uint64_t compressed_bytes = 0;
#endif  // BUILDFLAG(IS_APPLE)

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_ANDROID) || \
    BUILDFLAG(IS_FUCHSIA)
  uint64_t rss_anon_bytes = 0;
  uint64_t vm_swap_bytes = 0;
#endif  // BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) ||
        // BUILDFLAG(IS_ANDROID)

#if BUILDFLAG(IS_WIN)
  uint64_t private_bytes = 0;
#endif  // BUILDFLAG(IS_WIN)

  // On iOS,
  //   TBD: https://crbug.com/714961
};

// Provides performance metrics for a specified process (CPU usage and IO
// counters). Use CreateCurrentProcessMetrics() to get an instance for the
// current process, or CreateProcessMetrics() to get an instance for an
// arbitrary process. Then, access the information with the different get
// methods.
//
// This class exposes a few platform-specific APIs for parsing memory usage, but
// these are not intended to generalize to other platforms, since the memory
// models differ substantially.
//
// To obtain consistent memory metrics, use the memory_instrumentation service.
//
// For further documentation on memory, see
// https://chromium.googlesource.com/chromium/src/+/HEAD/docs/README.md#Memory
class BASE_EXPORT ProcessMetrics {
 public:
  ProcessMetrics(const ProcessMetrics&) = delete;
  ProcessMetrics& operator=(const ProcessMetrics&) = delete;

  ~ProcessMetrics();

  // Creates a ProcessMetrics for the specified process.
#if !BUILDFLAG(IS_MAC)
  static std::unique_ptr<ProcessMetrics> CreateProcessMetrics(
      ProcessHandle process);
#else

  // The port provider needs to outlive the ProcessMetrics object returned by
  // this function. If NULL is passed as provider, the returned object
  // only returns valid metrics if |process| is the current process.
  static std::unique_ptr<ProcessMetrics> CreateProcessMetrics(
      ProcessHandle process,
      PortProvider* port_provider);
#endif  // !BUILDFLAG(IS_MAC)

  // Creates a ProcessMetrics for the current process. This a cross-platform
  // convenience wrapper for CreateProcessMetrics().
  static std::unique_ptr<ProcessMetrics> CreateCurrentProcessMetrics();

  // Provides synchronous access to memory metrics for a process. This interface
  // has platform-specific restrictions:
  //  * On Android, due to sandboxing restrictions, processes can only access
  //    memory metrics for themselves.
  //  * On Linux, due to sandboxing restrictions, only the privileged browser
  //    process has access to memory metrics for sandboxed child processes.
  //  * On Fuchsia, due to the API expecting a ProcessId rather than a
  //    ProcessHandle, processes can only access memory metrics for themselves
  //    or for children of base::GetDefaultJob().
  base::expected<ProcessMemoryInfo, ProcessUsageError> GetMemoryInfo() const;

  // Returns the percentage of time spent executing, across all threads of the
  // process, in the interval since the last time the method was called, using
  // the current |cumulative_cpu|. Since this considers the total execution time
  // across all threads in a process, the result can easily exceed 100% in
  // multi-thread processes running on multi-core systems. In general the result
  // is therefore a value in the range 0% to
  // SysInfo::NumberOfProcessors() * 100%.
  //
  // To obtain the percentage of total available CPU resources consumed by this
  // process over the interval, the caller must divide by NumberOfProcessors().
  //
  // Since this API measures usage over an interval, it will return zero on the
  // first call, and an actual value only on the second and subsequent calls.
  [[nodiscard]] double GetPlatformIndependentCPUUsage(TimeDelta cumulative_cpu);

  // Same as the above, but automatically calls GetCumulativeCPUUsage() to
  // determine the current cumulative CPU. Returns nullopt if
  // GetCumulativeCPUUsage() fails.
  base::expected<double, ProcessCPUUsageError> GetPlatformIndependentCPUUsage();

  // Returns the cumulative CPU usage across all threads of the process since
  // process start, or nullopt on error. In case of multi-core processors, a
  // process can consume CPU at a rate higher than wall-clock time, e.g. two
  // cores at full utilization will result in a time delta of 2 seconds/per 1
  // wall-clock second.
  base::expected<TimeDelta, ProcessCPUUsageError> GetCumulativeCPUUsage();

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_ANDROID) || \
    BUILDFLAG(IS_AIX)
  // Emits the cumulative CPU usage for all currently active threads since they
  // were started into the output parameter (replacing its current contents).
  // Threads that have already terminated will not be reported. Thus, the sum of
  // these times may not equal the value returned by GetCumulativeCPUUsage().
  // Returns false on failure. We return the usage via an output parameter to
  // allow reuse of CPUUsagePerThread's std::vector by the caller, e.g. to avoid
  // allocations between repeated calls to this method.
  // NOTE: Currently only supported on Linux/Android.
  using CPUUsagePerThread = std::vector<std::pair<PlatformThreadId, TimeDelta>>;
  bool GetCumulativeCPUUsagePerThread(CPUUsagePerThread&);
#endif  // BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) ||
        // BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_AIX)

  // Returns the number of average idle cpu wakeups per second since the last
  // call.
  int GetIdleWakeupsPerSecond();

#if BUILDFLAG(IS_APPLE)
  // Returns the number of average "package idle exits" per second, which have
  // a higher energy impact than a regular wakeup, since the last call.
  //
  // From the powermetrics man page:
  // "With the exception of some Mac Pro systems, Mac and
  // iOS systems are typically single package systems, wherein all CPUs are
  // part of a single processor complex (typically a single IC die) with shared
  // logic that can include (depending on system specifics) shared last level
  // caches, an integrated memory controller etc. When all CPUs in the package
  // are idle, the hardware can power-gate significant portions of the shared
  // logic in addition to each individual processor's logic, as well as take
  // measures such as placing DRAM in to self-refresh (also referred to as
  // auto-refresh), place interconnects into lower-power states etc"
  int GetPackageIdleWakeupsPerSecond();
#endif

#if BUILDFLAG(IS_POSIX)
  // Returns the number of file descriptors currently open by the process, or
  // -1 on error.
  int GetOpenFdCount() const;

  // Returns the soft limit of file descriptors that can be opened by the
  // process, or -1 on error.
  int GetOpenFdSoftLimit() const;
#endif  // BUILDFLAG(IS_POSIX)

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_ANDROID)
  // Minor and major page fault count as reported by /proc/[pid]/stat.
  // Returns true for success.
  bool GetPageFaultCounts(PageFaultCounts* counts) const;
#endif  // BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) ||
        // BUILDFLAG(IS_ANDROID)

  // Returns total memory usage of malloc.
  size_t GetMallocUsage();

 private:
#if !BUILDFLAG(IS_MAC)
  explicit ProcessMetrics(ProcessHandle process);
#else
  ProcessMetrics(ProcessHandle process, PortProvider* port_provider);
#endif  // !BUILDFLAG(IS_MAC)

#if BUILDFLAG(IS_APPLE) || BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || \
    BUILDFLAG(IS_AIX)
  int CalculateIdleWakeupsPerSecond(uint64_t absolute_idle_wakeups);
#endif
#if BUILDFLAG(IS_APPLE)
  // The subset of wakeups that cause a "package exit" can be tracked on macOS.
  // See |GetPackageIdleWakeupsForSecond| comment for more info.
  int CalculatePackageIdleWakeupsPerSecond(
      uint64_t absolute_package_idle_wakeups);

  // Queries the port provider if it's set.
  mach_port_t TaskForHandle(ProcessHandle process_handle) const;
#endif

#if BUILDFLAG(IS_WIN)
  win::ScopedHandle process_;
#else
  ProcessHandle process_;
#endif

  // Used to store the previous times and CPU usage counts so we can
  // compute the CPU usage between calls.
  TimeTicks last_cpu_time_;
#if !BUILDFLAG(IS_FREEBSD) || !BUILDFLAG(IS_POSIX)
  TimeDelta last_cumulative_cpu_;
#endif

#if BUILDFLAG(IS_APPLE) || BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || \
    BUILDFLAG(IS_AIX)
  // Same thing for idle wakeups.
  TimeTicks last_idle_wakeups_time_;
  uint64_t last_absolute_idle_wakeups_;
#endif

#if BUILDFLAG(IS_APPLE)
  // And same thing for package idle exit wakeups.
  TimeTicks last_package_idle_wakeups_time_;
  uint64_t last_absolute_package_idle_wakeups_;

  // Works around a race condition when combining two task_info() calls to
  // measure CPU time.
  TimeDelta last_measured_cpu_;
#endif

#if BUILDFLAG(IS_MAC)
  raw_ptr<PortProvider> port_provider_;
#endif  // BUILDFLAG(IS_MAC)
};

// Returns the memory committed by the system in KBytes.
// Returns 0 if it can't compute the commit charge.
BASE_EXPORT size_t GetSystemCommitCharge();

// Returns the maximum number of file descriptors that can be open by a process
// at once. If the number is unavailable, a conservative best guess is returned.
BASE_EXPORT size_t GetMaxFds();

// Returns the maximum number of handles that can be open at once per process.
BASE_EXPORT size_t GetHandleLimit();

#if BUILDFLAG(IS_POSIX)
// Increases the file descriptor soft limit to |max_descriptors| or the OS hard
// limit, whichever is lower. If the limit is already higher than
// |max_descriptors|, then nothing happens.
BASE_EXPORT void IncreaseFdLimitTo(unsigned int max_descriptors);
#endif  // BUILDFLAG(IS_POSIX)

#if BUILDFLAG(IS_WIN) || BUILDFLAG(IS_APPLE) || BUILDFLAG(IS_LINUX) ||      \
    BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_AIX) || \
    BUILDFLAG(IS_FUCHSIA)
// Data about system-wide memory consumption. Values are in KB. Available on
// Windows, Mac, Linux, Android and Chrome OS.
//
// Total memory are available on all platforms that implement
// GetSystemMemoryInfo(). Total/free swap memory are available on all platforms
// except on Mac. Buffers/cached/active_anon/inactive_anon/active_file/
// inactive_file/dirty/reclaimable/pswpin/pswpout/pgmajfault are available on
// Linux/Android/Chrome OS. Shmem/slab are Chrome OS only.
// Speculative/file_backed/purgeable are Mac and iOS only.
// Free is absent on Windows (see "avail_phys" below).
struct BASE_EXPORT SystemMemoryInfoKB {
  SystemMemoryInfoKB();
  SystemMemoryInfoKB(const SystemMemoryInfoKB& other);
  SystemMemoryInfoKB& operator=(const SystemMemoryInfoKB& other);

  // Serializes the platform specific fields to value.
  Value::Dict ToDict() const;

  int total = 0;

#if !BUILDFLAG(IS_WIN)
  int free = 0;
#endif

#if BUILDFLAG(IS_WIN)
  // "This is the amount of physical memory that can be immediately reused
  // without having to write its contents to disk first. It is the sum of the
  // size of the standby, free, and zero lists." (MSDN).
  // Standby: not modified pages of physical ram (file-backed memory) that are
  // not actively being used.
  int avail_phys = 0;
#endif

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_ANDROID) || \
    BUILDFLAG(IS_AIX)
  // This provides an estimate of available memory as described here:
  // https://git.kernel.org/cgit/linux/kernel/git/torvalds/linux.git/commit/?id=34e431b0ae398fc54ea69ff85ec700722c9da773
  // NOTE: this is ONLY valid in kernels 3.14 and up.  Its value will always
  // be 0 in earlier kernel versions.
  // Note: it includes _all_ file-backed memory (active + inactive).
  int available = 0;
#endif

#if !BUILDFLAG(IS_APPLE)
  int swap_total = 0;
  int swap_free = 0;
#endif

#if BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || \
    BUILDFLAG(IS_AIX) || BUILDFLAG(IS_FUCHSIA)
  int buffers = 0;
  int cached = 0;
  int active_anon = 0;
  int inactive_anon = 0;
  int active_file = 0;
  int inactive_file = 0;
  int dirty = 0;
  int reclaimable = 0;
#endif  // BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_LINUX) ||
        // BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_AIX) BUILDFLAG(IS_FUCHSIA)

#if BUILDFLAG(IS_CHROMEOS)
  int shmem = 0;
  int slab = 0;
#endif  // BUILDFLAG(IS_CHROMEOS)

#if BUILDFLAG(IS_APPLE)
  int speculative = 0;
  int file_backed = 0;
  int purgeable = 0;
#endif  // BUILDFLAG(IS_APPLE)
};

// On Linux/Android/Chrome OS, system-wide memory consumption data is parsed
// from /proc/meminfo and /proc/vmstat. On Windows/Mac, it is obtained using
// system API calls.
//
// Fills in the provided |meminfo| structure. Returns true on success.
// Exposed for memory debugging widget.
BASE_EXPORT bool GetSystemMemoryInfo(SystemMemoryInfoKB* meminfo);

#endif  // BUILDFLAG(IS_WIN) || BUILDFLAG(IS_APPLE) || BUILDFLAG(IS_LINUX) ||
        // BUILDFLAG(IS_CHROMEOS) BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_AIX) ||
        // BUILDFLAG(IS_FUCHSIA)

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_ANDROID) || \
    BUILDFLAG(IS_AIX)
// Parse the data found in /proc/<pid>/stat and return the sum of the
// CPU-related ticks.  Returns -1 on parse error.
// Exposed for testing.
BASE_EXPORT int ParseProcStatCPU(std::string_view input);

// Get the number of threads of |process| as available in /proc/<pid>/stat.
// This should be used with care as no synchronization with running threads is
// done. This is mostly useful to guarantee being single-threaded.
// Returns 0 on failure.
BASE_EXPORT int64_t GetNumberOfThreads(ProcessHandle process);

// /proc/self/exe refers to the current executable.
BASE_EXPORT extern const char kProcSelfExe[];

// Parses a string containing the contents of /proc/meminfo
// returns true on success or false for a parsing error
// Exposed for testing.
BASE_EXPORT bool ParseProcMeminfo(std::string_view input,
                                  SystemMemoryInfoKB* meminfo);

// Returns the memory committed by the system in KBytes, as from
// GetSystemCommitCharge(), using data from `meminfo` instead of /proc/meminfo.
// Exposed for testing.
BASE_EXPORT size_t
GetSystemCommitChargeFromMeminfo(const SystemMemoryInfoKB& meminfo);

// Data from /proc/vmstat.
struct BASE_EXPORT VmStatInfo {
  // Serializes the platform specific fields to value.
  Value::Dict ToDict() const;

  uint64_t pswpin = 0;
  uint64_t pswpout = 0;
  uint64_t pgmajfault = 0;
  uint64_t oom_kill = 0;
};

// Retrieves data from /proc/vmstat about system-wide vm operations.
// Fills in the provided |vmstat| structure. Returns true on success.
BASE_EXPORT bool GetVmStatInfo(VmStatInfo* vmstat);

// Parses a string containing the contents of /proc/vmstat
// returns true on success or false for a parsing error
// Exposed for testing.
BASE_EXPORT bool ParseProcVmstat(std::string_view input, VmStatInfo* vmstat);

// Data from /proc/diskstats about system-wide disk I/O.
struct BASE_EXPORT SystemDiskInfo {
  SystemDiskInfo();
  SystemDiskInfo(const SystemDiskInfo&);
  SystemDiskInfo& operator=(const SystemDiskInfo&);

  // Serializes the platform specific fields to value.
  Value::Dict ToDict() const;

  uint64_t reads = 0;
  uint64_t reads_merged = 0;
  uint64_t sectors_read = 0;
  uint64_t read_time = 0;
  uint64_t writes = 0;
  uint64_t writes_merged = 0;
  uint64_t sectors_written = 0;
  uint64_t write_time = 0;
  uint64_t io = 0;
  uint64_t io_time = 0;
  uint64_t weighted_io_time = 0;
};

// Checks whether the candidate string is a valid disk name, [hsv]d[a-z]+
// for a generic disk or mmcblk[0-9]+ for the MMC case.
// Names of disk partitions (e.g. sda1) are not valid.
BASE_EXPORT bool IsValidDiskName(std::string_view candidate);

// Retrieves data from /proc/diskstats about system-wide disk I/O.
// Fills in the provided |diskinfo| structure. Returns true on success.
BASE_EXPORT bool GetSystemDiskInfo(SystemDiskInfo* diskinfo);

// Returns the amount of time spent in user space since boot across all CPUs.
BASE_EXPORT TimeDelta GetUserCpuTimeSinceBoot();

#endif  // BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) ||
        // BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_AIX)

#if BUILDFLAG(IS_CHROMEOS)
// Data from files in directory /sys/block/zram0 about ZRAM usage.
struct BASE_EXPORT SwapInfo {
  SwapInfo()
      : num_reads(0),
        num_writes(0),
        compr_data_size(0),
        orig_data_size(0),
        mem_used_total(0) {}

  // Serializes the platform specific fields to value.
  Value::Dict ToDict() const;

  uint64_t num_reads = 0;
  uint64_t num_writes = 0;
  uint64_t compr_data_size = 0;
  uint64_t orig_data_size = 0;
  uint64_t mem_used_total = 0;
};

// Parses a string containing the contents of /sys/block/zram0/mm_stat.
// This should be used for the new ZRAM sysfs interfaces.
// Returns true on success or false for a parsing error.
// Exposed for testing.
BASE_EXPORT bool ParseZramMmStat(std::string_view mm_stat_data,
                                 SwapInfo* swap_info);

// Parses a string containing the contents of /sys/block/zram0/stat
// This should be used for the new ZRAM sysfs interfaces.
// Returns true on success or false for a parsing error.
// Exposed for testing.
BASE_EXPORT bool ParseZramStat(std::string_view stat_data, SwapInfo* swap_info);

// In ChromeOS, reads files from /sys/block/zram0 that contain ZRAM usage data.
// Fills in the provided |swap_data| structure.
// Returns true on success or false for a parsing error.
BASE_EXPORT bool GetSwapInfo(SwapInfo* swap_info);

// Data about GPU memory usage. These fields will be -1 if not supported.
struct BASE_EXPORT GraphicsMemoryInfoKB {
  // Serializes the platform specific fields to value.
  Value::Dict ToDict() const;

  int gpu_objects = -1;
  int64_t gpu_memory_size = -1;
};

// Report on Chrome OS graphics memory. Returns true on success.
// /run/debugfs_gpu is a bind mount into /sys/kernel/debug and synchronously
// reading the in-memory files in /sys is fast in most cases. On platform that
// reading the graphics memory info is slow, this function returns false.
BASE_EXPORT bool GetGraphicsMemoryInfo(GraphicsMemoryInfoKB* gpu_meminfo);

#endif  // BUILDFLAG(IS_CHROMEOS)

struct BASE_EXPORT SystemPerformanceInfo {
  SystemPerformanceInfo();
  SystemPerformanceInfo(const SystemPerformanceInfo& other);
  SystemPerformanceInfo& operator=(const SystemPerformanceInfo& other);

  // Serializes the platform specific fields to value.
  Value::Dict ToDict() const;

  // Total idle time of all processes in the system (units of 100 ns).
  uint64_t idle_time = 0;
  // Number of bytes read.
  uint64_t read_transfer_count = 0;
  // Number of bytes written.
  uint64_t write_transfer_count = 0;
  // Number of bytes transferred (e.g. DeviceIoControlFile)
  uint64_t other_transfer_count = 0;
  // The amount of read operations.
  uint64_t read_operation_count = 0;
  // The amount of write operations.
  uint64_t write_operation_count = 0;
  // The amount of other operations.
  uint64_t other_operation_count = 0;
  // The number of pages written to the system's pagefiles.
  uint64_t pagefile_pages_written = 0;
  // The number of write operations performed on the system's pagefiles.
  uint64_t pagefile_pages_write_ios = 0;
  // The number of pages of physical memory available to processes running on
  // the system.
  uint64_t available_pages = 0;
  // The number of pages read from disk to resolve page faults.
  uint64_t pages_read = 0;
  // The number of read operations initiated to resolve page faults.
  uint64_t page_read_ios = 0;
};

// Retrieves performance counters from the operating system.
// Fills in the provided |info| structure. Returns true on success.
BASE_EXPORT bool GetSystemPerformanceInfo(SystemPerformanceInfo* info);

// Collects and holds performance metrics for system memory and disk.
// Provides functionality to retrieve the data on various platforms and
// to serialize the stored data.
class BASE_EXPORT SystemMetrics {
 public:
  SystemMetrics();

  static SystemMetrics Sample();

  // Serializes the system metrics to value.
  Value::Dict ToDict() const;

 private:
  FRIEND_TEST_ALL_PREFIXES(SystemMetricsTest, SystemMetrics);

  size_t committed_memory_;
#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_ANDROID)
  SystemMemoryInfoKB memory_info_;
  VmStatInfo vmstat_info_;
  SystemDiskInfo disk_info_;
#endif
#if BUILDFLAG(IS_CHROMEOS)
  SwapInfo swap_info_;
  GraphicsMemoryInfoKB gpu_memory_info_;
#endif
#if BUILDFLAG(IS_WIN)
  SystemPerformanceInfo performance_;
#endif
};

#if BUILDFLAG(IS_APPLE)
enum class MachVMRegionResult {
  // There were no more memory regions between |address| and the end of the
  // virtual address space.
  Finished,

  // All output parameters are invalid.
  Error,

  // All output parameters are filled in.
  Success
};

// Returns info on the first memory region at or after |address|, including
// protection values. On Success, |size| reflects the size of the
// memory region.
// Returns info on the first memory region at or after |address|, including
// resident memory and share mode.
// |size| and |info| are output parameters, only valid on Success.
BASE_EXPORT MachVMRegionResult GetBasicInfo(mach_port_t task,
                                            mach_vm_size_t* size,
                                            mach_vm_address_t* address,
                                            vm_region_basic_info_64* info);

// Returns info on the first memory region at or after |address|, including
// resident memory and share mode. On Success, |size| reflects the size of the
// memory region.
// |size| and |info| are output parameters, only valid on Success.
// |address| is an in-out parameter, than represents both the address to start
// looking, and the start address of the memory region.
BASE_EXPORT MachVMRegionResult GetTopInfo(mach_port_t task,
                                          mach_vm_size_t* size,
                                          mach_vm_address_t* address,
                                          vm_region_top_info_data_t* info);
#endif  // BUILDFLAG(IS_APPLE)

}  // namespace base

#endif  // BASE_PROCESS_PROCESS_METRICS_H_
