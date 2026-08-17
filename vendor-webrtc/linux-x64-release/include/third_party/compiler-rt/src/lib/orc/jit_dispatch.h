//===------ jit_dispatch.h - Call back to an ORC controller -----*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// This file is a part of the ORC runtime support library.
//
//===----------------------------------------------------------------------===//

#ifndef ORC_RT_JIT_DISPATCH_H
#define ORC_RT_JIT_DISPATCH_H

#include "common.h"
#include "wrapper_function_utils.h"

namespace orc_rt {

class JITDispatch {
public:
  JITDispatch(const void *FnTag) : FnTag(FnTag) {}

  WrapperFunctionResult operator()(const char *ArgData, size_t ArgSize) {
    // Since the functions cannot be zero/unresolved on Windows, the following
    // reference taking would always be non-zero, thus generating a compiler
    // warning otherwise.
#if !defined(_WIN32)
    if (ORC_RT_UNLIKELY(!&__orc_rt_jit_dispatch_ctx))
      return WrapperFunctionResult::createOutOfBandError(
                 "__orc_rt_jit_dispatch_ctx not set")
          .release();
    if (ORC_RT_UNLIKELY(!&__orc_rt_jit_dispatch))
      return WrapperFunctionResult::createOutOfBandError(
                 "__orc_rt_jit_dispatch not set")
          .release();
#endif

    return __orc_rt_jit_dispatch(&__orc_rt_jit_dispatch_ctx, FnTag, ArgData,
                                 ArgSize);
  }

private:
  const void *FnTag;
};

} // namespace orc_rt

#endif // ORC_RT_JIT_DISPATCH_H
