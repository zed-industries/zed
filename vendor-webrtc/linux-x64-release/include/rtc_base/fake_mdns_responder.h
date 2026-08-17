/*
 *  Copyright 2018 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_FAKE_MDNS_RESPONDER_H_
#define RTC_BASE_FAKE_MDNS_RESPONDER_H_

#include <map>
#include <memory>
#include <string>

#include "absl/strings/string_view.h"
#include "rtc_base/ip_address.h"
#include "rtc_base/mdns_responder_interface.h"
#include "rtc_base/thread.h"

namespace webrtc {

// This class posts tasks on the given `thread` to invoke callbacks. It's the
// callback's responsibility to be aware of potential destruction of state it
// depends on, e.g., using WeakPtrFactory or PendingTaskSafetyFlag.
class FakeMdnsResponder : public MdnsResponderInterface {
 public:
  explicit FakeMdnsResponder(Thread* thread) : thread_(thread) {}
  ~FakeMdnsResponder() = default;

  void CreateNameForAddress(const IPAddress& addr,
                            NameCreatedCallback callback) override {
    std::string name;
    if (addr_name_map_.find(addr) != addr_name_map_.end()) {
      name = addr_name_map_[addr];
    } else {
      name = std::to_string(next_available_id_++) + ".local";
      addr_name_map_[addr] = name;
    }
    thread_->PostTask([callback, addr, name]() { callback(addr, name); });
  }
  void RemoveNameForAddress(const IPAddress& addr,
                            NameRemovedCallback callback) override {
    auto it = addr_name_map_.find(addr);
    if (it != addr_name_map_.end()) {
      addr_name_map_.erase(it);
    }
    bool result = it != addr_name_map_.end();
    thread_->PostTask([callback, result]() { callback(result); });
  }

  IPAddress GetMappedAddressForName(absl::string_view name) const {
    for (const auto& addr_name_pair : addr_name_map_) {
      if (addr_name_pair.second == name) {
        return addr_name_pair.first;
      }
    }
    return IPAddress();
  }

 private:
  uint32_t next_available_id_ = 0;
  std::map<IPAddress, std::string> addr_name_map_;
  Thread* const thread_;
};

}  // namespace webrtc

#endif  // RTC_BASE_FAKE_MDNS_RESPONDER_H_
