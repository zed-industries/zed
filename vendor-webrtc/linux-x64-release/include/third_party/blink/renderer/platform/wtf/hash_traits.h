/*
 * Copyright (C) 2005, 2006, 2007, 2008, 2011, 2012 Apple Inc. All rights
 * reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_HASH_TRAITS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_HASH_TRAITS_H_

#include <string.h>

#include <concepts>
#include <limits>
#include <memory>
#include <type_traits>
#include <utility>

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_functions.h"
#include "third_party/blink/renderer/platform/wtf/hash_table_deleted_value_type.h"
#include "third_party/blink/renderer/platform/wtf/std_lib_extras.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"

namespace WTF {

// A hash traits type is required for a type when the type is used as the key
// or value of a HashTable-based classes. See documentation in
// GenericHashTraitsBase for the HashTraits API.
//
// A hash traits type can be defined as
// - a specialization of the HashTraits template, which will be automatically
//   used, or
// - a standalone hash traits type, which should be passed as the *Traits
//   template parameters of HashTable-based classes.
// The former is preferred if the hash traits defines the default hash behavior
// of the type. The latter is suitable when a type has multiple hash behaviors,
// e.g. CaseFoldingHashTraits defines an alternative hash behavior of strings.
//
// This file contains definitions of hash traits for integral types,
// floating-point types, enums, raw and smart pointers, std::pair, etc.
// These types can be used as hash key or value directly.
//
// This file also contains hash traits types that can be used as the base class
// of hash traits of other types.
//
// A simple hash traits type for a key type can be like:
//   template <>
//   HashTraits<KeyType> : GenericHashTraits<KeyType> {
//     static unsigned GetHash(const KeyType& key) { ...; }
//     static KeyType EmptyValue() { ...; }
//     static KeyType DeletedValue() { ...; }
//   };
//
// A hash traits type for a value type can be even simpler. See documentation
// in GenericHashTraitsBase for which functions/flags are for key types only
// (i.e. not needed for a value type).
//
template <typename T>
struct HashTraits;

class String;

namespace internal {

template <typename T>
struct GenericHashTraitsBase {
  STATIC_ONLY(GenericHashTraitsBase);

  using TraitType = T;

  // Type for functions that do not take ownership, such as contains.
  // If overridden, the type must be assignable to T.
  using PeekInType = const T&;

  // Types for iterators.
  using IteratorGetType = T*;
  using IteratorConstGetType = const T*;
  using IteratorReferenceType = T&;
  using IteratorConstReferenceType = const T&;

  // Type for return value of functions that do not transfer ownership, such
  // as get.
  using PeekOutType = T;
  static const T& Peek(const T& value) { return value; }

  // Computes the hash code.
  // This is for key types only.
  static unsigned GetHash(const T&) = delete;

  // Whether two values are equal. By default, operator== is used.
  // This is for key types only.
  static bool Equal(const T& a, const T& b) { return a == b; }

  // When this is true, the hash table can optimize lookup operations by
  // skipping checks for empty or deleted values. It can be true only if
  // Equal(a, b) can reliably return false where
  //    IsHashTraitsEmptyValue(a) != IsHashTraitsEmptyValue(b) or
  //    IsHashTraitsDeletedValue(a) != IsHashTraitsDeletedValue(b).
  // If this is false, the hash table will never call Equal(a, b) where a or b
  // is an empty or a deleted value. When T is a pointer type, Equal(a, b) can
  // dereference a and b safely without checking if a or b is nullptr or an
  // invalid pointer that represents the deleted value.
  // This is for key types only.
  static constexpr bool kSafeToCompareToEmptyOrDeleted = true;

  // Defines the empty value which is used to fill unused slots in the hash
  // table. This function is preferred to IsEmptyValue() when the empty value
  // can be represented with a value that can be safely and cheaply
  // compared/assigned to another value. By default, the default constructor
  // is used.
  static T EmptyValue() { return T(); }

  // Checks if a given value is an empty value. If this is defined, the hash
  // table will call this function (through IsHashTraitsEmptyValue()) to check
  // if a slot is empty. Otherwise `v == EmptyValue()` will be used.
  static void IsEmptyValue(const T& v) = delete;

  // When this is true, the hash table can optimize allocation of empty hash
  // slots with zeroed memory without calling EmptyValue().
  // This flag can be set to true if any of the following conditions is true:
  // 1. EmptyValue() is defined and all bytes of the return value are zero.
  // 2. IsEmptyValue() is defined and it returns true for a value containing all
  //    zero bytes.
  // Otherwise this flag must be set to false.
  static constexpr bool kEmptyValueIsZero = false;

  // Defines the deleted value which is used to fill the slot for a hash entry
  // when the entry is deleted from the hash table. A hash traits type must
  // define either this function or both IsDeletedValue() and
  // ConstructDeletedValue(). This function is preferred to IsDeletedValue()
  // and ConstructDeletedValue() when the deleted value can be represented with
  // a value that can be safely and cheaply compared/assigned to another value.
  // This is for key types only.
  // NOTE: The destructor of the returned value *may not* be called, so the
  // value should not own any dynamically allocated resources.
  static T DeletedValue() = delete;

  // Checks if a given value is a deleted value. If this is defined, the hash
  // table will call this function (through IsHashTraitsDeletedValue()) to check
  // if a slot is deleted. Otherwise `v == DeletedValue()` will be used.
  // This is for key types only.
  static bool IsDeletedValue(const T& v) = delete;

  // Constructs a deleted value in-place in the given memory space.
  // When this is called, T's destructor on the slot has been called, so this
  // function should not call destructor again (e.g. by assigning a value
  // to `slot`), unless T is trivially destructible.
  // This must be defined if IsDeletedValue() is defined, and will be called
  // through ConstructHashTraitsDeletedValue(). Otherwise
  // `slot = DeletedValue()` will be used.
  // This is for key types only.
  // NOTE: The destructor of the constructed value *will not* be called, so the
  // value should not own any dynamically allocated resources.
  static void ConstructDeletedValue(T& slot) = delete;

  // The starting table size. Can be overridden when we know beforehand that a
  // hash table will have at least N entries.
#if defined(MEMORY_TOOL_REPLACES_ALLOCATOR)
  // The allocation pool for nodes is one big chunk that ASAN has no insight
  // into, so it can cloak errors. Make it as small as possible to force nodes
  // to be allocated individually where ASAN can see them.
  static constexpr unsigned kMinimumTableSize = 1;
#else
  static constexpr unsigned kMinimumTableSize = 8;
#endif

  // The NeedsToForbidGCOnMove flag is used to make the hash table move
  // operations safe when GC is enabled: if a move constructor invokes
  // an allocation triggering the GC then it should be invoked within GC
  // forbidden scope.
  template <typename U = void>
  struct NeedsToForbidGCOnMove {
    // TODO(yutak): Consider using of std:::is_trivially_move_constructible
    // when it is accessible.
    static constexpr bool value =
        !std::is_trivial_v<T> || !std::is_standard_layout_v<T>;
  };

  // The kCanTraceConcurrently value is used by Oilpan concurrent marking. Only
  // type for which HashTraits<T>::kCanTraceConcurrently is true can be traced
  // on a concurrent thread.
  static constexpr bool kCanTraceConcurrently = false;
  // Used by Oilpan compaction. Only types that return true here will be
  // compacted.
  static constexpr bool kSupportsCompaction = false;
};

template <typename T, auto empty_value, auto deleted_value>
struct IntOrEnumHashTraits : internal::GenericHashTraitsBase<T> {
  static_assert(std::is_integral_v<T> || std::is_enum_v<T>);
  static unsigned GetHash(T key) { return WTF::HashInt(key); }
  static constexpr bool kEmptyValueIsZero =
      static_cast<int64_t>(empty_value) == 0;
  static constexpr T EmptyValue() { return static_cast<T>(empty_value); }
  static constexpr T DeletedValue() { return static_cast<T>(deleted_value); }

  static constexpr bool kSupportsCompaction = true;
};

}  // namespace internal

// Default integer traits disallow both 0 and -1 as keys (max value instead of
// -1 for unsigned).
template <typename T, T empty_value = 0, T deleted_value = static_cast<T>(-1)>
struct IntHashTraits
    : internal::IntOrEnumHashTraits<T, empty_value, deleted_value> {
  static_assert(std::is_integral_v<T>);
};

// Default traits for an enum type.  0 is very popular, and -1 is also popular.
// So we use -128 and -127.
template <typename T>
struct EnumHashTraits : internal::IntOrEnumHashTraits<T, -128, -127> {
  static_assert(std::is_enum_v<T>);
};

template <typename T>
struct GenericHashTraits : internal::GenericHashTraitsBase<T> {
  static_assert(!std::is_integral_v<T>);
  static_assert(!std::is_enum_v<T>);
  static_assert(!std::is_floating_point_v<T>);
};

template <typename T>
  requires std::integral<T>
struct GenericHashTraits<T> : IntHashTraits<T> {};

template <typename T>
  requires std::is_enum_v<T>
struct GenericHashTraits<T> : EnumHashTraits<T> {};

template <typename T>
  requires std::floating_point<T>
struct GenericHashTraits<T> : internal::GenericHashTraitsBase<T> {
  static unsigned GetHash(T key) { return HashFloat(key); }
  static bool Equal(T a, T b) { return FloatEqualForHash(a, b); }
  static constexpr T EmptyValue() { return std::numeric_limits<T>::infinity(); }
  static constexpr T DeletedValue() {
    return -std::numeric_limits<T>::infinity();
  }

  static constexpr bool kSupportsCompaction = true;
};

// Default integral traits disallow both 0 and max as keys -- use these traits
// to allow zero and disallow max - 1.
template <typename T>
struct IntWithZeroKeyHashTraits
    : IntHashTraits<T,
                    std::numeric_limits<T>::max(),
                    std::numeric_limits<T>::max() - 1> {};

// This hash traits can be used in cases where the key is already a good hash.
struct AlreadyHashedTraits : GenericHashTraits<unsigned> {
  static unsigned GetHash(unsigned key) { return key; }
};
struct AlreadyHashedWithZeroKeyTraits : IntWithZeroKeyHashTraits<unsigned> {
  static unsigned GetHash(unsigned key) { return key; }
};

template <typename P>
struct GenericHashTraits<P*> : internal::GenericHashTraitsBase<P*> {
  static unsigned GetHash(P* key) { return HashPointer(key); }
  static constexpr bool kEmptyValueIsZero = true;
  static constexpr P* DeletedValue() { return reinterpret_cast<P*>(-1); }
};

template <typename P>
struct GenericHashTraits<scoped_refptr<P>>
    : internal::GenericHashTraitsBase<scoped_refptr<P>> {
  static_assert(sizeof(void*) == sizeof(scoped_refptr<P>),
                "Unexpected RefPtr size."
                " RefPtr needs to be single pointer to support deleted value.");

  static unsigned GetHash(P* key) { return HashPointer(key); }
  static unsigned GetHash(const scoped_refptr<P>& key) {
    return GetHash(key.get());
  }

  static bool Equal(const scoped_refptr<P>& a, const scoped_refptr<P>& b) {
    return a == b;
  }
  static bool Equal(P* a, const scoped_refptr<P>& b) { return a == b; }
  static bool Equal(const scoped_refptr<P>& a, P* b) { return a == b; }

  class RefPtrValuePeeker {
    DISALLOW_NEW();

   public:
    ALWAYS_INLINE RefPtrValuePeeker(P* p) : ptr_(p) {}
    template <typename U>
    RefPtrValuePeeker(const scoped_refptr<U>& p) : ptr_(p.get()) {}

    ALWAYS_INLINE operator P*() const { return ptr_; }

   private:
    P* ptr_;
  };

  static constexpr bool kEmptyValueIsZero = true;
  static bool IsEmptyValue(const scoped_refptr<P>& value) { return !value; }

  static bool IsDeletedValue(const scoped_refptr<P>& value) {
    return *reinterpret_cast<void* const*>(&value) ==
           reinterpret_cast<const void*>(-1);
  }

  static void ConstructDeletedValue(scoped_refptr<P>& slot) {
    *reinterpret_cast<void**>(&slot) = reinterpret_cast<void*>(-1);
  }

  typedef RefPtrValuePeeker PeekInType;
  typedef scoped_refptr<P>* IteratorGetType;
  typedef const scoped_refptr<P>* IteratorConstGetType;
  typedef scoped_refptr<P>& IteratorReferenceType;
  typedef const scoped_refptr<P>& IteratorConstReferenceType;

  typedef P* PeekOutType;
  static PeekOutType Peek(const scoped_refptr<P>& value) { return value.get(); }

  template <typename = void>
  struct NeedsToForbidGCOnMove {
    static constexpr bool value = false;
  };
};

template <typename T>
struct GenericHashTraits<std::unique_ptr<T>>
    : internal::GenericHashTraitsBase<std::unique_ptr<T>> {
  static unsigned GetHash(T* key) { return HashPointer(key); }
  static unsigned GetHash(const std::unique_ptr<T>& key) {
    return GetHash(key.get());
  }

  static bool Equal(const std::unique_ptr<T>& a, const std::unique_ptr<T>& b) {
    return a == b;
  }
  static bool Equal(const std::unique_ptr<T>& a, const T* b) {
    return a.get() == b;
  }
  static bool Equal(const T* a, const std::unique_ptr<T>& b) {
    return a == b.get();
  }

  static constexpr bool kEmptyValueIsZero = true;
  static bool IsEmptyValue(const std::unique_ptr<T>& value) { return !value; }

  using PeekInType = T*;

  using PeekOutType = T*;
  static PeekOutType Peek(const std::unique_ptr<T>& value) {
    return value.get();
  }

  static void ConstructDeletedValue(std::unique_ptr<T>& slot) {
    // Dirty trick: implant an invalid pointer to unique_ptr. Destructor isn't
    // called for deleted buckets, so this is okay.
    new (NotNullTag::kNotNull, &slot)
        std::unique_ptr<T>(reinterpret_cast<T*>(1u));
  }
  static bool IsDeletedValue(const std::unique_ptr<T>& value) {
    return value.get() == reinterpret_cast<T*>(1u);
  }

  template <typename = void>
  struct NeedsToForbidGCOnMove {
    static constexpr bool value = false;
  };
};

// HashTraits<T> is defined as GenericHashTraits<T> by default.
// The separation of HashTraits<T> and GenericHashTraits<T> is to allow
// a specialized HashTraits<T> to inherit GenericHashTraits<T>.
template <typename T>
struct HashTraits : GenericHashTraits<T> {};

// This hash traits type requires the following methods in class T, unless
// the corresponding hash traits method is overridden:
//   // Computes the hash code, for GetHash().
//   unsigned GetHash() const;
//   // Creates the deleted value, for ConstructDeletedValue().
//   T(HashTableDeletedValueType);
//   // Checks if `this` is a deleted value, for IsDeletedValue().
//   bool IsHashTableDeletedValue() const;
// Also requires T() and operator== if EmptyValue() and Equal() are not
// overridden, respectively, which is the same as GenericHashTraits<T>.
template <typename T>
struct SimpleClassHashTraits : GenericHashTraits<T> {
  static_assert(std::is_class_v<T>);
  static unsigned GetHash(const T& key) { return key.GetHash(); }
  static constexpr bool kEmptyValueIsZero = true;
  template <typename U = void>
  struct NeedsToForbidGCOnMove {
    static constexpr bool value = false;
  };
  static void ConstructDeletedValue(T& slot) {
    new (NotNullTag::kNotNull, &slot) T(kHashTableDeletedValue);
  }
  static bool IsDeletedValue(const T& value) {
    return value.IsHashTableDeletedValue();
  }
};

// Defined in string_hash.h.
template <>
struct HashTraits<String>;

namespace internal {

template <typename Traits, typename Enabled = void>
struct HashTraitsEmptyValueChecker {
  static bool IsEmptyValue(const typename Traits::TraitType& value) {
    return value == Traits::EmptyValue();
  }
};
template <typename Traits>
struct HashTraitsEmptyValueChecker<
    Traits,
    std::enable_if_t<
        std::is_same_v<decltype(Traits::IsEmptyValue(
                           std::declval<typename Traits::TraitType>())),
                       bool>>> {
  static bool IsEmptyValue(const typename Traits::TraitType& value) {
    return Traits::IsEmptyValue(value);
  }
};

template <typename Traits, typename Enabled = void>
struct HashTraitsDeletedValueHelper {
  static bool IsDeletedValue(const typename Traits::TraitType& value) {
    return value == Traits::DeletedValue();
  }
  static void ConstructDeletedValue(typename Traits::TraitType& slot) {
    new (NotNullTag::kNotNull, &slot)
        typename Traits::TraitType(Traits::DeletedValue());
  }
};
template <typename Traits>
struct HashTraitsDeletedValueHelper<
    Traits,
    std::enable_if_t<
        std::is_same_v<decltype(Traits::IsDeletedValue(
                           std::declval<typename Traits::TraitType>())),
                       bool>>> {
  static bool IsDeletedValue(const typename Traits::TraitType& value) {
    return Traits::IsDeletedValue(value);
  }
  // Traits must also define ConstructDeletedValue() if it defines
  // IsDeletedValue().
  static void ConstructDeletedValue(typename Traits::TraitType& slot) {
    Traits::ConstructDeletedValue(slot);
  }
};

}  // namespace internal

// This function selects either the EmptyValue() function or the IsEmptyValue()
// function to check for empty values.
template <typename Traits, typename T>
inline bool IsHashTraitsEmptyValue(const T& value) {
  return internal::HashTraitsEmptyValueChecker<Traits>::IsEmptyValue(value);
}

// This function selects either the DeletedValue() function or the
// IsDeletedValue() function to check for deleted values.
template <typename Traits, typename T>
inline bool IsHashTraitsDeletedValue(const T& value) {
  return internal::HashTraitsDeletedValueHelper<Traits>::IsDeletedValue(value);
}

// This function selects either the DeletedValue() function or the
// ConstructDeletedValue() function to construct a deleted value.
template <typename Traits, typename T>
inline void ConstructHashTraitsDeletedValue(T& slot) {
  internal::HashTraitsDeletedValueHelper<Traits>::ConstructDeletedValue(slot);
}

template <typename Traits, typename T>
inline bool IsHashTraitsEmptyOrDeletedValue(const T& value) {
  return IsHashTraitsEmptyValue<Traits, T>(value) ||
         IsHashTraitsDeletedValue<Traits, T>(value);
}

// A HashTraits type for T to delegate all HashTraits API to a field.
template <typename T,
          auto field,
          typename FieldTraits = HashTraits<
              std::remove_reference_t<decltype(std::declval<T>().*field)>>>
struct OneFieldHashTraits : GenericHashTraits<T> {
  using TraitType = T;
  static unsigned GetHash(const T& p) { return FieldTraits::GetHash(p.*field); }
  static bool Equal(const T& a, const T& b) {
    return FieldTraits::Equal(a.*field, b.*field);
  }
  static constexpr bool kSafeToCompareToEmptyOrDeleted =
      FieldTraits::kSafeToCompareToEmptyOrDeleted;

  static constexpr bool kEmptyValueIsZero = FieldTraits::kEmptyValueIsZero;
  static T EmptyValue() { return T(FieldTraits::EmptyValue()); }

  static bool IsEmptyValue(const T& value) {
    return IsHashTraitsEmptyValue<FieldTraits>(value.*field);
  }

  static void ConstructDeletedValue(T& slot) {
    ConstructHashTraitsDeletedValue<FieldTraits>(slot.*field);
  }
  static bool IsDeletedValue(const T& value) {
    return IsHashTraitsDeletedValue<FieldTraits>(value.*field);
  }

  static constexpr unsigned kMinimumTableSize = FieldTraits::kMinimumTableSize;

  template <typename U = void>
  struct NeedsToForbidGCOnMove {
    static const bool value =
        FieldTraits::template NeedsToForbidGCOnMove<>::value;
  };
};

// A HashTraits type for T to delegate all HashTraits API to two fields.
template <
    typename T,
    auto first_field,
    auto second_field,
    typename FirstTraits = HashTraits<
        std::remove_reference_t<decltype(std::declval<T>().*first_field)>>,
    typename SecondTraits = HashTraits<
        std::remove_reference_t<decltype(std::declval<T>().*second_field)>>>
struct TwoFieldsHashTraits : OneFieldHashTraits<T, first_field, FirstTraits> {
  using TraitType = T;
  static unsigned GetHash(const T& p) {
    return HashInts(FirstTraits::GetHash(p.*first_field),
                    SecondTraits::GetHash(p.*second_field));
  }
  static bool Equal(const T& a, const T& b) {
    return FirstTraits::Equal(a.*first_field, b.*first_field) &&
           SecondTraits::Equal(a.*second_field, b.*second_field);
  }
  static constexpr bool kSafeToCompareToEmptyOrDeleted =
      FirstTraits::kSafeToCompareToEmptyOrDeleted &&
      SecondTraits::kSafeToCompareToEmptyOrDeleted;

  static constexpr bool kEmptyValueIsZero =
      FirstTraits::kEmptyValueIsZero && SecondTraits::kEmptyValueIsZero;
  static T EmptyValue() {
    return T(FirstTraits::EmptyValue(), SecondTraits::EmptyValue());
  }

  static bool IsEmptyValue(const T& value) {
    return IsHashTraitsEmptyValue<FirstTraits>(value.*first_field) &&
           IsHashTraitsEmptyValue<SecondTraits>(value.*second_field);
  }

  // ConstructDeletedValue(), IsDeletedValue(), kMinimumTableSize delegate to
  // the first field, inherited from OneFieldHashTraits.

  template <typename U = void>
  struct NeedsToForbidGCOnMove {
    static const bool value =
        FirstTraits::template NeedsToForbidGCOnMove<>::value ||
        SecondTraits::template NeedsToForbidGCOnMove<>::value;
  };
};

template <typename FirstTraitsArg,
          typename SecondTraitsArg,
          typename P = std::pair<typename FirstTraitsArg::TraitType,
                                 typename SecondTraitsArg::TraitType>>
struct PairHashTraits : TwoFieldsHashTraits<P,
                                            &P::first,
                                            &P::second,
                                            FirstTraitsArg,
                                            SecondTraitsArg> {
  using TraitType = P;
  using FirstTraits = FirstTraitsArg;
  using SecondTraits = SecondTraitsArg;
};

template <typename First, typename Second>
struct HashTraits<std::pair<First, Second>>
    : public PairHashTraits<HashTraits<First>, HashTraits<Second>> {};

// Shortcut of HashTraits<T>::GetHash(), which can deduct T automatically.
template <typename T>
unsigned GetHash(const T& key) {
  return HashTraits<T>::GetHash(key);
}

}  // namespace WTF

using WTF::AlreadyHashedTraits;
using WTF::AlreadyHashedWithZeroKeyTraits;
using WTF::EnumHashTraits;
using WTF::GenericHashTraits;
using WTF::HashTraits;
using WTF::IntHashTraits;
using WTF::IntWithZeroKeyHashTraits;
using WTF::OneFieldHashTraits;
using WTF::PairHashTraits;
using WTF::SimpleClassHashTraits;
using WTF::TwoFieldsHashTraits;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_HASH_TRAITS_H_
