// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_MAP_FIELD_LITE_H__
#define GOOGLE_PROTOBUF_MAP_FIELD_LITE_H__

#include <cstddef>
#include <type_traits>

#include "absl/log/absl_check.h"
#include "google/protobuf/internal_visibility.h"
#include "google/protobuf/io/coded_stream.h"
#include "google/protobuf/map.h"
#include "google/protobuf/parse_context.h"
#include "google/protobuf/port.h"
#include "google/protobuf/wire_format_lite.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

#ifdef SWIG
#error "You cannot SWIG proto headers"
#endif

namespace google {
namespace protobuf {
namespace internal {

// This class provides access to map field using generated api. It is used for
// internal generated message implementation only. Users should never use this
// directly.
template <typename Key, typename T>
class MapFieldLite {
 public:
  typedef Map<Key, T> MapType;

  constexpr MapFieldLite() : map_() {}
  explicit MapFieldLite(Arena* arena) : map_(arena) {}
  MapFieldLite(ArenaInitialized, Arena* arena) : MapFieldLite(arena) {}

  MapFieldLite(InternalVisibility, Arena* arena) : map_(arena) {}
  MapFieldLite(InternalVisibility, Arena* arena, const MapFieldLite& from)
      : map_(arena) {
    MergeFrom(from);
  }

#ifdef NDEBUG
  ~MapFieldLite() { map_.~Map(); }
#else
  ~MapFieldLite() {
    ABSL_DCHECK_EQ(map_.arena(), nullptr);
    // We want to destruct the map in such a way that we can verify
    // that we've done that, but also be sure that we've deallocated
    // everything (as opposed to leaving an allocation behind with no
    // data in it, as would happen if a vector was resize'd to zero.
    // Map::Swap with an empty map accomplishes that.
    decltype(map_) swapped_map(map_.arena());
    map_.InternalSwap(&swapped_map);
  }
#endif
  // Accessors
  const Map<Key, T>& GetMap() const { return map_; }
  Map<Key, T>* MutableMap() { return &map_; }

  // Convenient methods for generated message implementation.
  int size() const { return static_cast<int>(map_.size()); }
  void Clear() { return map_.clear(); }
  void MergeFrom(const MapFieldLite& other) {
    internal::MapMergeFrom(map_, other.map_);
  }
  void Swap(MapFieldLite* other) { map_.swap(other->map_); }
  void InternalSwap(MapFieldLite* other) { map_.InternalSwap(&other->map_); }

  static constexpr size_t InternalGetArenaOffset(
      internal::InternalVisibility access) {
    return PROTOBUF_FIELD_OFFSET(MapFieldLite, map_) +
           decltype(map_)::InternalGetArenaOffset(access);
  }

 private:
  typedef void DestructorSkippable_;

  // map_ is inside an anonymous union so we can explicitly control its
  // destruction
  union {
    Map<Key, T> map_;
  };

  friend class google::protobuf::Arena;
};

// True if IsInitialized() is true for value field in all elements of t. T is
// expected to be message.  It's useful to have this helper here to keep the
// protobuf compiler from ever having to emit loops in IsInitialized() methods.
// We want the C++ compiler to inline this or not as it sees fit.
template <typename Key, typename T>
bool AllAreInitialized(const MapFieldLite<Key, T>& field) {
  const auto& t = field.GetMap();
  for (typename Map<Key, T>::const_iterator it = t.begin(); it != t.end();
       ++it) {
    if (!it->second.IsInitialized()) return false;
  }
  return true;
}

template <typename MEntry>
struct MapEntryToMapField : MapEntryToMapField<typename MEntry::SuperType> {};

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_MAP_FIELD_LITE_H__
