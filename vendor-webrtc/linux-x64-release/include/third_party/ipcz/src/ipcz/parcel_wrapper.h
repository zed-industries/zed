// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_PARCEL_WRAPPER_
#define IPCZ_SRC_IPCZ_PARCEL_WRAPPER_

#include <cstddef>

#include "ipcz/api_object.h"
#include "ipcz/ipcz.h"
#include "ipcz/parcel.h"
#include "third_party/abseil-cpp/absl/base/macros.h"
#include "util/ref_counted.h"

namespace ipcz {

// A parcel wrapper owns a Parcel received from a portal, after the Parcel has
// been retrieved from the portal by the application.
//
// Applications can use these objects to perform two-phase reads of parcels
// without blocking the receiving portal, and to report their own
// application-level validation failures to ipcz via the Reject() API.
class ParcelWrapper : public APIObjectImpl<ParcelWrapper, APIObject::kParcel> {
 public:
  explicit ParcelWrapper(std::unique_ptr<Parcel> parcel);

  Parcel& parcel() {
    ABSL_ASSERT(parcel_);
    return *parcel_;
  }

  void SetParcel(std::unique_ptr<Parcel> parcel) {
    parcel_ = std::move(parcel);
  }

  std::unique_ptr<Parcel> TakeParcel() { return std::move(parcel_); }

  // APIObject:
  IpczResult Close() override;

  // Signals application-level rejection of this parcel. `context` is an opaque
  // value passed by the application and propagated to the driver when
  // appropriate. See the Reject() API.
  IpczResult Reject(uintptr_t context);

  IpczResult Get(IpczGetFlags flags,
                 void* data,
                 size_t* num_data_bytes,
                 IpczHandle* handles,
                 size_t* num_handles,
                 IpczHandle* parcel);
  IpczResult BeginGet(IpczBeginGetFlags flags,
                      const volatile void** data,
                      size_t* num_data_bytes,
                      IpczHandle* handles,
                      size_t* num_handles,
                      IpczTransaction* transaction);
  IpczResult EndGet(IpczTransaction transaction,
                    IpczEndGetFlags flags,
                    IpczHandle* parcel);

 private:
  ~ParcelWrapper() override;

  std::unique_ptr<Parcel> parcel_;
  bool in_two_phase_get_ = false;
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_PARCEL_WRAPPER_
