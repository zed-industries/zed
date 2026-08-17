/*===------ LLVM/Offload helpers for kernel languages (CUDA/HIP) -*- c++ -*-===
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 *
 *===-----------------------------------------------------------------------===
 */

#include <stddef.h>

#define __host__ __attribute__((host))
#define __device__ __attribute__((device))
#define __global__ __attribute__((global))
#define __shared__ __attribute__((shared))
#define __constant__ __attribute__((constant))
#define __managed__ __attribute__((managed))

extern "C" {

typedef struct dim3 {
  dim3() {}
  dim3(unsigned x) : x(x) {}
  unsigned x = 0, y = 0, z = 0;
} dim3;

// TODO: For some reason the CUDA device compilation requires this declaration
// to be present on the device while it is only used on the host.
unsigned __llvmPushCallConfiguration(dim3 gridDim, dim3 blockDim,
                                     size_t sharedMem = 0, void *stream = 0);
}
