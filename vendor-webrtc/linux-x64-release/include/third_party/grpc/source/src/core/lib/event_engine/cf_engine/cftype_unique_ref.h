// Copyright 2023 The gRPC Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_CF_ENGINE_CFTYPE_UNIQUE_REF_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_CF_ENGINE_CFTYPE_UNIQUE_REF_H
#include <grpc/support/port_platform.h>

#ifdef GPR_APPLE
#include <AvailabilityMacros.h>
#ifdef AVAILABLE_MAC_OS_X_VERSION_10_12_AND_LATER

#include <CoreFoundation/CoreFoundation.h>

namespace grpc_event_engine::experimental {

template <typename T>
class CFTypeUniqueRef {
  static_assert(std::is_convertible<T, CFTypeRef>::value,
                "T should be `CFXxxRef` type");

 public:
  /* implicit */
  CFTypeUniqueRef(T cf_type_ref = nullptr) : cf_type_ref_(cf_type_ref) {}
  ~CFTypeUniqueRef() { reset(); }

  CFTypeUniqueRef(CFTypeUniqueRef const&) = delete;
  CFTypeUniqueRef& operator=(CFTypeUniqueRef const&) = delete;

  CFTypeUniqueRef(CFTypeUniqueRef&& other) : cf_type_ref_(other.release()) {};
  CFTypeUniqueRef& operator=(CFTypeUniqueRef&& other) {
    reset(other.release());
    return *this;
  }

  operator T() { return cf_type_ref_; }

  // Note: this is for passing a CFTypeRef as output parameter to a CF API, the
  // current ref is released (if any) regardless of whether new value is set
  T* operator&() {
    reset();
    return &cf_type_ref_;
  }

  T release() {
    T old = cf_type_ref_;
    cf_type_ref_ = nullptr;
    return old;
  }

  void reset(T other = nullptr) {
    if (cf_type_ref_ == other) {
      return;
    }
    T old = cf_type_ref_;
    cf_type_ref_ = other;
    if (old) {
      CFRelease(old);
    }
  }

 private:
  T cf_type_ref_;
};

}  // namespace grpc_event_engine::experimental

#endif  // AVAILABLE_MAC_OS_X_VERSION_10_12_AND_LATER
#endif  // GPR_APPLE

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_CF_ENGINE_CFTYPE_UNIQUE_REF_H
