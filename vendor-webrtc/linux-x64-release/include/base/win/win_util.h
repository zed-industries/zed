// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// =============================================================================
// PLEASE READ
//
// In general, you should not be adding stuff to this file.
//
// - If your thing is only used in one place, just put it in a reasonable
//   location in or near that one place. It's nice you want people to be able
//   to re-use your function, but realistically, if it hasn't been necessary
//   before after so many years of development, it's probably not going to be
//   used in other places in the future unless you know of them now.
//
// - If your thing is used by multiple callers and is UI-related, it should
//   probably be in app/win/ instead. Try to put it in the most specific file
//   possible (avoiding the *_util files when practical).
//
// =============================================================================

#ifndef BASE_WIN_WIN_UTIL_H_
#define BASE_WIN_WIN_UTIL_H_

#include <stdint.h>

#include <optional>
#include <string>
#include <string_view>
#include <vector>

#include "base/auto_reset.h"
#include "base/base_export.h"
#include "base/functional/callback_forward.h"
#include "base/strings/cstring_view.h"
#include "base/types/expected.h"
#include "base/win/scoped_handle.h"
#include "base/win/windows_types.h"

struct IPropertyStore;
struct _tagpropertykey;
using PROPERTYKEY = _tagpropertykey;
struct tagPOINTER_DEVICE_INFO;
using POINTER_DEVICE_INFO = tagPOINTER_DEVICE_INFO;

namespace base {

struct NativeLibraryLoadError;

namespace win {

// Returns the string representing the current user sid. Does not modify
// |user_sid| on failure.
BASE_EXPORT bool GetUserSidString(std::wstring* user_sid);

// Returns false if user account control (UAC) has been disabled with the
// EnableLUA registry flag. Returns true if user account control is enabled.
// NOTE: The EnableLUA registry flag, which is ignored on Windows XP
// machines, might still exist and be set to 0 (UAC disabled), in which case
// this function will return false. You should therefore check this flag only
// if the OS is Vista or later.
BASE_EXPORT bool UserAccountControlIsEnabled();

// Sets the boolean value for a given key in given IPropertyStore.
BASE_EXPORT bool SetBooleanValueForPropertyStore(
    IPropertyStore* property_store,
    const PROPERTYKEY& property_key,
    bool property_bool_value);

// Sets the string value for a given key in given IPropertyStore.
BASE_EXPORT bool SetStringValueForPropertyStore(
    IPropertyStore* property_store,
    const PROPERTYKEY& property_key,
    const wchar_t* property_string_value);

// Sets the CLSID value for a given key in a given IPropertyStore.
BASE_EXPORT bool SetClsidForPropertyStore(IPropertyStore* property_store,
                                          const PROPERTYKEY& property_key,
                                          const CLSID& property_clsid_value);

// Sets the application id in given IPropertyStore. The function is used to tag
// application/Chrome shortcuts, and set app details for Chrome windows.
BASE_EXPORT bool SetAppIdForPropertyStore(IPropertyStore* property_store,
                                          const wchar_t* app_id);

// Adds the specified |command| using the specified |name| to the AutoRun key.
// |root_key| could be HKCU or HKLM or the root of any user hive.
BASE_EXPORT bool AddCommandToAutoRun(HKEY root_key,
                                     const std::wstring& name,
                                     const std::wstring& command);
// Removes the command specified by |name| from the AutoRun key. |root_key|
// could be HKCU or HKLM or the root of any user hive.
BASE_EXPORT bool RemoveCommandFromAutoRun(HKEY root_key,
                                          const std::wstring& name);

// Reads the command specified by |name| from the AutoRun key. |root_key|
// could be HKCU or HKLM or the root of any user hive. Used for unit-tests.
BASE_EXPORT bool ReadCommandFromAutoRun(HKEY root_key,
                                        const std::wstring& name,
                                        std::wstring* command);

// Sets whether to crash the process during exit. This is inspected by DLLMain
// and used to intercept unexpected terminations of the process (via calls to
// exit(), abort(), _exit(), ExitProcess()) and convert them into crashes.
// Note that not all mechanisms for terminating the process are covered by
// this. In particular, TerminateProcess() is not caught.
BASE_EXPORT void SetShouldCrashOnProcessDetach(bool crash);
BASE_EXPORT bool ShouldCrashOnProcessDetach();

// Adjusts the abort behavior so that crash reports can be generated when the
// process is aborted.
BASE_EXPORT void SetAbortBehaviorForCrashReporting();

// Checks whether the supplied `hwnd` is in Windows 10 tablet mode. Will return
// false on versions below 10. This function is deprecated; all new code should
// use `IsDeviceInTabletMode()` and ensure it can support async content.
BASE_EXPORT bool IsWindows10TabletMode(HWND hwnd);

// Checks whether a device is in tablet mode and runs a callback that takes a
// bit that represents whether the device is in tablet mode. Use this function
// for accurate results on all platforms. A device is considered to be in tablet
// mode when the internal display is on and not in extend mode, in addition to
// being undocked.
BASE_EXPORT void IsDeviceInTabletMode(HWND hwnd,
                                      OnceCallback<void(bool)> callback);

// The device convertibility functions below return references to cached data
// to allow for complete test scenarios. See:
// ScopedDeviceConvertibilityStateForTesting.
//
// Returns a reference to a cached value computed on first-use that is true only
// if the device is a tablet, convertible, or detachable according to
// RtlGetDeviceFamilyInfoEnum. Looks for the following values: Tablet(2),
// Convertible(5), or Detachable(6).
// https://learn.microsoft.com/en-us/windows-hardware/customize/desktop/unattend/microsoft-windows-deployment-deviceform
BASE_EXPORT bool& IsDeviceFormConvertible();

// Returns a reference to a cached boolean that is true if the device hardware
// is convertible. The value is determined via a WMI query for
// Win32_SystemEnclosure. This should only be executed for a small amount of
// devices that don't have ConvertibleChassis or ConvertibilityEnabled keys set.
// https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-systemenclosure
BASE_EXPORT bool& IsChassisConvertible();

// Returns a reference to a cached boolean optional. If a value exists, it means
// that the queried registry key, ConvertibilityEnabled, exists. Used by Surface
// for devices that can't set deviceForm or ChassisType. The RegKey need not
// exist, but if it does it will override other checks.
BASE_EXPORT std::optional<bool>& GetConvertibilityEnabledOverride();

// Returns a reference to a cached boolean optional. If a value exists, it means
// that the queried registry key, ConvertibleChassis, exists. Windows may cache
// the results of convertible chassis queries, preventing the need for running
// the expensive WMI query. This should always be checked prior to running
// `IsChassisConvertible()`.
BASE_EXPORT std::optional<bool>& GetConvertibleChassisKeyValue();

// Returns a function pointer that points to the lambda function that
// tracks if the device's convertible slate mode state has ever changed, which
// would indicate that proper GPIO drivers are available for a convertible
// machine. A pointer is used so that the function at the address can be
// replaced for testing purposes.
bool (*&HasCSMStateChanged(void))();

// Returns true if the device can be converted between table and desktop modes.
// This function may make blocking calls to system facilities to make this
// determination. As such, it must not be called from contexts that disallow
// blocking (i.e., the UI thread). The steps to determine the convertibility are
// based on the following publication:
// https://learn.microsoft.com/en-us/windows-hardware/customize/desktop/settings-for-better-tablet-experiences?source=recommendations
BASE_EXPORT bool QueryDeviceConvertibility();

// Return true if the device is physically used as a tablet independently of
// Windows tablet mode. It checks if the device:
// - Is running Windows 8 or newer,
// - Has a touch digitizer,
// - Is not docked,
// - Has a supported rotation sensor,
// - Is not in laptop mode,
// - prefers the mobile or slate power management profile (per OEM choice), and
// - Is in slate mode.
// This function optionally sets the `reason` parameter to determine as to why
// or why not a device was deemed to be a tablet.
BASE_EXPORT bool IsDeviceUsedAsATablet(std::string* reason);

// Executes `callback` that takes as arguments, a bit that indicates whether
// a keyboard is detected along with a reason string ptr that will be set to to
// the detection method that was used to detect the keyboard.
BASE_EXPORT void IsDeviceSlateWithKeyboard(
    HWND hwnd,
    OnceCallback<void(bool, std::string)> callback);

// Get the size of a struct up to and including the specified member.
// This is necessary to set compatible struct sizes for different versions
// of certain Windows APIs (e.g. SystemParametersInfo).
#define SIZEOF_STRUCT_WITH_SPECIFIED_LAST_MEMBER(struct_name, member) \
  offsetof(struct_name, member) +                                     \
      (sizeof static_cast<struct_name*>(NULL)->member)

// Returns true if the machine is enrolled to a domain.
BASE_EXPORT bool IsEnrolledToDomain();

// Returns true if either the device is joined to Azure Active Directory (AD) or
// one or more Azure AD work accounts have been added on the device. This call
// trigger some I/O when loading netapi32.dll to determine the management state.
BASE_EXPORT bool IsJoinedToAzureAD();

// Returns true if the machine is being managed by an MDM system.
BASE_EXPORT bool IsDeviceRegisteredWithManagement();

// Returns true if the current process can make USER32 or GDI32 calls such as
// CreateWindow and CreateDC. Windows 8 and above allow the kernel component
// of these calls to be disabled (also known as win32k lockdown) which can
// cause undefined behaviour such as crashes. This function can be used to
// guard areas of code using these calls and provide a fallback path if
// necessary.
// Because they are not always needed (and not needed at all in processes that
// have the win32k lockdown), USER32 and GDI32 are delayloaded. Attempts to
// load them in those processes will cause a crash. Any code which uses USER32
// or GDI32 and may run in a locked-down process MUST be guarded using this
// method. Before the dlls were delayloaded, method calls into USER32 and GDI32
// did not work, so adding calls to this method to guard them simply avoids
// unnecessary method calls.
BASE_EXPORT bool IsUser32AndGdi32Available();

// Takes a snapshot of the modules loaded in the |process|. The returned
// HMODULEs are not add-ref'd, so they should not be closed and may be
// invalidated at any time (should a module be unloaded). |process| requires
// the PROCESS_QUERY_INFORMATION and PROCESS_VM_READ permissions.
BASE_EXPORT bool GetLoadedModulesSnapshot(HANDLE process,
                                          std::vector<HMODULE>* snapshot);

// Adds or removes the MICROSOFT_TABLETPENSERVICE_PROPERTY property with the
// TABLET_DISABLE_FLICKS & TABLET_DISABLE_FLICKFALLBACKKEYS flags in order to
// disable pen flick gestures for the given HWND.
BASE_EXPORT void EnableFlicks(HWND hwnd);
BASE_EXPORT void DisableFlicks(HWND hwnd);

// Enable high-DPI support for the current process.
BASE_EXPORT void EnableHighDPISupport();

// Returns a string representation of |rguid|.
BASE_EXPORT std::wstring WStringFromGUID(const ::GUID& rguid);

// Attempts to pin user32.dll to ensure it remains loaded. If it isn't loaded
// yet, the module will first be loaded and then the pin will be attempted. If
// pinning is successful, returns true. If the module cannot be loaded and/or
// pinned, |error| is set and the method returns false.
BASE_EXPORT bool PinUser32(NativeLibraryLoadError* error = nullptr);

// Gets a pointer to a function within user32.dll, if available. If user32.dll
// cannot be loaded or the function cannot be found, this function returns
// nullptr and sets |error|. Once loaded, user32.dll is pinned, and therefore
// the function pointer returned by this function will never change and can be
// cached.
BASE_EXPORT void* GetUser32FunctionPointer(
    const char* function_name,
    NativeLibraryLoadError* error = nullptr);

// Returns the name of a desktop or a window station.
BASE_EXPORT std::wstring GetWindowObjectName(HANDLE handle);

// Gets information about the pointer device. When successful, updates `result`
// and returns true, otherwise returns false without modifying `result`.
// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getpointerdevice
BASE_EXPORT bool GetPointerDevice(HANDLE device, POINTER_DEVICE_INFO& result);

// Gets information about the pointer devices attached to the system.
// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getpointerdevices
BASE_EXPORT std::optional<std::vector<POINTER_DEVICE_INFO>> GetPointerDevices();

// Registers a window to process the WM_POINTERDEVICECHANGE event, and
// optionally WM_POINTERDEVICEINRANGE and WM_POINTERDEVICEOUTOFRANGE events when
// `notify_proximity_changes` is set.
// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerpointerdevicenotifications
BASE_EXPORT bool RegisterPointerDeviceNotifications(
    HWND hwnd,
    bool notify_proximity_changes = false);

// Checks if the calling thread is running under a desktop with the name
// given by |desktop_name|. |desktop_name| is ASCII case insensitive (non-ASCII
// characters will be compared with exact matches).
BASE_EXPORT bool IsRunningUnderDesktopName(std::wstring_view desktop_name);

// Returns true if current session is a remote session.
BASE_EXPORT bool IsCurrentSessionRemote();

// IsAppVerifierLoaded() indicates whether Application Verifier is *already*
// loaded into the current process.
BASE_EXPORT bool IsAppVerifierLoaded();

// Replaces the name of each environment variable embedded in the specified
// string with the string equivalent of the value of the variable, then returns
// the resulting string.
//
// The implementation calls the `ExpandEnvironmentStrings` WinAPI, meaning:
// * Each %variableName% portion is replaced with the current value of that
//   environment variable.
// * Case is ignored when looking up the environment-variable name.
// * If the name is not found, the %variableName% portion is left unexpanded.
//
// If `ExpandEnvironmentStrings` fails, `std::nullopt` is returned.
BASE_EXPORT std::optional<std::wstring> ExpandEnvironmentVariables(
    wcstring_view str);

// Returns the name of the type of object referenced by `handle` (e.g.,
// "Process" or "Section"), or an error code. This function will fail with
// STATUS_INVALID_HANDLE if called with the pseudo handle returned by
// `::GetCurrentProcess()` or `GetCurrentProcessHandle()`.
BASE_EXPORT expected<std::wstring, NTSTATUS> GetObjectTypeName(HANDLE handle);

// Process Power Throttling APIs are only available on Windows 11. By default,
// Windows will throttle processes based on various heuristics (power plan,
// media playback state, MMCSS apis, app visibility, etc). This can result in
// the process set to a lower Quality of Service (QoS) as well as having
// requests for high resolution timers ignored. The purpose is to provide
// improved performance and battery life, but can lead to unwanted regressions
// in some scenarios. It is important to note that such settings get applied to
// child processes as well. Callers can explicitly tell the OS to enable or
// disable throttling for specific processes with the SetProcessInformation API
// and the PROCESS_POWER_THROTTLING_STATE structure.
// https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessinformation
// Before Win11 22H2 there was no way to query the current state using
// GetProcessInformation. This is needed in Process::GetPriority to accurately
// determine the current priority. Calls made to set the process power
// throttling state before 22H2 are a no-op.
enum class ProcessPowerState { kUnset, kDisabled, kEnabled };

// Returns the current state of the process power speed throttling. Returns
// kEnabled if the process is explicitly set to EcoQoS. Returns kDisabled if the
// process is explicitly set to HighQoS. Returns kUnset if the setting is not
// explicitly set and therefore the OS decides the process power speed
// throttling state.
BASE_EXPORT ProcessPowerState GetProcessEcoQoSState(HANDLE process);

// Sets the state of the process power speed throttling. State set to kEnabled
// explicitly sets the process to EcoQoS. State set to kDisabled explicitly sets
// the process to HighQoS. State set to kUnset results in the OS deciding the
// throttling state. Returns true if the state was successfully set, false
// otherwise. Calls made to SetProcessEcoQoSState before 22H2 are a no-op and
// return false.
BASE_EXPORT bool SetProcessEcoQoSState(HANDLE process, ProcessPowerState state);

// Returns the state of the process power timer resolution throttling. Returns
// kEnabled if the process is explicitly set to ignore requests for high
// resolution timers. Returns kDisabled if the process is explicitly set to not
// ignore requests for high resolution timers. Returns kUnset if the setting is
// not expliclity set and therefore the OS decides the throttling state.
BASE_EXPORT ProcessPowerState GetProcessTimerThrottleState(HANDLE process);

// Sets the state of the process power timer resolution throttling.
// State set to kEnabled explicitly sets the process to ignore requests for high
// resolution timers. State set to kDisabled explicitly sets the process to
// allow requests for high resolution timers. State set to kUnset results in the
// OS deciding the throttling state. Returns true if the state was successfully
// set, false otherwise.  Calls made to SetProcessTimerThrottleState before 22H2
// are a no-op and return false.
BASE_EXPORT bool SetProcessTimerThrottleState(HANDLE process,
                                              ProcessPowerState state);

// Returns the serial number of the device.  Needs to be called from a COM
// enabled thread.
BASE_EXPORT std::optional<std::wstring> GetSerialNumber();

// Allows changing the domain enrolled state for the life time of the object.
// The original state is restored upon destruction.
class BASE_EXPORT ScopedDomainStateForTesting {
 public:
  explicit ScopedDomainStateForTesting(bool state);

  ScopedDomainStateForTesting(const ScopedDomainStateForTesting&) = delete;
  ScopedDomainStateForTesting& operator=(const ScopedDomainStateForTesting&) =
      delete;

  ~ScopedDomainStateForTesting();

 private:
  bool initial_state_;
};

// Allows changing the Azure Active Directory join state for the lifetime of the
// object. The original state is restored upon destruction.
class BASE_EXPORT ScopedAzureADJoinStateForTesting {
 public:
  explicit ScopedAzureADJoinStateForTesting(bool state);
  ScopedAzureADJoinStateForTesting(const ScopedAzureADJoinStateForTesting&) =
      delete;
  ScopedAzureADJoinStateForTesting& operator=(
      const ScopedAzureADJoinStateForTesting&) = delete;
  ~ScopedAzureADJoinStateForTesting();

 private:
  const bool initial_state_;
};

// Allows changing the return values of convertibility functions for the
// lifetime of the object. The original state is restored upon destruction.
class BASE_EXPORT
    [[maybe_unused, nodiscard]] ScopedDeviceConvertibilityStateForTesting {
 public:
  using QueryFunction = bool (*)();
  ScopedDeviceConvertibilityStateForTesting(
      bool form_convertible,
      bool chassis_convertible,
      QueryFunction csm_changed,
      std::optional<bool> convertible_chassis_key,
      std::optional<bool> convertibility_enabled);
  ScopedDeviceConvertibilityStateForTesting(
      const ScopedDeviceConvertibilityStateForTesting&) = delete;
  ScopedDeviceConvertibilityStateForTesting& operator=(
      const ScopedDeviceConvertibilityStateForTesting&) = delete;
  ~ScopedDeviceConvertibilityStateForTesting();

 private:
  AutoReset<bool> initial_form_convertible_;
  AutoReset<bool> initial_chassis_convertible_;
  AutoReset<QueryFunction> initial_csm_changed_;
  AutoReset<std::optional<bool>> initial_convertible_chassis_key_;
  AutoReset<std::optional<bool>> initial_convertibility_enabled_;
};

}  // namespace win
}  // namespace base

#endif  // BASE_WIN_WIN_UTIL_H_
