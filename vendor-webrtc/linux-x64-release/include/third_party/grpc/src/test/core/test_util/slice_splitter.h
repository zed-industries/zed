//
//
// Copyright 2015 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

#ifndef GRPC_TEST_CORE_TEST_UTIL_SLICE_SPLITTER_H
#define GRPC_TEST_CORE_TEST_UTIL_SLICE_SPLITTER_H

// utility function to split/merge slices together to help create test
// cases

#include <grpc/slice.h>
#include <stddef.h>

typedef enum {
  // merge all input slices into a single slice
  GRPC_SLICE_SPLIT_MERGE_ALL,
  // leave slices as is
  GRPC_SLICE_SPLIT_IDENTITY,
  // split slices into one byte chunks
  GRPC_SLICE_SPLIT_ONE_BYTE
} grpc_slice_split_mode;

// allocates *dst_slices; caller must unref all slices in dst_slices then free
// it
void grpc_split_slices(grpc_slice_split_mode mode, grpc_slice* src_slices,
                       size_t src_slice_count, grpc_slice** dst_slices,
                       size_t* dst_slice_count);

void grpc_split_slices_to_buffer(grpc_slice_split_mode mode,
                                 grpc_slice* src_slices, size_t src_slice_count,
                                 grpc_slice_buffer* dst);
void grpc_split_slice_buffer(grpc_slice_split_mode mode, grpc_slice_buffer* src,
                             grpc_slice_buffer* dst);

grpc_slice grpc_slice_merge(grpc_slice* slices, size_t nslices);

const char* grpc_slice_split_mode_name(grpc_slice_split_mode mode);

#endif  // GRPC_TEST_CORE_TEST_UTIL_SLICE_SPLITTER_H
