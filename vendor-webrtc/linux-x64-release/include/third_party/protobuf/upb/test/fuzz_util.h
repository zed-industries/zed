// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_TEST_FUZZ_UTIL_H_
#define UPB_TEST_FUZZ_UTIL_H_

#include <string>
#include <vector>

#include "upb/mem/arena.h"
#include "upb/mini_table/extension_registry.h"
#include "upb/mini_table/message.h"

namespace upb {
namespace fuzz {

struct MiniTableFuzzInput {
  // MiniDescriptors for N messages, in the format accepted by
  // upb_MiniTable_Build().
  std::vector<std::string> mini_descriptors;

  // MiniDescriptors for N enums, in the format accepted by
  // upb_MiniTableEnum_Build().
  std::vector<std::string> enum_mini_descriptors;

  // A MiniDescriptor for N extensions, in the format accepted by
  // upb_MiniTableExtension_Build().
  std::string extensions;

  // Integer indexes into the message or enum mini tables lists.  These specify
  // which message or enum to use for each sub-message or enum field.  We mod
  // by the total number of enums or messages so that any link value can be
  // valid.
  std::vector<uint32_t> links;
};

// Builds an arbitrary mini table corresponding to the random data in `input`.
// This function should be capable of producing any mini table that can
// successfully build, and any topology of messages and enums (including
// cycles).
//
// As currently written, it effectively fuzzes the mini descriptor parser also,
// and can therefore trigger any bugs in that parser. To better isolate these
// two, we may want to change this implementation to use the mini descriptor
// builder API so we are producing mini descriptors in a known good format. That
// would mostly eliminate the chance of crashing the mini descriptor parser
// itself.
//
// TODO: maps.  If we give maps some space in the regular encoding instead of
// using a separate function, we could get that for free.
const upb_MiniTable* BuildMiniTable(const MiniTableFuzzInput& input,
                                    upb_ExtensionRegistry** exts,
                                    upb_Arena* arena);

}  // namespace fuzz
}  // namespace upb

#endif  // UPB_TEST_FUZZ_UTIL_H_
