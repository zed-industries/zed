//===--- GPU helper functions for printf using RPC ------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "hdr/types/FILE.h"
#include "src/__support/GPU/utils.h"
#include "src/__support/RPC/rpc_client.h"
#include "src/__support/arg_list.h"
#include "src/__support/macros/config.h"
#include "src/stdio/gpu/file.h"
#include "src/string/string_utils.h"

namespace LIBC_NAMESPACE_DECL {

template <uint32_t opcode>
LIBC_INLINE int vfprintf_impl(::FILE *__restrict file,
                              const char *__restrict format, size_t format_size,
                              va_list vlist) {
  uint64_t mask = gpu::get_lane_mask();
  rpc::Client::Port port = rpc::client.open<opcode>();

  if constexpr (opcode == LIBC_PRINTF_TO_STREAM ||
                opcode == LIBC_PRINTF_TO_STREAM_PACKED) {
    port.send([&](rpc::Buffer *buffer, uint32_t) {
      buffer->data[0] = reinterpret_cast<uintptr_t>(file);
    });
  }

  size_t args_size = 0;
  port.send_n(format, format_size);
  port.recv([&](rpc::Buffer *buffer, uint32_t) {
    args_size = static_cast<size_t>(buffer->data[0]);
  });
  port.send_n(vlist, args_size);

  uint32_t ret = 0;
  for (;;) {
    const char *str = nullptr;
    port.recv([&](rpc::Buffer *buffer, uint32_t) {
      ret = static_cast<uint32_t>(buffer->data[0]);
      str = reinterpret_cast<const char *>(buffer->data[1]);
    });
    // If any lanes have a string argument it needs to be copied back.
    if (!gpu::ballot(mask, str))
      break;

    uint64_t size = str ? internal::string_length(str) + 1 : 0;
    port.send_n(str, size);
  }

  port.close();
  return ret;
}

LIBC_INLINE int vfprintf_internal(::FILE *__restrict stream,
                                  const char *__restrict format,
                                  size_t format_size, va_list vlist) {
  // The AMDPGU backend uses a packed struct for its varargs. We pass it as a
  // separate opcode so the server knows how much to advance the pointers.
#if defined(LIBC_TARGET_ARCH_IS_AMDGPU)
  if (stream == stdout)
    return vfprintf_impl<LIBC_PRINTF_TO_STDOUT_PACKED>(stream, format,
                                                       format_size, vlist);
  else if (stream == stderr)
    return vfprintf_impl<LIBC_PRINTF_TO_STDERR_PACKED>(stream, format,
                                                       format_size, vlist);
  else
    return vfprintf_impl<LIBC_PRINTF_TO_STREAM_PACKED>(stream, format,
                                                       format_size, vlist);
#else
  if (stream == stdout)
    return vfprintf_impl<LIBC_PRINTF_TO_STDOUT>(stream, format, format_size,
                                                vlist);
  else if (stream == stderr)
    return vfprintf_impl<LIBC_PRINTF_TO_STDERR>(stream, format, format_size,
                                                vlist);
  else
    return vfprintf_impl<LIBC_PRINTF_TO_STREAM>(stream, format, format_size,
                                                vlist);
#endif
}

} // namespace LIBC_NAMESPACE_DECL
