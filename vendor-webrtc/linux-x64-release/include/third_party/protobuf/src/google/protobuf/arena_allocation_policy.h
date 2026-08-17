// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_ARENA_ALLOCATION_POLICY_H__
#define GOOGLE_PROTOBUF_ARENA_ALLOCATION_POLICY_H__

#include <cstddef>
#include <cstdint>

namespace google {
namespace protobuf {
namespace internal {

// `AllocationPolicy` defines `Arena` allocation policies. Applications can
// customize the initial and maximum sizes for arena allocation, as well as set
// custom allocation and deallocation functions. `AllocationPolicy` is for
// protocol buffer internal use only, and typically created from a user facing
// public configuration class such as `ArenaOptions`.
struct AllocationPolicy {
  static constexpr size_t kDefaultStartBlockSize = 256;
  static constexpr size_t kDefaultMaxBlockSize = 32 << 10;

  size_t start_block_size = kDefaultStartBlockSize;
  size_t max_block_size = kDefaultMaxBlockSize;

  void* (*block_alloc)(size_t) = nullptr;
  void (*block_dealloc)(void*, size_t) = nullptr;

  bool IsDefault() const {
    return start_block_size == kDefaultStartBlockSize &&
           max_block_size == kDefaultMaxBlockSize && block_alloc == nullptr &&
           block_dealloc == nullptr;
  }
};

// Tagged pointer to an AllocationPolicy.
class TaggedAllocationPolicyPtr {
 public:
  constexpr TaggedAllocationPolicyPtr() : policy_(0) {}

  explicit TaggedAllocationPolicyPtr(AllocationPolicy* policy)
      : policy_(reinterpret_cast<uintptr_t>(policy)) {}

  void set_policy(AllocationPolicy* policy) {
    auto bits = policy_ & kTagsMask;
    policy_ = reinterpret_cast<uintptr_t>(policy) | bits;
  }

  AllocationPolicy* get() {
    return reinterpret_cast<AllocationPolicy*>(policy_ & kPtrMask);
  }
  const AllocationPolicy* get() const {
    return reinterpret_cast<const AllocationPolicy*>(policy_ & kPtrMask);
  }

  AllocationPolicy& operator*() { return *get(); }
  const AllocationPolicy& operator*() const { return *get(); }

  AllocationPolicy* operator->() { return get(); }
  const AllocationPolicy* operator->() const { return get(); }

  bool is_user_owned_initial_block() const {
    return static_cast<bool>(get_mask<kUserOwnedInitialBlock>());
  }
  void set_is_user_owned_initial_block(bool v) {
    set_mask<kUserOwnedInitialBlock>(v);
  }

  uintptr_t get_raw() const { return policy_; }

 private:
  enum : uintptr_t {
    kUserOwnedInitialBlock = 1,
  };

  static constexpr uintptr_t kTagsMask = 7;
  static constexpr uintptr_t kPtrMask = ~kTagsMask;

  template <uintptr_t kMask>
  uintptr_t get_mask() const {
    return policy_ & kMask;
  }
  template <uintptr_t kMask>
  void set_mask(bool v) {
    if (v) {
      policy_ |= kMask;
    } else {
      policy_ &= ~kMask;
    }
  }
  uintptr_t policy_;
};

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_ARENA_ALLOCATION_POLICY_H__
