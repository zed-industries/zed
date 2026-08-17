// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_SYSTEM_SYS_INFO_H_
#define BASE_SYSTEM_SYS_INFO_H_

#include <stddef.h>
#include <stdint.h>

#include <compare>
#include <iosfwd>
#include <map>
#include <optional>
#include <string>
#include <string_view>

#include "base/base_export.h"
#include "base/functional/callback_forward.h"
#include "base/gtest_prod_util.h"
#include "base/metrics/field_trial_params.h"
#include "base/time/time.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_MAC)
#include "base/feature_list.h"
#endif

namespace base {

#if BUILDFLAG(IS_MAC)
// When enabled, NumberOfProcessors() returns the number of physical processors
// instead of the number of logical processors if CPU security mitigations are
// enabled for the current process.
BASE_EXPORT BASE_DECLARE_FEATURE(kNumberOfCoresWithCpuSecurityMitigation);
#endif

#if BUILDFLAG(IS_CHROMEOS)
// Strings for environment variables.
BASE_EXPORT extern const char kLsbReleaseKey[];
BASE_EXPORT extern const char kLsbReleaseTimeKey[];
#endif

namespace debug {
FORWARD_DECLARE_TEST(SystemMetricsTest, ParseMeminfo);
}

namespace test {
class ScopedAmountOfPhysicalMemoryOverride;
}

class FilePath;
struct SystemMemoryInfoKB;

class BASE_EXPORT SysInfo {
 public:
  // Returns the number of processors/cores available for the current
  // application. This is typically the number of logical cores installed on the
  // system, but could instead be the number of physical cores when
  // SetCpuSecurityMitigationsEnabled() has been invoked to indicate that CPU
  // security mitigations are enabled on Mac.
  // On some platforms this may cache the resulting value in its implementation,
  // e.g. on Linux/ChromeOS where this function cannot run in a sandbox and so
  // a cached value must be returned.
  static int NumberOfProcessors();

  // Returns the number of the most efficient logical processors for the current
  // application. This is typically e-cores on Intel hybrid architecture, or
  // LITTLE cores on ARM bit.LITTLE architecture.
  // Returns 0 on symmetric architecture or when it failed to recognize.
  // This function will cache the result value in its implementation.
  static int NumberOfEfficientProcessors();

  // Return the number of bytes of physical memory on the current machine.
  // If low-end device mode is manually enabled via command line flag, this
  // will return the lesser of the actual physical memory, or 512MB.
  static uint64_t AmountOfPhysicalMemory();

  // Return the number of bytes of current available physical memory on the
  // machine.
  // (The amount of memory that can be allocated without any significant
  // impact on the system. It can lead to freeing inactive file-backed
  // and/or speculative file-backed memory).
  static uint64_t AmountOfAvailablePhysicalMemory();

  // Return the number of bytes of virtual memory of this process. A return
  // value of zero means that there is no limit on the available virtual
  // memory.
  static uint64_t AmountOfVirtualMemory();

  // Return the number of megabytes of physical memory on the current machine.
  static int AmountOfPhysicalMemoryMB() {
    return static_cast<int>(AmountOfPhysicalMemory() / 1024 / 1024);
  }

  // Return the number of megabytes of available virtual memory, or zero if it
  // is unlimited.
  static int AmountOfVirtualMemoryMB() {
    return static_cast<int>(AmountOfVirtualMemory() / 1024 / 1024);
  }

  // Return the available disk space in bytes on the volume containing |path|,
  // or -1 on failure.
  static int64_t AmountOfFreeDiskSpace(const FilePath& path);

  // Return the total disk space in bytes on the volume containing |path|, or -1
  // on failure.
  static int64_t AmountOfTotalDiskSpace(const FilePath& path);

#if BUILDFLAG(IS_FUCHSIA)
  // Sets the total amount of disk space to report under the specified |path|.
  // If |bytes| is -ve then any existing entry for |path| is removed.
  static void SetAmountOfTotalDiskSpace(const FilePath& path, int64_t bytes);
#endif

  // Returns system uptime.
  static TimeDelta Uptime();

  // Returns a descriptive string for the current machine model or an empty
  // string if the machine model is unknown or an error occurred.
  // e.g. "MacPro1,1" on Mac, "iPhone9,3" on iOS or "Nexus 5" on Android. Only
  // implemented on macOS, iOS, Android, Chrome OS and Windows. This returns an
  // empty string on other platforms.
  //
  // For macOS, a useful reference of the resulting strings returned by this
  // function and their corresponding hardware can be found at
  // https://everymac.com/systems/by_capability/mac-specs-by-machine-model-machine-id.html
  //
  // For iOS, corresponding hardware can be found at
  // https://deviceatlas.com/resources/clientside/ios-hardware-identification
  static std::string HardwareModelName();

  // Returns the SOC manufacturer's name or an empty string if the manufacturer
  // is unknown or an error occurred.
  // e.g. "Google" on Pixel 8 Pro. Only implemented on Android, returns an
  // empty string on other platforms.
  static std::string SocManufacturer();

#if BUILDFLAG(IS_MAC)
  struct HardwareModelNameSplit {
    std::string category;
    int model = 0;
    int variant = 0;
  };
  // Hardware model names on the Mac are of the shape "Mac𝓍,𝓎" where the
  // prefix is the general category, the 𝓍 is the model, and the 𝓎 is the
  // variant. This function takes the hardware model name as returned by
  // HardwareModelName() above, and returns it split into its constituent parts.
  // Returns nullopt if the value cannot be parsed.
  //
  // /!\ WARNING
  //
  // This is NOT A USEFUL FUNCTION and SHOULD NOT BE USED. While the `model`
  // value does inform as to what generation of hardware it is within the
  // `category`, this is not useful in determining the capabilities of the
  // hardware. Instead of using the `model` value, check the actual capabilities
  // of the hardware to verify what it can do rather than relying on a hardware
  // model name. In addition, while the `category` value used to have meaning
  // and could be used to determine the type of hardware (e.g. desktop vs
  // laptop), in 2022 Apple started using the generic category of "Mac", thus
  // removing its usefulness when used alone. While the entire model string as
  // returned by HardwareModelName() above can be useful for identifying a
  // specific piece of equipment, splitting apart it is not useful.
  //
  // Do not add any further callers! When the aforementioned 2022-era hardware
  // is the minimum requirement for Chromium, remove this function and adjust
  // all callers appropriately.
  static std::optional<HardwareModelNameSplit> SplitHardwareModelNameDoNotUse(
      std::string_view name);
#endif

  struct HardwareInfo {
    std::string manufacturer;
    std::string model;
  };
  // Returns via |callback| a struct containing descriptive UTF-8 strings for
  // the current machine manufacturer and model, or empty strings if the
  // information is unknown or an error occurred. Implemented on Windows, macOS,
  // iOS, Linux, Chrome OS and Android.
  static void GetHardwareInfo(base::OnceCallback<void(HardwareInfo)> callback);

  // Returns the name of the host operating system.
  static std::string OperatingSystemName();

  // Returns the version of the host operating system.
  static std::string OperatingSystemVersion();

  // Retrieves detailed numeric values for the OS version.
  // DON'T USE THIS ON THE MAC OR WINDOWS to determine the current OS release
  // for OS version-specific feature checks and workarounds. If you must use an
  // OS version check instead of a feature check, use
  // base::mac::MacOSVersion()/MacOSMajorVersion() family from
  // base/mac/mac_util.h, or base::win::GetVersion() from
  // base/win/windows_version.h.
  static void OperatingSystemVersionNumbers(int32_t* major_version,
                                            int32_t* minor_version,
                                            int32_t* bugfix_version);

#if BUILDFLAG(IS_POSIX)
  // Struct containing the the kernel version number of the host operating
  // system.
  struct BASE_EXPORT KernelVersionNumber {
    // Queries the current kernel version number using uname and parses the
    // release string to construct the KernelVersionNumber struct. This does not
    // cache the result.
    static KernelVersionNumber Current();

    friend bool operator==(const KernelVersionNumber& v1,
                           const KernelVersionNumber& v2) = default;

    friend auto operator<=>(const KernelVersionNumber& v1,
                            const KernelVersionNumber& v2) = default;

    int32_t major = 0;
    int32_t minor = 0;
    int32_t bugfix = 0;
  };
#endif  // BUILDFLAG(IS_POSIX)

  // Returns the architecture of the running operating system.
  // Exact return value may differ across platforms.
  // e.g. a 32-bit x86 kernel on a 64-bit capable CPU will return "x86",
  //      whereas a x86-64 kernel on the same CPU will return "x86_64"
  static std::string OperatingSystemArchitecture();

  // Returns the architecture of the running process, which might be different
  // than the architecture returned by OperatingSystemArchitecture() (e.g.
  // macOS Rosetta, a 32-bit binary on a 64-bit OS, etc).
  // Will return one of: "x86", "x86_64", "ARM", "ARM_64", or an empty string if
  // none of the above.
  static std::string ProcessCPUArchitecture();

  // Returns the CPU model name of the system. If it can not be figured out,
  // an empty string is returned.
  // More detailed info can be obtained from base/cpu.h.
  static std::string CPUModelName();

  // Return the smallest amount of memory (in bytes) which the VM system will
  // allocate.
  static size_t VMAllocationGranularity();

#if BUILDFLAG(IS_CHROMEOS)
  // Set |value| and return true if LsbRelease contains information about |key|.
  static bool GetLsbReleaseValue(const std::string& key, std::string* value);

  // Convenience function for GetLsbReleaseValue("CHROMEOS_RELEASE_BOARD",...).
  // Returns "unknown" if CHROMEOS_RELEASE_BOARD is not set. Otherwise, returns
  // the full name of the board. Note that the returned value often differs
  // between developers' systems and devices that use official builds. E.g. for
  // a developer-built image, the function could return 'glimmer', while in an
  // official build, it may be something like 'glimmer-signed-mp-v4keys'.
  //
  // NOTE: Strings returned by this function should be treated as opaque values
  // within Chrome (e.g. for reporting metrics elsewhere). If you need to make
  // Chrome behave differently for different Chrome OS devices, either directly
  // check for the hardware feature that you care about (preferred) or add a
  // command-line flag to Chrome and pass it from session_manager (based on
  // whether a USE flag is set or not). See https://goo.gl/BbBkzg for more
  // details.
  static std::string GetLsbReleaseBoard();

  // Returns the creation time of /etc/lsb-release. (Used to get the date and
  // time of the Chrome OS build).
  static Time GetLsbReleaseTime();

  // Returns true when actually running in a Chrome OS environment.
  static bool IsRunningOnChromeOS();

  // Overrides |lsb_release| and |lsb_release_time|. Overrides cannot be nested.
  // Call ResetChromeOSVersionInfoForTest() to restore the previous values.
  // Prefer base::test::ScopedChromeOSVersionInfo to calling this function.
  static void SetChromeOSVersionInfoForTest(const std::string& lsb_release,
                                            const Time& lsb_release_time);

  // Undoes the function above.
  static void ResetChromeOSVersionInfoForTest();

  // Returns the kernel version of the host operating system.
  static std::string KernelVersion();

  // Crashes if running on Chrome OS non-test image. Use only for really
  // sensitive and risky use cases. Only works while running in verified mode,
  // this check an easily be bypassed in dev mode.
  static void CrashIfChromeOSNonTestImage();
#endif  // BUILDFLAG(IS_CHROMEOS)

#if BUILDFLAG(IS_ANDROID)
  // Returns the Android build's codename.
  static std::string GetAndroidBuildCodename();

  // Returns the Android build ID.
  static std::string GetAndroidBuildID();

  // Returns the Android hardware system property, equivalent to Java's
  // Build.HARDWARE.
  static std::string GetAndroidHardware();

  // Returns the Android hardware EGL system property.
  static std::string GetAndroidHardwareEGL();
#endif  // BUILDFLAG(IS_ANDROID)

#if BUILDFLAG(IS_IOS)
  // Returns the iOS build number string which is normally an alphanumeric
  // string like 12E456. This build number can differentiate between different
  // versions of iOS that may have the same major/minor/bugfix version numbers.
  // For example, iOS beta releases have the same version number but different
  // build number strings.
  static std::string GetIOSBuildNumber();

  // Overrides the hardware model name. The overridden value is used instead of
  // `StringSysctl({CTL_HW, HW_MACHINE})`. `name` should not be empty.
  static void OverrideHardwareModelName(std::string name);
#endif  // BUILDFLAG(IS_IOS)

  // Returns true for low-end devices that may require extreme tradeoffs,
  // including user-visible changes, for acceptable performance.
  // For general memory optimizations, consider |AmountOfPhysicalMemoryMB|.
  //
  // On Android this returns true when memory <= 1GB on Android O and later.
  // This is not the same as "low-memory".
  // On Desktop this returns true when memory <= 2GB.
  static bool IsLowEndDevice();

  // The same as IsLowEndDevice() except on Android.
  //
  // On Android this returns:
  //   true when IsLowEndDevice() returns true.
  //   true when the physical memory of the device is 4gb or 6gb and
  //             the feature: kPartialLowEndModeOnMidEndDevices() is enabled.
  static bool IsLowEndDeviceOrPartialLowEndModeEnabled();
  static bool IsLowEndDeviceOrPartialLowEndModeEnabled(
      const FeatureParam<bool>& param_for_exclusion);

#if BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_CHROMEOS)
  // Returns true for Android devices whose memory is X GB, considering
  // carveouts. The carveouts is memory reserved by the system, e.g.
  // for drivers, MTE, etc. It's very common for querying app to see
  // hundreds MBs less than actual physical memory installed on the system.
  // Addendum: This logic should also work for ChromeOS.
  static bool Is3GbDevice();
  static bool Is4GbDevice();
  static bool Is6GbDevice();
  // Returns true for Android devices whose memory is 4GB or 6GB, considering
  // carveouts.
  static bool Is4GbOr6GbDevice();
#endif  // BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_CHROMEOS)

#if BUILDFLAG(IS_MAC)
  // Indicates that CPU security mitigations are enabled for the current
  // process. This is used to control the behavior of NumberOfProcessors(), see
  // comment on that method.
  static void SetCpuSecurityMitigationsEnabled();

  // Resets all state associated with CPU security mitigations.
  static void ResetCpuSecurityMitigationsEnabledForTesting();
#endif

 private:
  friend class test::ScopedAmountOfPhysicalMemoryOverride;
  FRIEND_TEST_ALL_PREFIXES(SysInfoTest, AmountOfAvailablePhysicalMemory);
  FRIEND_TEST_ALL_PREFIXES(debug::SystemMetricsTest, ParseMeminfo);

  static int NumberOfEfficientProcessorsImpl();
  static uint64_t AmountOfPhysicalMemoryImpl();
  static uint64_t AmountOfAvailablePhysicalMemoryImpl();
  static bool IsLowEndDeviceImpl();
  static HardwareInfo GetHardwareInfoSync();

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_ANDROID) || \
    BUILDFLAG(IS_AIX)
  static uint64_t AmountOfAvailablePhysicalMemory(
      const SystemMemoryInfoKB& meminfo);
#endif

  // Sets the amount of physical memory in MB for testing, thus allowing tests
  // to run irrespective of the host machine's configuration.
  static std::optional<uint64_t> SetAmountOfPhysicalMemoryMbForTesting(
      uint64_t amount_of_memory_mb);
  static void ClearAmountOfPhysicalMemoryMbForTesting();
};

#if BUILDFLAG(IS_POSIX)
// Stream operator so that SysInfo::KernelVersionNumber can be logged with a
// consistent format.
BASE_EXPORT std::ostream& operator<<(std::ostream& out,
                                     const SysInfo::KernelVersionNumber& v);
#endif  // BUILDFLAG(IS_POSIX)

}  // namespace base

#endif  // BASE_SYSTEM_SYS_INFO_H_
