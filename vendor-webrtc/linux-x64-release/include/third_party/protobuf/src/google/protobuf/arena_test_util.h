// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_ARENA_TEST_UTIL_H__
#define GOOGLE_PROTOBUF_ARENA_TEST_UTIL_H__

#include <cstddef>

#include "absl/container/flat_hash_set.h"
#include "absl/log/absl_check.h"
#include "google/protobuf/arena.h"
#include "google/protobuf/io/coded_stream.h"
#include "google/protobuf/io/zero_copy_stream_impl_lite.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace internal {

struct ArenaTestPeer {
  static void ReturnArrayMemory(Arena* arena, void* p, size_t size) {
    arena->ReturnArrayMemory(p, size);
  }
  static auto PeekCleanupListForTesting(Arena* arena) {
    return arena->PeekCleanupListForTesting();
  }
  template <typename T, typename... U>
  static constexpr auto GetConstructType() {
    return Arena::GetConstructType<T, U...>();
  }
  using ConstructType = Arena::ConstructType;
};

struct CleanupGrowthInfo {
  size_t space_used;
  absl::flat_hash_set<void*> cleanups;
};

template <typename Func>
CleanupGrowthInfo CleanupGrowth(Arena& arena, Func f) {
  auto old_space_used = arena.SpaceUsed();
  auto old_cleanups = ArenaTestPeer::PeekCleanupListForTesting(&arena);
  f();
  auto new_space_used = arena.SpaceUsed();
  auto new_cleanups = ArenaTestPeer::PeekCleanupListForTesting(&arena);
  CleanupGrowthInfo res;
  res.space_used = new_space_used - old_space_used;
  res.cleanups.insert(new_cleanups.begin(), new_cleanups.end());
  for (auto p : old_cleanups) res.cleanups.erase(p);
  return res;
}

class NoHeapChecker {
 public:
  NoHeapChecker() { capture_alloc.Hook(); }
  ~NoHeapChecker();

 private:
  class NewDeleteCapture {
   public:
    // TODO: Implement this for opensource protobuf.
    void Hook() {}
    void Unhook() {}
    int alloc_count() { return 0; }
    int free_count() { return 0; }
  } capture_alloc;
};

// Owns the internal T only if it's not owned by an arena.
// T needs to be arena constructible and destructor skippable.
template <typename T>
class ArenaHolder {
 public:
  explicit ArenaHolder(Arena* arena)
      : field_(Arena::Create<T>(arena)), owned_by_arena_(arena != nullptr) {
    ABSL_DCHECK(google::protobuf::Arena::is_arena_constructable<T>::value);
    ABSL_DCHECK(google::protobuf::Arena::is_destructor_skippable<T>::value);
  }

  ~ArenaHolder() {
    if (!owned_by_arena_) {
      delete field_;
    }
  }

  T* get() { return field_; }
  T* operator->() { return field_; }
  T& operator*() { return *field_; }

 private:
  T* field_;
  bool owned_by_arena_;
};

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_ARENA_TEST_UTIL_H__
