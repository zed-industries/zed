// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_MEMBER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_MEMBER_H_

#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/heap/thread_state_storage.h"
#include "third_party/blink/renderer/platform/heap/write_barrier.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/construct_traits.h"
#include "third_party/blink/renderer/platform/wtf/hash_functions.h"
#include "third_party/blink/renderer/platform/wtf/hash_traits.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"
#include "v8/include/cppgc/member.h"  // IWYU pragma: export
#include "v8/include/cppgc/tagged-member.h"

namespace blink {

template <typename T>
using Member = cppgc::Member<T>;

template <typename T>
using WeakMember = cppgc::WeakMember<T>;

template <typename T>
using UntracedMember = cppgc::UntracedMember<T>;

namespace subtle {

template <typename T>
using UncompressedMember = cppgc::subtle::UncompressedMember<T>;

template <typename T, typename Tag1, typename Tag2>
using TaggedUncompressedMember =
    cppgc::subtle::TaggedUncompressedMember<T, Tag1, Tag2>;
}

template <typename T>
inline bool IsHashTableDeletedValue(const Member<T>& m) {
  return m == cppgc::kSentinelPointer;
}

constexpr auto kMemberDeletedValue = cppgc::kSentinelPointer;

template <typename T>
struct ThreadingTrait<blink::Member<T>> {
  STATIC_ONLY(ThreadingTrait);
  static constexpr ThreadAffinity kAffinity = ThreadingTrait<T>::kAffinity;
};

template <typename T>
struct ThreadingTrait<blink::WeakMember<T>> {
  STATIC_ONLY(ThreadingTrait);
  static constexpr ThreadAffinity kAffinity = ThreadingTrait<T>::kAffinity;
};

template <typename T>
struct ThreadingTrait<blink::UntracedMember<T>> {
  STATIC_ONLY(ThreadingTrait);
  static constexpr ThreadAffinity kAffinity = ThreadingTrait<T>::kAffinity;
};

template <typename T>
inline void swap(Member<T>& a, Member<T>& b) {
  a.Swap(b);
}

static constexpr bool kBlinkMemberGCHasDebugChecks =
    !std::is_same<cppgc::internal::DefaultMemberCheckingPolicy,
                  cppgc::internal::DisabledCheckingPolicy>::value;

// We should never bloat the Member<> wrapper.
// NOTE: The Member<void*> works as we never use this Member in a trace method.
static_assert(kBlinkMemberGCHasDebugChecks ||
                  sizeof(Member<void*>) <= sizeof(void*),
              "Member<> should stay small!");

}  // namespace blink

namespace WTF {

template <typename T>
struct IsTraceable<blink::Member<T>> {
  STATIC_ONLY(IsTraceable);
  static const bool value = true;
};

template <typename T>
struct IsWeak<blink::WeakMember<T>> : std::true_type {};

template <typename T>
struct IsTraceable<blink::WeakMember<T>> {
  STATIC_ONLY(IsTraceable);
  static const bool value = true;
};

// Peeker type that allows for using all kinds of Member, Persistent, and T*
// interchangeably. This is necessary for collection methods that are called
// directly with any of those types.
template <typename T>
class ValuePeeker final {
  DISALLOW_NEW();

 public:
  // NOLINTNEXTLINE
  ALWAYS_INLINE ValuePeeker(T* ptr) : ptr_(ptr) {}
  template <typename U>
  // NOLINTNEXTLINE
  ALWAYS_INLINE ValuePeeker(const blink::Member<U>& m) : ptr_(m.Get()) {}
  template <typename U>
  // NOLINTNEXTLINE
  ALWAYS_INLINE ValuePeeker(const blink::WeakMember<U>& m) : ptr_(m.Get()) {}
  template <typename U>
  // NOLINTNEXTLINE
  ALWAYS_INLINE ValuePeeker(const blink::UntracedMember<U>& m)
      : ptr_(m.Get()) {}
  template <typename U>
  // NOLINTNEXTLINE
  ALWAYS_INLINE ValuePeeker(const blink::Persistent<U>& p) : ptr_(p.Get()) {}
  template <typename U>
  // NOLINTNEXTLINE
  ALWAYS_INLINE ValuePeeker(const blink::WeakPersistent<U>& p)
      : ptr_(p.Get()) {}

  // NOLINTNEXTLINE
  ALWAYS_INLINE operator T*() const { return ptr_; }
  // NOLINTNEXTLINE
  ALWAYS_INLINE operator blink::Member<T>() const { return ptr_; }
  // NOLINTNEXTLINE
  ALWAYS_INLINE operator blink::WeakMember<T>() const { return ptr_; }
  // NOLINTNEXTLINE
  ALWAYS_INLINE operator blink::UntracedMember<T>() const { return ptr_; }

 private:
  T* ptr_;
};

// Default hash for hash tables with Member<>-derived elements.
template <typename T, typename MemberType>
struct BaseMemberHashTraits : SimpleClassHashTraits<MemberType> {
  STATIC_ONLY(BaseMemberHashTraits);

  // Heap hash containers allow to operate with raw pointers, e.g.
  //   HeapHashSet<Member<GCed>> set;
  //   set.find(raw_ptr);
  // Therefore, provide two hashing functions, one for raw pointers, another for
  // Member. Prefer compressing raw pointers instead of decompressing Members,
  // assuming the former is cheaper.
  static unsigned GetHash(const T* key) {
#if defined(CPPGC_POINTER_COMPRESSION)
    cppgc::internal::CompressedPointer st(key);
#else
    cppgc::internal::RawPointer st(key);
#endif
    return WTF::GetHash(st.GetAsInteger());
  }
  template <typename Member,
            std::enable_if_t<WTF::IsAnyMemberType<Member>::value>* = nullptr>
  static unsigned GetHash(const Member& m) {
    return WTF::GetHash(m.GetRawStorage().GetAsInteger());
  }

  static constexpr bool kEmptyValueIsZero = true;

  using PeekInType = ValuePeeker<T>;
  using PeekOutType = T*;
  using IteratorGetType = MemberType*;
  using IteratorConstGetType = const MemberType*;
  using IteratorReferenceType = MemberType&;
  using IteratorConstReferenceType = const MemberType&;

  static PeekOutType Peek(const MemberType& value) { return value.Get(); }

  static void ConstructDeletedValue(MemberType& slot) {
    slot = cppgc::kSentinelPointer;
  }

  static bool IsDeletedValue(const MemberType& value) {
    return value == cppgc::kSentinelPointer;
  }
};

// Custom HashTraits<Member<Type>> can inherit this type.
template <typename T>
struct MemberHashTraits : BaseMemberHashTraits<T, blink::Member<T>> {
  static constexpr bool kCanTraceConcurrently = true;
  static constexpr bool kSupportsCompaction = true;
};
template <typename T>
struct HashTraits<blink::Member<T>> : MemberHashTraits<T> {};

// Custom HashTraits<WeakMember<Type>> can inherit this type.
template <typename T>
struct WeakMemberHashTraits : BaseMemberHashTraits<T, blink::WeakMember<T>> {
  static constexpr bool kCanTraceConcurrently = true;
  static constexpr bool kSupportsCompaction = true;
};
template <typename T>
struct HashTraits<blink::WeakMember<T>> : WeakMemberHashTraits<T> {};

// Custom HashTraits<UntracedMember<Type>> can inherit this type.
template <typename T>
struct UntracedMemberHashTraits
    : BaseMemberHashTraits<T, blink::UntracedMember<T>> {};
template <typename T>
struct HashTraits<blink::UntracedMember<T>> : UntracedMemberHashTraits<T> {};

template <typename T>
class MemberConstructTraits {
  STATIC_ONLY(MemberConstructTraits);

 public:
  template <typename... Args>
  static T* Construct(void* location, Args&&... args) {
    // `Construct()` creates a new Member which must not be visible to the
    // concurrent marker yet, similar to regular ctors in Member.
    return new (NotNullTag::kNotNull, location) T(std::forward<Args>(args)...);
  }

  template <typename... Args>
  static T* ConstructAndNotifyElement(void* location, Args&&... args) {
    // `ConstructAndNotifyElement()` updates an existing Member which might
    // also be concurrently traced while we update it. The regular ctors
    // for Member don't use an atomic write which can lead to data races.
    T* object = new (NotNullTag::kNotNull, location)
        T(std::forward<Args>(args)..., typename T::AtomicInitializerTag());
    NotifyNewElement(object);
    return object;
  }

  static void NotifyNewElement(T* element) {
    blink::WriteBarrier::DispatchForObject(element);
  }

  static void NotifyNewElements(base::span<T> members) {
    // Checking the first element is sufficient for determining whether a
    // marking or generational barrier is required.
    if (members.empty() ||
               !blink::WriteBarrier::IsWriteBarrierNeeded(&members.front())) [[likely]] {
      return;
    }
    for (auto& member : members) {
      blink::WriteBarrier::DispatchForObject(&member);
    }
  }
};

template <typename T, typename Traits, typename Allocator>
class ConstructTraits<blink::Member<T>, Traits, Allocator> final
    : public MemberConstructTraits<blink::Member<T>> {};

template <typename T, typename Traits, typename Allocator>
class ConstructTraits<blink::WeakMember<T>, Traits, Allocator> final
    : public MemberConstructTraits<blink::WeakMember<T>> {};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_MEMBER_H_
