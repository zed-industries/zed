/*
 *  Copyright 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_ANDROID_NETWORK_MONITOR_H_
#define SDK_ANDROID_SRC_JNI_ANDROID_NETWORK_MONITOR_H_

#include <stdint.h>

#include <map>
#include <optional>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/field_trials_view.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "rtc_base/network_monitor.h"
#include "rtc_base/network_monitor_factory.h"
#include "rtc_base/string_utils.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"
#include "sdk/android/src/jni/jni_helpers.h"

namespace webrtc {
namespace test {
class AndroidNetworkMonitorTest;
}  // namespace test

namespace jni {

typedef int64_t NetworkHandle;

// c++ equivalent of java NetworkChangeDetector.ConnectionType.
enum NetworkType {
  NETWORK_UNKNOWN,
  NETWORK_ETHERNET,
  NETWORK_WIFI,
  NETWORK_5G,
  NETWORK_4G,
  NETWORK_3G,
  NETWORK_2G,
  NETWORK_UNKNOWN_CELLULAR,
  NETWORK_BLUETOOTH,
  NETWORK_VPN,
  NETWORK_NONE
};

// The information is collected from Android OS so that the native code can get
// the network type and handle (Android network ID) for each interface.
struct NetworkInformation {
  std::string interface_name;
  NetworkHandle handle;
  NetworkType type;
  NetworkType underlying_type_for_vpn;
  std::vector<IPAddress> ip_addresses;

  NetworkInformation();
  NetworkInformation(const NetworkInformation&);
  NetworkInformation(NetworkInformation&&);
  ~NetworkInformation();
  NetworkInformation& operator=(const NetworkInformation&);
  NetworkInformation& operator=(NetworkInformation&&);

  std::string ToString() const;
};

class AndroidNetworkMonitor : public NetworkMonitorInterface {
 public:
  AndroidNetworkMonitor(JNIEnv* env,
                        const JavaRef<jobject>& j_application_context,
                        const FieldTrialsView& field_trials);
  ~AndroidNetworkMonitor() override;

  // TODO(sakal): Remove once down stream dependencies have been updated.
  static void SetAndroidContext(JNIEnv* jni, jobject context) {}

  void Start() override;
  void Stop() override;

  // Does `this` NetworkMonitorInterface implement BindSocketToNetwork?
  // Only Android returns true.
  virtual bool SupportsBindSocketToNetwork() const override { return true; }

  NetworkBindingResult BindSocketToNetwork(int socket_fd,
                                           const IPAddress& address,
                                           absl::string_view if_name) override;

  InterfaceInfo GetInterfaceInfo(absl::string_view if_name) override;

  // Always expected to be called on the network thread.
  void SetNetworkInfos(const std::vector<NetworkInformation>& network_infos);

  void NotifyConnectionTypeChanged(JNIEnv* env,
                                   const JavaRef<jobject>& j_caller);
  void NotifyOfNetworkConnect(JNIEnv* env,
                              const JavaRef<jobject>& j_caller,
                              const JavaRef<jobject>& j_network_info);
  void NotifyOfNetworkDisconnect(JNIEnv* env,
                                 const JavaRef<jobject>& j_caller,
                                 jlong network_handle);
  void NotifyOfActiveNetworkList(JNIEnv* env,
                                 const JavaRef<jobject>& j_caller,
                                 const JavaRef<jobjectArray>& j_network_infos);
  void NotifyOfNetworkPreference(JNIEnv* env,
                                 const JavaRef<jobject>& j_caller,
                                 const JavaRef<jobject>& j_connection_type,
                                 jint preference);

  // Visible for testing.
  void OnNetworkConnected_n(const NetworkInformation& network_info);

  // Visible for testing.
  std::optional<NetworkHandle> FindNetworkHandleFromAddressOrName(
      const IPAddress& address,
      absl::string_view ifname) const;

 private:
  void reset();
  void OnNetworkDisconnected_n(NetworkHandle network_handle);
  void OnNetworkPreference_n(NetworkType type, NetworkPreference preference);

  NetworkPreference GetNetworkPreference(AdapterType) const;
  std::optional<NetworkHandle> FindNetworkHandleFromIfname(
      absl::string_view ifname) const;

  const int android_sdk_int_;
  ScopedJavaGlobalRef<jobject> j_application_context_;
  ScopedJavaGlobalRef<jobject> j_network_monitor_;
  Thread* const network_thread_;
  bool started_ RTC_GUARDED_BY(network_thread_) = false;
  std::map<std::string, NetworkHandle, AbslStringViewCmp>
      network_handle_by_if_name_ RTC_GUARDED_BY(network_thread_);
  std::map<IPAddress, NetworkHandle> network_handle_by_address_
      RTC_GUARDED_BY(network_thread_);
  std::map<NetworkHandle, NetworkInformation> network_info_by_handle_
      RTC_GUARDED_BY(network_thread_);
  std::map<AdapterType, NetworkPreference> network_preference_by_adapter_type_
      RTC_GUARDED_BY(network_thread_);
  bool find_network_handle_without_ipv6_temporary_part_
      RTC_GUARDED_BY(network_thread_) = false;
  bool surface_cellular_types_ RTC_GUARDED_BY(network_thread_) = false;

  // NOTE: if bind_using_ifname_ is TRUE
  // then the adapter name is used with substring matching as follows:
  // An adapater name repored by android as 'wlan0'
  // will be matched with 'v4-wlan0' ("v4-wlan0".find("wlan0") != npos).
  // This applies to adapter_type_by_name_, vpn_underlying_adapter_type_by_name_
  // and FindNetworkHandleFromIfname.
  bool bind_using_ifname_ RTC_GUARDED_BY(network_thread_) = true;

  // NOTE: disable_is_adapter_available_ is a kill switch for the impl.
  // of IsAdapterAvailable().
  bool disable_is_adapter_available_ RTC_GUARDED_BY(network_thread_) = false;

  scoped_refptr<PendingTaskSafetyFlag> safety_flag_
      RTC_PT_GUARDED_BY(network_thread_) = nullptr;

  const FieldTrialsView& field_trials_;

  friend class webrtc::test::AndroidNetworkMonitorTest;
};

class AndroidNetworkMonitorFactory : public NetworkMonitorFactory {
 public:
  // Deprecated. Pass in application context to this class.
  AndroidNetworkMonitorFactory();

  AndroidNetworkMonitorFactory(JNIEnv* env,
                               const JavaRef<jobject>& j_application_context);

  ~AndroidNetworkMonitorFactory() override;

  NetworkMonitorInterface* CreateNetworkMonitor(
      const FieldTrialsView& field_trials) override;

 private:
  ScopedJavaGlobalRef<jobject> j_application_context_;
};

}  // namespace jni
}  // namespace webrtc

// TODO(magjed): Remove once external clients are updated.
namespace webrtc_jni {

using webrtc::jni::AndroidNetworkMonitor;
using webrtc::jni::AndroidNetworkMonitorFactory;

}  // namespace webrtc_jni

#endif  // SDK_ANDROID_SRC_JNI_ANDROID_NETWORK_MONITOR_H_
