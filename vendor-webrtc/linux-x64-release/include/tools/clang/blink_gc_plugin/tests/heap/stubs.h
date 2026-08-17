// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef HEAP_STUBS_H_
#define HEAP_STUBS_H_

#include <stddef.h>
#include <stdint.h>

#define GC_PLUGIN_IGNORE(reason) \
  __attribute__((annotate("blink_gc_plugin_ignore")))

#define STACK_ALLOCATED_IGNORE(reason) \
  __attribute__((annotate("stack_allocated_ignore")))

namespace base {

template <typename T>
class WeakPtr {
 public:
  ~WeakPtr() {}
  operator T*() const { return 0; }
  T* operator->() const { return 0; }
};

template <typename T>
class WeakPtrFactory {
 public:
  explicit WeakPtrFactory(T*) {}
  ~WeakPtrFactory() {}
  WeakPtr<T> GetWeakPtr() { return WeakPtr<T>(); }
};

template <typename T, typename Traits = void>
class raw_ptr {};

template <typename T, typename Traits = void>
class raw_ref {};

}  // namespace base
namespace WTF {

template<typename T> class RefCounted { };

template<typename T> class RawPtr {
 public:
  operator T*() const { return 0; }
  T* operator->() const { return 0; }
};

template<typename T> class scoped_refptr {
 public:
  ~scoped_refptr() {}
  operator T*() const { return 0; }
  T* operator->() const { return 0; }
};

class PartitionAllocator {
 public:
  static const bool isGarbageCollected = false;
};

template <typename T,
          size_t inlineCapacity = 0,
          typename Allocator = PartitionAllocator>
class Vector {
 public:
  using iterator = T*;
  using const_iterator = const T*;
  using reverse_iterator = T*;
  using const_reverse_iterator = const T*;

  size_t size();
  T& operator[](size_t);

  ~Vector() {}
};

template <typename T,
          size_t inlineCapacity = 0,
          typename Allocator = PartitionAllocator>
class Deque {
 public:
  using iterator = T*;
  using const_iterator = const T*;
  using reverse_iterator = T*;
  using const_reverse_iterator = const T*;

  ~Deque() {}
};

template <typename ValueArg,
          typename TraitsArg = void,
          typename Allocator = PartitionAllocator>
class HashSet {
 public:
  typedef ValueArg* iterator;
  typedef const ValueArg* const_iterator;
  typedef ValueArg* reverse_iterator;
  typedef const ValueArg* const_reverse_iterator;

  ~HashSet() {}
};

template <typename ValueArg,
          typename TraitsArg = void,
          typename Allocator = PartitionAllocator>
class LinkedHashSet {
 public:
  typedef ValueArg* iterator;
  typedef const ValueArg* const_iterator;
  typedef ValueArg* reverse_iterator;
  typedef const ValueArg* const_reverse_iterator;

  ~LinkedHashSet() {}
};

template <typename ValueArg,
          typename TraitsArg = void,
          typename Allocator = PartitionAllocator>
class HashCountedSet {
 public:
  ~HashCountedSet() {}
};

template <typename KeyArg,
          typename MappedArg,
          typename KeyTraitsArg = void,
          typename MappedTraitsArg = void,
          typename Allocator = PartitionAllocator>
class HashMap {
 public:
  typedef MappedArg* iterator;
  typedef const MappedArg* const_iterator;
  typedef MappedArg* reverse_iterator;
  typedef const MappedArg* const_reverse_iterator;

  ~HashMap() {}
};
}

// Empty namespace declaration to exercise internal
// handling of namespace equality.
namespace std {
  /* empty */
}

namespace std {

template<typename T> class unique_ptr {
 public:
  ~unique_ptr() {}
  operator T*() const { return 0; }
  T* operator->() const { return 0; }
};

template <typename T, typename... Args>
unique_ptr<T> make_unique(Args&&... args) {
  return unique_ptr<T>();
}

template <typename Key>
class set {};
template <typename Key>
class unordered_set {};
template <typename Key, typename Value>
class map {};
template <typename Key, typename Value>
class unordered_map {};
template <typename Elem>
class vector {};
template <typename Elem, size_t N>
class array {
 public:
  const Elem& operator[](size_t n) const { return elems_[n]; }

  const Elem* begin() const { return &elems_[0]; }
  const Elem* end() const { return &elems_[N]; }

 private:
  GC_PLUGIN_IGNORE("A mock of an array for testing") Elem elems_[N];
};
template <typename T1, typename T2>
class pair {};
template <typename T>
class optional {};
template <class... Ts>
class variant {};

}  // namespace std

namespace base {

template <typename T>
std::unique_ptr<T> WrapUnique(T* ptr) {
  return std::unique_ptr<T>();
}

}  // namespace base

namespace absl {

template <typename T>
class optional {};

template <class... Ts>
class variant {};

}  // namespace absl

namespace cppgc {

class Visitor {
 public:
  template <typename T, void (T::*method)(Visitor*)>
  void RegisterWeakMembers(const T* obj);

  template <typename T>
  void Trace(const T&);

  template <typename T>
  void TraceMultiple(const T* start, size_t len);

  template <typename K, typename V>
  void TraceEphemeron(const K& key, const V* value);
};

namespace internal {
class WriteBarrierPolicyImpl;
class NoWriteBarrierPolicyImpl;
class CheckingPolicyImpl;
class StorateTypeImpl;
class LocationPolicyImpl;

class StrongMemberTag;
class WeakMemberTag;
class UntracedMemberTag;

template <typename StorageType>
class MemberBase {};

template <typename T,
          typename WeaknessTag,
          typename WriteBarrierPolicy,
          typename CheckingPolicy,
          typename StorageType>
class BasicMember : public MemberBase<StorageType> {
 public:
  operator T*() const { return 0; }
  T* operator->() const { return 0; }
  bool operator!() const { return false; }
};

class StrongPersistentPolicy;
class WeakPersistentPolicy;

class PersistentBase {};

template <typename T,
          typename WeaknessPolicy,
          typename LocationPolicy,
          typename CheckingPolicy>
class BasicPersistent : public PersistentBase {
 public:
  operator T*() const { return 0; }
  T* operator->() const { return 0; }
  bool operator!() const { return false; }
};

class StrongCrossThreadPersistentPolicy;
class WeakCrossThreadPersistentPolicy;

class CrossThreadPersistentBase : public PersistentBase {};

template <typename T,
          typename WeaknessPolicy,
          typename LocationPolicy,
          typename CheckingPolicy>
class BasicCrossThreadPersistent : public CrossThreadPersistentBase {
 public:
  operator T*() const { return 0; }
  T* operator->() const { return 0; }
  bool operator!() const { return false; }
};

}  // namespace internal

template <typename T>
class GarbageCollected {
 public:
  void* operator new(size_t, void* location) { return location; }

 private:
  void* operator new(size_t) = delete;
  void* operator new[](size_t) = delete;
};

template <typename T, typename... Args>
T* MakeGarbageCollected(int, Args&&... args) {
  return new (reinterpret_cast<void*>(0x87654321)) T(args...);
}

class GarbageCollectedMixin {
 public:
  virtual void Trace(Visitor*) const {}
};

template <typename T>
using Member = internal::BasicMember<T,
                                     internal::StrongMemberTag,
                                     internal::WriteBarrierPolicyImpl,
                                     internal::CheckingPolicyImpl,
                                     internal::StorateTypeImpl>;
template <typename T>
using WeakMember = internal::BasicMember<T,
                                         internal::WeakMemberTag,
                                         internal::WriteBarrierPolicyImpl,
                                         internal::CheckingPolicyImpl,
                                         internal::StorateTypeImpl>;

template <typename T>
using UntracedMember = internal::BasicMember<T,
                                             internal::UntracedMemberTag,
                                             internal::NoWriteBarrierPolicyImpl,
                                             internal::CheckingPolicyImpl,
                                             internal::StorateTypeImpl>;

template <typename T>
using Persistent = internal::BasicPersistent<T,
                                             internal::StrongPersistentPolicy,
                                             internal::LocationPolicyImpl,
                                             internal::CheckingPolicyImpl>;
template <typename T>
using WeakPersistent = internal::BasicPersistent<T,
                                                 internal::WeakPersistentPolicy,
                                                 internal::LocationPolicyImpl,
                                                 internal::CheckingPolicyImpl>;

namespace subtle {

template <typename T>
using CrossThreadPersistent = internal::BasicCrossThreadPersistent<
    T,
    internal::StrongCrossThreadPersistentPolicy,
    internal::LocationPolicyImpl,
    internal::CheckingPolicyImpl>;
template <typename T>
using CrossThreadWeakPersistent = internal::BasicCrossThreadPersistent<
    T,
    internal::WeakCrossThreadPersistentPolicy,
    internal::LocationPolicyImpl,
    internal::CheckingPolicyImpl>;

}  // namespace subtle

}  // namespace cppgc

namespace v8 {

template <typename T>
class TracedReference {
 public:
  operator T*() const { return 0; }
  T* operator->() const { return 0; }
  bool operator!() const { return false; }
};

}  // namespace v8

namespace blink {

using Visitor = cppgc::Visitor;

template <typename T>
using GarbageCollected = cppgc::GarbageCollected<T>;
template <typename T, typename... Args>
T* MakeGarbageCollected(Args&&... args) {
  return cppgc::MakeGarbageCollected<T>(0, args...);
}

using GarbageCollectedMixin = cppgc::GarbageCollectedMixin;

template <typename T>
using Member = cppgc::Member<T>;
template <typename T>
using WeakMember = cppgc::WeakMember<T>;
template <typename T>
using UntracedMember = cppgc::UntracedMember<T>;
template <typename T>
using Persistent = cppgc::Persistent<T>;
template <typename T>
using WeakPersistent = cppgc::WeakPersistent<T>;
template <typename T>
using CrossThreadPersistent = cppgc::subtle::CrossThreadPersistent<T>;
template <typename T>
using CrossThreadWeakPersistent = cppgc::subtle::CrossThreadWeakPersistent<T>;

template <typename T>
using TraceWrapperV8Reference = v8::TracedReference<T>;

using namespace WTF;

#define DISALLOW_NEW()                                            \
 public:                                                          \
  void* operator new(size_t, void* location) { return location; } \
                                                                  \
 private:                                                         \
  void* operator new(size_t) = delete

#define STACK_ALLOCATED()                                  \
 public:                                                   \
  using IsStackAllocatedTypeMarker [[maybe_unused]] = int; \
                                                           \
 private:                                                  \
  void* operator new(size_t) = delete;                     \
  void* operator new(size_t, void*) = delete

template <typename T>
class RefCountedGarbageCollected : public GarbageCollected<T> {};

class HeapAllocator {
public:
    static const bool isGarbageCollected = true;
};

template <typename T, size_t inlineCapacity = 0>
class HeapVector : public GarbageCollected<HeapVector<T, inlineCapacity>>,
                   public Vector<T, inlineCapacity, HeapAllocator> {};

template <typename T, size_t inlineCapacity = 0>
class HeapDeque : public GarbageCollected<HeapDeque<T, inlineCapacity>>,
                  public Vector<T, inlineCapacity, HeapAllocator> {};

template <typename T>
class HeapHashSet : public GarbageCollected<HeapHashSet<T>>,
                    public HashSet<T, void, HeapAllocator> {};

template <typename T>
class HeapLinkedHashSet : public GarbageCollected<HeapLinkedHashSet<T>>,
                          public LinkedHashSet<T, void, HeapAllocator> {};

template <typename T>
class HeapHashCountedSet : public GarbageCollected<HeapHashCountedSet<T>>,
                           public HashCountedSet<T, void, HeapAllocator> {};

template <typename K, typename V>
class HeapHashMap : public GarbageCollected<HeapHashMap<K, V>>,
                    public HashMap<K, V, void, void, HeapAllocator> {};

template<typename T>
struct TraceIfNeeded {
  static void Trace(Visitor*, const T&);
};

}  // namespace blink

#endif
