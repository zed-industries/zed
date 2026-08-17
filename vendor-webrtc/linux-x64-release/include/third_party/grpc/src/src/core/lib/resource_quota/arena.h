//
//
// Copyright 2017 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

// \file Arena based allocator
// Allows very fast allocation of memory, but that memory cannot be freed until
// the arena as a whole is freed
// Tracks the total memory allocated against it, so that future arenas can
// pre-allocate the right amount of memory

#ifndef GRPC_SRC_CORE_LIB_RESOURCE_QUOTA_ARENA_H
#define GRPC_SRC_CORE_LIB_RESOURCE_QUOTA_ARENA_H

#include <grpc/event_engine/memory_allocator.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>

#include <atomic>
#include <cstddef>
#include <iosfwd>
#include <memory>
#include <utility>

#include "src/core/lib/promise/context.h"
#include "src/core/lib/resource_quota/memory_quota.h"
#include "src/core/util/alloc.h"
#include "src/core/util/construct_destruct.h"

namespace grpc_core {

class Arena;

template <typename T>
struct ArenaContextType;

namespace arena_detail {

// Tracks all registered arena context types (these should only be registered
// via ArenaContextTraits at static initialization time).
class BaseArenaContextTraits {
 public:
  // Count of number of contexts that have been allocated.
  static uint16_t NumContexts() {
    return static_cast<uint16_t>(RegisteredTraits().size());
  }

  // Number of bytes required to store the context pointers on an arena.
  static size_t ContextSize() { return NumContexts() * sizeof(void*); }

  // Call the registered destruction function for a context.
  static void Destroy(uint16_t id, void* ptr) {
    if (ptr == nullptr) return;
    RegisteredTraits()[id](ptr);
  }

 protected:
  // Allocate a new context id and register the destruction function.
  static uint16_t MakeId(void (*destroy)(void* ptr)) {
    auto& traits = RegisteredTraits();
    const uint16_t id = static_cast<uint16_t>(traits.size());
    traits.push_back(destroy);
    return id;
  }

 private:
  static std::vector<void (*)(void*)>& RegisteredTraits() {
    static NoDestruct<std::vector<void (*)(void*)>> registered_traits;
    return *registered_traits;
  }
};

// Traits for a specific context type.
template <typename T>
class ArenaContextTraits : public BaseArenaContextTraits {
 public:
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static uint16_t id() { return id_; }

 private:
  static const uint16_t id_;
};

template <typename T>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline void DestroyArenaContext(void* p) {
  ArenaContextType<T>::Destroy(static_cast<T*>(p));
}

template <typename T>
const uint16_t ArenaContextTraits<T>::id_ =
    BaseArenaContextTraits::MakeId(DestroyArenaContext<T>);

template <typename T, typename A, typename B>
struct IfArray {
  using Result = A;
};

template <typename T, typename A, typename B>
struct IfArray<T[], A, B> {
  using Result = B;
};

struct UnrefDestroy {
  void operator()(const Arena* arena) const;
};

}  // namespace arena_detail

class ArenaFactory : public RefCounted<ArenaFactory> {
 public:
  virtual RefCountedPtr<Arena> MakeArena() = 0;
  virtual void FinalizeArena(Arena* arena) = 0;

  MemoryAllocator& allocator() { return allocator_; }

 protected:
  explicit ArenaFactory(MemoryAllocator allocator)
      : allocator_(std::move(allocator)) {}

 private:
  MemoryAllocator allocator_;
};

MemoryAllocator DefaultMemoryAllocatorForSimpleArenaAllocator();
RefCountedPtr<ArenaFactory> SimpleArenaAllocator(
    size_t initial_size = 1024,
    MemoryAllocator allocator =
        DefaultMemoryAllocatorForSimpleArenaAllocator());

class Arena final : public RefCounted<Arena, NonPolymorphicRefCount,
                                      arena_detail::UnrefDestroy> {
 public:
  // Create an arena, with \a initial_size bytes in the first allocated buffer.
  static RefCountedPtr<Arena> Create(size_t initial_size,
                                     RefCountedPtr<ArenaFactory> arena_factory);

  // Destroy all `ManagedNew` allocated objects.
  // Allows safe destruction of these objects even if they need context held by
  // the arena.
  // Idempotent.
  // TODO(ctiller): eliminate ManagedNew.
  void DestroyManagedNewObjects();

  // Return the total amount of memory allocated by this arena.
  size_t TotalUsedBytes() const {
    return total_used_.load(std::memory_order_relaxed);
  }

  // Allocate \a size bytes from the arena.
  void* Alloc(size_t size) {
    size = GPR_ROUND_UP_TO_ALIGNMENT_SIZE(size);
    size_t begin = total_used_.fetch_add(size, std::memory_order_relaxed);
    if (begin + size <= initial_zone_size_) {
      return reinterpret_cast<char*>(this) + begin;
    } else {
      return AllocZone(size);
    }
  }

  // Allocates T from the arena.
  // The caller is responsible for calling p->~T(), but should NOT delete.
  // TODO(roth): We currently assume that all callers need alignment of 16
  // bytes, which may be wrong in some cases. When we have time, we should
  // change this to instead use the alignment of the type being allocated by
  // this method.
  template <typename T, typename... Args>
  T* New(Args&&... args) {
    T* t = static_cast<T*>(Alloc(sizeof(T)));
    new (t) T(std::forward<Args>(args)...);
    return t;
  }

  // Like New, but has the arena call p->~T() at arena destruction time.
  // The caller should NOT delete.
  template <typename T, typename... Args>
  T* ManagedNew(Args&&... args) {
    auto* p = New<ManagedNewImpl<T>>(std::forward<Args>(args)...);
    p->Link(&managed_new_head_);
    return &p->t;
  }

  template <typename T, typename... Args>
  absl::enable_if_t<std::is_same<typename T::RefCountedUnrefBehaviorType,
                                 UnrefCallDtor>::value,
                    RefCountedPtr<T>>
  MakeRefCounted(Args&&... args) {
    return RefCountedPtr<T>(New<T>(std::forward<Args>(args)...));
  }

  class PooledDeleter {
   public:
    PooledDeleter() = default;
    explicit PooledDeleter(std::nullptr_t) : delete_(false) {}
    template <typename T>
    void operator()(T* p) {
      // TODO(ctiller): promise based filter hijacks ownership of some pointers
      // to make them appear as PoolPtr without really transferring ownership,
      // by setting the arena to nullptr.
      // This is a transitional hack and should be removed once promise based
      // filter is removed.
      if (delete_) delete p;
    }

    bool has_freelist() const { return delete_; }

   private:
    bool delete_ = true;
  };

  class ArrayPooledDeleter {
   public:
    ArrayPooledDeleter() = default;
    explicit ArrayPooledDeleter(std::nullptr_t) : delete_(false) {}
    template <typename T>
    void operator()(T* p) {
      // TODO(ctiller): promise based filter hijacks ownership of some pointers
      // to make them appear as PoolPtr without really transferring ownership,
      // by setting the arena to nullptr.
      // This is a transitional hack and should be removed once promise based
      // filter is removed.
      if (delete_) delete[] p;
    }

    bool has_freelist() const { return delete_; }

   private:
    bool delete_ = true;
  };

  template <typename T>
  using PoolPtr =
      std::unique_ptr<T, typename arena_detail::IfArray<
                             T, PooledDeleter, ArrayPooledDeleter>::Result>;

  // Make a unique_ptr to T that is allocated from the arena.
  // When the pointer is released, the memory may be reused for other
  // MakePooled(.*) calls.
  // CAUTION: The amount of memory allocated is rounded up to the nearest
  //          value in Arena::PoolSizes, and so this may pessimize total
  //          arena size.
  template <typename T, typename... Args>
  static PoolPtr<T> MakePooled(Args&&... args) {
    return PoolPtr<T>(new T(std::forward<Args>(args)...), PooledDeleter());
  }

  template <typename T>
  static PoolPtr<T> MakePooledForOverwrite() {
    return PoolPtr<T>(new T, PooledDeleter());
  }

  // Make a unique_ptr to an array of T that is allocated from the arena.
  // When the pointer is released, the memory may be reused for other
  // MakePooled(.*) calls.
  // One can use MakePooledArray<char> to allocate a buffer of bytes.
  // CAUTION: The amount of memory allocated is rounded up to the nearest
  //          value in Arena::PoolSizes, and so this may pessimize total
  //          arena size.
  template <typename T>
  PoolPtr<T[]> MakePooledArray(size_t n) {
    return PoolPtr<T[]>(new T[n], ArrayPooledDeleter());
  }

  // Like MakePooled, but with manual memory management.
  // The caller is responsible for calling DeletePooled() on the returned
  // pointer, and expected to call it with the same type T as was passed to this
  // function (else the free list returned to the arena will be corrupted).
  template <typename T, typename... Args>
  T* NewPooled(Args&&... args) {
    return new T(std::forward<Args>(args)...);
  }

  template <typename T>
  void DeletePooled(T* p) {
    delete p;
  }

  // Context accessors
  // Prefer to use the free-standing `GetContext<>` and `SetContext<>` functions
  // for modern promise-based code -- however legacy filter stack based code
  // often needs to access these directly.
  template <typename T>
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION T* GetContext() {
    return static_cast<T*>(
        contexts()[arena_detail::ArenaContextTraits<T>::id()]);
  }

  template <typename T>
  void SetContext(T* context) {
    void*& slot = contexts()[arena_detail::ArenaContextTraits<T>::id()];
    if (slot != nullptr) {
      ArenaContextType<T>::Destroy(static_cast<T*>(slot));
    }
    slot = context;
    DCHECK_EQ(GetContext<T>(), context);
  }

  static size_t ArenaOverhead() {
    return GPR_ROUND_UP_TO_ALIGNMENT_SIZE(sizeof(Arena));
  }
  static size_t ArenaZoneOverhead() {
    return GPR_ROUND_UP_TO_ALIGNMENT_SIZE(sizeof(Zone));
  }

 private:
  friend struct arena_detail::UnrefDestroy;

  struct Zone {
    Zone* prev;
  };

  struct ManagedNewObject {
    ManagedNewObject* next = nullptr;
    void Link(std::atomic<ManagedNewObject*>* head);
    virtual ~ManagedNewObject() = default;
  };

  template <typename T>
  struct ManagedNewImpl : public ManagedNewObject {
    T t;
    template <typename... Args>
    explicit ManagedNewImpl(Args&&... args) : t(std::forward<Args>(args)...) {}
  };

  // Initialize an arena.
  // Parameters:
  //   initial_size: The initial size of the whole arena in bytes. These bytes
  //   are contained within 'zone 0'. If the arena user ends up requiring more
  //   memory than the arena contains in zone 0, subsequent zones are allocated
  //   on demand and maintained in a tail-linked list.
  //
  //   initial_alloc: Optionally, construct the arena as though a call to
  //   Alloc() had already been made for initial_alloc bytes. This provides a
  //   quick optimization (avoiding an atomic fetch-add) for the common case
  //   where we wish to create an arena and then perform an immediate
  //   allocation.
  explicit Arena(size_t initial_size,
                 RefCountedPtr<ArenaFactory> arena_factory);

  ~Arena();

  void* AllocZone(size_t size);
  void Destroy() const;
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION void** contexts() {
    return reinterpret_cast<void**>(this + 1);
  }

  // Keep track of the total used size. We use this in our call sizing
  // hysteresis.
  const size_t initial_zone_size_;
  std::atomic<size_t> total_used_;
  std::atomic<size_t> total_allocated_{initial_zone_size_};
  // If the initial arena allocation wasn't enough, we allocate additional zones
  // in a reverse linked list. Each additional zone consists of (1) a pointer to
  // the zone added before this zone (null if this is the first additional zone)
  // and (2) the allocated memory. The arena itself maintains a pointer to the
  // last zone; the zone list is reverse-walked during arena destruction only.
  std::atomic<Zone*> last_zone_{nullptr};
  std::atomic<ManagedNewObject*> managed_new_head_{nullptr};
  RefCountedPtr<ArenaFactory> arena_factory_;
};

// Arena backed single-producer-single-consumer queue
// Based on implementation from
// https://www.1024cores.net/home/lock-free-algorithms/queues/unbounded-spsc-queue
template <typename T, bool kOptimizeAlignment = true>
class ArenaSpsc {
 public:
  explicit ArenaSpsc(Arena* arena) : arena_(arena) {}

  ~ArenaSpsc() {
    while (Pop().has_value()) {
    }
  }

  ArenaSpsc(const ArenaSpsc&) = delete;
  ArenaSpsc& operator=(const ArenaSpsc&) = delete;

  // Push `value` onto the queue; at most one thread can be calling this at a
  // time.
  void Push(T value) {
    Node* n = AllocNode();
    Construct(&n->value, std::move(value));
    n->next.store(nullptr, std::memory_order_relaxed);
    head_->next.store(n, std::memory_order_release);
    head_ = n;
  }

  // Pop a value from the queue; at most one thread can be calling this at a
  // time. If the queue was empty when called, returns nullopt.
  std::optional<T> Pop() {
    Node* n = tail_.load(std::memory_order_relaxed);
    Node* next = n->next.load(std::memory_order_acquire);
    if (next == nullptr) return std::nullopt;
    T result = std::move(next->value);
    Destruct(&next->value);
    tail_.store(next, std::memory_order_release);
    return result;
  }

 private:
  struct Node {
    Node() {}
    ~Node() {}
    explicit Node(std::nullptr_t) : next{nullptr} {}
    std::atomic<Node*> next;
    union {
      T value;
    };
  };

  Node* AllocNode() {
    if (first_ != tail_copy_) {
      Node* n = first_;
      first_ = first_->next.load(std::memory_order_relaxed);
      return n;
    }
    tail_copy_ = tail_.load(std::memory_order_acquire);
    if (first_ != tail_copy_) {
      Node* n = first_;
      first_ = first_->next.load(std::memory_order_relaxed);
      return n;
    }
    return arena_->New<Node>();
  }

  Arena* const arena_;
  Node first_node_{nullptr};
  // Accessed mainly by consumer, infrequently by producer
  std::atomic<Node*> tail_{&first_node_};
  // Ensure alignment on next cacheline to deliminate producer and consumer
  // Head of queue
  alignas(kOptimizeAlignment ? GPR_CACHELINE_SIZE
                             : alignof(Node*)) Node* head_{&first_node_};
  // Last unused node
  Node* first_{&first_node_};
  // Helper, points somewhere between first and tail
  Node* tail_copy_{&first_node_};
};

// Arenas form a context for activities
template <>
struct ContextType<Arena> {};

namespace arena_detail {
inline void UnrefDestroy::operator()(const Arena* arena) const {
  arena->Destroy();
}
}  // namespace arena_detail

namespace promise_detail {

template <typename T>
class Context<T, absl::void_t<decltype(ArenaContextType<T>::Destroy)>> {
 public:
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static T* get() {
    return GetContext<Arena>()->GetContext<T>();
  }
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static void set(T* value) {
    GetContext<Arena>()->SetContext(value);
  }
};

}  // namespace promise_detail

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_RESOURCE_QUOTA_ARENA_H
