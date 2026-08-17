//
//
// Copyright 2018 gRPC authors.
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

// The CFStream handle acts as an event synchronization entity for
// read/write/open/error/eos events happening on CFStream streams.

#ifndef GRPC_SRC_CORE_LIB_IOMGR_CFSTREAM_HANDLE_H
#define GRPC_SRC_CORE_LIB_IOMGR_CFSTREAM_HANDLE_H

#include <grpc/support/port_platform.h>

#include "src/core/lib/iomgr/port.h"

#ifdef GRPC_CFSTREAM
#import <CoreFoundation/CoreFoundation.h>

#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/lockfree_event.h"
#include "src/core/util/memory.h"

class GrpcLibraryInitHolder {
 public:
  GrpcLibraryInitHolder();
  virtual ~GrpcLibraryInitHolder();
};

class CFStreamHandle : public GrpcLibraryInitHolder {
 public:
  static CFStreamHandle* CreateStreamHandle(CFReadStreamRef read_stream,
                                            CFWriteStreamRef write_stream);
  /// Use CreateStreamHandle function instead of using this directly.
  CFStreamHandle(CFReadStreamRef read_stream, CFWriteStreamRef write_stream);
  CFStreamHandle(const CFStreamHandle& ref) = delete;
  CFStreamHandle(CFStreamHandle&& ref) = delete;
  CFStreamHandle& operator=(const CFStreamHandle& rhs) = delete;
  ~CFStreamHandle() override;

  void NotifyOnOpen(grpc_closure* closure);
  void NotifyOnRead(grpc_closure* closure);
  void NotifyOnWrite(grpc_closure* closure);
  void Shutdown(grpc_error_handle error);

  void Ref(const char* file = "", int line = 0, const char* reason = nullptr);
  void Unref(const char* file = "", int line = 0, const char* reason = nullptr);

 private:
  static void ReadCallback(CFReadStreamRef stream, CFStreamEventType type,
                           void* client_callback_info);
  static void WriteCallback(CFWriteStreamRef stream, CFStreamEventType type,
                            void* client_callback_info);
  static void* Retain(void* info);
  static void Release(void* info);

  grpc_core::LockfreeEvent open_event_;
  grpc_core::LockfreeEvent read_event_;
  grpc_core::LockfreeEvent write_event_;

  dispatch_queue_t dispatch_queue_;

  gpr_refcount refcount_;
};

#ifdef DEBUG
#define CFSTREAM_HANDLE_REF(handle, reason) \
  (handle)->Ref(__FILE__, __LINE__, (reason))
#define CFSTREAM_HANDLE_UNREF(handle, reason) \
  (handle)->Unref(__FILE__, __LINE__, (reason))
#else
#define CFSTREAM_HANDLE_REF(handle, reason) (handle)->Ref()
#define CFSTREAM_HANDLE_UNREF(handle, reason) (handle)->Unref()
#endif

#endif

#endif  // GRPC_SRC_CORE_LIB_IOMGR_CFSTREAM_HANDLE_H
