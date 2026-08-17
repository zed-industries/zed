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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_DYNAMIC_ANNOTATIONS_H
#define GRPC_SRC_CORE_LIB_IOMGR_DYNAMIC_ANNOTATIONS_H

#include <grpc/support/port_platform.h>

#ifdef GRPC_TSAN_ENABLED

#define TSAN_ANNOTATE_HAPPENS_BEFORE(addr) \
  AnnotateHappensBefore(__FILE__, __LINE__, (void*)(addr))
#define TSAN_ANNOTATE_HAPPENS_AFTER(addr) \
  AnnotateHappensAfter(__FILE__, __LINE__, (void*)(addr))
#define TSAN_ANNOTATE_RWLOCK_CREATE(addr) \
  AnnotateRWLockCreate(__FILE__, __LINE__, (void*)(addr))
#define TSAN_ANNOTATE_RWLOCK_DESTROY(addr) \
  AnnotateRWLockDestroy(__FILE__, __LINE__, (void*)(addr))
#define TSAN_ANNOTATE_RWLOCK_ACQUIRED(addr, is_w) \
  AnnotateRWLockAcquired(__FILE__, __LINE__, (void*)(addr), (is_w))
#define TSAN_ANNOTATE_RWLOCK_RELEASED(addr, is_w) \
  AnnotateRWLockReleased(__FILE__, __LINE__, (void*)(addr), (is_w))

#ifdef __cplusplus
extern "C" {
#endif
void AnnotateHappensBefore(const char* file, int line, const volatile void* cv);
void AnnotateHappensAfter(const char* file, int line, const volatile void* cv);
void AnnotateRWLockCreate(const char* file, int line,
                          const volatile void* lock);
void AnnotateRWLockDestroy(const char* file, int line,
                           const volatile void* lock);
void AnnotateRWLockAcquired(const char* file, int line,
                            const volatile void* lock, long is_w);
void AnnotateRWLockReleased(const char* file, int line,
                            const volatile void* lock, long is_w);
#ifdef __cplusplus
}
#endif

#else  // GRPC_TSAN_ENABLED

#define TSAN_ANNOTATE_HAPPENS_BEFORE(addr)
#define TSAN_ANNOTATE_HAPPENS_AFTER(addr)
#define TSAN_ANNOTATE_RWLOCK_CREATE(addr)
#define TSAN_ANNOTATE_RWLOCK_DESTROY(addr)
#define TSAN_ANNOTATE_RWLOCK_ACQUIRED(addr, is_w)
#define TSAN_ANNOTATE_RWLOCK_RELEASED(addr, is_w)

#endif  // GRPC_TSAN_ENABLED

#endif  // GRPC_SRC_CORE_LIB_IOMGR_DYNAMIC_ANNOTATIONS_H
