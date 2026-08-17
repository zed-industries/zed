//
//
// Copyright 2016 gRPC authors.
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
//
//

#ifndef GRPC_SRC_CORE_LIB_IOMGR_ERROR_H
#define GRPC_SRC_CORE_LIB_IOMGR_ERROR_H

#include <grpc/slice.h>
#include <grpc/status.h>
#include <grpc/support/port_platform.h>
#include <grpc/support/time.h>
#include <inttypes.h>
#include <stdbool.h>

#include "absl/log/check.h"
#include "absl/status/status.h"
#include "src/core/lib/debug/trace.h"
#include "src/core/lib/slice/slice_internal.h"
#include "src/core/util/crash.h"
#include "src/core/util/spinlock.h"
#include "src/core/util/status_helper.h"

/// Opaque representation of an error.

typedef absl::Status grpc_error_handle;

#define GRPC_ERROR_CREATE(desc) \
  StatusCreate(absl::StatusCode::kUnknown, desc, DEBUG_LOCATION, {})

absl::Status grpc_status_create(absl::StatusCode code, absl::string_view msg,
                                const grpc_core::DebugLocation& location,
                                size_t children_count, absl::Status* children);

// Create an error that references some other errors.
#define GRPC_ERROR_CREATE_REFERENCING(desc, errs, count)                      \
  grpc_status_create(absl::StatusCode::kUnknown, desc, DEBUG_LOCATION, count, \
                     errs)

// Consumes all the errors in the vector and forms a referencing error from
// them. If the vector is empty, return absl::OkStatus().
template <typename VectorType>
static absl::Status grpc_status_create_from_vector(
    const grpc_core::DebugLocation& location, absl::string_view desc,
    VectorType* error_list) {
  absl::Status error;
  if (error_list->size() != 0) {
    error = grpc_status_create(absl::StatusCode::kUnknown, desc, location,
                               error_list->size(), error_list->data());
    error_list->clear();
  }
  return error;
}

#define GRPC_ERROR_CREATE_FROM_VECTOR(desc, error_list) \
  grpc_status_create_from_vector(DEBUG_LOCATION, desc, error_list)

absl::Status grpc_os_error(const grpc_core::DebugLocation& location, int err,
                           const char* call_name);

inline absl::Status grpc_assert_never_ok(absl::Status error) {
  CHECK(!error.ok());
  return error;
}

/// create an error associated with errno!=0 (an 'operating system' error)
#define GRPC_OS_ERROR(err, call_name) \
  grpc_assert_never_ok(grpc_os_error(DEBUG_LOCATION, err, call_name))

absl::Status grpc_wsa_error(const grpc_core::DebugLocation& location, int err,
                            absl::string_view call_name);

/// windows only: create an error associated with WSAGetLastError()!=0
#define GRPC_WSA_ERROR(err, call_name) \
  grpc_wsa_error(DEBUG_LOCATION, err, call_name)

grpc_error_handle grpc_error_set_int(grpc_error_handle src,
                                     grpc_core::StatusIntProperty which,
                                     intptr_t value);
/// It is an error to pass nullptr as `p`. Caller should allocate a phony
/// intptr_t for `p`, even if the value of `p` is not used.
bool grpc_error_get_int(grpc_error_handle error,
                        grpc_core::StatusIntProperty which, intptr_t* p);
grpc_error_handle grpc_error_set_str(grpc_error_handle src,
                                     grpc_core::StatusStrProperty which,
                                     absl::string_view str);
/// Returns false if the specified string is not set.
bool grpc_error_get_str(grpc_error_handle error,
                        grpc_core::StatusStrProperty which, std::string* str);

/// Add a child error: an error that is believed to have contributed to this
/// error occurring. Allows root causing high level errors from lower level
/// errors that contributed to them. The src error takes ownership of the
/// child error.
///
/// Edge Conditions -
/// 1) If either of \a src or \a child is absl::OkStatus(), returns a reference
/// to the other argument. 2) If both \a src and \a child are absl::OkStatus(),
/// returns absl::OkStatus(). 3) If \a src and \a child point to the same error,
/// returns a single reference. (Note that, 2 references should have been
/// received to the error in this case.)
grpc_error_handle grpc_error_add_child(grpc_error_handle src,
                                       grpc_error_handle child);

bool grpc_log_error(const char* what, grpc_error_handle error, const char* file,
                    int line);
inline bool grpc_log_if_error(const char* what, grpc_error_handle error,
                              const char* file, int line) {
  return error.ok() ? true : grpc_log_error(what, error, file, line);
}

#define GRPC_LOG_IF_ERROR(what, error) \
  (grpc_log_if_error((what), (error), __FILE__, __LINE__))

/// Helper class to get & set grpc_error_handle in a thread-safe fashion.
/// This could be considered as atomic<grpc_error_handle>.
class AtomicError {
 public:
  AtomicError() = default;
  explicit AtomicError(grpc_error_handle error) { error_ = error; }

  AtomicError(const AtomicError&) = delete;
  AtomicError& operator=(const AtomicError&) = delete;

  /// returns get() == absl::OkStatus()
  bool ok() {
    gpr_spinlock_lock(&lock_);
    bool ret = error_.ok();
    gpr_spinlock_unlock(&lock_);
    return ret;
  }

  grpc_error_handle get() {
    gpr_spinlock_lock(&lock_);
    grpc_error_handle ret = error_;
    gpr_spinlock_unlock(&lock_);
    return ret;
  }

  void set(grpc_error_handle error) {
    gpr_spinlock_lock(&lock_);
    error_ = error;
    gpr_spinlock_unlock(&lock_);
  }

 private:
  grpc_error_handle error_;
  gpr_spinlock lock_ = GPR_SPINLOCK_STATIC_INITIALIZER;
};

#endif  // GRPC_SRC_CORE_LIB_IOMGR_ERROR_H
