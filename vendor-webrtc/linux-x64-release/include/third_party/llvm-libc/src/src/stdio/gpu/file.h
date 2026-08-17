//===--- GPU helper functions for file I/O using RPC ----------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "src/__support/RPC/rpc_client.h"
#include "src/__support/macros/config.h"
#include "src/string/string_utils.h"

#include "hdr/stdio_macros.h" // For stdin/out/err
#include "hdr/types/FILE.h"

namespace LIBC_NAMESPACE_DECL {
namespace file {

enum Stream {
  File = 0,
  Stdin = 1,
  Stdout = 2,
  Stderr = 3,
};

// When copying between the client and server we need to indicate if this is one
// of the special streams. We do this by enocding the low order bits of the
// pointer to indicate if we need to use the host's standard stream.
LIBC_INLINE uintptr_t from_stream(::FILE *f) {
  if (f == stdin)
    return reinterpret_cast<uintptr_t>(f) | Stdin;
  if (f == stdout)
    return reinterpret_cast<uintptr_t>(f) | Stdout;
  if (f == stderr)
    return reinterpret_cast<uintptr_t>(f) | Stderr;
  return reinterpret_cast<uintptr_t>(f);
}

// Get the associated stream out of an encoded number.
LIBC_INLINE ::FILE *to_stream(uintptr_t f) {
  ::FILE *stream = reinterpret_cast<FILE *>(f & ~0x3ull);
  Stream type = static_cast<Stream>(f & 0x3ull);
  if (type == Stdin)
    return stdin;
  if (type == Stdout)
    return stdout;
  if (type == Stderr)
    return stderr;
  return stream;
}

template <uint32_t opcode>
LIBC_INLINE uint64_t write_impl(::FILE *file, const void *data, size_t size) {
  uint64_t ret = 0;
  rpc::Client::Port port = rpc::client.open<opcode>();

  if constexpr (opcode == LIBC_WRITE_TO_STREAM) {
    port.send([&](rpc::Buffer *buffer, uint32_t) {
      buffer->data[0] = reinterpret_cast<uintptr_t>(file);
    });
  }

  port.send_n(data, size);
  port.recv([&](rpc::Buffer *buffer, uint32_t) {
    ret = reinterpret_cast<uint64_t *>(buffer->data)[0];
  });
  port.close();
  return ret;
}

LIBC_INLINE uint64_t write(::FILE *f, const void *data, size_t size) {
  if (f == stdout)
    return write_impl<LIBC_WRITE_TO_STDOUT>(f, data, size);
  else if (f == stderr)
    return write_impl<LIBC_WRITE_TO_STDERR>(f, data, size);
  else
    return write_impl<LIBC_WRITE_TO_STREAM>(f, data, size);
}

LIBC_INLINE uint64_t read_from_stream(::FILE *file, void *buf, size_t size) {
  uint64_t ret = 0;
  uint64_t recv_size;
  rpc::Client::Port port = rpc::client.open<LIBC_READ_FROM_STREAM>();
  port.send([=](rpc::Buffer *buffer, uint32_t) {
    buffer->data[0] = size;
    buffer->data[1] = from_stream(file);
  });
  port.recv_n(&buf, &recv_size, [&](uint64_t) { return buf; });
  port.recv([&](rpc::Buffer *buffer, uint32_t) { ret = buffer->data[0]; });
  port.close();
  return ret;
}

LIBC_INLINE uint64_t read(::FILE *f, void *data, size_t size) {
  return read_from_stream(f, data, size);
}

} // namespace file
} // namespace LIBC_NAMESPACE_DECL
