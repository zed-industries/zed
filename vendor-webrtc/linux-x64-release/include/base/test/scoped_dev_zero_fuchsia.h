// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_SCOPED_DEV_ZERO_FUCHSIA_H_
#define BASE_TEST_SCOPED_DEV_ZERO_FUCHSIA_H_

#include <lib/fdio/namespace.h>

#include "base/memory/ref_counted.h"
#include "base/memory/scoped_refptr.h"
#include "base/threading/sequence_bound.h"
#include "base/threading/thread.h"

namespace base {

// An object that causes /dev/zero to exist during its lifetime. A reference to
// this class may be held by tests that require access to /dev/zero for the
// lifetime of that need.
class ScopedDevZero final : public RefCounted<ScopedDevZero> {
 public:
  REQUIRE_ADOPTION_FOR_REFCOUNTED_TYPE();

  // Returns a reference to the process-global /dev/zero. This must only be
  // called, and the returned reference released, on the main thread. Returns
  // null in case of failure to create the instance. It is good practice for
  // tests to ASSERT the returned pointer.
  static scoped_refptr<ScopedDevZero> Get();

  ScopedDevZero(const ScopedDevZero&) = delete;
  ScopedDevZero operator=(const ScopedDevZero&) = delete;

 private:
  friend class RefCounted<ScopedDevZero>;
  class Server;

  ScopedDevZero();
  ~ScopedDevZero();

  // Spins off the server thread and binds its pesudo-dir to /dev, returning
  // true if all goes well, or false in case of any error.
  bool Initialize();

  // A raw pointer to the process's single instance. Multiple references to this
  // instance may be handed out to consumers.
  static ScopedDevZero* instance_;
  Thread io_thread_;
  fdio_ns_t* global_namespace_ = nullptr;
  SequenceBound<Server> server_;
};

}  // namespace base

#endif  // BASE_TEST_SCOPED_DEV_ZERO_FUCHSIA_H_
