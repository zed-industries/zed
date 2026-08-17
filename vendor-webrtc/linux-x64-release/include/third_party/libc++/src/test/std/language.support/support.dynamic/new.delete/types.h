//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_LANGUAGE_SUPPORT_SUPPORT_DYNAMIC_NEW_DELETE_TYPES_H
#define TEST_STD_LANGUAGE_SUPPORT_SUPPORT_DYNAMIC_NEW_DELETE_TYPES_H

#include <cstddef>

#include "test_macros.h"

struct LifetimeInformation {
    void* address_constructed = nullptr;
    void* address_destroyed = nullptr;
};

struct TrackLifetime {
    TrackLifetime(LifetimeInformation& info) : info_(&info) {
        info_->address_constructed = this;
    }
    TrackLifetime(TrackLifetime const&) = default;
    ~TrackLifetime() {
        info_->address_destroyed = this;
    }
    LifetimeInformation* info_;
};

#if TEST_STD_VER >= 17
struct alignas(__STDCPP_DEFAULT_NEW_ALIGNMENT__ * 2) TrackLifetimeOverAligned {
    TrackLifetimeOverAligned(LifetimeInformation& info) : info_(&info) {
        info_->address_constructed = this;
    }
    ~TrackLifetimeOverAligned() {
        info_->address_destroyed = this;
    }
    LifetimeInformation* info_;
};

struct alignas(std::max_align_t) TrackLifetimeMaxAligned {
    TrackLifetimeMaxAligned(LifetimeInformation& info) : info_(&info) {
        info_->address_constructed = this;
    }
    TrackLifetimeMaxAligned(TrackLifetimeMaxAligned const&) = default;
    ~TrackLifetimeMaxAligned() {
        info_->address_destroyed = this;
    }
    LifetimeInformation* info_;
};

struct alignas(__STDCPP_DEFAULT_NEW_ALIGNMENT__ * 2) OverAligned {

};

struct alignas(std::max_align_t) MaxAligned {

};
#endif // TEST_STD_VER >= 17

template <class F>
void test_with_interesting_alignments(F f) {
  // First, check the basic case, a large allocation with alignment == size.
  f(/* size */ 64, /* alignment */ 64);

  // Size being a multiple of alignment also needs to be supported.
  f(/* size */ 64, /* alignment */ 32);

  // Test with a non power-of-two size.
  f(/* size */ 10, /* alignment */ 64);

  // When aligned allocation is implemented using aligned_alloc,
  // that function requires a minimum alignment of sizeof(void*).
  //
  // Check that we can also create overaligned allocations with
  // an alignment argument less than sizeof(void*).
  f(/* size */ 2, /* alignment */ 2);

  // When implemented using the C11 aligned_alloc() function,
  // that requires that size be a multiple of alignment.
  // However, the C++ operator new has no such requirements.
  //
  // Check that we can create an overaligned allocation that does
  // adhere to not have this constraint.
  f(/* size */ 1, /* alignment */ 128);

  // Finally, test size > alignment, but with size not being
  // a multiple of alignment.
  f(/* size */ 65, /* alignment */ 32);
}

#endif // TEST_STD_LANGUAGE_SUPPORT_SUPPORT_DYNAMIC_NEW_DELETE_TYPES_H
