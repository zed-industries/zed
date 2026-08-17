// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_GENERATOR_FILE_LAYOUT_H
#define UPB_GENERATOR_FILE_LAYOUT_H

#include <string>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "upb/base/status.hpp"
#include "upb/mini_descriptor/decode.h"
#include "upb/reflection/def.h"
#include "upb/reflection/def.hpp"
#include "upb/reflection/descriptor_bootstrap.h"

// Must be last
#include "upb/port/def.inc"

namespace upb {
namespace generator {

enum WhichEnums {
  kAllEnums = 0,
  kClosedEnums = 1,
};

std::vector<upb::EnumDefPtr> SortedEnums(upb::FileDefPtr file,
                                         WhichEnums which);

// Ordering must match upb/def.c!
//
// The ordering is significant because each upb_MessageDef* will point at the
// corresponding upb_MiniTable and we just iterate through the list without
// any search or lookup.
std::vector<upb::MessageDefPtr> SortedMessages(upb::FileDefPtr file);

// Ordering must match upb/def.c!
//
// The ordering is significant because each upb_FieldDef* will point at the
// corresponding upb_MiniTableExtension and we just iterate through the list
// without any search or lookup.
std::vector<upb::FieldDefPtr> SortedExtensions(upb::FileDefPtr file);

std::vector<upb::FieldDefPtr> FieldNumberOrder(upb::MessageDefPtr message);

// DefPoolPair is a pair of DefPools: one for 32-bit and one for 64-bit.
class DefPoolPair {
 public:
  DefPoolPair() {
    pool32_._SetPlatform(kUpb_MiniTablePlatform_32Bit);
    pool64_._SetPlatform(kUpb_MiniTablePlatform_64Bit);
  }

  upb::FileDefPtr AddFile(const UPB_DESC(FileDescriptorProto) * file_proto,
                          upb::Status* status) {
    upb::FileDefPtr file32 = pool32_.AddFile(file_proto, status);
    upb::FileDefPtr file64 = pool64_.AddFile(file_proto, status);
    if (!file32) return file32;
    return file64;
  }

  upb::FileDefPtr GetFile(absl::string_view name) const {
    return pool64_.FindFileByName(name.data());
  }

  const upb_MiniTable* GetMiniTable32(upb::MessageDefPtr m) const {
    return pool32_.FindMessageByName(m.full_name()).mini_table();
  }

  const upb_MiniTable* GetMiniTable64(upb::MessageDefPtr m) const {
    return pool64_.FindMessageByName(m.full_name()).mini_table();
  }

  const upb_MiniTableField* GetField32(upb::FieldDefPtr f) const {
    return GetFieldFromPool(&pool32_, f);
  }

  const upb_MiniTableField* GetField64(upb::FieldDefPtr f) const {
    return GetFieldFromPool(&pool64_, f);
  }

 private:
  static const upb_MiniTableField* GetFieldFromPool(const upb::DefPool* pool,
                                                    upb::FieldDefPtr f) {
    if (f.is_extension()) {
      return pool->FindExtensionByName(f.full_name()).mini_table();
    } else {
      return pool->FindMessageByName(f.containing_type().full_name())
          .FindFieldByNumber(f.number())
          .mini_table();
    }
  }

  upb::DefPool pool32_;
  upb::DefPool pool64_;
};

}  // namespace generator
}  // namespace upb

#include "upb/port/undef.inc"

#endif  // UPB_GENERATOR_FILE_LAYOUT_H
