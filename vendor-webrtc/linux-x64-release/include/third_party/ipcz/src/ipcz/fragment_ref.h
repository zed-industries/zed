// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_FRAGMENT_REF_H_
#define IPCZ_SRC_IPCZ_FRAGMENT_REF_H_

#include <algorithm>
#include <type_traits>
#include <utility>

#include "ipcz/fragment.h"
#include "ipcz/fragment_descriptor.h"
#include "ipcz/ref_counted_fragment.h"
#include "third_party/abseil-cpp/absl/base/macros.h"
#include "util/ref_counted.h"

namespace ipcz {

class NodeLinkMemory;

namespace internal {

// Base class for any FragmentRef<T>, implementing common behavior for managing
// the underlying RefCountedFragment.
class GenericFragmentRef {
 public:
  GenericFragmentRef();

  // Does not increase the ref count of the underlying RefCountedFragment,
  // effectively assuming ownership of a previously acquired ref.
  GenericFragmentRef(Ref<NodeLinkMemory> memory, const Fragment& fragment);

  ~GenericFragmentRef();

  const Ref<NodeLinkMemory>& memory() const { return memory_; }
  const Fragment& fragment() const { return fragment_; }

  bool is_null() const { return fragment_.is_null(); }
  bool is_addressable() const { return fragment_.is_addressable(); }
  bool is_pending() const { return fragment_.is_pending(); }

  void reset();
  Fragment release();

  int32_t ref_count_for_testing() const {
    return AsRefCountedFragment()->ref_count_for_testing();
  }

 protected:
  RefCountedFragment* AsRefCountedFragment() const {
    return static_cast<RefCountedFragment*>(fragment_.address());
  }

  // The NodeLinkMemory who ultimately owns this fragment's memory. May be null
  // if the FragmentRef is unmanaged.
  Ref<NodeLinkMemory> memory_;

  Fragment fragment_;
};

}  // namespace internal

// Holds a reference to a RefCountedFragment. When this object is destroyed, the
// underlying ref count is decreased. If the ref count is decreased to zero, the
// underlying Fragment is returned to its NodeLinkMemory.
//
// Some FragmentRefs may be designated as "unmanaged", meaning that they will
// never attempt to free the underlying Fragment. These refs are used to
// preserve type compatibility with other similar (but managed) FragmentRefs
// when the underlying Fragment isn't dynamically allocated and can't be freed.
//
// For example most RouterLinkState fragments are dynamically allocated and
// managed by FragmentRefs, but some instances are allocated at fixed locations
// within the NodeLinkMemory and cannot be freed or reused. In both cases, ipcz
// can refer to these objects using a FragmentRef<RouterLinkState>.
template <typename T>
class FragmentRef : public internal::GenericFragmentRef {
 public:
  static_assert(std::is_base_of<RefCountedFragment, T>::value,
                "T must inherit RefCountedFragment for FragmentRef<T>");

  constexpr FragmentRef() = default;
  constexpr FragmentRef(std::nullptr_t) : FragmentRef() {}

  // Adopts an existing ref to the RefCountedFragment located at the beginning
  // of `fragment`, which is a Fragment owned by `memory.
  FragmentRef(decltype(kAdoptExistingRef),
              Ref<NodeLinkMemory> memory,
              const Fragment& fragment)
      : GenericFragmentRef(std::move(memory), fragment) {
    ABSL_ASSERT(memory_);
    ABSL_ASSERT(fragment_.is_null() || fragment_.size() >= sizeof(T));
  }

  // Constructs an unmanaged FragmentRef, which references `fragment` and
  // updates its refcount, but which never attempts to release `fragment` back
  // to its NodeLinkMemory. This is only safe to use with Fragments which cannot
  // be freed.
  FragmentRef(decltype(RefCountedFragment::kUnmanagedRef),
              const Fragment& fragment)
      : GenericFragmentRef(nullptr, fragment) {
    ABSL_ASSERT(fragment_.is_null() || fragment_.size() >= sizeof(T));
  }

  FragmentRef(const FragmentRef<T>& other)
      : GenericFragmentRef(other.memory(), other.fragment()) {
    if (!fragment_.is_null()) {
      ABSL_ASSERT(fragment_.is_addressable());
      AsRefCountedFragment()->AddRef();
    }
  }

  FragmentRef(FragmentRef<T>&& other) noexcept
      : GenericFragmentRef(std::move(other.memory_), other.fragment_) {
    other.release();
  }

  FragmentRef<T>& operator=(const FragmentRef<T>& other) {
    reset();
    memory_ = other.memory();
    fragment_ = other.fragment();
    if (!fragment_.is_null()) {
      ABSL_ASSERT(fragment_.is_addressable());
      AsRefCountedFragment()->AddRef();
    }
    return *this;
  }

  FragmentRef<T>& operator=(FragmentRef<T>&& other) {
    reset();
    memory_ = std::move(other.memory_);
    fragment_ = other.release();
    return *this;
  }

  T* get() const { return static_cast<T*>(fragment_.address()); }
  T* operator->() const { return get(); }
  T& operator*() const { return *get(); }
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_FRAGMENT_REF_H_
