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

#ifndef GRPC_TEST_CORE_TEST_UTIL_HISTOGRAM_H
#define GRPC_TEST_CORE_TEST_UTIL_HISTOGRAM_H

#include <grpc/support/port_platform.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct grpc_histogram grpc_histogram;

grpc_histogram* grpc_histogram_create(double resolution,
                                      double max_bucket_start);
void grpc_histogram_destroy(grpc_histogram* h);
void grpc_histogram_add(grpc_histogram* h, double x);

/// The following merges the second histogram into the first. It only works
/// if they have the same buckets and resolution. Returns 0 on failure, 1
/// on success
int grpc_histogram_merge(grpc_histogram* dst, const grpc_histogram* src);

double grpc_histogram_percentile(grpc_histogram* histogram, double percentile);
double grpc_histogram_mean(grpc_histogram* histogram);
double grpc_histogram_stddev(grpc_histogram* histogram);
double grpc_histogram_variance(grpc_histogram* histogram);
double grpc_histogram_maximum(grpc_histogram* histogram);
double grpc_histogram_minimum(grpc_histogram* histogram);
double grpc_histogram_count(grpc_histogram* histogram);
double grpc_histogram_sum(grpc_histogram* histogram);
double grpc_histogram_sum_of_squares(grpc_histogram* histogram);

const uint32_t* grpc_histogram_get_contents(grpc_histogram* histogram,
                                            size_t* count);
void grpc_histogram_merge_contents(grpc_histogram* histogram,
                                   const uint32_t* data, size_t data_count,
                                   double min_seen, double max_seen, double sum,
                                   double sum_of_squares, double count);

#ifdef __cplusplus
}
#endif

#endif  // GRPC_TEST_CORE_TEST_UTIL_HISTOGRAM_H
