/*===------ LLVM/Offload helpers for kernel languages (CUDA/HIP) -*- c++ -*-===
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 *
 *===-----------------------------------------------------------------------===
 */

#include "__llvm_offload.h"

extern "C" {
unsigned llvmLaunchKernel(const void *func, dim3 gridDim, dim3 blockDim,
                          void **args, size_t sharedMem = 0, void *stream = 0);
}
