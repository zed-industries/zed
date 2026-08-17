// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_BINDER_H_
#define BASE_ANDROID_BINDER_H_

#include <android/binder_ibinder.h>
#include <android/binder_parcel.h>
#include <android/binder_status.h>
#include <jni.h>

#include <cstdint>
#include <type_traits>
#include <utility>
#include <vector>

#include "base/android/scoped_java_ref.h"
#include "base/base_export.h"
#include "base/check.h"
#include "base/containers/span.h"
#include "base/files/scoped_file.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/ref_counted.h"
#include "base/numerics/safe_conversions.h"
#include "base/synchronization/lock.h"
#include "base/types/expected.h"
#include "base/types/expected_macros.h"

// DEFINE_BINDER_CLASS() generates a definition for a unique binder class.
// Binder classes are used by the binder implementation to enforce a kind of
// type safety, requiring client IBinders to be associated with the same class
// as the remote object's original IBinder.
//
// Objects implementing SupportsBinder<T> must specify such a class as the T;
// and clients wishing to perform transactions against such objects must use a
// TypedBinderRef<T> to do so.
//
// See usage comments on SupportsBinder<T> below.
#define _BINDER_CLASS_LINE(line) _BINDER_CLASS_LINE2(line)
#define _BINDER_CLASS_LINE2(line) #line
#define DEFINE_BINDER_CLASS(name)                                           \
  struct name : public base::android::internal::BinderClassBase {           \
    using BinderRef = base::android::TypedBinderRef<name>;                  \
    static inline AIBinder_Class* GetBinderClass() {                        \
      static AIBinder_Class* const clazz = RegisterBinderClass(             \
          #name ":" __FILE__ ":" _BINDER_CLASS_LINE(__LINE__));             \
      return clazz;                                                         \
    }                                                                       \
    static inline base::android::TypedBinderRef<name> AdoptBinderRef(       \
        base::android::BinderRef binder) {                                  \
      return base::android::TypedBinderRef<name>::Adopt(std::move(binder)); \
    }                                                                       \
  }

namespace base::android {

class BinderRef;
class Parcel;

template <typename T>
using BinderStatusOr = expected<T, binder_status_t>;

// Provides a read-only view into a AParcel. Does not retain ownership of the
// AParcel, which must outlive this object.
class BASE_EXPORT ParcelReader {
 public:
  explicit ParcelReader(const AParcel* parcel);
  explicit ParcelReader(const Parcel& parcel);
  ParcelReader(const ParcelReader&);
  ParcelReader& operator=(const ParcelReader&);
  ~ParcelReader();

  // A subset of the NDK functions defined for reading from an AParcel. Others
  // may be exposed here as needed.
  BinderStatusOr<BinderRef> ReadBinder() const;
  BinderStatusOr<int32_t> ReadInt32() const;
  BinderStatusOr<uint32_t> ReadUint32() const;
  BinderStatusOr<uint64_t> ReadUint64() const;
  BinderStatusOr<ScopedFD> ReadFileDescriptor() const;

  // Reads a byte array from the parcel. `allocator` is called with a single
  // size_t argument for the number of bytes in the array and must return a
  // pointer to at least that much memory, into which ReadByteArray() will copy
  // the array data before returning. If the parcel contains an empty or null
  // byte array, `allocator` is not invoked. If `allocator` is invoked and
  // returns null, ReadByteArray() returns an error.
  template <typename Allocator>
  BinderStatusOr<void> ReadByteArray(Allocator allocator) const {
    auto c_allocator = [](void* context, int32_t length, int8_t** out) {
      const auto& allocator = *static_cast<Allocator*>(context);
      const auto size = saturated_cast<size_t>(length);
      if (!size) {
        *out = nullptr;
        return true;
      }

      // Binder API wants int8_t for bytes, but we generally use uint8_t.
      uint8_t* const data = allocator(size);
      *out = reinterpret_cast<int8_t*>(data);
      return !!data;
    };
    return ReadByteArrayImpl(c_allocator, &allocator);
  }

 private:
  BinderStatusOr<void> ReadByteArrayImpl(AParcel_byteArrayAllocator allocator,
                                         void* context) const;

  raw_ptr<const AParcel> parcel_;
};

// Provides a writable view into a AParcel. Does not retain ownership of the
// AParcel, which must outlive this object.
class BASE_EXPORT ParcelWriter {
 public:
  explicit ParcelWriter(AParcel* parcel);
  explicit ParcelWriter(Parcel& parcel);
  ParcelWriter(const ParcelWriter&);
  ParcelWriter& operator=(const ParcelWriter&);
  ~ParcelWriter();

  // A subset of the NDK functions defined for writing to an AParcel. Others may
  // be exposed here as needed.
  BinderStatusOr<void> WriteBinder(BinderRef binder) const;
  BinderStatusOr<void> WriteInt32(int32_t value) const;
  BinderStatusOr<void> WriteUint32(uint32_t value) const;
  BinderStatusOr<void> WriteUint64(uint64_t value) const;
  BinderStatusOr<void> WriteByteArray(span<const uint8_t> bytes) const;
  BinderStatusOr<void> WriteFileDescriptor(ScopedFD fd) const;

 private:
  raw_ptr<AParcel> parcel_;
};

// Wraps unique ownership of an AParcel. This is similar to the NDK's
// ScopedAParcel, but it uses our internal BinderApi to invoke NDK functions.
class BASE_EXPORT Parcel {
 public:
  Parcel();
  explicit Parcel(AParcel* parcel);
  Parcel(Parcel&& other);
  Parcel& operator=(Parcel&& other);
  ~Parcel();

  explicit operator bool() const { return parcel_ != nullptr; }

  const AParcel* get() const { return parcel_; }
  AParcel* get() { return parcel_; }
  [[nodiscard]] AParcel* release() { return std::exchange(parcel_, nullptr); }

  void reset();

  ParcelReader reader() const { return ParcelReader(*this); }
  ParcelWriter writer() { return ParcelWriter(*this); }

 private:
  raw_ptr<AParcel> parcel_ = nullptr;
};

// A BinderRef owns a strong ref-count on an AIBinder. This is like the NDK's
// SpAIBinder, but it uses our internal BinderApi to invoke NDK functions.
class BASE_EXPORT BinderRef {
 public:
  BinderRef();
  explicit BinderRef(AIBinder* binder);
  BinderRef(const BinderRef& other);
  BinderRef& operator=(const BinderRef& other);
  BinderRef(BinderRef&& other);
  BinderRef& operator=(BinderRef&& other);
  ~BinderRef();

  explicit operator bool() const { return binder_ != nullptr; }

  AIBinder* get() const { return binder_; }
  [[nodiscard]] AIBinder* release() { return std::exchange(binder_, nullptr); }

  void reset();

  // Returns a new strong reference to this binder as a local Java object
  // reference.
  ScopedJavaLocalRef<jobject> ToJavaBinder(JNIEnv* env) const;

  // Returns a new strong reference to an existing Java binder as a BinderRef.
  static BinderRef FromJavaBinder(JNIEnv* env, jobject java_binder);

  // Attempts to associate this binder with `binder_class`. Generally should be
  // used via TypedBinderRef<T>::Adopt() or the equivalent T::AdoptBinderRef()
  // for some binder class T.
  bool AssociateWithClass(AIBinder_Class* binder_class);

 protected:
  // Protected to force usage through a strongly typed subclass, ensuring that
  // transaction clients have an associated binder class. See documentation on
  // TypedBinderRef<T> below.
  BinderStatusOr<Parcel> PrepareTransaction();
  BinderStatusOr<Parcel> TransactImpl(transaction_code_t code,
                                      Parcel parcel,
                                      binder_flags_t flags);

 protected:
  raw_ptr<AIBinder> binder_ = nullptr;
};

namespace internal {

// Base class for classes generated by DEFINE_BINDER_CLASS().
class BASE_EXPORT BinderClassBase {
 public:
  static AIBinder_Class* RegisterBinderClass(const char* descriptor);
};

// Common implementation for SupportsBinder<T> below. Instances of this base
// class handle IBinder callbacks and forward events for destruction and
// incoming transactions to a templated subclass.
class BASE_EXPORT SupportsBinderBase
    : public RefCountedThreadSafe<SupportsBinderBase> {
 public:
  explicit SupportsBinderBase(AIBinder_Class* binder_class);

  // Called for every incoming transaction on the underlying IBinder. Note that
  // this is called from the binder thread pool so implementations must be
  // thread-safe.
  virtual BinderStatusOr<void> OnBinderTransaction(transaction_code_t code,
                                                   const ParcelReader& in,
                                                   const ParcelWriter& out) = 0;

  // Called any time the underlying IBinder is destroyed. Note that this may be
  // invoked multiple times, as the underlying IBinder exists only as long as
  // there are living BinderRefs referencing this object. If BinderRefs are
  // created and then all destroyed, this will be invoked once. If subsequent
  // BinderRefs are created and then all destroyed, this will be invoked again.
  //
  // Similar to OnBinderTransaction, this is invoked from the binder thread pool
  // and implementations must be thread-safe.
  virtual void OnBinderDestroyed();

 protected:
  friend class RefCountedThreadSafe<SupportsBinderBase>;
  friend class BinderClassBase;

  virtual ~SupportsBinderBase();

  // Creates a strong reference to the underlying IBinder, allocating a new
  // IBinder if one did not already exist for this object.
  BinderRef GetBinder();

 private:
  void OnBinderDestroyedBase();

  // Binder class callbacks.
  static void* OnIBinderCreate(void* self);
  static void OnIBinderDestroy(void* self);
  static binder_status_t OnIBinderTransact(AIBinder* binder,
                                           transaction_code_t code,
                                           const AParcel* in,
                                           AParcel* out);

  const raw_ptr<AIBinder_Class> binder_class_;

  Lock lock_;

  // A weak reference to the underlying IBinder, if one exists.
  raw_ptr<AIBinder_Weak> weak_binder_ GUARDED_BY(lock_) = nullptr;

  // As long as any IBinder is alive for this object, we retain an extra ref
  // count on `this` to ensure that transactions can be handled safely.
  scoped_refptr<SupportsBinderBase> self_for_binder_ GUARDED_BY(lock_);
};

}  // namespace internal

// A BinderRef which has been associated with a specific binder class.
template <typename T>
class TypedBinderRef : public BinderRef {
 public:
  static_assert(std::is_base_of_v<android::internal::BinderClassBase, T>,
                "Invalid binder class type");
  TypedBinderRef() = default;

  // Asserts that the binder can be associated with class T. This is safe to
  // call when it's known that the binder hasn't been associated with any other
  // class in the calling process yet.
  explicit TypedBinderRef(BinderRef binder) {
    CHECK(!binder || binder.AssociateWithClass(T::GetBinderClass()));
    binder_ = binder.release();
  }

  TypedBinderRef(const TypedBinderRef&) = default;
  TypedBinderRef& operator=(const TypedBinderRef&) = default;
  TypedBinderRef(TypedBinderRef&&) = default;
  TypedBinderRef& operator=(TypedBinderRef&&) = default;
  ~TypedBinderRef() = default;

  // Adopts a BinderRef that is not already associated with another binder
  // class, associating it with T. If `binder` is already associated with T this
  // is a no-op which only narrows the ref type.
  //
  // If `binder` was already associated with a binder class other than T, the
  // reference is dropped and this returns null.
  //
  // For convenience clients may instead prefer to call this method via
  // T::AdoptBinderRef() as defined by DEFINE_BINDER_CLASS(T).
  static TypedBinderRef<T> Adopt(BinderRef binder) {
    TypedBinderRef<T> typed_binder;
    if (binder.AssociateWithClass(T::GetBinderClass())) {
      typed_binder.binder_ = binder.release();
    }
    return typed_binder;
  }

  // Prepares a new transaction on this binder, returning a Parcel that can be
  // populated and then sent via Transact() or TransactOneWay() below.
  BinderStatusOr<Parcel> PrepareTransaction() {
    return BinderRef::PrepareTransaction();
  }

  // Transact with a `parcel` created by a call to PrepareTransaction() on the
  // same binder. Returns the output parcel from the transaction. `code` is
  // an arbitrary value with interface-specific meaning.
  BinderStatusOr<Parcel> Transact(transaction_code_t code, Parcel parcel) {
    return TransactImpl(code, std::move(parcel), /*flags=*/0);
  }

  // Like Transact(), but this internally prepares a transacation and passes the
  // allocated Parcel into `fn`. After `fn` returns the Parcel is transacted.
  template <typename Fn>
  BinderStatusOr<Parcel> Transact(transaction_code_t code, Fn fn) {
    ASSIGN_OR_RETURN(auto parcel, PrepareTransaction());
    RETURN_IF_ERROR(fn(ParcelWriter(parcel.get())));
    return Transact(code, std::move(parcel));
  }

  // Like Transact() but asynchronous. Discards the empty response parcel.
  BinderStatusOr<void> TransactOneWay(transaction_code_t code, Parcel parcel) {
    RETURN_IF_ERROR(TransactImpl(code, std::move(parcel), FLAG_ONEWAY));
    return ok();
  }

  // Like TransactOneWay(), but this internally prepares a transaction
  // passes the allocated Parcel into `fn`. After `fn` returns the Parcel is
  // transacted.
  template <typename Fn>
  BinderStatusOr<void> TransactOneWay(transaction_code_t code, Fn fn) {
    ASSIGN_OR_RETURN(auto parcel, PrepareTransaction());
    RETURN_IF_ERROR(fn(ParcelWriter(parcel.get())));
    return TransactOneWay(code, std::move(parcel));
  }
};

// Base class for objects which support native binder transactions. Example
// usage:
//
//   // In some common header.
//   DEFINE_BINDER_CLASS(ThingyInterface);
//
//   // The interface implementation.
//   class Thingy : public base::android::SupportsBinder<ThingyInterface> {
//    public:
//     ... (normal class stuff, plus overrides of SupportsBinder methods)
//   };
//
//   // The client. `ref` generally comes from the parent process untyped,
//   // specifically from some SupportsBinder<T> subclass calling GetBinder().
//   void UseThingy(BinderRef ref) {
//     auto thingy = ThingyInterface::AdoptBinderRef(std::move(ref));
//     ... (do transactions with `thingy`)
//   }
template <typename T>
class BASE_EXPORT SupportsBinder : public internal::SupportsBinderBase {
 public:
  static_assert(std::is_base_of_v<android::internal::BinderClassBase, T>,
                "Invalid binder class type");

  SupportsBinder() : SupportsBinderBase(T::GetBinderClass()) {}

  // Creates a strong reference to the underlying IBinder, allocating a new
  // IBinder if one did not already exist for this object.
  TypedBinderRef<T> GetBinder() {
    return TypedBinderRef<T>(SupportsBinderBase::GetBinder());
  }

 protected:
  ~SupportsBinder() override = default;
};

// Indicates whether Binder NDK functionality is generally available to the
// caller. If this returns false, BinderRefs will always be null and
// SupportsBinder<T> implementations will never receive binder transactions; but
// definitions within this header are otherwise still safe to reference and use.
BASE_EXPORT bool IsNativeBinderAvailable();

// Stashes a global collection of BinderRefs for later retrieval by
// TakeBinderFromParent(). This is intended for use by generic multiprocess
// support code to retain interfaces from the parent process so application-
// specific logic in the child process can retrieve them later. It should be
// called at most once per process, and as early as possible.
BASE_EXPORT void SetBindersFromParent(std::vector<BinderRef> binders);

// Retrieves (by index) a BinderRef which was stashed earlier by
// SetBindersFromParent(). If there is no binder for the given index, the
// returned BinderRef is null. This consumes the binder for that index, so
// subsequent calls for the same index will always return null.
BASE_EXPORT BinderRef TakeBinderFromParent(size_t index);

}  // namespace base::android

#endif  // BASE_ANDROID_BINDER_H_
