// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_THREADING_THREAD_LOCAL_STORAGE_H_
#define BASE_THREADING_THREAD_LOCAL_STORAGE_H_

#include <stdint.h>

#include "base/base_export.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_WIN)
#include "base/win/windows_types.h"
#elif BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)
#include <pthread.h>
#endif

namespace base {

class SamplingHeapProfiler;

namespace debug {
class GlobalActivityTracker;
}  // namespace debug

namespace trace_event {
class MallocDumpProvider;
}  // namespace trace_event

namespace internal {

class ThreadLocalStorageTestInternal;

// WARNING: You should *NOT* use this class directly.
// PlatformThreadLocalStorage is a low-level abstraction of the OS's TLS
// interface. Instead, you should use one of the following:
// * ThreadLocalOwnedPointer (from thread_local.h) for unique_ptrs.
// * ThreadLocalStorage::StaticSlot/Slot for more direct control of the slot.
class BASE_EXPORT PlatformThreadLocalStorage {
 public:
#if BUILDFLAG(IS_WIN)
  typedef unsigned long TLSKey;
  enum : unsigned { TLS_KEY_OUT_OF_INDEXES = TLS_OUT_OF_INDEXES };
#elif BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)
  typedef pthread_key_t TLSKey;
  // The following is a "reserved key" which is used in our generic Chromium
  // ThreadLocalStorage implementation.  We expect that an OS will not return
  // such a key, but if it is returned (i.e., the OS tries to allocate it) we
  // will just request another key.
  enum { TLS_KEY_OUT_OF_INDEXES = 0x7FFFFFFF };
#endif

  // The following methods need to be supported on each OS platform, so that
  // the Chromium ThreadLocalStore functionality can be constructed.
  // Chromium will use these methods to acquire a single OS slot, and then use
  // that to support a much larger number of Chromium slots (independent of the
  // OS restrictions).
  // The following returns true if it successfully is able to return an OS
  // key in |key|.
  static bool AllocTLS(TLSKey* key);
  // Note: FreeTLS() doesn't have to be called, it is fine with this leak, OS
  // might not reuse released slot, you might just reset the TLS value with
  // SetTLSValue().
  static void FreeTLS(TLSKey key);
  static void SetTLSValue(TLSKey key, void* value);
  static void* GetTLSValue(TLSKey key) {
#if BUILDFLAG(IS_WIN)
    return TlsGetValue(key);
#elif BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)
    return pthread_getspecific(key);
#endif
  }

  // Each platform (OS implementation) is required to call this method on each
  // terminating thread when the thread is about to terminate.  This method
  // will then call all registered destructors for slots in Chromium
  // ThreadLocalStorage, until there are no slot values remaining as having
  // been set on this thread.
  // Destructors may end up being called multiple times on a terminating
  // thread, as other destructors may re-set slots that were previously
  // destroyed.
#if BUILDFLAG(IS_WIN)
  // Since Windows which doesn't support TLS destructor, the implementation
  // should use GetTLSValue() to retrieve the value of TLS slot.
  static void OnThreadExit();
#elif BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)
  // |Value| is the data stored in TLS slot, The implementation can't use
  // GetTLSValue() to retrieve the value of slot as it has already been reset
  // in Posix.
  static void OnThreadExit(void* value);
#endif
};

}  // namespace internal

// Wrapper for thread local storage.  This class doesn't do much except provide
// an API for portability.
class BASE_EXPORT ThreadLocalStorage {
 public:
  // Prototype for the TLS destructor function, which can be optionally used to
  // cleanup thread local storage on thread exit.  'value' is the data that is
  // stored in thread local storage.
  typedef void (*TLSDestructorFunc)(void* value);

  // A key representing one value stored in TLS. Use as a class member or a
  // local variable. If you need a static storage duration variable, use the
  // following pattern with a NoDestructor<Slot>:
  // void MyDestructorFunc(void* value);
  // ThreadLocalStorage::Slot& ImportantContentTLS() {
  //   static NoDestructor<ThreadLocalStorage::Slot> important_content_tls(
  //       &MyDestructorFunc);
  //   return *important_content_tls;
  // }
  class BASE_EXPORT Slot final {
   public:
    // |destructor| is a pointer to a function to perform per-thread cleanup of
    // this object.  If set to nullptr, no cleanup is done for this TLS slot.
    explicit Slot(TLSDestructorFunc destructor = nullptr);

    Slot(const Slot&) = delete;
    Slot& operator=(const Slot&) = delete;

    // If a destructor was set for this slot, removes the destructor so that
    // remaining threads exiting will not free data.
    ~Slot();

    // Get the thread-local value stored in slot 'slot'.
    // Values are guaranteed to initially be zero.
    void* Get() const;

    // Set the thread-local value stored in slot 'slot' to
    // value 'value'.
    void Set(void* value);

   private:
    void Initialize(TLSDestructorFunc destructor);
    void Free();

    static constexpr size_t kInvalidSlotValue = static_cast<size_t>(-1);
    size_t slot_ = kInvalidSlotValue;
    uint32_t version_ = 0;
  };

  ThreadLocalStorage(const ThreadLocalStorage&) = delete;
  ThreadLocalStorage& operator=(const ThreadLocalStorage&) = delete;

 private:
  // In most cases, most callers should not need access to HasBeenDestroyed().
  // If you are working in code that runs during thread destruction, contact the
  // base OWNERs for advice and then make a friend request.
  //
  // Returns |true| if Chrome's implementation of TLS is being or has been
  // destroyed during thread destruction. Attempting to call Slot::Get() during
  // destruction is disallowed and will hit a DCHECK. Any code that relies on
  // TLS during thread destruction must first check this method before calling
  // Slot::Get().
  friend class SequenceCheckerImpl;
  friend class SamplingHeapProfiler;
  friend class ThreadCheckerImpl;
  friend class internal::ThreadLocalStorageTestInternal;
  friend class trace_event::MallocDumpProvider;
  friend class debug::GlobalActivityTracker;
  static bool HasBeenDestroyed();
};

}  // namespace base

#endif  // BASE_THREADING_THREAD_LOCAL_STORAGE_H_
