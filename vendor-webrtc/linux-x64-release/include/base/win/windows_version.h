// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_WINDOWS_VERSION_H_
#define BASE_WIN_WINDOWS_VERSION_H_

#include <stddef.h>

#include <string>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/gtest_prod_util.h"
#include "base/version.h"

using DWORD = unsigned long;  // NOLINT(runtime/int)
using HANDLE = void*;
struct _OSVERSIONINFOEXW;
struct _SYSTEM_INFO;

namespace base {
namespace test {
class ScopedOSInfoOverride;
}  // namespace test
}  // namespace base

namespace base {
namespace win {

// The running version of Windows.  This is declared outside OSInfo for
// syntactic sugar reasons; see the declaration of GetVersion() below.
// NOTE: Keep these in order so callers can do things like
// "if (base::win::GetVersion() >= base::win::Version::VISTA) ...".
enum class Version {
  PRE_XP = 0,  // Not supported.
  XP = 1,
  SERVER_2003 = 2,   // Also includes XP Pro x64 and Server 2003 R2.
  VISTA = 3,         // Also includes Windows Server 2008.
  WIN7 = 4,          // Also includes Windows Server 2008 R2.
  WIN8 = 5,          // Also includes Windows Server 2012.
  WIN8_1 = 6,        // Also includes Windows Server 2012 R2.
  WIN10 = 7,         // Threshold 1: Version 1507, Build 10240.
  WIN10_TH2 = 8,     // Threshold 2: Version 1511, Build 10586.
  WIN10_RS1 = 9,     // Redstone 1: Version 1607, Build 14393.
                     // Also includes Windows Server 2016
  WIN10_RS2 = 10,    // Redstone 2: Version 1703, Build 15063.
  WIN10_RS3 = 11,    // Redstone 3: Version 1709, Build 16299.
  WIN10_RS4 = 12,    // Redstone 4: Version 1803, Build 17134.
  WIN10_RS5 = 13,    // Redstone 5: Version 1809, Build 17763.
                     // Also includes Windows Server 2019
  WIN10_19H1 = 14,   // 19H1: Version 1903, Build 18362.
  WIN10_19H2 = 15,   // 19H2: Version 1909, Build 18363.
  WIN10_20H1 = 16,   // 20H1: Build 19041.
  WIN10_20H2 = 17,   // 20H2: Build 19042.
  WIN10_21H1 = 18,   // 21H1: Build 19043.
  WIN10_21H2 = 19,   // Win10 21H2: Build 19044.
  WIN10_22H2 = 20,   // Win10 21H2: Build 19045.
  SERVER_2022 = 21,  // Server 2022: Build 20348.
  WIN11 = 22,        // Win11 21H2: Build 22000.
  WIN11_22H2 = 23,   // Win11 22H2: Build 22621.
  WIN11_23H2 = 24,   // Win11 23H2: Build 22631.
  WIN11_24H2 = 25,   // Win11 24H2: Build 26100.
  WIN_LAST,          // Indicates error condition.
};

// A rough bucketing of the available types of versions of Windows. This is used
// to distinguish enterprise enabled versions from home versions and potentially
// server versions. Keep these values in the same order, since they are used as
// is for metrics histogram ids.
enum VersionType {
  SUITE_HOME = 0,
  SUITE_PROFESSIONAL,
  SUITE_SERVER,
  SUITE_ENTERPRISE,
  SUITE_EDUCATION,
  SUITE_EDUCATION_PRO,
  SUITE_LAST,
};

// A singleton that can be used to query various pieces of information about the
// OS and process state. Note that this doesn't use the base Singleton class, so
// it can be used without an AtExitManager.
class BASE_EXPORT OSInfo {
 public:
  struct VersionNumber {
    uint32_t major;
    uint32_t minor;
    uint32_t build;
    uint32_t patch;
  };

  struct ServicePack {
    int major;
    int minor;
  };

  // The processor architecture this copy of Windows natively uses.  For
  // example, given an x64-capable processor, we have three possibilities:
  //   32-bit Chrome running on 32-bit Windows:           X86_ARCHITECTURE
  //   32-bit Chrome running on 64-bit Windows via WOW64: X64_ARCHITECTURE
  //   64-bit Chrome running on 64-bit Windows:           X64_ARCHITECTURE
  enum WindowsArchitecture {
    X86_ARCHITECTURE,
    X64_ARCHITECTURE,
    IA64_ARCHITECTURE,
    ARM64_ARCHITECTURE,
    OTHER_ARCHITECTURE,
  };

  static OSInfo* GetInstance();

  OSInfo(const OSInfo&) = delete;
  OSInfo& operator=(const OSInfo&) = delete;

  // Separate from the rest of OSInfo so they can be used during early process
  // initialization.
  static WindowsArchitecture GetArchitecture();
  // This is necessary because GetArchitecture doesn't return correct OS
  // architectures for x86/x64 binaries running on ARM64 - it says the OS is
  // x86/x64. This function returns true if the process is an x86 or x64 process
  // running emulated on ARM64.
  static bool IsRunningEmulatedOnArm64();

  // Returns the OS Version as returned from a call to GetVersionEx().
  const Version& version() const { return version_; }

  // Returns detailed version info containing major, minor, build and patch.
  const VersionNumber& version_number() const { return version_number_; }

  // The Kernel32* set of functions return the OS version as determined by a
  // call to VerQueryValue() on kernel32.dll. This avoids any running App Compat
  // shims from manipulating the version reported.
  static Version Kernel32Version();
  static VersionNumber Kernel32VersionNumber();
  static base::Version Kernel32BaseVersion();

  // These helper functions return information about common scenarios of
  // interest in regards to WOW emulation.
  bool IsWowDisabled() const;    // Chrome bitness matches OS bitness.
  bool IsWowX86OnAMD64() const;  // Chrome x86 on an AMD64 host machine.
  bool IsWowX86OnARM64() const;  // Chrome x86 on an ARM64 host machine.
  bool IsWowAMD64OnARM64()
      const;                     // Chrome AMD64 build on an ARM64 host machine.
  bool IsWowX86OnOther() const;  // Chrome x86 on some other x64 host machine.

  // Functions to determine Version Type (e.g. Enterprise/Home) and Service Pack
  // value. See above for definitions of these values.
  const VersionType& version_type() const LIFETIME_BOUND {
    return version_type_;
  }
  const ServicePack& service_pack() const LIFETIME_BOUND {
    return service_pack_;
  }
  const std::string& service_pack_str() const LIFETIME_BOUND {
    return service_pack_str_;
  }

  // Returns the number of processors on the system.
  const int& processors() const { return processors_; }

  // Returns the allocation granularity. See
  // https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/ns-sysinfoapi-system_info.
  const size_t& allocation_granularity() const {
    return allocation_granularity_;
  }

  // Processor info as read from registry.
  std::string processor_model_name();
  std::string processor_vendor_name();

  // Returns the "ReleaseId" (Windows 10 release number) from the registry.
  const std::string& release_id() const LIFETIME_BOUND { return release_id_; }

  // It returns true if the Windows SKU is N edition.
  bool IsWindowsNSku() const;

 private:
  friend class base::test::ScopedOSInfoOverride;
  FRIEND_TEST_ALL_PREFIXES(OSInfo, MajorMinorBuildToVersion);

  // This enum contains a variety of 32-bit process types that could be
  // running with consideration towards WOW64.
  enum class WowProcessMachine {
    kDisabled,  // Chrome bitness matches OS bitness.
    kX86,       // 32-bit (x86) Chrome.
    kARM32,     // 32-bit (arm32) Chrome.
    kOther,     // all other 32-bit Chrome.
    kUnknown,
  };

  // This enum contains a variety of 64-bit host machine architectures that
  // could be running with consideration towards WOW64.
  enum class WowNativeMachine {
    kARM64,  // 32-bit Chrome running on ARM64 Windows.
    kAMD64,  // 32-bit Chrome running on AMD64 Windows.
    kOther,  // 32-bit Chrome running on all other 64-bit Windows.
    kUnknown,
  };

  // This is separate from GetInstance() so that ScopedOSInfoOverride
  // can override it in tests.
  static OSInfo** GetInstanceStorage();

  OSInfo(const _OSVERSIONINFOEXW& version_info,
         const _SYSTEM_INFO& system_info,
         DWORD os_type);
  ~OSInfo();

  // Returns a Version value for a given OS version tuple.
  static Version MajorMinorBuildToVersion(uint32_t major,
                                          uint32_t minor,
                                          uint32_t build);

  // Returns the architecture of the process machine within the WOW emulator.
  WowProcessMachine GetWowProcessMachineArchitecture(const int process_machine);

  // Returns the architecture of the native (host) machine using the WOW
  // emulator.
  WowNativeMachine GetWowNativeMachineArchitecture(const int native_machine);

  void InitializeWowStatusValuesFromLegacyApi(HANDLE process_handle);

  void InitializeWowStatusValuesForProcess(HANDLE process_handle);

  Version version_;
  VersionNumber version_number_;
  VersionType version_type_;
  ServicePack service_pack_;

  // Represents the version of the OS associated to a release of
  // Windows 10. Each version may have different releases (such as patch
  // updates). This is the identifier of the release.
  // Example:
  //    Windows 10 Version 1809 (OS build 17763) has multiple releases
  //    (i.e. build 17763.1, build 17763.195, build 17763.379, ...).
  // See https://docs.microsoft.com/en-us/windows/windows-10/release-information
  // for more information.
  std::string release_id_;

  // A string, such as "Service Pack 3", that indicates the latest Service Pack
  // installed on the system. If no Service Pack has been installed, the string
  // is empty.
  std::string service_pack_str_;
  int processors_;
  size_t allocation_granularity_;
  WowProcessMachine wow_process_machine_;
  WowNativeMachine wow_native_machine_;
  std::string processor_model_name_;
  std::string processor_vendor_name_;
  DWORD os_type_;
};

// Because this is by far the most commonly-requested value from the above
// singleton, we add a global-scope accessor here as syntactic sugar.
BASE_EXPORT Version GetVersion();

}  // namespace win
}  // namespace base

#endif  // BASE_WIN_WINDOWS_VERSION_H_
