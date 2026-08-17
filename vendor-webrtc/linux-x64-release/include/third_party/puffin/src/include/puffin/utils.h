// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef SRC_INCLUDE_PUFFIN_UTILS_H_
#define SRC_INCLUDE_PUFFIN_UTILS_H_

#include <string>
#include <vector>

#include "puffin/common.h"
#include "puffin/stream.h"

namespace puffin {

// Converts an array of |ByteExtens| or |BitExtents| to a string. Each extent
// has format "offset:length" and are comma separated.
template <typename T>
std::string ExtentsToString(const T& extents) {
  std::string str;
  for (const auto& extent : extents) {
    str += std::to_string(extent.offset) + ":" + std::to_string(extent.length) +
           ",";
  }
  return str;
}

// Locates deflates in a deflate stream |data| with estimated minimum length of
// |size|. The data should start with a valid deflate stream otherwise, false is
// returned. |virtual_offset| defines the offset the |data| starts in the
// original deflate stream. It is used to calculate the location of deflates in
// |deflates| based on the given offset. |compressed_size| is the size of the
// input that was determined to have valid deflate blocks (including
// uncompressed blocks). This function does not clear the content of |deflates|
// and will append found deflates to the end of it.
bool LocateDeflatesInDeflateStream(const uint8_t* data,
                                   uint64_t size,
                                   uint64_t virtual_offset,
                                   std::vector<BitExtent>* deflates,
                                   uint64_t* compressed_size);

// Locates deflates in a zlib buffer |data| by removing header and footer bytes
// from the zlib stream.
bool LocateDeflatesInZlib(const Buffer& data, std::vector<BitExtent>* deflates);

// Uses the function above, to locate deflates (bit addressed) in a given file
// |file_path| using the list of zlib blocks |zlibs|.
bool LocateDeflatesInZlibBlocks(const std::string& file_path,
                                const std::vector<ByteExtent>& zlibs,
                                std::vector<BitExtent>* deflates);

// Searches for deflate locations in a gzip stream. The results are saved in
// |deflates|.
bool LocateDeflatesInGzip(const Buffer& data, std::vector<BitExtent>* deflates);

// Search for the deflates in a zip archive, and put the result in |deflates|.
bool LocateDeflatesInZipArchive(const Buffer& data,
                                std::vector<BitExtent>* deflates);

// Reads the deflates in from |deflates| and returns a list of its subblock
// locations. Each subblock in practice is a deflate stream by itself.
// Assumption is that the first subblock in each deflate in |deflates| start in
// byte boundary.
bool FindDeflateSubBlocks(const UniqueStreamPtr& src,
                          const std::vector<ByteExtent>& deflates,
                          std::vector<BitExtent>* subblock_deflates);

// Finds the location of puffs in the deflate stream |src| based on the location
// of |deflates| and populates the |puffs|. We assume |deflates| are sorted by
// their offset value. |out_puff_size| will be the size of the puff stream.
bool FindPuffLocations(const UniqueStreamPtr& src,
                       const std::vector<BitExtent>& deflates,
                       std::vector<ByteExtent>* puffs,
                       uint64_t* out_puff_size);

// Removes any BitExtents from both |extents1| and |extents2| if the data it
// points to is found in both |extents1| and |extents2|. The order of the
// remaining BitExtents is preserved.
void RemoveEqualBitExtents(const Buffer& data1,
                           const Buffer& data2,
                           std::vector<BitExtent>* extents1,
                           std::vector<BitExtent>* extents2);

// Using |data| it removes all the deflate extents from |deflates| which have
// the problem identified in crbug.com/915559. Each element of |deflates| should
// contain exactly one deflate block otherwire it returns false.
bool RemoveDeflatesWithBadDistanceCaches(const Buffer& data,
                                         std::vector<BitExtent>* deflates);

}  // namespace puffin

#endif  // SRC_INCLUDE_PUFFIN_UTILS_H_
