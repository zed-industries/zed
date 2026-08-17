// Copyright 2017 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef LIBMEMTRACK_WRAPPER_H_
#define LIBMEMTRACK_WRAPPER_H_

#include <stdint.h>

// Wrapper on top of libmemtrack API.

// Opaque structure with memory stats.
// See $ANDROID/system/core/libmemtrack/include/memtrack/memtrack.h for details.
struct libmemtrack_proc;

// These numbers are vendor-specific and can't be trusted as a stable metric
// across different hardware or driver versions.
class MemtrackProc {
 public:
  explicit MemtrackProc(int pid);
  ~MemtrackProc();

  uint64_t graphics_total() const;
  uint64_t graphics_pss() const;
  uint64_t gl_total() const;
  uint64_t gl_pss() const;
  uint64_t other_total() const;
  uint64_t other_pss() const;

  bool has_errors() const { return proc_ == nullptr; };

 private:
  MemtrackProc(const MemtrackProc&) = delete;
  void operator=(const MemtrackProc&) = delete;

  libmemtrack_proc* proc_ = nullptr;
};

#endif  // LIBMEMTRACK_WRAPPER_H_
