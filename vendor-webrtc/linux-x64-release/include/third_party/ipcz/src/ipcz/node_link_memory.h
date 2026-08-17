// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_NODE_LINK_MEMORY_H_
#define IPCZ_SRC_IPCZ_NODE_LINK_MEMORY_H_

#include <atomic>
#include <cstddef>
#include <cstdint>
#include <functional>
#include <vector>

#include "ipcz/buffer_id.h"
#include "ipcz/buffer_pool.h"
#include "ipcz/driver_memory.h"
#include "ipcz/driver_memory_mapping.h"
#include "ipcz/features.h"
#include "ipcz/fragment_descriptor.h"
#include "ipcz/fragment_ref.h"
#include "ipcz/ipcz.h"
#include "ipcz/link_side.h"
#include "ipcz/ref_counted_fragment.h"
#include "ipcz/router_link_state.h"
#include "ipcz/sublink_id.h"
#include "third_party/abseil-cpp/absl/container/flat_hash_map.h"
#include "third_party/abseil-cpp/absl/synchronization/mutex.h"
#include "third_party/abseil-cpp/absl/types/span.h"
#include "util/ref_counted.h"

namespace ipcz {

class Node;
class NodeLink;

// NodeLinkMemory owns and manages all shared memory resource allocation on a
// single NodeLink. Each end of a NodeLink has its own NodeLinkMemory instance
// cooperatively managing the same dynamic pool of memory, shared exclusively
// between the two endpoint nodes.
class NodeLinkMemory : public RefCounted<NodeLinkMemory> {
 public:
  static constexpr BufferId kPrimaryBufferId{0};

  // The maximum number of initial portals supported on ConnectNode() API calls.
  // The first kMaxInitialPortals SublinkIds on a NodeLinkMemory will always be
  // reserved for use by initial portals.
  static constexpr size_t kMaxInitialPortals = 12;

  // The side of the node link to which this NodeLinkMemory belongs.
  LinkSide link_side() const { return link_side_; }

  // The set of runtime features available to both nodes using this memory.
  const Features& available_features() const { return available_features_; }

  // Sets a reference to the NodeLink using this NodeLinkMemory. This is called
  // by the NodeLink itself before any other methods can be called on the
  // NodeLinkMemory, and it's only reset to null once the NodeLink is
  // deactivated. This link may be used to share information with the remote
  // node, where another NodeLinkMemory is cooperatively managing the same
  // memory pool as this one. `link` must belong to the same side of the node
  // link as this object.
  void SetNodeLink(Ref<NodeLink> link);

  // Allocates a new DriverMemory object and initializes its contents to be
  // suitable as the primary buffer of a new NodeLinkMemory. Returns the memory
  // along with a mapping of it.
  static DriverMemoryWithMapping AllocateMemory(const IpczDriver& driver);

  // Constructs a new NodeLinkMemory with BufferId 0 (the primary buffer) mapped
  // as `primary_buffer_memory`. The buffer must have been created and
  // initialized by a prior call to AllocateMemory() above.
  static Ref<NodeLinkMemory> Create(Ref<Node> node,
                                    LinkSide side,
                                    const Features& remote_features,
                                    DriverMemoryMapping primary_buffer_memory);

  // Returns a new BufferId which should still be unused by any buffer in this
  // NodeLinkMemory's BufferPool, or that of its peer NodeLinkMemory. When
  // allocating new a buffer to add to the BufferPool, its BufferId should be
  // procured by calling this method.
  BufferId AllocateNewBufferId();

  // Returns the first of `count` newly allocated, contiguous sublink IDs for
  // use on the corresponding NodeLink.
  SublinkId AllocateSublinkIds(size_t count);

  // Returns a ref to the RouterLinkState for the `i`th initial portal on the
  // NodeLink, established by the Connect() call which created this link. Unlike
  // other RouterLinkStates which are allocated dynamically, these have a fixed
  // location within the NodeLinkMemory's primary buffer. The returned
  // FragmentRef is unmanaged and will never free its underlying fragment.
  FragmentRef<RouterLinkState> GetInitialRouterLinkState(size_t i);

  // Resolves `descriptor` to a concrete Fragment. If the descriptor is null or
  // describes a region of memory which exceeds the bounds of the identified
  // buffer, this returns a null Fragment. If the descriptor's BufferId is not
  // yet registered with this NodeLinkMemory, this returns a pending Fragment
  // with the same BufferId and dimensions as `descriptor`.
  Fragment GetFragment(const FragmentDescriptor& descriptor);

  // Adopts an existing reference to a RefCountedFragment within `fragment`,
  // which must be a valid, properly aligned, and sufficiently sized fragment to
  // hold a T. This does NOT increment the ref count of the RefCountedFragment.
  template <typename T>
  FragmentRef<T> AdoptFragmentRef(const Fragment& fragment) {
    ABSL_ASSERT(sizeof(T) <= fragment.size());
    return FragmentRef<T>(kAdoptExistingRef, WrapRefCounted(this), fragment);
  }

  // Attempts to adopt an existing reference to a RefCountedFragment located at
  // `fragment`. Returns null if the fragment descriptor is null, misaligned,
  // or of insufficient size. This does NOT increment the ref count of the
  // RefCountedFragment.
  template <typename T>
  FragmentRef<T> AdoptFragmentRefIfValid(const FragmentDescriptor& descriptor) {
    if (descriptor.is_null() || descriptor.size() < sizeof(T) ||
        descriptor.offset() % 8 != 0) {
      return {};
    }

    const Fragment fragment = GetFragment(descriptor);
    if (fragment.is_null()) {
      return {};
    }

    return AdoptFragmentRef<T>(fragment);
  }

  // Adds a new buffer to the underlying BufferPool to use as additional
  // allocation capacity for blocks of size `block_size`. Note that the
  // contents of the mapped region must already be initialized as a
  // BlockAllocator.
  bool AddBlockBuffer(BufferId id,
                      size_t block_size,
                      DriverMemoryMapping mapping);

  // Allocates a Fragment of `size` bytes from the underlying BufferPool. May
  // return a null Fragment if there was no readily available capacity.
  Fragment AllocateFragment(size_t size);

  // Attempts to allocate a Fragment of at least `size` bytes. If there are no
  // readily available fragments large enough, this may return a fragment
  // smaller than `size`.
  Fragment AllocateFragmentBestEffort(size_t size);

  // Frees a Fragment previously allocated through this NodeLinkMemory. Returns
  // true on success. Returns false if `fragment` does not represent an
  // allocated fragment within this NodeLinkMemory.
  bool FreeFragment(const Fragment& fragment);

  // Allocates a fragment to store a new RouterLinkState and initializes a new
  // RouterLinkState instance there. If no capacity is currently available to
  // allocate an appropriate fragment, this may return null.
  FragmentRef<RouterLinkState> TryAllocateRouterLinkState();

  // Allocates a fragment to store a new RouterLinkState and initializes a new
  // RouterLinkState instance there. Calls `callback` with a reference to the
  // new fragment once allocated. Unlike TryAllocateRouterLinkState(), this
  // allocation always succeeds eventually unless driver memory allocation
  // itself begins to fail unrecoverably. If the allocation can succeed
  // synchronously, `callback` may be called before this method returns.
  using RouterLinkStateCallback =
      std::function<void(FragmentRef<RouterLinkState>)>;
  void AllocateRouterLinkState(RouterLinkStateCallback callback);

  // Runs `callback` as soon as the identified buffer is added to the underlying
  // BufferPool. If the buffer is already present here, `callback` is run
  // immediately.
  void WaitForBufferAsync(BufferId id,
                          BufferPool::WaitForBufferCallback callback);

  // Called when the corresponding NodeLink has had its transport disconnected.
  // This should clean up any resources that might have been retained pending
  // future transport activity.
  void NotifyLinkDisconnected();

 private:
  struct PrimaryBuffer;

  friend class RefCounted<NodeLinkMemory>;

  // Constructs a new NodeLinkMemory over `mapping`, which must correspond to
  // a DriverMemory whose contents have already been initialized as a
  // NodeLinkMemory primary buffer.
  NodeLinkMemory(Ref<Node> node,
                 LinkSide side,
                 const Features& remote_features,
                 DriverMemoryMapping mapping);
  ~NodeLinkMemory();

  // Indicates whether the NodeLinkMemory should be allowed to expand its
  // allocation capacity further for blocks of size `block_size`.
  bool CanExpandBlockCapacity(size_t block_size);

  // Attempts to expand the total block allocation capacity for blocks of
  // `block_size` bytes. `callback` may be called synchronously or
  // asynchronously with a result indicating whether the expansion succeeded.
  using RequestBlockCapacityCallback = std::function<void(bool)>;
  void RequestBlockCapacity(size_t block_size,
                            RequestBlockCapacityCallback callback);
  void OnCapacityRequestComplete(size_t block_size, bool success);

  // Initializes `fragment` as a new RouterLinkState and returns a ref to it.
  FragmentRef<RouterLinkState> InitializeRouterLinkStateFragment(
      const Fragment& fragment);

  const Ref<Node> node_;
  const LinkSide link_side_;
  const Features available_features_;
  const bool allow_memory_expansion_for_parcel_data_;

  // Atomic ID generators for buffers and sublinks allocated by this side of the
  // link when memv2 is enabled.
  std::atomic<uint64_t> next_buffer_id_{1};
  std::atomic<uint64_t> next_sublink_id_{kMaxInitialPortals};

  // The underlying BufferPool. Note that this object is itself thread-safe, so
  // access to it is not synchronized by NodeLinkMemory.
  BufferPool buffer_pool_;

  // Mapping for this link's fixed primary buffer.
  const absl::Span<uint8_t> primary_buffer_memory_;
  PrimaryBuffer& primary_buffer_;

  absl::Mutex mutex_;

  // The NodeLink which is using this NodeLinkMemory. Used to communicate with
  // the NodeLinkMemory on the other side of the link.
  Ref<NodeLink> node_link_ ABSL_GUARDED_BY(mutex_);

  // Callbacks to invoke when a pending capacity request is fulfilled for a
  // specific block size. Also used to prevent stacking of capacity requests for
  // the same block size.
  using CapacityCallbackList = std::vector<RequestBlockCapacityCallback>;
  absl::flat_hash_map<uint32_t, CapacityCallbackList> capacity_callbacks_
      ABSL_GUARDED_BY(mutex_);
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_NODE_LINK_MEMORY_H_
