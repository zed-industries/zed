// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_FONT_UNIQUE_NAME_LOOKUP_FONT_TABLE_PERSISTENCE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_FONT_UNIQUE_NAME_LOOKUP_FONT_TABLE_PERSISTENCE_H_

#include "base/files/file.h"
#include "base/memory/read_only_shared_memory_region.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/font_unique_name_lookup/font_unique_name_table.pb.h"

namespace blink {

// Persistence functions for storing to and reading from disk the font table
// lookup structure. The storage format is a pickle of a PersistenceHash prefix
// computed over the protobuf, followed by a dump of the FontUniqueNameTable
// protobuf. The persistence functions ensure that the format is followed, that
// persisting stores the checksum and reading validates the checksum.
namespace font_table_persistence {

// Load a FontUniqueNameTable protobuf from a persistence file, and if the
// checksum validates correctly, allocate the MappedReadOnlyRegion and copy the
// read ProtoBuf into it. Returns true on success, false on failures such as
// inabilitiy to access the file, empty file, incorrect checksum, or failing to
// parse the read dump as a FontUniqueNameTable protobuf.
bool BLINK_COMMON_EXPORT
LoadFromFile(base::FilePath file_path,
             base::MappedReadOnlyRegion* name_table_region);

// Store a FontUniqueNameTable protobuf to a dump file, prefixing the protobuf
// dump with a PersistenceHash checksum. Returns true on success, false on
// failures such as failing to open the file or failing to write the dump into
// it.
bool BLINK_COMMON_EXPORT
PersistToFile(const base::MappedReadOnlyRegion& name_table_region,
              base::FilePath file_path);

}  // namespace font_table_persistence

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_FONT_UNIQUE_NAME_LOOKUP_FONT_TABLE_PERSISTENCE_H_
