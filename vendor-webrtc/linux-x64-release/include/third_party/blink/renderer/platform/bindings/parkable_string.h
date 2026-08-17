// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_PARKABLE_STRING_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_PARKABLE_STRING_H_

#include <memory>
#include <utility>

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "base/gtest_prod_util.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/synchronization/lock.h"
#include "base/thread_annotations.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/platform/bindings/buildflags.h"
#include "third_party/blink/renderer/platform/disk_data_metadata.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/size_assertions.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/threading.h"

// ParkableString represents a string that may be parked in memory, that it its
// underlying memory address may change. Its content can be retrieved with the
// |ToString()| method.
// As a consequence, the inner pointer should never be cached, and only touched
// through a string returned by the |ToString()| method.
// It is safe to call `ToString()` and destroy ParkableStrings from any thread,
// although the interactions with the ParkableStringManager must always be
// performed on the main thread.
namespace blink {

class Digestor;
class DiskDataAllocator;
class WebProcessMemoryDump;
struct BackgroundTaskParams;

// A parked string is parked by calling |Park()|, and unparked by calling
// |ToString()| on a parked string.
// |Lock()| does *not* unpark a string.
class PLATFORM_EXPORT ParkableStringImpl
    : public WTF::ThreadSafeRefCounted<ParkableStringImpl> {
 public:
  enum class ParkingMode {
    kSynchronousOnly,
    kCompress,
    kToDisk,
    kCompressThenToDisk
  };
  enum class AgeOrParkResult {
    kSuccessOrTransientFailure,
    kNonTransientFailure
  };
  enum class Age { kYoung = 0, kOld = 1, kVeryOld = 2 };
  enum class CompressionAlgorithm {
    kZlib = 0,
    kSnappy = 1,
#if BUILDFLAG(HAS_ZSTD_COMPRESSION)
    kZstd = 2
#endif
  };

  constexpr static size_t kDigestSize = 32;  // SHA256.
  using SecureDigest = Vector<uint8_t, kDigestSize>;
  // Computes a secure hash of a |string|, to be passed to |MakeParkable()|.
  //
  // TODO(lizeb): This is the "right" way of hashing a string. Move this code
  // into WTF, and make sure it's the only way that is used.
  static std::unique_ptr<SecureDigest> HashString(StringImpl* string);
  // Updates a digest to include the string width. This should be called after
  // the Digestor has consumed all of the bytes of a string. Afterward, the
  // digest can be used in MakeParkable.
  static void UpdateDigestWithEncoding(Digestor* digestor, bool is_8bit);

  // Not all ParkableStringImpls are actually parkable.
  static scoped_refptr<ParkableStringImpl> MakeNonParkable(
      scoped_refptr<StringImpl>&& impl);
  // |digest| is as returned by |HashString()|, hence not nullptr.
  static scoped_refptr<ParkableStringImpl> MakeParkable(
      scoped_refptr<StringImpl>&& impl,
      std::unique_ptr<SecureDigest> digest);

  static CompressionAlgorithm GetCompressionAlgorithm();

  ParkableStringImpl(const ParkableStringImpl&) = delete;
  ParkableStringImpl& operator=(const ParkableStringImpl&) = delete;

  void Lock();
  void Unlock();

  // The returned string may be used as a normal one, as long as the
  // returned value (or a copy of it) is alive.
  const String& ToString();

  // See the matching String methods.
  bool is_8bit() const {
    if (!may_be_parked())
      return string_.Is8Bit();

    return metadata_->is_8bit_;
  }
  unsigned length() const {
    if (!may_be_parked())
      return string_.length();

    return metadata_->length_;
  }
  size_t CharactersSizeInBytes() const;

  size_t MemoryFootprintForDump() const;

  struct MemoryUsage {
    size_t this_size;
    raw_ptr<const void> string_impl;
    size_t string_impl_size;
  };
  MemoryUsage MemoryUsageForSnapshot() const;

  // Returns true iff the string can be parked. This does not mean that the
  // string can be parked now, merely that it is eligible to be parked at some
  // point.
  bool may_be_parked() const { return !!metadata_; }

  // Note: Public member functions below must only be called on strings for
  // which |may_be_parked()| returns true. Otherwise, these will either trigger
  // a DCHECK() or crash.

  // Tries to either age or park a string:
  //
  // - If the string is already old, tries to park it.
  // - If it is very old and parked, tries to write it to disk.
  // - Otherwise, tries to age it.
  //
  // The action doesn't necessarily succeed. either due to a temporary
  // or potentially lasting condition.
  //
  // As parking may be synchronous, this can call back into
  // ParkableStringManager.
  AgeOrParkResult MaybeAgeOrParkString();

  // A parked string cannot be accessed until it has been |Unpark()|-ed.
  //
  // Parking may be synchronous, and will be if compressed data is already
  // available. If |mode| is |kIfCompressedDataExists|, then parking will always
  // be synchronous.
  //
  // Must not be called if |may_be_parked()| returns false.
  //
  // Returns true if the string is being parked or has been parked.
  bool Park(ParkingMode mode);

  // Returns true if the string is parked, takes the lock inside.
  bool is_parked() const LOCKS_EXCLUDED(metadata_->lock_);
  bool is_on_disk() const LOCKS_EXCLUDED(metadata_->lock_);

  // Returns whether synchronous parking is possible, that is the string was
  // parked in the past.
  bool has_compressed_data() const { return !!metadata_->compressed_; }
  bool has_on_disk_data() const { return !!metadata_->on_disk_metadata_; }

  // Returns the compressed size, must not be called unless the string has a
  // compressed representation.
  size_t compressed_size() const {
    DCHECK(has_compressed_data());
    return metadata_->compressed_->size();
  }

  // Returns the on-disk size, must not be called unless the string has data
  // on-disk.
  size_t on_disk_size() const {
    DCHECK(has_on_disk_data());
    return metadata_->on_disk_metadata_->size();
  }

  Age age_for_testing() {
    base::AutoLock locker(metadata_->lock_);
    return metadata_->age_;
  }

  bool background_task_in_progress_for_testing() const {
    return metadata_->background_task_in_progress_;
  }

  const SecureDigest* digest() const {
    AssertOnValidThread();
    DCHECK(metadata_);
    return &metadata_->digest_;
  }

  void Release() const LOCKS_EXCLUDED(metadata_->lock_) {
    if (!may_be_parked()) {
      if (RefCountedThreadSafeBase::Release()) {
        delete this;
      }
      return;
    }
    base::ReleasableAutoLock locker(&metadata_->lock_);
    if (HasOneRef()) {
      // Release the lock early because, if we are in the main thread,
      // `ParkableStringManager::RemoveOnMainThread()` will try to take the lock
      // inside a locked scope.
      locker.Release();
      ReleaseAndRemoveIfNeeded();
      return;
    }
    RefCountedThreadSafeBase::Release();
  }

 private:
  enum class State : uint8_t;
  enum class Status : uint8_t;
  friend class ParkableStringManager;

  // |digest| is as returned by calling HashString() on |impl|, or nullptr for
  // a non-parkable instance.
  ParkableStringImpl(scoped_refptr<StringImpl>&& impl,
                     std::unique_ptr<SecureDigest> digest);

  ~ParkableStringImpl();

  // Note: Private member  functions below must only be called on strings for
  // which |may_be_parked()| returns true. Otherwise, these will either trigger
  // a DCHECK() or crash.

  // Doesn't make the string young. May be called from any thread.
  void LockWithoutMakingYoung() {
    base::AutoLock locker(metadata_->lock_);
    metadata_->lock_depth_ += 1;
  }

  void MakeYoung() EXCLUSIVE_LOCKS_REQUIRED(metadata_->lock_) {
    metadata_->age_ = Age::kYoung;
  }
  // Whether the string is referenced or locked. The return value is valid as
  // long as |lock_| is held.
  Status CurrentStatus() const EXCLUSIVE_LOCKS_REQUIRED(metadata_->lock_);
  bool CanParkNow() const EXCLUSIVE_LOCKS_REQUIRED(metadata_->lock_);
  bool ParkInternal(ParkingMode mode)
      EXCLUSIVE_LOCKS_REQUIRED(metadata_->lock_);
  void Unpark() EXCLUSIVE_LOCKS_REQUIRED(metadata_->lock_);
  String UnparkInternal() EXCLUSIVE_LOCKS_REQUIRED(metadata_->lock_);

  // Called by `Release()` when the ref count would reach 0 to post or execute
  // the removal of the entry from the `ParkableStringManager` on the Main
  // thread. The removal can be cancelled if the Main Thread takes a new
  // reference on the string before the posted task is executed.
  void ReleaseAndRemoveIfNeeded() const;

  void PostBackgroundCompressionTask(ParkingMode mode);
  static void CompressInBackground(std::unique_ptr<BackgroundTaskParams>);
  // Called on the main thread after compression is done.
  // |params| is the same as the one passed to
  // |PostBackgroundCompressionTask()|,
  // |compressed| is the compressed data, nullptr if compression failed.
  // |parking_thread_time| is the CPU time used by the background compression
  // task.
  void OnParkingCompleteOnMainThread(
      std::unique_ptr<BackgroundTaskParams> params,
      std::unique_ptr<Vector<uint8_t>> compressed,
      base::TimeDelta parking_thread_time);

  void PostBackgroundWritingTask(std::unique_ptr<ReservedChunk> reserved_chunk)
      EXCLUSIVE_LOCKS_REQUIRED(metadata_->lock_);
  static void WriteToDiskInBackground(std::unique_ptr<BackgroundTaskParams>,
                                      DiskDataAllocator* data_allocator);
  // Called on the main thread after writing is done.
  // |params| is the same as the one passed to PostBackgroundWritingTask()|,
  // |metadata| is the on-disk metadata, nullptr if writing failed.
  // |writing_time| is the elapsed background thread time used by disk writing.
  void OnWritingCompleteOnMainThread(
      std::unique_ptr<BackgroundTaskParams> params,
      std::unique_ptr<DiskDataMetadata> metadata,
      base::TimeDelta writing_time);

  void DiscardUncompressedData() EXCLUSIVE_LOCKS_REQUIRED(metadata_->lock_);
  void DiscardCompressedData() EXCLUSIVE_LOCKS_REQUIRED(metadata_->lock_);

  int lock_depth_for_testing() {
    base::AutoLock locker_(metadata_->lock_);
    return metadata_->lock_depth_;
  }

  // Returns true if the string is parked. Doesn't take the lock inside but
  // expects it to be held before entering.
  bool is_parked_no_lock() const EXCLUSIVE_LOCKS_REQUIRED(metadata_->lock_);
  bool is_on_disk_no_lock() const EXCLUSIVE_LOCKS_REQUIRED(metadata_->lock_);

  bool is_compression_failed_no_lock() const
      EXCLUSIVE_LOCKS_REQUIRED(metadata_->lock_);

  // Metadata only used for parkable ParkableStrings.
  struct ParkableMetadata {
    ParkableMetadata(String string, std::unique_ptr<SecureDigest> digest);
    ParkableMetadata(const ParkableMetadata&) = delete;
    ParkableMetadata& operator=(const ParkableMetadata&) = delete;

    // `lock_` protects access to the metadata and prevents concurrent
    // execution of parking and unparking operations.
    base::Lock lock_;
    unsigned int lock_depth_ GUARDED_BY(lock_);

    State state_ GUARDED_BY(lock_);
    bool background_task_in_progress_{false};
    bool compression_failed_ GUARDED_BY(lock_);
    std::unique_ptr<Vector<uint8_t>> compressed_;
    std::unique_ptr<DiskDataMetadata> on_disk_metadata_;
    const SecureDigest digest_;
    base::TimeTicks last_disk_parking_time_;

    // A string can be young, old or very old. It starts young, and ages with
    // |MaybeAgeOrParkString()|.
    //
    // Transitions are:
    // Young -> Old -> Very old: By calling |MaybeAgeOrParkString()|.
    // (Old | Very Old) -> Young: When the string is accessed, either by
    //                            |Lock()|-ing it or calling |ToString()|.
    //
    // Thread safety: it is typically not safe to guard only one part of a
    // bitfield with a mutex, but this is correct here, as the other members are
    // const (and never change).
    Age age_ : 3 GUARDED_BY(lock_);
    const bool is_8bit_ : 1;
    const unsigned length_;
  };

  // Access to `string_` is guarded by `metadata_->lock_` with 2 exceptions:
  // 1. There is no lock in unparkable ParkableStringImpls.
  // 2. Concurrent `AsanPoisonString()` and `AsanUnpoisonString()` are
  // prevented through lock levels.
  String string_;
  const std::unique_ptr<ParkableMetadata> metadata_;

#if DCHECK_IS_ON()
  const base::PlatformThreadId owning_thread_;
#endif

  void AssertOnValidThread() const {
#if DCHECK_IS_ON()
    DCHECK_EQ(owning_thread_, CurrentThread());
#endif
  }

 public:
  FRIEND_TEST_ALL_PREFIXES(ParkableStringTest, Equality);
  FRIEND_TEST_ALL_PREFIXES(ParkableStringTest, EqualityNoUnparking);
  FRIEND_TEST_ALL_PREFIXES(ParkableStringTest, LockUnlock);
  FRIEND_TEST_ALL_PREFIXES(ParkableStringTest, LockParkedString);
  FRIEND_TEST_ALL_PREFIXES(ParkableStringTest, ReportMemoryDump);
  FRIEND_TEST_ALL_PREFIXES(ParkableStringTest, MemoryFootprintForDump);
};

#if !DCHECK_IS_ON()
// 3 pointers:
// - vtable (from RefCounted)
// - string_.Impl()
// - metadata_
ASSERT_SIZE(ParkableStringImpl, void* [3]);
#endif

class PLATFORM_EXPORT ParkableString final {
  DISALLOW_NEW();

 public:
  ParkableString() : impl_(nullptr) {}
  explicit ParkableString(scoped_refptr<StringImpl>&& impl);
  ParkableString(scoped_refptr<StringImpl>&& impl,
                 std::unique_ptr<ParkableStringImpl::SecureDigest> digest);
  ParkableString(const ParkableString& rhs) : impl_(rhs.impl_) {}
  ~ParkableString();

  // Locks a string. A string is unlocked when the number of Lock()/Unlock()
  // calls match. A locked string cannot be parked.
  // Can be called from any thread.
  void Lock() const;

  // Unlocks a string.
  // Can be called from any thread.
  void Unlock() const;

  void OnMemoryDump(WebProcessMemoryDump* pmd, const String& name) const;

  // See the matching String methods.
  bool Is8Bit() const;
  bool IsNull() const { return !impl_; }
  unsigned length() const { return impl_ ? impl_->length() : 0; }
  bool may_be_parked() const { return impl_ && impl_->may_be_parked(); }

  ParkableStringImpl* Impl() const { return impl_ ? impl_.get() : nullptr; }
  // Returns an unparked version of the string.
  // The string is guaranteed to be valid for
  // max(lifetime of a copy of the returned reference, current thread task).
  const String& ToString() const;
  size_t CharactersSizeInBytes() const;

  // Causes the string to be unparked. Note that the pointer must not be
  // cached.
  base::span<const char> SpanChar() const {
    return base::as_chars(ToString().Span8());
  }
  const base::span<const uint16_t> SpanUint16() const {
    return ToString().SpanUint16();
  }

 private:
  scoped_refptr<ParkableStringImpl> impl_;
};

static_assert(sizeof(ParkableString) == sizeof(void*),
              "ParkableString should be small");

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_PARKABLE_STRING_H_
