// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef SRC_INCLUDE_PUFFIN_PUFFDIFF_H_
#define SRC_INCLUDE_PUFFIN_PUFFDIFF_H_

#include <string>
#include <vector>

#include "puffin/common.h"
#include "puffin/src/include/puffin/puffpatch.h"
#include "puffin/stream.h"

namespace puffin {

enum class PatchAlgorithm {
  kBsdiff = 0,  // Unsupported by chromium/src's Puffin implementation.
  kZucchini = 1,
};

// Performs a diff operation between input deflate streams and creates a patch
// that is used in the client to recreate the |dst| from |src|.
// |src|          IN   Source deflate stream.
// |dst|          IN   Destination deflate stream.
// |src_deflates| IN   Deflate locations in |src|.
// |dst_deflates| IN   Deflate locations in |dst|.
// |compressors|  IN   Compressors to use in the underlying bsdiff, e.g. bz2,
//                     brotli.
// |patchAlgorithm|    IN   The patchAlgorithm used to create patches between
//                     uncompressed bytes, e.g. bsdiff, zucchini.
// |tmp_filepath| IN   A path to a temporary file. The caller has the
//                     responsibility of unlinking the file after the call to
//                     |PuffDiff| finishes.
// |puffin_patch| OUT  The patch that later can be used in |PuffPatch|.
bool PuffDiff(UniqueStreamPtr src,
              UniqueStreamPtr dst,
              const std::vector<BitExtent>& src_deflates,
              const std::vector<BitExtent>& dst_deflates,
              const std::vector<puffin::CompressorType>& compressors,
              PatchAlgorithm patchAlgorithm,
              const std::string& tmp_filepath,
              Buffer* patch);

// This function uses bsdiff as the patch algorithm.
bool PuffDiff(UniqueStreamPtr src,
              UniqueStreamPtr dst,
              const std::vector<BitExtent>& src_deflates,
              const std::vector<BitExtent>& dst_deflates,
              const std::vector<puffin::CompressorType>& compressors,
              const std::string& tmp_filepath,
              Buffer* patch);

// Similar to the function above, except that it accepts raw buffer rather than
// stream.
bool PuffDiff(const Buffer& src,
              const Buffer& dst,
              const std::vector<BitExtent>& src_deflates,
              const std::vector<BitExtent>& dst_deflates,
              const std::vector<puffin::CompressorType>& compressors,
              const std::string& tmp_filepath,
              Buffer* patch);

// The default puffdiff function that uses both bz2 and brotli to compress the
// patch data.
bool PuffDiff(const Buffer& src,
              const Buffer& dst,
              const std::vector<BitExtent>& src_deflates,
              const std::vector<BitExtent>& dst_deflates,
              const std::string& tmp_filepath,
              Buffer* patch);

Status PuffDiff(const std::string& src_file_path,
                const std::string& dest_file_path,
                const std::string& output_patch_path);

}  // namespace puffin

#endif  // SRC_INCLUDE_PUFFIN_PUFFDIFF_H_
