// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_MEMORY_REF_COUNTED_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_MEMORY_REF_COUNTED_H_

#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_alloc_base/atomic_ref_count.h"
#include "partition_alloc/partition_alloc_base/check.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_base/memory/scoped_refptr.h"

namespace partition_alloc::internal::base {
namespace subtle {

class PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) RefCountedThreadSafeBase {
 public:
  RefCountedThreadSafeBase(const RefCountedThreadSafeBase&) = delete;
  RefCountedThreadSafeBase& operator=(const RefCountedThreadSafeBase&) = delete;

  bool HasOneRef() const;
  bool HasAtLeastOneRef() const;

 protected:
  explicit constexpr RefCountedThreadSafeBase(StartRefCountFromZeroTag) {}
  explicit constexpr RefCountedThreadSafeBase(StartRefCountFromOneTag)
      : ref_count_(1) {
#if PA_BUILDFLAG(DCHECKS_ARE_ON)
    needs_adopt_ref_ = true;
#endif
  }

#if PA_BUILDFLAG(DCHECKS_ARE_ON)
  ~RefCountedThreadSafeBase();
#else
  ~RefCountedThreadSafeBase() = default;
#endif

// Release and AddRef are suitable for inlining on X86 because they generate
// very small code sequences. On other platforms (ARM), it causes a size
// regression and is probably not worth it.
#if PA_BUILDFLAG(PA_ARCH_CPU_X86_FAMILY)
  // Returns true if the object should self-delete.
  bool Release() const { return ReleaseImpl(); }
  void AddRef() const { AddRefImpl(); }
  void AddRefWithCheck() const { AddRefWithCheckImpl(); }
#else
  // Returns true if the object should self-delete.
  bool Release() const;
  void AddRef() const;
  void AddRefWithCheck() const;
#endif

 private:
  template <typename U>
  friend scoped_refptr<U> AdoptRef(U*);

  void Adopted() const {
#if PA_BUILDFLAG(DCHECKS_ARE_ON)
    PA_BASE_DCHECK(needs_adopt_ref_);
    needs_adopt_ref_ = false;
#endif
  }

  PA_ALWAYS_INLINE void AddRefImpl() const {
#if PA_BUILDFLAG(DCHECKS_ARE_ON)
    PA_BASE_DCHECK(!in_dtor_);
    // This RefCounted object is created with non-zero reference count.
    // The first reference to such a object has to be made by AdoptRef or
    // MakeRefCounted.
    PA_BASE_DCHECK(!needs_adopt_ref_);
#endif
    ref_count_.Increment();
  }

  PA_ALWAYS_INLINE void AddRefWithCheckImpl() const {
#if PA_BUILDFLAG(DCHECKS_ARE_ON)
    PA_BASE_DCHECK(!in_dtor_);
    // This RefCounted object is created with non-zero reference count.
    // The first reference to such a object has to be made by AdoptRef or
    // MakeRefCounted.
    PA_BASE_DCHECK(!needs_adopt_ref_);
#endif
    PA_BASE_CHECK(ref_count_.Increment() > 0);
  }

  PA_ALWAYS_INLINE bool ReleaseImpl() const {
#if PA_BUILDFLAG(DCHECKS_ARE_ON)
    PA_BASE_DCHECK(!in_dtor_);
    PA_BASE_DCHECK(!ref_count_.IsZero());
#endif
    if (!ref_count_.Decrement()) {
#if PA_BUILDFLAG(DCHECKS_ARE_ON)
      in_dtor_ = true;
#endif
      return true;
    }
    return false;
  }

  mutable AtomicRefCount ref_count_{0};
#if PA_BUILDFLAG(DCHECKS_ARE_ON)
  mutable bool needs_adopt_ref_ = false;
  mutable bool in_dtor_ = false;
#endif
};

}  // namespace subtle

// Forward declaration.
template <class T, typename Traits>
class RefCountedThreadSafe;

// Default traits for RefCountedThreadSafe<T>.  Deletes the object when its ref
// count reaches 0.  Overload to delete it on a different thread etc.
template <typename T>
struct DefaultRefCountedThreadSafeTraits {
  static void Destruct(const T* x) {
    // Delete through RefCountedThreadSafe to make child classes only need to be
    // friend with RefCountedThreadSafe instead of this struct, which is an
    // implementation detail.
    RefCountedThreadSafe<T, DefaultRefCountedThreadSafeTraits>::DeleteInternal(
        x);
  }
};

//
// A thread-safe variant of RefCounted<T>
//
//   class MyFoo : public base::RefCountedThreadSafe<MyFoo> {
//    ...
//   };
//
// If you're using the default trait, then you should add compile time
// asserts that no one else is deleting your object.  i.e.
//    private:
//     friend class base::RefCountedThreadSafe<MyFoo>;
//     ~MyFoo();
//
// We can use REQUIRE_ADOPTION_FOR_REFCOUNTED_TYPE() with RefCountedThreadSafe
// too. See the comment above the RefCounted definition for details.
template <class T, typename Traits = DefaultRefCountedThreadSafeTraits<T>>
class RefCountedThreadSafe : public subtle::RefCountedThreadSafeBase {
 public:
  static constexpr subtle::StartRefCountFromZeroTag kRefCountPreference =
      subtle::kStartRefCountFromZeroTag;

  explicit RefCountedThreadSafe()
      : subtle::RefCountedThreadSafeBase(T::kRefCountPreference) {}

  RefCountedThreadSafe(const RefCountedThreadSafe&) = delete;
  RefCountedThreadSafe& operator=(const RefCountedThreadSafe&) = delete;

  void AddRef() const { AddRefImpl(T::kRefCountPreference); }

  void Release() const {
    if (subtle::RefCountedThreadSafeBase::Release()) {
      PA_ANALYZER_SKIP_THIS_PATH();
      Traits::Destruct(static_cast<const T*>(this));
    }
  }

 protected:
  ~RefCountedThreadSafe() = default;

 private:
  friend struct DefaultRefCountedThreadSafeTraits<T>;
  template <typename U>
  static void DeleteInternal(const U* x) {
    delete x;
  }

  void AddRefImpl(subtle::StartRefCountFromZeroTag) const {
    subtle::RefCountedThreadSafeBase::AddRef();
  }

  void AddRefImpl(subtle::StartRefCountFromOneTag) const {
    subtle::RefCountedThreadSafeBase::AddRefWithCheck();
  }
};

}  // namespace partition_alloc::internal::base

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_MEMORY_REF_COUNTED_H_
