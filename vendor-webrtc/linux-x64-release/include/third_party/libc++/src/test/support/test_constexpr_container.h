//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_TEST_CONSTEXPR_CONTAINER_H
#define SUPPORT_TEST_CONSTEXPR_CONTAINER_H

// A dummy container with enough constexpr support to test the standard
// insert iterators, such as `back_insert_iterator`.

#include <algorithm>
#include <cassert>
#include <cstddef>
#include <utility>

#include "test_macros.h"

#if TEST_STD_VER >= 14

template<class T, int N>
class ConstexprFixedCapacityDeque {
    T data_[N];
    int size_ = 0;
public:
    using value_type = T;
    using iterator = T *;
    using const_iterator = T const *;

    constexpr ConstexprFixedCapacityDeque() = default;
    constexpr iterator begin() { return data_; }
    constexpr iterator end() { return data_ + size_; }
    constexpr const_iterator begin() const { return data_; }
    constexpr const_iterator end() const { return data_ + size_; }
    constexpr std::size_t size() const { return size_; }
    constexpr const T& front() const { assert(size_ >= 1); return data_[0]; }
    constexpr const T& back() const { assert(size_ >= 1); return data_[size_-1]; }

    constexpr iterator insert(const_iterator pos, T t) {
        int i = static_cast<int>(pos - data_);
        if (i != size_) {
            std::move_backward(data_ + i, data_ + size_, data_ + size_ + 1);
        }
        data_[i] = std::move(t);
        size_ += 1;
        return data_ + i;
    }

    constexpr void push_back(T t) { insert(end(), std::move(t)); }
    constexpr void push_front(T t) { insert(begin(), std::move(t)); }
};

#endif // TEST_STD_VER >= 14

#endif // SUPPORT_TEST_CONSTEXPR_CONTAINER_H
