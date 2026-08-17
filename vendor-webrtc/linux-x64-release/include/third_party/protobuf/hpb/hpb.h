// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef PROTOBUF_HPB_HPB_H_
#define PROTOBUF_HPB_HPB_H_

#include <type_traits>

#include "absl/base/attributes.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/hpb/arena.h"
#include "google/protobuf/hpb/backend/upb/interop.h"
#include "google/protobuf/hpb/extension.h"
#include "google/protobuf/hpb/internal/internal.h"
#include "google/protobuf/hpb/internal/message_lock.h"
#include "google/protobuf/hpb/internal/template_help.h"
#include "google/protobuf/hpb/ptr.h"
#include "google/protobuf/hpb/status.h"
#include "upb/wire/decode.h"

#ifdef HPB_BACKEND_UPB
#include "google/protobuf/hpb/backend/upb/upb.h"
#else
#error hpb backend must be specified
#endif

namespace hpb {

#ifdef HPB_BACKEND_UPB
namespace backend = ::hpb::internal::backend::upb;
#endif

template <typename T>
typename T::Proxy CreateMessage(hpb::Arena& arena) {
  return typename T::Proxy(upb_Message_New(T::minitable(), arena.ptr()),
                           arena.ptr());
}

template <typename T>
typename T::Proxy CloneMessage(Ptr<T> message, hpb::Arena& arena) {
  return hpb::internal::PrivateAccess::Proxy<T>(
      hpb::internal::DeepClone(hpb::interop::upb::GetMessage(message),
                               T::minitable(), arena.ptr()),
      arena.ptr());
}

// Deprecated; do not use. There is one extant caller which we plan to migrate.
// Tracking deletion TODO: b/385138477
template <typename T>
[[deprecated("Use CloneMessage(Ptr<T>, hpb::Arena&) instead.")]]
typename T::Proxy CloneMessage(Ptr<T> message, upb_Arena* arena) {
  return ::hpb::internal::PrivateAccess::Proxy<T>(
      ::hpb::internal::DeepClone(hpb::interop::upb::GetMessage(message),
                                 T::minitable(), arena),
      arena);
}

template <typename T>
void DeepCopy(Ptr<const T> source_message, Ptr<T> target_message) {
  static_assert(!std::is_const_v<T>);
  ::hpb::internal::DeepCopy(hpb::interop::upb::GetMessage(target_message),
                            hpb::interop::upb::GetMessage(source_message),
                            T::minitable(),
                            hpb::interop::upb::GetArena(target_message));
}

template <typename T>
void DeepCopy(Ptr<const T> source_message, T* target_message) {
  static_assert(!std::is_const_v<T>);
  DeepCopy(source_message, Ptr(target_message));
}

template <typename T>
void DeepCopy(const T* source_message, T* target_message) {
  static_assert(!std::is_const_v<T>);
  DeepCopy(Ptr(source_message), Ptr(target_message));
}

template <typename T>
void ClearMessage(hpb::internal::PtrOrRawMutable<T> message) {
  backend::ClearMessage(message);
}

template <typename T>
ABSL_MUST_USE_RESULT bool Parse(
    internal::PtrOrRaw<T> message, absl::string_view bytes,
    const ::hpb::ExtensionRegistry& extension_registry =
        hpb::ExtensionRegistry::EmptyRegistry()) {
  static_assert(!std::is_const_v<T>);
  upb_Message_Clear(hpb::interop::upb::GetMessage(message),
                    ::hpb::interop::upb::GetMiniTable(message));
  auto* arena = hpb::interop::upb::GetArena(message);
  return upb_Decode(bytes.data(), bytes.size(),
                    hpb::interop::upb::GetMessage(message),
                    ::hpb::interop::upb::GetMiniTable(message),
                    hpb::internal::GetUpbExtensions(extension_registry),
                    /* options= */ 0, arena) == kUpb_DecodeStatus_Ok;
}

template <typename T>
absl::StatusOr<T> Parse(absl::string_view bytes,
                        const ::hpb::ExtensionRegistry& extension_registry =
                            hpb::ExtensionRegistry::EmptyRegistry()) {
  T message;
  auto* arena = hpb::interop::upb::GetArena(&message);
  upb_DecodeStatus status =
      upb_Decode(bytes.data(), bytes.size(), message.msg(),
                 ::hpb::interop::upb::GetMiniTable(&message),
                 hpb::internal::GetUpbExtensions(extension_registry),
                 /* options= */ 0, arena);
  if (status == kUpb_DecodeStatus_Ok) {
    return message;
  }
  return MessageDecodeError(status);
}

template <typename T>
absl::StatusOr<absl::string_view> Serialize(internal::PtrOrRaw<T> message,
                                            hpb::Arena& arena) {
  return ::hpb::internal::Serialize(hpb::interop::upb::GetMessage(message),
                                    ::hpb::interop::upb::GetMiniTable(message),
                                    arena.ptr(), 0);
}

}  // namespace hpb

#endif  // PROTOBUF_HPB_HPB_H_
