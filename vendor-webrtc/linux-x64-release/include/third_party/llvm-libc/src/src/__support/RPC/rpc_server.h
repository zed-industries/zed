//===-- Shared memory RPC server instantiation ------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// This file is intended to be used externally as part of the `shared/`
// interface. For that purpose, we manually define a few options normally
// handled by the libc build system.
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC___SUPPORT_RPC_RPC_SERVER_H
#define LLVM_LIBC_SRC___SUPPORT_RPC_RPC_SERVER_H

// Workaround for missing __has_builtin in < GCC 10.
#ifndef __has_builtin
#define __has_builtin(x) 0
#endif

// Workaround for missing __builtin_is_constant_evaluated in < GCC 10.
#ifndef __builtin_is_constant_evaluated
#define __builtin_is_constant_evaluated(x) 0
#endif

// Configs for using the LLVM libc writer interface.
#define LIBC_COPT_USE_C_ASSERT
#define LIBC_COPT_MEMCPY_USE_EMBEDDED_TINY
#define LIBC_COPT_ARRAY_ARG_LIST
#define LIBC_COPT_PRINTF_DISABLE_WRITE_INT
#define LIBC_COPT_PRINTF_DISABLE_INDEX_MODE
#define LIBC_COPT_PRINTF_DISABLE_STRERROR

// The 'long double' type is 8 bytes.
#define LIBC_TYPES_LONG_DOUBLE_IS_FLOAT64

#include "shared/rpc.h"
#include "shared/rpc_opcodes.h"

#include "src/__support/arg_list.h"
#include "src/stdio/printf_core/converter.h"
#include "src/stdio/printf_core/parser.h"
#include "src/stdio/printf_core/writer.h"

#include "hdr/stdio_overlay.h"
#include "hdr/stdlib_overlay.h"

namespace LIBC_NAMESPACE_DECL {
namespace internal {

// Minimal replacement for 'std::vector' that works for trivial types.
template <typename T> class TempVector {
  static_assert(cpp::is_trivially_constructible<T>::value &&
                    cpp::is_trivially_destructible<T>::value,
                "Not a trivial type.");
  T *data;
  size_t current;
  size_t capacity;

public:
  LIBC_INLINE TempVector() : data(nullptr), current(0), capacity(0) {}

  LIBC_INLINE ~TempVector() { free(data); }

  LIBC_INLINE void push_back(const T &value) {
    if (current == capacity)
      grow();
    data[current] = T(value);
    ++current;
  }

  LIBC_INLINE void push_back(T &&value) {
    if (current == capacity)
      grow();
    data[current] = T(static_cast<T &&>(value));
    ++current;
  }

  LIBC_INLINE void pop_back() { --current; }

  LIBC_INLINE bool empty() { return current == 0; }

  LIBC_INLINE size_t size() { return current; }

  LIBC_INLINE T &operator[](size_t index) { return data[index]; }

  LIBC_INLINE T &back() { return data[current - 1]; }

private:
  LIBC_INLINE void grow() {
    size_t new_capacity = capacity ? capacity * 2 : 1;
    void *new_data = realloc(data, new_capacity * sizeof(T));
    data = static_cast<T *>(new_data);
    capacity = new_capacity;
  }
};

struct TempStorage {
  LIBC_INLINE char *alloc(size_t size) {
    storage.push_back(reinterpret_cast<char *>(malloc(size)));
    return storage.back();
  }

  LIBC_INLINE ~TempStorage() {
    for (size_t i = 0; i < storage.size(); ++i)
      free(storage[i]);
  }

  TempVector<char *> storage;
};

// Get the associated stream out of an encoded number.
LIBC_INLINE static ::FILE *to_stream(uintptr_t f) {
  enum Stream {
    File = 0,
    Stdin = 1,
    Stdout = 2,
    Stderr = 3,
  };

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

template <bool packed, uint32_t num_lanes>
LIBC_INLINE static void handle_printf(rpc::Server::Port &port,
                                      TempStorage &temp_storage) {
  FILE *files[num_lanes] = {nullptr};
  // Get the appropriate output stream to use.
  if (port.get_opcode() == LIBC_PRINTF_TO_STREAM ||
      port.get_opcode() == LIBC_PRINTF_TO_STREAM_PACKED) {
    port.recv([&](rpc::Buffer *buffer, uint32_t id) {
      files[id] = reinterpret_cast<FILE *>(buffer->data[0]);
    });
  } else if (port.get_opcode() == LIBC_PRINTF_TO_STDOUT ||
             port.get_opcode() == LIBC_PRINTF_TO_STDOUT_PACKED) {
    for (uint32_t i = 0; i < num_lanes; ++i)
      files[i] = stdout;
  } else {
    for (uint32_t i = 0; i < num_lanes; ++i)
      files[i] = stderr;
  }

  uint64_t format_sizes[num_lanes] = {0};
  void *format[num_lanes] = {nullptr};

  uint64_t args_sizes[num_lanes] = {0};
  void *args[num_lanes] = {nullptr};

  // Recieve the format string and arguments from the client.
  port.recv_n(format, format_sizes,
              [&](uint64_t size) { return temp_storage.alloc(size); });

  // Parse the format string to get the expected size of the buffer.
  for (uint32_t lane = 0; lane < num_lanes; ++lane) {
    if (!format[lane])
      continue;

    printf_core::WriteBuffer<
        printf_core::WriteMode::FILL_BUFF_AND_DROP_OVERFLOW>
        wb(nullptr, 0);
    printf_core::Writer writer(wb);

    internal::DummyArgList<packed> printf_args;
    printf_core::Parser<internal::DummyArgList<packed> &> parser(
        reinterpret_cast<const char *>(format[lane]), printf_args);

    for (printf_core::FormatSection cur_section = parser.get_next_section();
         !cur_section.raw_string.empty();
         cur_section = parser.get_next_section())
      ;
    args_sizes[lane] = printf_args.read_count();
  }
  port.send([&](rpc::Buffer *buffer, uint32_t id) {
    buffer->data[0] = args_sizes[id];
  });
  port.recv_n(args, args_sizes,
              [&](uint64_t size) { return temp_storage.alloc(size); });

  // Identify any arguments that are actually pointers to strings on the client.
  // Additionally we want to determine how much buffer space we need to print.
  TempVector<void *> strs_to_copy[num_lanes];
  int buffer_size[num_lanes] = {0};
  for (uint32_t lane = 0; lane < num_lanes; ++lane) {
    if (!format[lane])
      continue;

    printf_core::WriteBuffer<
        printf_core::WriteMode::FILL_BUFF_AND_DROP_OVERFLOW>
        wb(nullptr, 0);
    printf_core::Writer writer(wb);

    internal::StructArgList<packed> printf_args(args[lane], args_sizes[lane]);
    printf_core::Parser<internal::StructArgList<packed>> parser(
        reinterpret_cast<const char *>(format[lane]), printf_args);

    for (printf_core::FormatSection cur_section = parser.get_next_section();
         !cur_section.raw_string.empty();
         cur_section = parser.get_next_section()) {
      if (cur_section.has_conv && cur_section.conv_name == 's' &&
          cur_section.conv_val_ptr) {
        strs_to_copy[lane].push_back(cur_section.conv_val_ptr);
        // Get the minimum size of the string in the case of padding.
        char c = '\0';
        cur_section.conv_val_ptr = &c;
        convert(&writer, cur_section);
      } else if (cur_section.has_conv) {
        // Ignore conversion errors for the first pass.
        convert(&writer, cur_section);
      } else {
        writer.write(cur_section.raw_string);
      }
    }
    buffer_size[lane] = writer.get_chars_written();
  }

  // Recieve any strings from the client and push them into a buffer.
  TempVector<void *> copied_strs[num_lanes];
  auto HasPendingCopies = [](TempVector<void *> v[num_lanes]) {
    for (uint32_t i = 0; i < num_lanes; ++i)
      if (!v[i].empty() && v[i].back())
        return true;
    return false;
  };
  while (HasPendingCopies(strs_to_copy)) {
    port.send([&](rpc::Buffer *buffer, uint32_t id) {
      void *ptr = !strs_to_copy[id].empty() ? strs_to_copy[id].back() : nullptr;
      buffer->data[1] = reinterpret_cast<uintptr_t>(ptr);
      if (!strs_to_copy[id].empty())
        strs_to_copy[id].pop_back();
    });
    uint64_t str_sizes[num_lanes] = {0};
    void *strs[num_lanes] = {nullptr};
    port.recv_n(strs, str_sizes,
                [&](uint64_t size) { return temp_storage.alloc(size); });
    for (uint32_t lane = 0; lane < num_lanes; ++lane) {
      if (!strs[lane])
        continue;

      copied_strs[lane].push_back(strs[lane]);
      buffer_size[lane] += str_sizes[lane];
    }
  }

  // Perform the final formatting and printing using the LLVM C library printf.
  int results[num_lanes] = {0};
  for (uint32_t lane = 0; lane < num_lanes; ++lane) {
    if (!format[lane])
      continue;

    char *buffer = temp_storage.alloc(buffer_size[lane]);
    printf_core::WriteBuffer<
        printf_core::WriteMode::FILL_BUFF_AND_DROP_OVERFLOW>
        wb(buffer, buffer_size[lane]);
    printf_core::Writer writer(wb);

    internal::StructArgList<packed> printf_args(args[lane], args_sizes[lane]);
    printf_core::Parser<internal::StructArgList<packed>> parser(
        reinterpret_cast<const char *>(format[lane]), printf_args);

    // Parse and print the format string using the arguments we copied from
    // the client.
    int ret = 0;
    for (printf_core::FormatSection cur_section = parser.get_next_section();
         !cur_section.raw_string.empty();
         cur_section = parser.get_next_section()) {
      // If this argument was a string we use the memory buffer we copied from
      // the client by replacing the raw pointer with the copied one.
      if (cur_section.has_conv && cur_section.conv_name == 's') {
        if (!copied_strs[lane].empty()) {
          cur_section.conv_val_ptr = copied_strs[lane].back();
          copied_strs[lane].pop_back();
        } else {
          cur_section.conv_val_ptr = nullptr;
        }
      }
      if (cur_section.has_conv) {
        ret = convert(&writer, cur_section);
        if (ret == -1)
          break;
      } else {
        writer.write(cur_section.raw_string);
      }
    }

    results[lane] = static_cast<int>(
        fwrite(buffer, 1, writer.get_chars_written(), files[lane]));
    if (results[lane] != writer.get_chars_written() || ret == -1)
      results[lane] = -1;
  }

  // Send the final return value and signal completion by setting the string
  // argument to null.
  port.send([&](rpc::Buffer *buffer, uint32_t id) {
    buffer->data[0] = static_cast<uint64_t>(results[id]);
    buffer->data[1] = reinterpret_cast<uintptr_t>(nullptr);
  });
}

template <uint32_t num_lanes>
LIBC_INLINE static rpc::Status handle_port_impl(rpc::Server::Port &port) {
  TempStorage temp_storage;

  switch (port.get_opcode()) {
  case LIBC_WRITE_TO_STREAM:
  case LIBC_WRITE_TO_STDERR:
  case LIBC_WRITE_TO_STDOUT:
  case LIBC_WRITE_TO_STDOUT_NEWLINE: {
    uint64_t sizes[num_lanes] = {0};
    void *strs[num_lanes] = {nullptr};
    FILE *files[num_lanes] = {nullptr};
    if (port.get_opcode() == LIBC_WRITE_TO_STREAM) {
      port.recv([&](rpc::Buffer *buffer, uint32_t id) {
        files[id] = reinterpret_cast<FILE *>(buffer->data[0]);
      });
    } else {
      for (uint32_t i = 0; i < num_lanes; ++i)
        files[i] = port.get_opcode() == LIBC_WRITE_TO_STDERR ? stderr : stdout;
    }

    port.recv_n(strs, sizes,
                [&](uint64_t size) { return temp_storage.alloc(size); });
    port.send([&](rpc::Buffer *buffer, uint32_t id) {
      flockfile(files[id]);
      buffer->data[0] = fwrite_unlocked(strs[id], 1, sizes[id], files[id]);
      if (port.get_opcode() == LIBC_WRITE_TO_STDOUT_NEWLINE &&
          buffer->data[0] == sizes[id])
        buffer->data[0] += fwrite_unlocked("\n", 1, 1, files[id]);
      funlockfile(files[id]);
    });
    break;
  }
  case LIBC_READ_FROM_STREAM: {
    uint64_t sizes[num_lanes] = {0};
    void *data[num_lanes] = {nullptr};
    port.recv([&](rpc::Buffer *buffer, uint32_t id) {
      data[id] = temp_storage.alloc(buffer->data[0]);
      sizes[id] =
          fread(data[id], 1, buffer->data[0], to_stream(buffer->data[1]));
    });
    port.send_n(data, sizes);
    port.send([&](rpc::Buffer *buffer, uint32_t id) {
      __builtin_memcpy(buffer->data, &sizes[id], sizeof(uint64_t));
    });
    break;
  }
  case LIBC_READ_FGETS: {
    uint64_t sizes[num_lanes] = {0};
    void *data[num_lanes] = {nullptr};
    port.recv([&](rpc::Buffer *buffer, uint32_t id) {
      data[id] = temp_storage.alloc(buffer->data[0]);
      const char *str = ::fgets(reinterpret_cast<char *>(data[id]),
                                static_cast<int>(buffer->data[0]),
                                to_stream(buffer->data[1]));
      sizes[id] = !str ? 0 : __builtin_strlen(str) + 1;
    });
    port.send_n(data, sizes);
    break;
  }
  case LIBC_OPEN_FILE: {
    uint64_t sizes[num_lanes] = {0};
    void *paths[num_lanes] = {nullptr};
    port.recv_n(paths, sizes,
                [&](uint64_t size) { return temp_storage.alloc(size); });
    port.recv_and_send([&](rpc::Buffer *buffer, uint32_t id) {
      FILE *file = fopen(reinterpret_cast<char *>(paths[id]),
                         reinterpret_cast<char *>(buffer->data));
      buffer->data[0] = reinterpret_cast<uintptr_t>(file);
    });
    break;
  }
  case LIBC_CLOSE_FILE: {
    port.recv_and_send([&](rpc::Buffer *buffer, uint32_t) {
      FILE *file = reinterpret_cast<FILE *>(buffer->data[0]);
      buffer->data[0] = ::fclose(file);
    });
    break;
  }
  case LIBC_EXIT: {
    // Send a response to the client to signal that we are ready to exit.
    port.recv_and_send([](rpc::Buffer *, uint32_t) {});
    port.recv([](rpc::Buffer *buffer, uint32_t) {
      int status = 0;
      __builtin_memcpy(&status, buffer->data, sizeof(int));
      exit(status);
    });
    break;
  }
  case LIBC_ABORT: {
    // Send a response to the client to signal that we are ready to abort.
    port.recv_and_send([](rpc::Buffer *, uint32_t) {});
    port.recv([](rpc::Buffer *, uint32_t) {});
    abort();
    break;
  }
  case LIBC_HOST_CALL: {
    uint64_t sizes[num_lanes] = {0};
    unsigned long long results[num_lanes] = {0};
    void *args[num_lanes] = {nullptr};
    port.recv_n(args, sizes,
                [&](uint64_t size) { return temp_storage.alloc(size); });
    port.recv([&](rpc::Buffer *buffer, uint32_t id) {
      using func_ptr_t = unsigned long long (*)(void *);
      auto func = reinterpret_cast<func_ptr_t>(buffer->data[0]);
      results[id] = func(args[id]);
    });
    port.send([&](rpc::Buffer *buffer, uint32_t id) {
      buffer->data[0] = static_cast<uint64_t>(results[id]);
    });
    break;
  }
  case LIBC_FEOF: {
    port.recv_and_send([](rpc::Buffer *buffer, uint32_t) {
      buffer->data[0] = feof(to_stream(buffer->data[0]));
    });
    break;
  }
  case LIBC_FERROR: {
    port.recv_and_send([](rpc::Buffer *buffer, uint32_t) {
      buffer->data[0] = ferror(to_stream(buffer->data[0]));
    });
    break;
  }
  case LIBC_CLEARERR: {
    port.recv_and_send([](rpc::Buffer *buffer, uint32_t) {
      clearerr(to_stream(buffer->data[0]));
    });
    break;
  }
  case LIBC_FSEEK: {
    port.recv_and_send([](rpc::Buffer *buffer, uint32_t) {
      buffer->data[0] =
          fseek(to_stream(buffer->data[0]), static_cast<long>(buffer->data[1]),
                static_cast<int>(buffer->data[2]));
    });
    break;
  }
  case LIBC_FTELL: {
    port.recv_and_send([](rpc::Buffer *buffer, uint32_t) {
      buffer->data[0] = ftell(to_stream(buffer->data[0]));
    });
    break;
  }
  case LIBC_FFLUSH: {
    port.recv_and_send([](rpc::Buffer *buffer, uint32_t) {
      buffer->data[0] = fflush(to_stream(buffer->data[0]));
    });
    break;
  }
  case LIBC_UNGETC: {
    port.recv_and_send([](rpc::Buffer *buffer, uint32_t) {
      buffer->data[0] =
          ungetc(static_cast<int>(buffer->data[0]), to_stream(buffer->data[1]));
    });
    break;
  }
  case LIBC_PRINTF_TO_STREAM_PACKED:
  case LIBC_PRINTF_TO_STDOUT_PACKED:
  case LIBC_PRINTF_TO_STDERR_PACKED: {
    handle_printf<true, num_lanes>(port, temp_storage);
    break;
  }
  case LIBC_PRINTF_TO_STREAM:
  case LIBC_PRINTF_TO_STDOUT:
  case LIBC_PRINTF_TO_STDERR: {
    handle_printf<false, num_lanes>(port, temp_storage);
    break;
  }
  case LIBC_REMOVE: {
    uint64_t sizes[num_lanes] = {0};
    void *args[num_lanes] = {nullptr};
    port.recv_n(args, sizes,
                [&](uint64_t size) { return temp_storage.alloc(size); });
    port.send([&](rpc::Buffer *buffer, uint32_t id) {
      buffer->data[0] = static_cast<uint64_t>(
          remove(reinterpret_cast<const char *>(args[id])));
    });
    break;
  }
  case LIBC_RENAME: {
    uint64_t oldsizes[num_lanes] = {0};
    uint64_t newsizes[num_lanes] = {0};
    void *oldpath[num_lanes] = {nullptr};
    void *newpath[num_lanes] = {nullptr};
    port.recv_n(oldpath, oldsizes,
                [&](uint64_t size) { return temp_storage.alloc(size); });
    port.recv_n(newpath, newsizes,
                [&](uint64_t size) { return temp_storage.alloc(size); });
    port.send([&](rpc::Buffer *buffer, uint32_t id) {
      buffer->data[0] = static_cast<uint64_t>(
          rename(reinterpret_cast<const char *>(oldpath[id]),
                 reinterpret_cast<const char *>(newpath[id])));
    });
    break;
  }
  case LIBC_SYSTEM: {
    uint64_t sizes[num_lanes] = {0};
    void *args[num_lanes] = {nullptr};
    port.recv_n(args, sizes,
                [&](uint64_t size) { return temp_storage.alloc(size); });
    port.send([&](rpc::Buffer *buffer, uint32_t id) {
      buffer->data[0] = static_cast<uint64_t>(
          system(reinterpret_cast<const char *>(args[id])));
    });
    break;
  }
  case LIBC_NOOP: {
    port.recv([](rpc::Buffer *, uint32_t) {});
    break;
  }
  default:
    return rpc::RPC_UNHANDLED_OPCODE;
  }

  return rpc::RPC_SUCCESS;
}

} // namespace internal
} // namespace LIBC_NAMESPACE_DECL

namespace LIBC_NAMESPACE_DECL {
namespace rpc {

// Handles any opcode generated from the 'libc' client code.
LIBC_INLINE ::rpc::Status handle_libc_opcodes(::rpc::Server::Port &port,
                                              uint32_t num_lanes) {
  switch (num_lanes) {
  case 1:
    return internal::handle_port_impl<1>(port);
  case 32:
    return internal::handle_port_impl<32>(port);
  case 64:
    return internal::handle_port_impl<64>(port);
  default:
    return ::rpc::RPC_ERROR;
  }
}

} // namespace rpc
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC___SUPPORT_RPC_RPC_SERVER_H
