// Protocol Buffers - Google's data interchange format
// Copyright 2024 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_HPB_BACKEND_UPB_UPB_H__
#define GOOGLE_PROTOBUF_HPB_BACKEND_UPB_UPB_H__

#include "google/protobuf/hpb/backend/upb/interop.h"
#include "google/protobuf/hpb/internal/template_help.h"
#include "google/protobuf/hpb/ptr.h"

namespace hpb::internal::backend::upb {

template <typename T>
void ClearMessage(hpb::internal::PtrOrRawMutable<T> message) {
  auto ptr = Ptr(message);
  auto minitable = hpb::interop::upb::GetMiniTable(ptr);
  upb_Message_Clear(hpb::interop::upb::GetMessage(ptr), minitable);
}

}  // namespace hpb::internal::backend::upb

#endif  // GOOGLE_PROTOBUF_HPB_BACKEND_UPB_UPB_H__
