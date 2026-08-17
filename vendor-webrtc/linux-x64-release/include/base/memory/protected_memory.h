// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/40284755): Remove this and spanify to fix the errors.
#pragma allow_unsafe_buffers
#endif

// Protected memory is memory holding security-sensitive data intended to be
// left read-only for the majority of its lifetime to avoid being overwritten
// by attackers. ProtectedMemory is a simple wrapper around platform-specific
// APIs to set memory read-write and read-only when required. Protected memory
// should be set read-write for the minimum amount of time required.
//
// Normally mutable variables are held in read-write memory and constant data
// is held in read-only memory to ensure it is not accidentally overwritten.
// In some cases we want to hold mutable variables in read-only memory, except
// when they are being written to, to ensure that they are not tampered with.
//
// ProtectedMemory is a container class intended to hold a single variable in
// read-only memory, except when explicitly set read-write. The variable can be
// set read-write by creating a scoped AutoWritableMemory object, the memory
// stays writable until the returned object goes out of scope and is destructed.
// The wrapped variable can be accessed using operator* and operator->.
//
// Instances of ProtectedMemory must be defined using DEFINE_PROTECTED_DATA
// and as global variables. Global definitions are required to avoid the linker
// placing statics in inlinable functions into a comdat section and setting the
// protected memory section read-write when they are merged. If a declaration of
// a protected variable is required DECLARE_PROTECTED_DATA should be used.
//
// Instances of `base::ProtectedMemory` use constant initialization. To allow
// protection of objects which do not provide constant initialization or would
// require a global constructor, `base::ProtectedMemory` provides lazy
// initialization through `ProtectedMemoryInitializer`. Additionally, on
// platforms where it is not possible to have the protected memory section start
// as read-only, the very first call to ProtectedMemoryInitializer will
// initialize the memory section to read-only. Explicit initialization through
// `ProtectedMemoryInitializer` is mandatory, even for objects that provide
// constant initialization. This ensures that in the unlikely event that the
// value is modified before the memory is initialized to read-only, it will be
// forced back to a known, safe, initial state before it ever used. If data is
// accessed without initialization a CHECK triggers. This CHECK is not expected
// to provided security guarantees, but to help catch programming errors.
//
// TODO(crbug.com/356428974): Improve protection offered by Protected Memory.
//
// `base::ProtectedMemory` requires T to be trivially destructible. T having
// a non-trivial constructor indicates that is holds data which can not be
// protected by `base::ProtectedMemory`.
//
// EXAMPLE:
//
//  struct Items { void* item1; };
//  static DEFINE_PROTECTED_DATA base::ProtectedMemory<Items> items;
//  void InitializeItems() {
//    // Explicitly set items read-write before writing to it.
//    auto writer = base::AutoWritableMemory(items);
//    writer->item1 = /* ... */;
//    assert(items->item1 != nullptr);
//    // items is set back to read-only on the destruction of writer
//  }
//
//  using FnPtr = void (*)(void);
//  DEFINE_PROTECTED_DATA base::ProtectedMemory<FnPtr> fnPtr;
//  FnPtr ResolveFnPtr(void) {
//    // `ProtectedMemoryInitializer` is a helper class for creating a static
//    // initializer for a ProtectedMemory variable. It implicitly sets the
//    // variable read-write during initialization.
//    static base::ProtectedMemoryInitializer initializer(&fnPtr,
//      reinterpret_cast<FnPtr>(dlsym(/* ... */)));
//    return *fnPtr;
//  }

#ifndef BASE_MEMORY_PROTECTED_MEMORY_H_
#define BASE_MEMORY_PROTECTED_MEMORY_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <type_traits>

#include "base/bits.h"
#include "base/check.h"
#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "base/gtest_prod_util.h"
#include "base/memory/page_size.h"
#include "base/memory/protected_memory_buildflags.h"
#include "base/memory/raw_ref.h"
#include "base/no_destructor.h"
#include "base/synchronization/lock.h"
#include "base/thread_annotations.h"
#include "build/build_config.h"

#if BUILDFLAG(PROTECTED_MEMORY_ENABLED)
#if BUILDFLAG(IS_WIN)
// Define a read-write prot section. The $a, $mem, and $z 'sub-sections' are
// merged alphabetically so $a and $z are used to define the start and end of
// the protected memory section, and $mem holds protected variables.
// (Note: Sections in Portable Executables are equivalent to segments in other
// executable formats, so this section is mapped into its own pages.)
#pragma section("prot$a", read, write)
#pragma section("prot$mem", read, write)
#pragma section("prot$z", read, write)

// We want the protected memory section to be read-only, not read-write so we
// instruct the linker to set the section read-only at link time. We do this
// at link time instead of compile time, because defining the prot section
// read-only would cause mis-compiles due to optimizations assuming that the
// section contents are constant.
#pragma comment(linker, "/SECTION:prot,R")

__declspec(allocate("prot$a"))
__declspec(selectany) char __start_protected_memory;
__declspec(allocate("prot$z"))
__declspec(selectany) char __stop_protected_memory;

#define DECLARE_PROTECTED_DATA constinit
#define DEFINE_PROTECTED_DATA constinit __declspec(allocate("prot$mem"))
#elif BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_ANDROID)
// This value is used to align the writers variable. That variable needs to be
// aligned to ensure that the protected memory section starts on a page
// boundary.
#if (PA_BUILDFLAG(IS_ANDROID) && PA_BUILDFLAG(PA_ARCH_CPU_64_BITS)) || \
    (PA_BUILDFLAG(IS_LINUX) && PA_BUILDFLAG(PA_ARCH_CPU_ARM64))
// arm64 supports 4kb, 16kb, and 64kb pages. Set to the largest of 64kb as that
// will guarantee the section is page aligned regardless of the choice.
inline constexpr int kProtectedMemoryAlignment = 65536;
#elif PA_BUILDFLAG(PA_ARCH_CPU_PPC64) || defined(ARCH_CPU_PPC64)
// Modern ppc64 systems support 4kB (shift = 12) and 64kB (shift = 16) page
// sizes. Set to the largest of 64kb as that will guarantee the section is page
// aligned regardless of the choice.
inline constexpr int kProtectedMemoryAlignment = 65536;
#elif defined(_MIPS_ARCH_LOONGSON) || PA_BUILDFLAG(PA_ARCH_CPU_LOONGARCH64) || \
    defined(ARCH_CPU_LOONGARCH64)
// 16kb page size
inline constexpr int kProtectedMemoryAlignment = 16384;
#else
// 4kb page size
inline constexpr int kProtectedMemoryAlignment = 4096;
#endif

__asm__(".section protected_memory, \"a\"\n\t");
__asm__(".section protected_memory_buffer, \"a\"\n\t");

// Explicitly mark these variables hidden so the symbols are local to the
// currently built component. Otherwise they are created with global (external)
// linkage and component builds would break because a single pair of these
// symbols would override the rest.
__attribute__((visibility("hidden"))) extern char __start_protected_memory;
__attribute__((visibility("hidden"))) extern char __stop_protected_memory;

#define DECLARE_PROTECTED_DATA constinit
#define DEFINE_PROTECTED_DATA \
  constinit __attribute__((section("protected_memory")))
#elif BUILDFLAG(IS_MAC)
// The segment the section is in is defined with a linker flag in
// build/config/mac/BUILD.gn
#define DECLARE_PROTECTED_DATA constinit
#define DEFINE_PROTECTED_DATA \
  constinit __attribute__((section("PROTECTED_MEMORY, protected_memory")))

extern char __start_protected_memory __asm(
    "section$start$PROTECTED_MEMORY$protected_memory");
extern char __stop_protected_memory __asm(
    "section$end$PROTECTED_MEMORY$protected_memory");
#else
#error "Protected Memory is not supported on this platform."
#endif

#else
#define DECLARE_PROTECTED_DATA constinit
#define DEFINE_PROTECTED_DATA DECLARE_PROTECTED_DATA
#endif  // BUILDFLAG(PROTECTED_MEMORY_ENABLED)

namespace base {

template <typename T>
class AutoWritableMemory;

FORWARD_DECLARE_TEST(ProtectedMemoryDeathTest, VerifyTerminationOnAccess);

namespace internal {
// Helper class which store the data and implement and initialization for
// constructing the underlying protected data lazily. The instance of T is only
// constructed when emplace is called.
template <typename T>
class ProtectedDataHolder {
 public:
  consteval ProtectedDataHolder() = default;

  T& GetReference() LIFETIME_BOUND { return *GetPointer(); }
  const T& GetReference() const LIFETIME_BOUND { return *GetPointer(); }

  T* GetPointer() {
    CHECK(constructed_);
    return reinterpret_cast<T*>(&data_);
  }
  const T* GetPointer() const {
    CHECK(constructed_);
    return reinterpret_cast<const T*>(&data_);
  }

  template <typename... U>
  void emplace(U&&... args) {
    if (constructed_) {
      std::destroy_at(reinterpret_cast<T*>(&data_));
      constructed_ = false;
    }

    std::construct_at(reinterpret_cast<T*>(&data_), std::forward<U>(args)...);
    constructed_ = true;
  }

 private:
  // Initializing with a constant/zero value ensures no global constructor is
  // required when instantiating `ProtectedDataHolder` and `ProtectedMemory`.
  alignas(T) uint8_t data_[sizeof(T)] = {};
  bool constructed_ = false;
};

}  // namespace internal

// The wrapper class for data of type `T` which is to be stored in protected
// memory. `ProtectedMemory` provides improved type safety in conjunction with
// the other classes, although the basic mechanisms like unlocking and
// re-locking of the memory would also work without it.
//
// To allow using `T`s which do not have constant initialization, the template
// parameter `ConstructLazily` enables a lazy initialization. In this case, an
// initialization before first access is mandatory (see
// `ProtectedMemoryInitializer`).
template <typename T>
class ProtectedMemory {
 public:
  // T must be trivially destructible. Otherwise it indicates that T holds data
  // which would not be covered by this write protection, i.e. data allocated on
  // heap. This check complements the verification in the constructor since
  // `ProtectedMemory` with `ConstructLazily` set to `true` is always trivially
  // destructible.
  static_assert(std::is_trivially_destructible_v<T>);

  // For lazily constructed data we enable this constructor only if there are
  // no arguments. For lazily constructed data no arguments are accepted as T is
  // not initialized when `ProtectedMemory<T>` is created but through
  // `ProtectedMemoryInitializer` instead.
  consteval explicit ProtectedMemory() : data_() {
    static_assert(std::is_trivially_destructible_v<ProtectedMemory>);
  }

  ProtectedMemory(const ProtectedMemory&) = delete;
  ProtectedMemory& operator=(const ProtectedMemory&) = delete;

  // Expose direct access to the encapsulated variable
  const T& operator*() const { return data_.GetReference(); }
  const T* operator->() const { return data_.GetPointer(); }

 private:
  friend class AutoWritableMemory<T>;
  FRIEND_TEST_ALL_PREFIXES(ProtectedMemoryDeathTest, VerifyTerminationOnAccess);

  internal::ProtectedDataHolder<T> data_;
};

#if BUILDFLAG(PROTECTED_MEMORY_ENABLED)
namespace internal {
// Checks that the byte at `ptr` is read-only.
BASE_EXPORT void CheckMemoryReadOnly(const void* ptr);

// Abstract out platform-specific methods to get the beginning and end of the
// PROTECTED_MEMORY_SECTION. ProtectedMemoryEnd returns a pointer to the byte
// past the end of the PROTECTED_MEMORY_SECTION.
inline constexpr void* kProtectedMemoryStart = &__start_protected_memory;
inline constexpr void* kProtectedMemoryEnd = &__stop_protected_memory;
}  // namespace internal
#endif  // BUILDFLAG(PROTECTED_MEMORY_ENABLED)

// Provide some common functionality for `AutoWritableMemory<T>`.
class BASE_EXPORT AutoWritableMemoryBase {
 protected:
#if BUILDFLAG(PROTECTED_MEMORY_ENABLED)
  // Checks that `object` is located within the interval
  // (internal::kProtectedMemoryStart, internal::kProtectedMemoryEnd).
  template <typename T>
  static bool IsObjectInProtectedSection(const T& object) {
    const T* const ptr = std::addressof(object);
    const T* const ptr_end = ptr + 1;
    return (ptr >= internal::kProtectedMemoryStart) &&
           (ptr_end <= internal::kProtectedMemoryEnd);
  }

  template <typename T>
  static void CheckObjectReadOnly(const T& object) {
    internal::CheckMemoryReadOnly(std::addressof(object));
  }

  template <typename T>
  static bool SetObjectReadWrite(T& object) {
    T* const ptr = std::addressof(object);
    T* const ptr_end = ptr + 1;
    return SetMemoryReadWrite(ptr, ptr_end);
  }

  static bool SetProtectedSectionReadOnly() {
    return SetMemoryReadOnly(internal::kProtectedMemoryStart,
                             internal::kProtectedMemoryEnd);
  }

  static bool IsSectionStartPageAligned() {
    const uintptr_t protected_memory_start =
        reinterpret_cast<uintptr_t>(internal::kProtectedMemoryStart);
    const uintptr_t page_start =
        bits::AlignDown(protected_memory_start, GetPageSize());
    return page_start == protected_memory_start;
  }

  // When linking, each DSO will have its own protected section. We can't keep
  // track of each section, yet we have to ensure to always unlock and re-lock
  // the correct section.
  //
  // We solve this by defining a separate global writers variable (explained
  // below) in every dynamic shared object (DSO) that includes this header. To
  // do that we use this structure to define global writer data without
  // duplicate symbol errors.
  //
  // Storing the data in a substructure is required to store `writers` within
  // the protected subsection. If `writers` and `writers_lock()` are located
  // directly in `AutoWritableMemoryBase`, for unknown reasons `writers` is not
  // placed into the protected section.
  struct WriterData {
    // `writers` is a global holding the number of ProtectedMemory instances set
    // writable, used to avoid races setting protected memory readable/writable.
    // When this reaches zero the protected memory region is set read only.
    // Access is controlled by writers_lock.
    //
    // Declare writers in the protected memory section to avoid the scenario
    // where an attacker could overwrite it with a large value and invoke code
    // that constructs and destructs an AutoWritableMemory. After such a call
    // protected memory would still be set writable because writers > 0.
#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_ANDROID)
    // On Linux, the protected memory section is not automatically page aligned.
    // This means that attempts to reset the protected memory region to readonly
    // will set some of the preceding section that is on the same page readonly
    // as well. By forcing the writers to be aligned on a multiple of the page
    // size, we can ensure the protected memory section starts on a page
    // boundary, preventing this issue.
    constinit __attribute__((section("protected_memory"),
                             aligned(kProtectedMemoryAlignment)))
#else
    DEFINE_PROTECTED_DATA
#endif
    static inline size_t writers GUARDED_BY(writers_lock()) = 0;

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_ANDROID)
    // On Linux, there is no guarantee the section following the protected
    // memory section is page aligned. This can result in attempts to change
    // the access permissions of the end of the protected memory section
    // overflowing to the next section. To ensure this doesn't happen, a buffer
    // section called protected_memory_buffer is created. Since the very first
    // variable declared after writers is put in this section, it will be
    // created as the next section after the protected memory section (since
    // sections are created in the order they are declared in the source file).
    // By explicitly setting the alignment of the variable to a multiple of the
    // page size, we can ensure this buffer section starts on a page boundary.
    // This guarantees that altering the access permissions of the end of the
    // protected memory section will not affect the next section. The variable
    // protected_memory_section_buffer serves no purpose other than to ensure
    // protected_memory_buffer section is created.
    constinit
        __attribute__((section("protected_memory_buffer"),
                       aligned(kProtectedMemoryAlignment))) static inline bool
            protected_memory_section_buffer = false;
#endif  // BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_ANDROID)

    // Synchronizes access to the writers variable and the simultaneous actions
    // that need to happen alongside writers changes, e.g. setting the protected
    // memory region readable when writers is decremented to 0.
    static Lock& writers_lock() {
      static NoDestructor<Lock> writers_lock;
      return *writers_lock;
    }
  };

 private:
  // Abstract out platform-specific memory APIs. |end| points to the byte
  // past the end of the region of memory having its memory protections
  // changed.
  static bool SetMemoryReadWrite(void* start, void* end);
  static bool SetMemoryReadOnly(void* start, void* end);
#endif  // BUILDFLAG(PROTECTED_MEMORY_ENABLED)
};

#if BUILDFLAG(PROTECTED_MEMORY_ENABLED)
// This class acts as a static initializer that initializes the protected memory
// region to read only. It will be engaged the first time a protected memory
// object is statically initialized.
class BASE_EXPORT AutoWritableMemoryInitializer
    : public AutoWritableMemoryBase {
 public:
#if BUILDFLAG(IS_WIN)
  AutoWritableMemoryInitializer() { CHECK(IsSectionStartPageAligned()); }
#else
  AutoWritableMemoryInitializer() LOCKS_EXCLUDED(WriterData::writers_lock()) {
    CHECK(IsSectionStartPageAligned());
    // This doesn't need to be run on Windows, because the linker can pre-set
    // the memory to read-only.
    AutoLock auto_lock(WriterData::writers_lock());
    // Reset the writers variable to 0 to ensure that the attacker didn't set
    // the variable to something large before the section was read-only.
    WriterData::writers = 0;
    CHECK(SetProtectedSectionReadOnly());
#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_ANDROID)
    // Set the protected_memory_section_buffer to true to ensure the buffer
    // section is created. If a variable is declared but not used the memory
    // section won't be created.
    WriterData::protected_memory_section_buffer = true;
#endif  // BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_ANDROID)
  }
#endif  // BUILDFLAG(IS_WIN)
};
#endif  // BUILDFLAG(PROTECTED_MEMORY_ENABLED)

// A class that sets a given ProtectedMemory variable writable while the
// AutoWritableMemory is in scope. This class implements the logic for setting
// the protected memory region read-only/read-write in a thread-safe manner.
//
// |AutoWritableMemory| affects the write-permissions of _all_ protected data
// for a DSO, not just of the instance that it's being passed! All protected
// data is stored within the same binary section. At the same time, the OS-level
// support enforcing write protection can only be changed at page level. To
// allow a more fine grained control a dedicated page per instance of protected
// data would be required.
template <typename T>
class AutoWritableMemory : public AutoWritableMemoryBase {
 public:
  explicit AutoWritableMemory(ProtectedMemory<T>& protected_memory)
#if BUILDFLAG(PROTECTED_MEMORY_ENABLED)
      LOCKS_EXCLUDED(WriterData::writers_lock())
#endif
      : protected_memory_(protected_memory) {
#if BUILDFLAG(PROTECTED_MEMORY_ENABLED)

    // Check that the data is located in the protected section to
    // ensure consistency of data.
    CHECK(IsObjectInProtectedSection(protected_memory_->data_));
    CHECK(IsObjectInProtectedSection(WriterData::writers));

    {
      AutoLock auto_lock(WriterData::writers_lock());

      if (WriterData::writers == 0) {
        CheckObjectReadOnly(protected_memory_->data_);
        CheckObjectReadOnly(WriterData::writers);
        CHECK(SetObjectReadWrite(WriterData::writers));
      }

      ++WriterData::writers;
    }

    CHECK(SetObjectReadWrite(protected_memory_->data_));
#endif  // BUILDFLAG(PROTECTED_MEMORY_ENABLED)
  }

  ~AutoWritableMemory()
#if BUILDFLAG(PROTECTED_MEMORY_ENABLED)
      LOCKS_EXCLUDED(WriterData::writers_lock())
#endif
  {
#if BUILDFLAG(PROTECTED_MEMORY_ENABLED)
    AutoLock auto_lock(WriterData::writers_lock());
    CHECK_GT(WriterData::writers, 0u);
    --WriterData::writers;

    if (WriterData::writers == 0) {
      // Lock the whole section of protected memory and set _all_ instances of
      // ProtectedMemory to non-writeable.
      CHECK(SetProtectedSectionReadOnly());
      CheckObjectReadOnly(
          *static_cast<const char*>(internal::kProtectedMemoryStart));
      CheckObjectReadOnly(WriterData::writers);
    }
#endif  // BUILDFLAG(PROTECTED_MEMORY_ENABLED)
  }

  AutoWritableMemory(AutoWritableMemory& original) = delete;
  AutoWritableMemory& operator=(AutoWritableMemory& original) = delete;
  AutoWritableMemory(AutoWritableMemory&& original) = delete;
  AutoWritableMemory& operator=(AutoWritableMemory&& original) = delete;

  T& GetProtectedData() { return protected_memory_->data_.GetReference(); }
  T* GetProtectedDataPtr() { return protected_memory_->data_.GetPointer(); }

  template <typename... U>
  void emplace(U&&... args) {
    protected_memory_->data_.emplace(std::forward<U>(args)...);
  }

 private:
  const raw_ref<ProtectedMemory<T>> protected_memory_;
};

// Helper class for creating simple ProtectedMemory static initializers.
class ProtectedMemoryInitializer {
 public:
  template <typename T, typename... U>
  explicit ProtectedMemoryInitializer(ProtectedMemory<T>& protected_memory,
                                      U&&... args) {
    InitializeAutoWritableMemory();
    AutoWritableMemory writer(protected_memory);
    writer.emplace(std::forward<U>(args)...);
  }

  ProtectedMemoryInitializer() = delete;
  ProtectedMemoryInitializer(const ProtectedMemoryInitializer&) = delete;
  ProtectedMemoryInitializer& operator=(const ProtectedMemoryInitializer&) =
      delete;

 private:
  void InitializeAutoWritableMemory() {
#if BUILDFLAG(PROTECTED_MEMORY_ENABLED)
    static AutoWritableMemoryInitializer memory_initializer;
#else
    // No-op if protected memory is not enabled.
#endif
  }
};

}  // namespace base

#endif  // BASE_MEMORY_PROTECTED_MEMORY_H_
