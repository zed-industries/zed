// Copyright 2009 The RE2 Authors.  All Rights Reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#ifndef UTIL_MALLOC_COUNTER_H_
#define UTIL_MALLOC_COUNTER_H_

namespace testing {
class MallocCounter {
 public:
  MallocCounter(int x) {}
  static const int THIS_THREAD_ONLY = 0;
  long long HeapGrowth() { return 0; }
  long long PeakHeapGrowth() { return 0; }
  void Reset() {}
};
}  // namespace testing

#endif  // UTIL_MALLOC_COUNTER_H_
