//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TRACKING_MEM_RES_H
#define TRACKING_MEM_RES_H

#include <cstddef>
#include <memory_resource>

class TrackingMemRes : public std::pmr::memory_resource {
public:
  TrackingMemRes(std::size_t* last_size, size_t* last_alignment)
      : last_size_(last_size), last_alignment_(last_alignment) {}

private:
  std::size_t* last_size_;
  std::size_t* last_alignment_;
  void* do_allocate(std::size_t size, size_t alignment) override {
    *last_size_      = size;
    *last_alignment_ = alignment;

    return std::pmr::new_delete_resource()->allocate(size, alignment);
  }

  void do_deallocate(void* ptr, std::size_t size, size_t alignment) override {
    *last_size_      = size;
    *last_alignment_ = alignment;
    std::pmr::new_delete_resource()->deallocate(ptr, size, alignment);
  }

  bool do_is_equal(const memory_resource& ptr) const noexcept override { return &ptr == this; }
};

#endif // TRACKING_MEM_RES_H
