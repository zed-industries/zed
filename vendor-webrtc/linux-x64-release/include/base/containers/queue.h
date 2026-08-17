// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_CONTAINERS_QUEUE_H_
#define BASE_CONTAINERS_QUEUE_H_

#include <queue>

#include "base/containers/circular_deque.h"

namespace base {

// Provides a definition of base::queue that's like std::queue but uses a
// base::circular_deque instead of std::deque. Since std::queue is just a
// wrapper for an underlying type, we can just provide a typedef for it that
// defaults to the base circular_deque.
template <class T, class Container = circular_deque<T>>
using queue = std::queue<T, Container>;

}  // namespace base

#endif  // BASE_CONTAINERS_QUEUE_H_
