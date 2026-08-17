// Copyright 2010 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// WMI (Windows Management and Instrumentation) is a big, complex, COM-based
// API that can be used to perform all sorts of things. Sometimes is the best
// way to accomplish something under windows but its lack of an approachable
// C++ interface prevents its use. This collection of functions is a step in
// that direction.
// There are two classes; WMIUtil and WMIProcessUtil. The first
// one contains generic helpers and the second one contains the only
// functionality that is needed right now which is to use WMI to launch a
// process.
// To use any function on this header you must call CoInitialize or
// CoInitializeEx beforehand.
//
// For more information about WMI programming:
// https://docs.microsoft.com/en-us/windows/win32/wmisdk

#ifndef BASE_WIN_WMI_H_
#define BASE_WIN_WMI_H_

#include <wrl/client.h>

#include <optional>
#include <string>
#include <string_view>

#include "base/base_export.h"
#include "base/win/wbemidl_shim.h"

namespace base {
namespace win {

// Enumeration of errors that can arise when connecting to a WMI server and
// running a query.
// Do not change ordering. This enum is captured as `WmiQueryError` in
// enums.xml.
enum class WmiError {
  kFailedToCreateInstance = 0,
  kFailedToConnectToWMI = 1,
  kFailedToSetSecurityBlanket = 2,
  kFailedToExecWMIQuery = 3,
  kMaxValue = kFailedToExecWMIQuery
};

// String used to connect to the CIMV2 WMI server.
BASE_EXPORT extern const wchar_t kCimV2ServerName[];

// String used to connect to the SecurityCenter2 WMI server.
BASE_EXPORT extern const wchar_t kSecurityCenter2ServerName[];

// Connects to a server named `server_name` on the local computer through COM
// and run the given WQL `query`. Sets `enumerator` with the values returned by
// that `query`. Will return a WmiError value if an error occurs, else returns
// std::nullopt.
BASE_EXPORT std::optional<WmiError> RunWmiQuery(
    const std::wstring& server_name,
    const std::wstring& query,
    Microsoft::WRL::ComPtr<IEnumWbemClassObject>* enumerator);

// Creates an instance of the WMI service connected to the local computer and
// returns its COM interface. If |set_blanket| is set to true, the basic COM
// security blanket is applied to the returned interface. This is almost
// always desirable unless you set the parameter to false and apply a custom
// COM security blanket.
// Returns true if succeeded and |wmi_services|: the pointer to the service.
BASE_EXPORT bool CreateLocalWmiConnection(
    bool set_blanket,
    Microsoft::WRL::ComPtr<IWbemServices>* wmi_services);

// Creates an instance of the WMI service connected to the resource and
// returns its COM interface. If |set_blanket| is set to true, the basic COM
// security blanket is applied to the returned interface. This is almost
// always desirable unless you set the parameter to false and apply a custom
// COM security blanket.
// Returns a valid ComPtr<IWbemServices> on success, nullptr on failure.
BASE_EXPORT Microsoft::WRL::ComPtr<IWbemServices> CreateWmiConnection(
    bool set_blanket,
    const std::wstring& resource);

// Creates a WMI method using from a WMI class named |class_name| that
// contains a method named |method_name|. Only WMI classes that are CIM
// classes can be created using this function.
// Returns true if succeeded and |class_instance| returns a pointer to the
// WMI method that you can fill with parameter values using SetParameter.
BASE_EXPORT bool CreateWmiClassMethodObject(
    IWbemServices* wmi_services,
    std::wstring_view class_name,
    std::wstring_view method_name,
    Microsoft::WRL::ComPtr<IWbemClassObject>* class_instance);

// Creates a new process from |command_line|. The advantage over CreateProcess
// is that it allows you to always break out from a Job object that the caller
// is attached to even if the Job object flags prevent that.
// Returns true and the process id in process_id if the process is launched
// successful. False otherwise.
// Note that a fully qualified path must be specified in most cases unless
// the program is not in the search path of winmgmt.exe.
// Processes created this way are children of wmiprvse.exe and run with the
// caller credentials.
// More info: http://msdn2.microsoft.com/en-us/library/aa394372(VS.85).aspx
BASE_EXPORT bool WmiLaunchProcess(const std::wstring& command_line,
                                  int* process_id);

// An encapsulation of information retrieved from the 'Win32_ComputerSystem' and
// 'Win32_Bios' WMI classes; see :
// https://docs.microsoft.com/en-us/windows/desktop/CIMWin32Prov/win32-computersystem
// https://docs.microsoft.com/en-us/windows/desktop/CIMWin32Prov/win32-systembios
// Note that while model and manufacturer can be obtained through WMI, it is
// more efficient to obtain them via SysInfo::GetHardwareInfo() which uses the
// registry.
class BASE_EXPORT WmiComputerSystemInfo {
 public:
  static WmiComputerSystemInfo Get();

  const std::wstring& serial_number() const { return serial_number_; }

 private:
  void PopulateSerialNumber(
      const Microsoft::WRL::ComPtr<IEnumWbemClassObject>& enumerator_bios);

  std::wstring serial_number_;
};

}  // namespace win
}  // namespace base

#endif  // BASE_WIN_WMI_H_
