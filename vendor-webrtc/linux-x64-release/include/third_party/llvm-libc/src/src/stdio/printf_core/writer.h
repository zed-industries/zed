//===-- Writer definition for printf ----------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDIO_PRINTF_CORE_WRITER_H
#define LLVM_LIBC_SRC_STDIO_PRINTF_CORE_WRITER_H

#include "src/__support/CPP/string_view.h"
#include "src/__support/macros/config.h"
#include "src/__support/macros/optimization.h"
#include "src/stdio/printf_core/core_structs.h"
#include "src/string/memory_utils/inline_memcpy.h"
#include "src/string/memory_utils/inline_memset.h"

#include <stddef.h>

namespace LIBC_NAMESPACE_DECL {
namespace printf_core {

enum class WriteMode {
  FILL_BUFF_AND_DROP_OVERFLOW,
  FLUSH_TO_STREAM,
  RESIZE_AND_FILL_BUFF,
  RUNTIME_DISPATCH,
};

// Helper to omit the template argument if we are using runtime dispatch and
// avoid multiple copies of the converter functions.
template <WriteMode write_mode> struct Mode {
#ifdef LIBC_COPT_PRINTF_RUNTIME_DISPATCH
  static constexpr WriteMode value = WriteMode::RUNTIME_DISPATCH;
#else
  static constexpr WriteMode value = write_mode;
#endif
};

template <WriteMode write_mode> struct WriteBuffer {
  using StreamWriter = int (*)(cpp::string_view, void *);
  char *buff;
  const char *init_buff; // for checking when resize.
  size_t buff_len;
  size_t buff_cur = 0;

  // The stream writer will be called when the buffer is full. It will be passed
  // string_views to write to the stream.
  const StreamWriter stream_writer;
  void *output_target;

  // The current writing mode in case the user wants runtime dispatch of the
  // stream writer with function pointers.
  [[maybe_unused]] WriteMode write_mode_;

  LIBC_INLINE WriteBuffer(char *buff, size_t buff_len, StreamWriter hook,
                          void *target)
      : buff(buff), init_buff(buff), buff_len(buff_len), stream_writer(hook),
        output_target(target), write_mode_(WriteMode::FLUSH_TO_STREAM) {}

  LIBC_INLINE WriteBuffer(char *buff, size_t buff_len)
      : buff(buff), init_buff(buff), buff_len(buff_len), stream_writer(nullptr),
        output_target(nullptr),
        write_mode_(WriteMode::FILL_BUFF_AND_DROP_OVERFLOW) {}

  LIBC_INLINE WriteBuffer(char *buff, size_t buff_len, StreamWriter hook)
      : buff(buff), init_buff(buff), buff_len(buff_len), stream_writer(hook),
        output_target(this), write_mode_(WriteMode::RESIZE_AND_FILL_BUFF) {}

  LIBC_INLINE int flush_to_stream(cpp::string_view new_str) {
    if (buff_cur > 0) {
      int retval = stream_writer({buff, buff_cur}, output_target);
      if (retval < 0)
        return retval;
    }
    if (new_str.size() > 0) {
      int retval = stream_writer(new_str, output_target);
      if (retval < 0)
        return retval;
    }
    buff_cur = 0;
    return WRITE_OK;
  }

  LIBC_INLINE int fill_remaining_to_buff(cpp::string_view new_str) {
    if (buff_cur < buff_len) {
      size_t bytes_to_write = buff_len - buff_cur;
      if (bytes_to_write > new_str.size()) {
        bytes_to_write = new_str.size();
      }
      inline_memcpy(buff + buff_cur, new_str.data(), bytes_to_write);
      buff_cur += bytes_to_write;
    }
    return WRITE_OK;
  }

  LIBC_INLINE int resize_and_write(cpp::string_view new_str) {
    return stream_writer(new_str, output_target);
  }

  // The overflow_write method is intended to be called to write the contents of
  // the buffer and new_str to the stream_writer if it exists. If a resizing
  // hook is provided, it will resize the buffer and write the contents. If
  // neither a stream_writer nor a resizing hook is provided, it will fill the
  // remaining space in the buffer with new_str and drop the overflow. Calling
  // this with an empty string will flush the buffer if relevant.

  LIBC_INLINE int overflow_write(cpp::string_view new_str) {
    if constexpr (write_mode == WriteMode::RUNTIME_DISPATCH) {
      if (write_mode_ == WriteMode::FILL_BUFF_AND_DROP_OVERFLOW)
        return fill_remaining_to_buff(new_str);
      else if (write_mode_ == WriteMode::FLUSH_TO_STREAM)
        return flush_to_stream(new_str);
      else if (write_mode_ == WriteMode::RESIZE_AND_FILL_BUFF)
        return resize_and_write(new_str);
    } else if constexpr (write_mode == WriteMode::FILL_BUFF_AND_DROP_OVERFLOW) {
      return fill_remaining_to_buff(new_str);
    } else if constexpr (write_mode == WriteMode::FLUSH_TO_STREAM) {
      return flush_to_stream(new_str);
    } else if constexpr (write_mode == WriteMode::RESIZE_AND_FILL_BUFF) {
      return resize_and_write(new_str);
    }
    __builtin_unreachable();
  }
};

template <WriteMode write_mode> class Writer final {
  WriteBuffer<write_mode> &wb;
  int chars_written = 0;

  LIBC_INLINE int pad(char new_char, size_t length) {
    // First, fill as much of the buffer as possible with the padding char.
    size_t written = 0;
    const size_t buff_space = wb.buff_len - wb.buff_cur;
    // ASSERT: length > buff_space
    if (buff_space > 0) {
      inline_memset(wb.buff + wb.buff_cur, new_char, buff_space);
      wb.buff_cur += buff_space;
      written = buff_space;
    }

    // Next, overflow write the rest of length using the mini_buff.
    constexpr size_t MINI_BUFF_SIZE = 64;
    char mini_buff[MINI_BUFF_SIZE];
    inline_memset(mini_buff, new_char, MINI_BUFF_SIZE);
    cpp::string_view mb_string_view(mini_buff, MINI_BUFF_SIZE);
    while (written + MINI_BUFF_SIZE < length) {
      int result = wb.overflow_write(mb_string_view);
      if (result != WRITE_OK)
        return result;
      written += MINI_BUFF_SIZE;
    }
    cpp::string_view mb_substr = mb_string_view.substr(0, length - written);
    return wb.overflow_write(mb_substr);
  }

public:
  LIBC_INLINE Writer(WriteBuffer<write_mode> &wb) : wb(wb) {}

  // Takes a string, copies it into the buffer if there is space, else passes it
  // to the overflow mechanism to be handled separately.
  LIBC_INLINE int write(cpp::string_view new_string) {
    chars_written += static_cast<int>(new_string.size());
    if (LIBC_LIKELY(wb.buff_cur + new_string.size() <= wb.buff_len)) {
      inline_memcpy(wb.buff + wb.buff_cur, new_string.data(),
                    new_string.size());
      wb.buff_cur += new_string.size();
      return WRITE_OK;
    }
    return wb.overflow_write(new_string);
  }

  // Takes a char and a length, memsets the next length characters of the buffer
  // if there is space, else calls pad which will loop and call the overflow
  // mechanism on a secondary buffer.
  LIBC_INLINE int write(char new_char, size_t length) {
    chars_written += static_cast<int>(length);

    if (LIBC_LIKELY(wb.buff_cur + length <= wb.buff_len)) {
      inline_memset(wb.buff + wb.buff_cur, static_cast<unsigned char>(new_char),
                    length);
      wb.buff_cur += length;
      return WRITE_OK;
    }
    return pad(new_char, length);
  }

  // Takes a char, copies it into the buffer if there is space, else passes it
  // to the overflow mechanism to be handled separately.
  LIBC_INLINE int write(char new_char) {
    chars_written += 1;
    if (LIBC_LIKELY(wb.buff_cur + 1 <= wb.buff_len)) {
      wb.buff[wb.buff_cur] = new_char;
      wb.buff_cur += 1;
      return WRITE_OK;
    }
    cpp::string_view char_string_view(&new_char, 1);
    return wb.overflow_write(char_string_view);
  }

  LIBC_INLINE int get_chars_written() { return chars_written; }
};

// Class-template auto deduction helpers.
Writer(WriteBuffer<WriteMode::FILL_BUFF_AND_DROP_OVERFLOW>)
    -> Writer<WriteMode::FILL_BUFF_AND_DROP_OVERFLOW>;
Writer(WriteBuffer<WriteMode::RESIZE_AND_FILL_BUFF>)
    -> Writer<WriteMode::RESIZE_AND_FILL_BUFF>;
Writer(WriteBuffer<WriteMode::FLUSH_TO_STREAM>)
    -> Writer<WriteMode::FLUSH_TO_STREAM>;

} // namespace printf_core
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDIO_PRINTF_CORE_WRITER_H
