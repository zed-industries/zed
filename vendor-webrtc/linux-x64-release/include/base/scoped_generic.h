// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_SCOPED_GENERIC_H_
#define BASE_SCOPED_GENERIC_H_

#include <stdlib.h>

#include <concepts>
#include <type_traits>
#include <utility>

#include "base/check.h"
#include "base/compiler_specific.h"
#include "base/memory/raw_ptr.h"

namespace base {

// This class acts like unique_ptr with a custom deleter (although is slightly
// less fancy in some of the more escoteric respects) except that it keeps a
// copy of the object rather than a pointer, and we require that the contained
// object has some kind of "invalid" value.
//
// Defining a scoper based on this class allows you to get a scoper for
// non-pointer types without having to write custom code for set, reset, and
// move, etc. and get almost identical semantics that people are used to from
// unique_ptr.
//
// It is intended that you will typedef this class with an appropriate deleter
// to implement clean up tasks for objects that act like pointers from a
// resource management standpoint but aren't, such as file descriptors and
// various types of operating system handles. Using unique_ptr for these
// things requires that you keep a pointer to the handle valid for the lifetime
// of the scoper (which is easy to mess up).
//
// For an object to be able to be put into a ScopedGeneric, it must support
// standard copyable semantics and have a specific "invalid" value. The traits
// must define a free function and also the invalid value to assign for
// default-constructed and released objects.
//
//   struct FooScopedTraits {
//     // It's assumed that this is a fast inline function with little-to-no
//     // penalty for duplicate calls. This must be a static function even
//     // for stateful traits.
//     static int InvalidValue() {
//       return 0;
//     }
//
//     // This free function will not be called if f == InvalidValue()!
//     static void Free(int f) {
//       ::FreeFoo(f);
//     }
//   };
//
//   using ScopedFoo = ScopedGeneric<int, FooScopedTraits>;
//
// A Traits type may choose to track ownership of objects in parallel with
// ScopedGeneric. To do so, it must implement the Acquire and Release methods,
// which will be called by ScopedGeneric during ownership transfers and extend
// the ScopedGenericOwnershipTracking tag type.
//
//   struct BarScopedTraits : public ScopedGenericOwnershipTracking {
//     using ScopedGenericType = ScopedGeneric<int, BarScopedTraits>;
//     static int InvalidValue() {
//       return 0;
//     }
//
//     static void Free(int b) {
//       ::FreeBar(b);
//     }
//
//     static void Acquire(const ScopedGenericType& owner, int b) {
//       ::TrackAcquisition(b, owner);
//     }
//
//     static void Release(const ScopedGenericType& owner, int b) {
//       ::TrackRelease(b, owner);
//     }
//   };
//
//   using ScopedBar = ScopedGeneric<int, BarScopedTraits>;
struct ScopedGenericOwnershipTracking {};

template <typename T, typename Traits>
class ScopedGeneric {
 private:
  // This must be first since it's used inline below.
  //
  // Use the empty base class optimization to allow us to have a D
  // member, while avoiding any space overhead for it when D is an
  // empty class.  See e.g. http://www.cantrip.org/emptyopt.html for a good
  // discussion of this technique.
  struct Data : public Traits {
    explicit Data(const T& in) : generic(in) {}
    Data(const T& in, const Traits& other) : Traits(other), generic(in) {}
    T generic;
  };

 public:
  typedef T element_type;
  typedef Traits traits_type;

  ScopedGeneric() : data_(traits_type::InvalidValue()) {}

  // Constructor. Takes responsibility for freeing the resource associated with
  // the object T.
  explicit ScopedGeneric(const element_type& value) : data_(value) {
    TrackAcquire(data_.generic);
  }

  // Constructor. Allows initialization of a stateful traits object.
  ScopedGeneric(const element_type& value, const traits_type& traits)
      : data_(value, traits) {
    TrackAcquire(data_.generic);
  }

  // Move constructor. Allows initialization from a ScopedGeneric rvalue.
  ScopedGeneric(ScopedGeneric<T, Traits>&& rvalue)
      : data_(rvalue.release(), rvalue.get_traits()) {
    TrackAcquire(data_.generic);
  }
  ScopedGeneric(const ScopedGeneric&) = delete;
  ScopedGeneric& operator=(const ScopedGeneric&) = delete;

  virtual ~ScopedGeneric() {
    CHECK(!receiving_);  // ScopedGeneric destroyed with active receiver.
    FreeIfNecessary();
  }

  // operator=. Allows assignment from a ScopedGeneric rvalue.
  ScopedGeneric& operator=(ScopedGeneric<T, Traits>&& rvalue) {
    reset(rvalue.release());
    return *this;
  }

  // Frees the currently owned object, if any. Then takes ownership of a new
  // object, if given. Self-resets are not allowed as on unique_ptr. See
  // http://crbug.com/162971
  void reset(const element_type& value = traits_type::InvalidValue()) {
    if (data_.generic != traits_type::InvalidValue() &&
        data_.generic == value) {
      abort();
    }
    FreeIfNecessary();
    data_.generic = value;
    TrackAcquire(value);
  }

  // Release the object. The return value is the current object held by this
  // object. After this operation, this object will hold a null value, and
  // will not own the object any more.
  [[nodiscard]] element_type release() {
    element_type old_generic =
        std::exchange(data_.generic, traits_type::InvalidValue());
    TrackRelease(old_generic);
    return old_generic;
  }

  // A helper class that provides a T* that can be used to take ownership of
  // a value returned from a function via out-parameter. When the Receiver is
  // destructed (which should usually be at the end of the statement in which
  // receive is called), ScopedGeneric::reset() will be called with the
  // Receiver's value.
  //
  // In the simple case of a function that assigns the value before it returns,
  // C++'s lifetime extension can be used as follows:
  //
  //    ScopedFoo foo;
  //    bool result = GetFoo(ScopedFoo::Receiver(foo).get());
  //
  // Note that the lifetime of the Receiver is extended until the semicolon,
  // and ScopedGeneric is assigned the value upon destruction of the Receiver,
  // so the following code would not work:
  //
  //    // BROKEN!
  //    ScopedFoo foo;
  //    UseFoo(&foo, GetFoo(ScopedFoo::Receiver(foo).get()));
  //
  // In more complicated scenarios, you may need to provide an explicit scope
  // for the Receiver, as in the following:
  //
  //    std::vector<ScopedFoo> foos(64);
  //
  //    {
  //      std::vector<ScopedFoo::Receiver> foo_receivers;
  //      for (auto foo : foos) {
  //        foo_receivers_.emplace_back(foo);
  //      }
  //      for (auto receiver : foo_receivers) {
  //        SubmitGetFooRequest(receiver.get());
  //      }
  //      WaitForFooRequests();
  //    }
  //    UseFoos(foos);
  class Receiver {
   public:
    explicit Receiver(ScopedGeneric& parent) : scoped_generic_(&parent) {
      // Check if we attempted to construct a Receiver for ScopedGeneric with an
      // existing Receiver.
      CHECK(!scoped_generic_->receiving_);
      scoped_generic_->receiving_ = true;
    }
    Receiver(const Receiver&) = delete;
    Receiver& operator=(const Receiver&) = delete;
    Receiver(Receiver&& move) {
      CHECK(!used_);       // Moving into already-used Receiver.
      CHECK(!move.used_);  // Moving from already-used Receiver.
      scoped_generic_ = move.scoped_generic_;
      move.scoped_generic_ = nullptr;
    }

    Receiver& operator=(Receiver&& move) {
      CHECK(!used_);       // Moving into already-used Receiver.
      CHECK(!move.used_);  // Moving from already-used Receiver.
      scoped_generic_ = move.scoped_generic_;
      move.scoped_generic_ = nullptr;
    }
    ~Receiver() {
      if (scoped_generic_) {
        CHECK(scoped_generic_->receiving_);
        scoped_generic_->reset(value_);
        scoped_generic_->receiving_ = false;
      }
    }
    // We hand out a pointer to a field in Receiver instead of directly to
    // ScopedGeneric's internal storage in order to make it so that users can't
    // accidentally silently break ScopedGeneric's invariants. This way, an
    // incorrect use-after-scope-exit is more detectable by ASan or static
    // analysis tools, as the pointer is only valid for the lifetime of the
    // Receiver, not the ScopedGeneric.
    T* get() {
      used_ = true;
      return &value_;
    }

   private:
    T value_ = Traits::InvalidValue();
    raw_ptr<ScopedGeneric<T, Traits>> scoped_generic_;
    bool used_ = false;
  };

  const element_type& get() const { return data_.generic; }

  // Returns true if this object doesn't hold the special null value for the
  // associated data type.
  bool is_valid() const { return data_.generic != traits_type::InvalidValue(); }

  bool operator==(const element_type& value) const {
    return data_.generic == value;
  }
  bool operator!=(const element_type& value) const {
    return data_.generic != value;
  }

  Traits& get_traits() LIFETIME_BOUND { return data_; }
  const Traits& get_traits() const LIFETIME_BOUND { return data_; }

 private:
  void FreeIfNecessary() {
    if (data_.generic != traits_type::InvalidValue()) {
      TrackRelease(data_.generic);
      data_.Free(data_.generic);
      data_.generic = traits_type::InvalidValue();
    }
  }

  void TrackAcquire(const T& value) {
    if constexpr (std::derived_from<Traits, ScopedGenericOwnershipTracking>) {
      if (value != traits_type::InvalidValue()) {
        data_.Acquire(static_cast<const ScopedGeneric&>(*this), value);
      }
    }
  }

  void TrackRelease(const T& value) {
    if constexpr (std::derived_from<Traits, ScopedGenericOwnershipTracking>) {
      if (value != traits_type::InvalidValue()) {
        data_.Release(static_cast<const ScopedGeneric&>(*this), value);
      }
    }
  }

  // Forbid comparison. If U != T, it totally doesn't make sense, and if U ==
  // T, it still doesn't make sense because you should never have the same
  // object owned by two different ScopedGenerics.
  template <typename T2, typename Traits2>
  bool operator==(const ScopedGeneric<T2, Traits2>& p2) const;
  template <typename T2, typename Traits2>
  bool operator!=(const ScopedGeneric<T2, Traits2>& p2) const;

  Data data_;
  bool receiving_ = false;
};

template <class T, class Traits>
void swap(const ScopedGeneric<T, Traits>& a,
          const ScopedGeneric<T, Traits>& b) {
  a.swap(b);
}

template <class T, class Traits>
bool operator==(const T& value, const ScopedGeneric<T, Traits>& scoped) {
  return value == scoped.get();
}

template <class T, class Traits>
bool operator!=(const T& value, const ScopedGeneric<T, Traits>& scoped) {
  return value != scoped.get();
}

}  // namespace base

#endif  // BASE_SCOPED_GENERIC_H_
