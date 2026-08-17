// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_METRICS_PERSISTENT_MEMORY_ALLOCATOR_H_
#define BASE_METRICS_PERSISTENT_MEMORY_ALLOCATOR_H_

#include <stdint.h>

#include <atomic>
#include <memory>
#include <string_view>
#include <type_traits>

#include "base/atomicops.h"
#include "base/base_export.h"
#include "base/check.h"
#include "base/compiler_specific.h"
#include "base/containers/span.h"
#include "base/files/file_path.h"
#include "base/gtest_prod_util.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/memory/shared_memory_mapping.h"
#include "build/build_config.h"

namespace metrics {
class FileMetricsProvider;
}

namespace base {

class HistogramBase;
class MemoryMappedFile;

// Simple allocator for pieces of a memory block that may be persistent
// to some storage or shared across multiple processes. This class resides
// under base/metrics because it was written for that purpose. It is,
// however, fully general-purpose and can be freely moved to base/memory
// if other uses are found.
//
// This class provides for thread-secure (i.e. safe against other threads
// or processes that may be compromised and thus have malicious intent)
// allocation of memory within a designated block and also a mechanism by
// which other threads can learn of these allocations.
//
// There is (currently) no way to release an allocated block of data because
// doing so would risk invalidating pointers held by other processes and
// greatly complicate the allocation algorithm.
//
// Construction of this object can accept new, clean (i.e. zeroed) memory
// or previously initialized memory. In the first case, construction must
// be allowed to complete before letting other allocators attach to the same
// segment. In other words, don't share the segment until at least one
// allocator has been attached to it.
//
// Note that memory not in active use is not accessed so it is possible to
// use virtual memory, including memory-mapped files, as backing storage with
// the OS "pinning" new (zeroed) physical RAM pages only as they are needed.
//
// OBJECTS: Although the allocator can be used in a "malloc" sense, fetching
// character arrays and manipulating that memory manually, the better way is
// generally to use the "object" methods to create and manage allocations. In
// this way the sizing, type-checking, and construction are all automatic. For
// this to work, however, every type of stored object must define two public
// "constexpr" values, kPersistentTypeId and kExpectedInstanceSize, as such:
//
// struct MyPersistentObjectType {
//     // SHA1(MyPersistentObjectType): Increment this if structure changes!
//     static constexpr uint32_t kPersistentTypeId = 0x3E15F6DE + 1;
//
//     // Expected size for 32/64-bit check. Update this if structure changes!
//     static constexpr size_t kExpectedInstanceSize = 20;
//
//     ...
// };
//
// kPersistentTypeId: This value is an arbitrary identifier that allows the
//   identification of these objects in the allocator, including the ability
//   to find them via iteration. The number is arbitrary but using the first
//   four bytes of the SHA1 hash of the type name means that there shouldn't
//   be any conflicts with other types that may also be stored in the memory.
//   The fully qualified name (e.g. base::debug::MyPersistentObjectType) could
//   be used to generate the hash if the type name seems common. Use a command
//   like this to get the hash: echo -n "MyPersistentObjectType" | sha1sum
//   If the structure layout changes, ALWAYS increment this number so that
//   newer versions of the code don't try to interpret persistent data written
//   by older versions with a different layout.
//
// kExpectedInstanceSize: This value is the hard-coded number that matches
//   what sizeof(T) would return. By providing it explicitly, the allocator can
//   verify that the structure is compatible between both 32-bit and 64-bit
//   versions of the code.
//
// Using New manages the memory and then calls the default constructor for the
// object. Given that objects are persistent, no destructor is ever called
// automatically though a caller can explicitly call Delete to destruct it and
// change the type to something indicating it is no longer in use.
//
// Though persistent memory segments are transferrable between programs built
// for different natural word widths, they CANNOT be exchanged between CPUs
// of different endianess. Attempts to do so will simply see the existing data
// as corrupt and refuse to access any of it.
class BASE_EXPORT PersistentMemoryAllocator {
 public:
  typedef uint32_t Reference;

  // All allocations and data-structures must be aligned to this byte boundary.
  // Alignment as large as the physical bus between CPU and RAM is _required_
  // for some architectures, is simply more efficient on other CPUs, and
  // generally a Good Idea(tm) for all platforms as it reduces/eliminates the
  // chance that a type will span cache lines. Alignment mustn't be less
  // than 8 to ensure proper alignment for all types. The rest is a balance
  // between reducing spans across multiple cache lines and wasted space spent
  // padding out allocations. An alignment of 16 would ensure that the block
  // header structure always sits in a single cache line. An average of about
  // 1/2 this value will be wasted with every allocation.
  static constexpr size_t kAllocAlignment = 8;

  // These states are used to indicate the overall condition of the memory
  // segment irrespective of what is stored within it. Because the data is
  // often persistent and thus needs to be readable by different versions of
  // a program, these values are fixed and can never change.
  enum MemoryState : uint8_t {
    // Persistent memory starts all zeros and so shows "uninitialized".
    MEMORY_UNINITIALIZED = 0,

    // The header has been written and the memory is ready for use.
    MEMORY_INITIALIZED = 1,

    // The data should be considered deleted. This would be set when the
    // allocator is being cleaned up. If file-backed, the file is likely
    // to be deleted but since deletion can fail for a variety of reasons,
    // having this extra status means a future reader can realize what
    // should have happened.
    MEMORY_DELETED = 2,

    // The data should be considered complete. This is usually set when the
    // browser is going to exit to indicate that it terminated cleanly and that
    // the memory should be well-formed. In theory, this is not perfect as it is
    // possible for the browser/device to crash after this has been set, but in
    // practice this should be a reasonable indication as to whether the data
    // comes from a completed or crashed session (if file-backed). Note that
    // this might not be set on certain platforms (e.g. Android, iOS) due to not
    // having a guaranteed clean shutdown path.
    MEMORY_COMPLETED = 3,

    // Outside code can create states starting with this number; these too
    // must also never change between code versions.
    MEMORY_USER_DEFINED = 100,
  };

  // Iterator for going through all iterable memory records in an allocator.
  // Like the allocator itself, iterators are lock-free and thread-secure.
  // That means that multiple threads can share an iterator and the same
  // reference will not be returned twice.
  //
  // The order of the items returned by an iterator matches the order in which
  // MakeIterable() was called on them. Once an allocation is made iterable,
  // it is always such so the only possible difference between successive
  // iterations is for more to be added to the end.
  //
  // Iteration, in general, is tolerant of corrupted memory. It will return
  // what it can and stop only when corruption forces it to. Bad corruption
  // could cause the same object to be returned many times but it will
  // eventually quit.
  class BASE_EXPORT Iterator {
   public:
    // Constructs an iterator on a given `allocator`, starting at the beginning.
    // The allocator must live beyond the lifetime of the iterator. This class
    // has read-only access to the allocator (hence "const") but the returned
    // references can be used on a read/write version, too.
    explicit Iterator(const PersistentMemoryAllocator* allocator);

    // As above but resuming from the `starting_after` reference. The first call
    // to GetNext() will return the next object found after that reference. The
    // reference must be to an "iterable" object; references to non-iterable
    // objects (those that never had MakeIterable() called for them) will cause
    // a run-time error.
    Iterator(const PersistentMemoryAllocator* allocator,
             Reference starting_after);

    Iterator(const Iterator&) = delete;
    Iterator& operator=(const Iterator&) = delete;

    ~Iterator();

    // Resets the iterator back to the beginning.
    void Reset();

    // Resets the iterator, resuming from the `starting_after` reference.
    void Reset(Reference starting_after);

    // Returns the previously retrieved reference, or kReferenceNull if none.
    // If constructor or reset with a starting_after location, this will return
    // that value.
    Reference GetLast();

    // Gets the next iterable, storing that type in `type_return`. The actual
    // return value is a reference to the allocation inside the allocator or
    // zero if there are no more. GetNext() may still be called again at a
    // later time to retrieve any new allocations that have been added.
    Reference GetNext(uint32_t* type_return, size_t* alloc_size = nullptr);

    // Similar to above but gets the next iterable of a specific `type_match`.
    // This should not be mixed with calls to GetNext() because any allocations
    // skipped here due to a type mis-match will never be returned by later
    // calls to GetNext() meaning it's possible to completely miss entries.
    Reference GetNextOfType(uint32_t type_match, size_t* alloc_size = nullptr);

    // As above but works using object type.
    template <typename T>
    Reference GetNextOfType() {
      return GetNextOfType(T::kPersistentTypeId);
    }

    // As above but works using objects and returns null if not found.
    template <typename T>
    const T* GetNextOfObject() {
      return GetAsObject<T>(GetNextOfType<T>());
    }

    // Converts references to objects. This is a convenience method so that
    // users of the iterator don't need to also have their own pointer to the
    // allocator over which the iterator runs in order to retrieve objects.
    // Because the iterator is not read/write, only "const" objects can be
    // fetched. Non-const objects can be fetched using the reference on a
    // non-const (external) pointer to the same allocator (or use const_cast
    // to remove the qualifier).
    template <typename T>
    const T* GetAsObject(Reference ref) const {
      return allocator_->GetAsObject<T>(ref);
    }

    // Similar to GetAsObject() but converts references to arrays of things.
    template <typename T>
    const T* GetAsArray(Reference ref, uint32_t type_id, size_t count) const {
      return allocator_->GetAsArray<T>(ref, type_id, count);
    }

    // Convert a generic pointer back into a reference. A null reference will
    // be returned if `memory` is not inside the persistent segment or does not
    // point to an object of the specified `type_id`.
    Reference GetAsReference(const void* memory, uint32_t type_id) const {
      return allocator_->GetAsReference(memory, type_id);
    }

    // As above but convert an object back into a reference.
    template <typename T>
    Reference GetAsReference(const T* obj) const {
      return allocator_->GetAsReference(obj);
    }

   private:
    // Weak-pointer to memory allocator being iterated over.
    raw_ptr<const PersistentMemoryAllocator> allocator_;

    // The last record that was returned.
    std::atomic<Reference> last_record_;

    // The number of records found; used for detecting loops.
    std::atomic<uint32_t> record_count_;
  };

  // Returned information about the internal state of the heap.
  struct MemoryInfo {
    size_t total;
    size_t free;
  };

  enum : Reference {
    // A common "null" reference value.
    kReferenceNull = 0,
  };

  enum : uint32_t {
    // A value that will match any type when doing lookups.
    kTypeIdAny = 0x00000000,

    // A value indicating that the type is in transition. Work is being done
    // on the contents to prepare it for a new type to come.
    kTypeIdTransitioning = 0xFFFFFFFF,
  };

  enum : size_t {
    kSizeAny = 1  // Constant indicating that any array size is acceptable.
  };

  // Indicates the mode for accessing the underlying data.
  enum AccessMode {
    kReadOnly,
    kReadWrite,
    // Open existing initialized data in R/W mode. If the passed data appears to
    // not have been initialized, does not write to it and instead marks the
    // allocator as corrupt (without writing anything to the underlying data.)
    kReadWriteExisting,
  };

  // This is the standard file extension (suitable for being passed to the
  // AddExtension() method of base::FilePath) for dumps of persistent memory.
  static const base::FilePath::CharType kFileExtension[];

  // The allocator operates on any arbitrary block of memory. Creation and
  // persisting or sharing of that block with another process is the
  // responsibility of the caller. The allocator needs to know only the
  // block's `base` address, the total `size` of the block, and any internal
  // `page` size (zero if not paged) across which allocations should not span.
  // The `id` is an arbitrary value the caller can use to identify a
  // particular memory segment. It will only be loaded during the initial
  // creation of the segment and can be checked by the caller for consistency.
  // The `name`, if provided, is used to distinguish histograms for this
  // allocator. Only the primary owner of the segment should define this value;
  // other processes can learn it from the shared state. If the access mode
  // is kReadOnly then no changes will be made to it. The resulting object
  // should be stored as a "const" pointer.
  //
  // PersistentMemoryAllocator does NOT take ownership of the memory block.
  // The caller must manage it and ensure it stays available throughout the
  // lifetime of this object.
  //
  // Memory segments for sharing must have had an allocator attached to them
  // before actually being shared. If the memory segment was just created, it
  // should be zeroed before being passed here. If it was an existing segment,
  // the values here will be compared to copies stored in the shared segment
  // as a guard against corruption.
  //
  // Make sure that the memory segment is acceptable (see IsMemoryAcceptable()
  // method below) before construction if the definition of the segment can
  // vary in any way at run-time. Invalid memory segments will cause a crash.
  PersistentMemoryAllocator(void* base,
                            size_t size,
                            size_t page_size,
                            uint64_t id,
                            std::string_view name,
                            AccessMode access_mode);

  PersistentMemoryAllocator(const PersistentMemoryAllocator&) = delete;
  PersistentMemoryAllocator& operator=(const PersistentMemoryAllocator&) =
      delete;

  virtual ~PersistentMemoryAllocator();

  // Checks if memory segment is acceptable for creation of an Allocator. This
  // doesn't do any analysis of the data and so doesn't guarantee that the
  // contents are valid, just that the paramaters won't cause the program to
  // abort. The IsCorrupt() method will report detection of data problems
  // found during construction and general operation.
  static bool IsMemoryAcceptable(const void* data,
                                 size_t size,
                                 size_t page_size,
                                 bool readonly);

  // Returns the internal identifier for this persistent memory segment.
  uint64_t Id() const;

  // Returns the internal name of this allocator (possibly an empty string).
  // The returned string_view references a bounded span within the shared
  // memory region. As such, it should be treated as a volatile but bounded
  // block of memory. In particular, clients should respect the 'length()' of
  // the returned view instead of relying on a terminating NUL char.
  std::string_view Name() const;

  // Is this segment open only for read?
  bool IsReadonly() const { return access_mode_ == kReadOnly; }

  // Manage the saved state of the memory.
  void SetMemoryState(uint8_t memory_state);
  uint8_t GetMemoryState() const;

  // Creates internal histograms for tracking memory use and allocation sizes
  // for allocator of `name` (which can simply be the result of Name()). This
  // is done separately from construction for situations such as when the
  // histograms will be backed by memory provided by this very allocator.
  //
  // IMPORTANT: tools/metrics/histograms/metadata/uma/histograms.xml must
  // be updated with the following histograms for each `name` param:
  //    UMA.PersistentAllocator.name.Errors
  //    UMA.PersistentAllocator.name.UsedPct
  void CreateTrackingHistograms(std::string_view name);

  // Flushes the persistent memory to any backing store. This typically does
  // nothing but is used by the FilePersistentMemoryAllocator to inform the
  // OS that all the data should be sent to the disk immediately. This is
  // useful in the rare case where something has just been stored that needs
  // to survive a hard shutdown of the machine like from a power failure.
  // The `sync` parameter indicates if this call should block until the flush
  // is complete but is only advisory and may or may not have an effect
  // depending on the capabilities of the OS. Synchronous flushes are allowed
  // only from threads that are allowed to do I/O but since `sync` is only
  // advisory, all flushes should be done on IO-capable threads.
  // TODO: Since `sync` is ignored on Windows, consider making it re-post on a
  // background thread with `sync` set to true so that `sync` is not just
  // advisory.
  void Flush(bool sync);

  // Direct access to underlying memory segment. If the segment is shared
  // across threads or processes, reading data through these values does
  // not guarantee consistency. Use with care. Do not write.
  const void* data() const { return const_cast<const char*>(mem_base_); }
  size_t length() const { return mem_size_; }
  size_t size() const { return mem_size_; }
  size_t page_size() const { return mem_page_; }
  size_t used() const;

  // Returns the object referenced by a `ref`. For safety reasons, the `type_id`
  // code and size-of(`T`) are compared to ensure the reference is valid
  // and cannot return an object outside of the memory segment. A `type_id` of
  // kTypeIdAny (zero) will match any though the size is still checked. NULL is
  // returned if any problem is detected, such as corrupted storage or incorrect
  // parameters. Callers MUST check that the returned value is not-null EVERY
  // TIME before accessing it or risk crashing! Once dereferenced, the pointer
  // is safe to reuse forever.
  //
  // It is essential that the object be of a fixed size. All fields must be of
  // a defined type that does not change based on the compiler or the CPU
  // natural word size. Acceptable are char, float, double, and (u)intXX_t.
  // Unacceptable are int, bool, and wchar_t which are implementation defined
  // with regards to their size.
  //
  // Alignment must also be consistent. A uint64_t after a uint32_t will pad
  // differently between 32 and 64 bit architectures. Either put the bigger
  // elements first, group smaller elements into blocks the size of larger
  // elements, or manually insert padding fields as appropriate for the
  // largest architecture, including at the end.
  //
  // To protect against mistakes, all objects must have the attribute
  // `kExpectedInstanceSize` (static constexpr size_t)  that is a hard-coded
  // numerical value -- NNN, not sizeof(T) -- that can be tested. If the
  // instance size is not fixed, at least one build will fail.
  //
  // If the size of a structure changes, the type-ID used to recognize it
  // should also change so later versions of the code don't try to read
  // incompatible structures from earlier versions.
  //
  // NOTE: Though this method will guarantee that an object of the specified
  // type can be accessed without going outside the bounds of the memory
  // segment, it makes no guarantees of the validity of the data within the
  // object itself. If it is expected that the contents of the segment could
  // be compromised with malicious intent, the object must be hardened as well.
  //
  // Though the persistent data may be "volatile" if it is shared with
  // other processes, such is not necessarily the case. The internal
  // "volatile" designation is discarded so as to not propagate the viral
  // nature of that keyword to the caller. It can add it back, if necessary,
  // based on knowledge of how the allocator is being used.
  template <typename T>
  T* GetAsObject(Reference ref, size_t* alloc_size = nullptr) {
    static_assert(std::is_standard_layout_v<T>, "only standard objects");
    static_assert(!std::is_array_v<T>, "use GetAsArray<>()");
    static_assert(T::kExpectedInstanceSize == sizeof(T), "inconsistent size");
    return const_cast<T*>(reinterpret_cast<volatile T*>(
        GetBlockData(ref, T::kPersistentTypeId, sizeof(T), alloc_size)));
  }
  template <typename T>
  const T* GetAsObject(Reference ref, size_t* alloc_size = nullptr) const {
    static_assert(std::is_standard_layout_v<T>, "only standard objects");
    static_assert(!std::is_array_v<T>, "use GetAsArray<>()");
    static_assert(T::kExpectedInstanceSize == sizeof(T), "inconsistent size");
    return const_cast<const T*>(reinterpret_cast<const volatile T*>(
        GetBlockData(ref, T::kPersistentTypeId, sizeof(T), alloc_size)));
  }

  // Like GetAsObject() but returns an array of simple, fixed-size types.
  //
  // Use a `count` of the required number of array elements, or kSizeAny.
  // The, optionally returned, `alloc_size` can be used to calculate the upper
  // bound but isn't reliable because padding can make space for extra elements
  // that were not written.
  //
  // Remember that an array of char is a string but may not be NUL terminated.
  //
  // There are no compile-time or run-time checks to ensure 32/64-bit size
  // compatibilty when using these accessors. Only use fixed-size types such
  // as char, float, double, or (u)intXX_t.
  template <typename T>
  T* GetAsArray(Reference ref,
                uint32_t type_id,
                size_t count,
                size_t* alloc_size = nullptr) {
    static_assert(std::is_fundamental_v<T>, "use GetAsObject<>()");
    return const_cast<T*>(reinterpret_cast<volatile T*>(
        GetBlockData(ref, type_id, count * sizeof(T), alloc_size)));
  }
  template <typename T>
  const T* GetAsArray(Reference ref,
                      uint32_t type_id,
                      size_t count,
                      size_t* alloc_size = nullptr) const {
    static_assert(std::is_fundamental_v<T>, "use GetAsObject<>()");
    return const_cast<const char*>(reinterpret_cast<const volatile T*>(
        GetBlockData(ref, type_id, count * sizeof(T), alloc_size)));
  }

  // Gets the corresponding reference for an object held in persistent memory.
  // If the `memory` is not valid or the type does not match, a kReferenceNull
  // result will be returned.
  Reference GetAsReference(const void* memory, uint32_t type_id) const;

  // Accesses the internal "type" of an object. This generally isn't necessary
  // but can be used to "clear" the type and so effectively mark it as deleted
  // even though the memory stays valid and allocated. Changing the type is
  // an atomic compare/exchange and so requires knowing the existing value.
  // It will return false if the existing type is not what is expected.
  //
  // Changing the type doesn't mean the data is compatible with the new type.
  // Passing true for `clear` will zero the memory after the type has been
  // changed away from `from_type_id` but before it becomes `to_type_id` meaning
  // that it is done in a manner that is thread-safe. Memory is guaranteed to
  // be zeroed atomically by machine-word in a monotonically increasing order.
  //
  // It will likely be necessary to reconstruct the type before it can be used.
  // Changing the type WILL NOT invalidate existing pointers to the data, either
  // in this process or others, so changing the data structure could have
  // unpredicatable results. USE WITH CARE!
  uint32_t GetType(Reference ref) const;
  bool ChangeType(Reference ref,
                  uint32_t to_type_id,
                  uint32_t from_type_id,
                  bool clear);

  // Allocated objects can be added to an internal list that can then be
  // iterated over by other processes. If an allocated object can be found
  // another way, such as by having its reference within a different object
  // that will be made iterable, then this call is not necessary. This always
  // succeeds unless corruption is detected; check IsCorrupted() to find out.
  // Once an object is made iterable, its position in iteration can never
  // change; new iterable objects will always be added after it in the series.
  // Changing the type does not alter its "iterable" status.
  void MakeIterable(Reference ref);

  // Gets the information about the amount of free space in the allocator. The
  // amount of free space should be treated as approximate due to extras from
  // alignment and metadata. Concurrent allocations from other threads will
  // also make the true amount less than what is reported.
  void GetMemoryInfo(MemoryInfo* meminfo) const;

  // If there is some indication that the memory has become corrupted,
  // calling this will attempt to prevent further damage by indicating to
  // all processes that something is not as expected.
  // If `allow_write` is false, the corrupt bit will not be written to the data.
  void SetCorrupt(bool allow_write = true) const;

  // This can be called to determine if corruption has been detected in the
  // segment, possibly my a malicious actor. Once detected, future allocations
  // will fail and iteration may not locate all objects.
  bool IsCorrupt() const;

  // Flag set if an allocation has failed because the memory segment was full.
  bool IsFull() const;

  // Update those "tracking" histograms which do not get updates during regular
  // operation, such as how much memory is currently used. This should be
  // called before such information is to be displayed or uploaded.
  void UpdateTrackingHistograms();

  // While the above works much like malloc & free, these next methods provide
  // an "object" interface similar to new and delete.

  // Reserves space in the memory segment of the desired `size` and `type_id`.
  //
  // A return value of zero indicates the allocation failed, otherwise the
  // returned reference can be used by any process to get a real pointer via
  // the GetAsObject() or GetAsArray() calls. The actual allocated size may be
  // larger and will always be a multiple of 8 bytes (64 bits).
  Reference Allocate(size_t size,
                     uint32_t type_id,
                     size_t* alloc_size = nullptr);

  // Allocates and constructs an object in persistent memory. The type must
  // have both (size_t) kExpectedInstanceSize and (uint32_t) kPersistentTypeId
  // static constexpr fields that are used to ensure compatibility between
  // software versions. An optional size parameter can be specified to force
  // the allocation to be bigger than the size of the object; this is useful
  // when the last field is actually variable length.
  template <typename T>
  T* New(size_t size) {
    static_assert(alignof(T) <= kAllocAlignment);
    if (size < sizeof(T)) {
      size = sizeof(T);
    }
    Reference ref = Allocate(size, T::kPersistentTypeId);
    void* mem =
        const_cast<void*>(GetBlockData(ref, T::kPersistentTypeId, size));
    if (!mem) {
      return nullptr;
    }
    DCHECK_EQ(0U, reinterpret_cast<uintptr_t>(mem) & (alignof(T) - 1));
    return new (mem) T();
  }
  template <typename T>
  T* New() {
    return New<T>(sizeof(T));
  }

  // Similar to New, above, but construct the object out of an existing memory
  // block and of an expected type. If `clear` is true, memory will be zeroed
  // before construction. Though this is not standard object behavior, it
  // is present to match with new allocations that always come from zeroed
  // memory. Anything previously present simply ceases to exist; no destructor
  // is called for it so explicitly Delete() the old object first if need be.
  // Calling this will not invalidate existing pointers to the object, either
  // in this process or others, so changing the object could have unpredictable
  // results. USE WITH CARE!
  template <typename T>
  T* New(Reference ref, uint32_t from_type_id, bool clear) {
    // Make sure the memory is appropriate. This won't be used until after
    // the type is changed but checking first avoids the possibility of having
    // to change the type back.
    size_t alloc_size = 0;
    void* mem = const_cast<void*>(GetBlockData(ref, 0, sizeof(T), &alloc_size));
    if (!mem) {
      return nullptr;
    }

    DCHECK_LE(sizeof(T), alloc_size) << "alloc not big enough for obj";

    // Ensure the allocator's internal alignment is sufficient for this object.
    // This protects against coding errors in the allocator.
    DCHECK_EQ(0U, reinterpret_cast<uintptr_t>(mem) & (alignof(T) - 1));
    // Change the type, clearing the memory if so desired. The new type is
    // "transitioning" so that there is no race condition with the construction
    // of the object should another thread be simultaneously iterating over
    // data. This will "acquire" the memory so no changes get reordered before
    // it.
    if (!ChangeType(ref, kTypeIdTransitioning, from_type_id, clear)) {
      return nullptr;
    }
    // Construct an object of the desired type on this memory, just as if
    // New() had been called to create it.
    T* obj = new (mem) T();
    // Finally change the type to the desired one. This will "release" all of
    // the changes above and so provide a consistent view to other threads.
    bool success =
        ChangeType(ref, T::kPersistentTypeId, kTypeIdTransitioning, false);
    DCHECK(success);
    return obj;
  }

  // Deletes an object by destructing it and then changing the type to a
  // different value (default 0).
  template <typename T>
  void Delete(T* obj, uint32_t new_type) {
    // Get the reference for the object.
    Reference ref = GetAsReference<T>(obj);
    // First change the type to "transitioning" so there is no race condition
    // where another thread could find the object through iteration while it
    // is been destructed. This will "acquire" the memory so no changes get
    // reordered before it. It will fail if `ref` is invalid.
    if (!ChangeType(ref, kTypeIdTransitioning, T::kPersistentTypeId, false)) {
      return;
    }
    // Destruct the object.
    obj->~T();
    // Finally change the type to the desired value. This will "release" all
    // the changes above.
    bool success = ChangeType(ref, new_type, kTypeIdTransitioning, false);
    DCHECK(success);
  }
  template <typename T>
  void Delete(T* obj) {
    Delete<T>(obj, 0);
  }

  // As above but works with objects allocated from persistent memory.
  template <typename T>
  Reference GetAsReference(const T* obj) const {
    return GetAsReference(obj, T::kPersistentTypeId);
  }

  // As above but works with an object allocated from persistent memory.
  template <typename T>
  void MakeIterable(const T* obj) {
    MakeIterable(GetAsReference<T>(obj));
  }

  // Returns a string_view of a c-style string that is located at the end of an
  // allocated memory block. It is the caller's responsibility to know/ensure
  // that `object` is of some type that ends with a c-style string and that
  // said string begins at `offset` bytes from `object`. `alloc_size` must be
  // the size of the allocation, as returned by the allocator. If `object` is
  // `nullptr` or `offset >= alloc_size` then an empty string_view is returned.
  // Users should treat the returned view as a volatile bounded memory region;
  // it references the underlying shared memory, whose contents can be changed
  // or corrupted at any time.  In particular, clients should respect the
  // 'length()' of the returned view instead of relying on a terminating NUL
  // char.
  static std::string_view StringViewAt(const void* object,
                                       size_t offset,
                                       size_t alloc_size);

 protected:
  enum MemoryType {
    MEM_EXTERNAL,
    MEM_MALLOC,
    MEM_VIRTUAL,
    MEM_SHARED,
    MEM_FILE,
  };

  struct Memory {
    Memory(void* b, MemoryType t) : base(b), type(t) {}

    raw_ptr<void> base;
    MemoryType type;
  };

  // Constructs the allocator. Everything is the same as the public allocator
  // except `memory` which is a structure with additional information besides
  // the base address.
  PersistentMemoryAllocator(Memory memory,
                            size_t size,
                            size_t page_size,
                            uint64_t id,
                            std::string_view name,
                            AccessMode access_mode);

  // Implementation of Flush that accepts how much to flush.
  virtual void FlushPartial(size_t length, bool sync);

  // RAW_PTR_EXCLUSION: Never allocated by PartitionAlloc (always mmap'ed), so
  // there is no benefit to using a raw_ptr, only cost.
  RAW_PTR_EXCLUSION volatile char* const
      mem_base_;               // Memory base. (char so sizeof guaranteed 1)
  const MemoryType mem_type_;  // Type of memory allocation.
  const uint32_t mem_size_;    // Size of entire memory segment.
  const uint32_t mem_page_;    // Page size allocations shouldn't cross.
  const size_t vm_page_size_;  // The page size used by the OS.

 private:
  struct SharedMetadata;
  struct BlockHeader;
  static const Reference kReferenceQueue;

  // The shared metadata is always located at the top of the memory segment.
  // These convenience functions eliminate constant casting of the base
  // pointer within the code.
  const SharedMetadata* shared_meta() const {
    return reinterpret_cast<const SharedMetadata*>(
        const_cast<const char*>(mem_base_));
  }
  SharedMetadata* shared_meta() {
    return reinterpret_cast<SharedMetadata*>(const_cast<char*>(mem_base_));
  }

  // Actual method for doing the allocation.
  Reference AllocateImpl(size_t size, uint32_t type_id, size_t* alloc_size);

  // Dereferences a block `ref` to retrieve a pointer to the block header for
  // the reference. This method ensures that the referenced block is valid for
  // the desired `type_id` and `size`. Optionally, if `alloc_sizes` is not
  // nullptr, the validated size of the underlying allocation is returned.
  //
  // Special cases for internal use only:
  //
  // * If `queue_ok` is true and `ref` is kReferenceQueueindicates then the
  //   block header for the allocation queue is returned.
  //
  // * if `free_ok` then the block header is allowed to point to a block that
  //   may not be in the `allocated` state. This bypasses block validation.
  //
  // Because they bypass block valoidation, it is not premitted to request the
  // `alloc_size` when either of `queue_ok` or `free_ok` are true.
  const volatile BlockHeader* GetBlock(Reference ref,
                                       uint32_t type_id,
                                       size_t size,
                                       bool queue_ok,
                                       bool free_ok,
                                       size_t* alloc_size = nullptr) const;
  volatile BlockHeader* GetBlock(Reference ref,
                                 uint32_t type_id,
                                 size_t size,
                                 bool queue_ok,
                                 bool free_ok,
                                 size_t* alloc_size = nullptr) {
    return const_cast<volatile BlockHeader*>(
        const_cast<const PersistentMemoryAllocator*>(this)->GetBlock(
            ref, type_id, size, queue_ok, free_ok, alloc_size));
  }

  // Gets the actual data within a block associated with a specific reference.
  const volatile void* GetBlockData(Reference ref,
                                    uint32_t type_id,
                                    size_t size,
                                    size_t* alloc_size = nullptr) const;
  volatile void* GetBlockData(Reference ref,
                              uint32_t type_id,
                              size_t size,
                              size_t* alloc_size = nullptr) {
    return const_cast<volatile void*>(
        const_cast<const PersistentMemoryAllocator*>(this)->GetBlockData(
            ref, type_id, size, alloc_size));
  }

  // Returns the offset to the first free space segment.
  uint32_t freeptr() const;

  // Returns the metadata version used in this allocator.
  uint32_t version() const;

  const AccessMode access_mode_;

  // Local version of "corrupted" flag.
  mutable std::atomic<bool> corrupt_ = false;

  // Histogram recording used space.
  raw_ptr<HistogramBase> used_histogram_ = nullptr;

  // TODO(crbug.com/40064026): Remove these. They are used to investigate
  // unexpected failures and code paths.
  friend class DelayedPersistentAllocation;
  friend class metrics::FileMetricsProvider;
  void DumpWithoutCrashing(Reference ref,
                           uint32_t expected_type,
                           size_t expected_size,
                           bool dump_block_header) const;

  friend class PersistentMemoryAllocatorTest;
  FRIEND_TEST_ALL_PREFIXES(PersistentMemoryAllocatorTest, AllocateAndIterate);
};

// This allocator uses a local memory block it allocates from the general
// heap. It is generally used when some kind of "death rattle" handler will
// save the contents to persistent storage during process shutdown. It is
// also useful for testing.
class BASE_EXPORT LocalPersistentMemoryAllocator
    : public PersistentMemoryAllocator {
 public:
  LocalPersistentMemoryAllocator(size_t size,
                                 uint64_t id,
                                 std::string_view name);

  LocalPersistentMemoryAllocator(const LocalPersistentMemoryAllocator&) =
      delete;
  LocalPersistentMemoryAllocator& operator=(
      const LocalPersistentMemoryAllocator&) = delete;

  ~LocalPersistentMemoryAllocator() override;

 private:
  // Allocates a block of local memory of the specified `size`, ensuring that
  // the memory will not be physically allocated until accessed and will read
  // as zero when that happens.
  static Memory AllocateLocalMemory(size_t size, std::string_view name);

  // Deallocates a block of local `memory` of the specified `size`.
  static void DeallocateLocalMemory(void* memory, size_t size, MemoryType type);
};

// This allocator takes a writable shared memory mapping object and performs
// allocation from it. The allocator takes ownership of the mapping object.
class BASE_EXPORT WritableSharedPersistentMemoryAllocator
    : public PersistentMemoryAllocator {
 public:
  WritableSharedPersistentMemoryAllocator(
      base::WritableSharedMemoryMapping memory,
      uint64_t id,
      std::string_view name);

  WritableSharedPersistentMemoryAllocator(
      const WritableSharedPersistentMemoryAllocator&) = delete;
  WritableSharedPersistentMemoryAllocator& operator=(
      const WritableSharedPersistentMemoryAllocator&) = delete;

  ~WritableSharedPersistentMemoryAllocator() override;

  // Ensure that the memory isn't so invalid that it would crash when passing it
  // to the allocator. This doesn't guarantee the data is valid, just that it
  // won't cause the program to abort. The existing IsCorrupt() call will handle
  // the rest.
  static bool IsSharedMemoryAcceptable(
      const base::WritableSharedMemoryMapping& memory);

 private:
  base::WritableSharedMemoryMapping shared_memory_;
};

// This allocator takes a read-only shared memory mapping object and performs
// allocation from it. The allocator takes ownership of the mapping object.
class BASE_EXPORT ReadOnlySharedPersistentMemoryAllocator
    : public PersistentMemoryAllocator {
 public:
  ReadOnlySharedPersistentMemoryAllocator(
      base::ReadOnlySharedMemoryMapping memory,
      uint64_t id,
      std::string_view name);

  ReadOnlySharedPersistentMemoryAllocator(
      const ReadOnlySharedPersistentMemoryAllocator&) = delete;
  ReadOnlySharedPersistentMemoryAllocator& operator=(
      const ReadOnlySharedPersistentMemoryAllocator&) = delete;

  ~ReadOnlySharedPersistentMemoryAllocator() override;

  // Ensure that the memory isn't so invalid that it would crash when passing it
  // to the allocator. This doesn't guarantee the data is valid, just that it
  // won't cause the program to abort. The existing IsCorrupt() call will handle
  // the rest.
  static bool IsSharedMemoryAcceptable(
      const base::ReadOnlySharedMemoryMapping& memory);

 private:
  base::ReadOnlySharedMemoryMapping shared_memory_;
};

// NACL doesn't support any kind of file access in build.
#if !BUILDFLAG(IS_NACL)
// This allocator takes a memory-mapped file object and performs allocation
// from it. The allocator takes ownership of the file object.
class BASE_EXPORT FilePersistentMemoryAllocator
    : public PersistentMemoryAllocator {
 public:
  // A `max_size` of zero will use the length of the file as the maximum
  // size. The `file` object must have been already created with sufficient
  // permissions (read, read/write, or read/write/extend).
  FilePersistentMemoryAllocator(std::unique_ptr<MemoryMappedFile> file,
                                size_t max_size,
                                uint64_t id,
                                std::string_view name,
                                AccessMode access_mode);

  FilePersistentMemoryAllocator(const FilePersistentMemoryAllocator&) = delete;
  FilePersistentMemoryAllocator& operator=(
      const FilePersistentMemoryAllocator&) = delete;

  ~FilePersistentMemoryAllocator() override;

  // Ensure that the file isn't so invalid that it would crash when passing it
  // to the allocator. This doesn't guarantee the file is valid, just that it
  // won't cause the program to abort. The existing IsCorrupt() call will handle
  // the rest.
  static bool IsFileAcceptable(const MemoryMappedFile& file, bool read_only);

  // Load all or a portion of the file into memory for fast access. This can
  // be used to force the disk access to be done on a background thread and
  // then have the data available to be read on the main thread with a greatly
  // reduced risk of blocking due to I/O. The risk isn't eliminated completely
  // because the system could always release the memory when under pressure
  // but this can happen to any block of memory (i.e. swapped out).
  void Cache();

 protected:
  // PersistentMemoryAllocator:
  void FlushPartial(size_t length, bool sync) override;

 private:
  std::unique_ptr<MemoryMappedFile> mapped_file_;
};
#endif  // !BUILDFLAG(IS_NACL)

// An allocation that is defined but not executed until required at a later
// time. This allows for potential users of an allocation to be decoupled
// from the logic that defines it. In addition, there can be multiple users
// of the same allocation or any region thereof that are guaranteed to always
// use the same space. It's okay to copy/move these objects.
//
// This is a top-level class instead of an inner class of the PMA so that it
// can be forward-declared in other header files without the need to include
// the full contents of this file.
class BASE_EXPORT DelayedPersistentAllocation {
 public:
  using Reference = PersistentMemoryAllocator::Reference;

  // Creates a delayed allocation using the specified `allocator`. When
  // needed, the memory will be allocated using the specified `type` and
  // `size`. If `offset` is given, the returned pointer will be at that
  // offset into the segment; this allows combining allocations into a
  // single persistent segment to reduce overhead and means an "all or
  // nothing" request. Note that `size` is always the total memory size
  // and `offset` is just indicating the start of a block within it.
  //
  // Once allocated, a reference to the segment will be stored at `ref`.
  // This shared location must be initialized to zero (0); it is checked
  // with every Get() request to see if the allocation has already been
  // done. If reading `ref` outside of this object, be sure to do an
  // "acquire" load. Don't write to it -- leave that to this object.
  DelayedPersistentAllocation(PersistentMemoryAllocator* allocator,
                              std::atomic<Reference>* ref,
                              uint32_t type,
                              size_t size,
                              size_t offset = 0);
  ~DelayedPersistentAllocation();

  // Gets a span to the defined allocation. This will realize the request
  // and update the reference provided during construction. The memory will
  // be zeroed the first time it is returned, after that it is shared with
  // all other Get() requests and so shows any changes made to it elsewhere.
  //
  // If the allocation fails for any reason, an empty span will be returned.
  // This works even on "const" objects because the allocation is already
  // defined, just delayed.
  template <typename T>
  span<T> Get() const {
    // PersistentMemoryAllocator only supports types with alignment at most
    // kAllocAlignment.
    static_assert(alignof(T) <= PersistentMemoryAllocator::kAllocAlignment);
    // The offset must be a multiple of the alignment or misaligned pointers
    // will result.
    CHECK_EQ(offset_ % alignof(T), 0u);
    span<uint8_t> untyped = GetUntyped();
    return UNSAFE_TODO(
        span(reinterpret_cast<T*>(untyped.data()), untyped.size() / sizeof(T)));
  }

  // Gets the internal reference value. If this returns a non-zero value then
  // a subsequent call to Get() will do nothing but convert that reference into
  // a memory location -- useful for accessing an existing allocation without
  // creating one unnecessarily.
  Reference reference() const {
    return reference_->load(std::memory_order_relaxed);
  }

 private:
  span<uint8_t> GetUntyped() const;

  // The underlying object that does the actual allocation of memory. Its
  // lifetime must exceed that of all DelayedPersistentAllocation objects
  // that use it.
  const raw_ptr<PersistentMemoryAllocator> allocator_;

  // The desired type and size of the allocated segment plus the offset
  // within it for the defined request.
  const uint32_t type_;
  const uint32_t size_;
  const uint32_t offset_;

  // The location at which a reference to the allocated segment is to be
  // stored once the allocation is complete. If multiple delayed allocations
  // share the same pointer then an allocation on one will amount to an
  // allocation for all.
  const raw_ptr<volatile std::atomic<Reference>, AllowPtrArithmetic> reference_;

  // No DISALLOW_COPY_AND_ASSIGN as it's okay to copy/move these objects.
};

}  // namespace base

#endif  // BASE_METRICS_PERSISTENT_MEMORY_ALLOCATOR_H_
