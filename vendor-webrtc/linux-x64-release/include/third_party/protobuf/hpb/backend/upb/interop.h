// Protocol Buffers - Google's data interchange format
// Copyright 2024 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_HPB_BACKEND_UPB_INTEROP_H__
#define GOOGLE_PROTOBUF_HPB_BACKEND_UPB_INTEROP_H__

// The sole public header in hpb/backend/upb

#include <cstring>

#include "absl/strings/string_view.h"
#include "google/protobuf/hpb/internal/internal.h"
#include "google/protobuf/hpb/ptr.h"
#include "upb/base/string_view.h"
#include "upb/mem/arena.h"
#include "upb/message/message.h"
#include "upb/mini_table/message.h"

namespace hpb::interop::upb {

/**
 * Moves ownership of a message created in a source arena.
 *
 * Utility function to provide a way to move ownership across languages or VMs.
 *
 * Warning: any minitable skew will incur arbitrary memory access. Ensuring
 * minitable compatibility is the responsibility of the caller.
 */
// TODO: b/365824801 - consider rename to OwnMessage
template <typename T>
T MoveMessage(upb_Message* msg, upb_Arena* arena) {
  return T(msg, arena);
}

template <typename T>
const upb_MiniTable* GetMiniTable(const T*) {
  return T::minitable();
}

template <typename T>
const upb_MiniTable* GetMiniTable(Ptr<T>) {
  return T::minitable();
}

template <typename T>
auto* GetMessage(T&& message) {
  return hpb::internal::PrivateAccess::GetInternalMsg(std::forward<T>(message));
}

template <typename T>
upb_Arena* GetArena(Ptr<T> message) {
  return hpb::internal::PrivateAccess::GetInternalArena(message);
}

template <typename T>
upb_Arena* GetArena(T* message) {
  return hpb::internal::PrivateAccess::GetInternalArena(message);
}

/**
 * Creates a const Handle to a upb message.
 *
 * The supplied arena must outlive the hpb handle.
 * All messages reachable from from the upb message must
 * outlive the hpb handle.
 *
 * That is:
 * upb allows message M on arena A to point to message M' on
 * arena A'. As a user of hpb, you must guarantee that both A and A'
 * outlive M and M'. In practice, this is enforced by using upb::Fuse,
 * or manual tracking.
 *
 * The upb message must not be mutated directly while the handle is alive.
 *
 * T must match actual type of `msg`.
 * TODO: b/361596328 - revisit GetArena for CHandles
 * TODO: b/362743843 - consider passing in MiniTable to ensure match
 */
template <typename T>
typename T::CProxy MakeCHandle(const upb_Message* msg, upb_Arena* arena) {
  return hpb::internal::PrivateAccess::CProxy<T>(msg, arena);
}

/**
 * Creates a Handle to a mutable upb message.
 *
 * The supplied arena must outlive the hpb handle.
 * All messages reachable from from the upb message must
 * outlive the hpb handle.
 */
template <typename T>
typename T::Proxy MakeHandle(upb_Message* msg, upb_Arena* arena) {
  return typename T::Proxy(msg, arena);
}

/**
 * Creates a message in the given arena and returns a handle to it.
 *
 * The supplied arena must outlive the hpb handle.
 * All messages reachable from from the upb message must
 * outlive the hpb handle.
 */
template <typename T>
typename T::Proxy CreateMessage(upb_Arena* arena) {
  return hpb::internal::PrivateAccess::CreateMessage<T>(arena);
}

inline absl::string_view FromUpbStringView(upb_StringView str) {
  return absl::string_view(str.data, str.size);
}

inline upb_StringView CopyToUpbStringView(absl::string_view str,
                                          upb_Arena* arena) {
  const size_t str_size = str.size();
  char* buffer = static_cast<char*>(upb_Arena_Malloc(arena, str_size));
  memcpy(buffer, str.data(), str_size);
  return upb_StringView_FromDataAndSize(buffer, str_size);
}

}  // namespace hpb::interop::upb

#endif  // GOOGLE_PROTOBUF_HPB_BACKEND_UPB_INTEROP_H__
