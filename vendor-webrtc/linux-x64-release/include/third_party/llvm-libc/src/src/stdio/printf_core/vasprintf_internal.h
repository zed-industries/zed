//===-- Internal Implementation of asprintf ---------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "hdr/func/free.h"
#include "hdr/func/malloc.h"
#include "hdr/func/realloc.h"
#include "src/__support/arg_list.h"
#include "src/stdio/printf.h"
#include "src/stdio/printf_core/core_structs.h"
#include "src/stdio/printf_core/printf_main.h"
#include "src/stdio/printf_core/writer.h"

namespace LIBC_NAMESPACE_DECL {
namespace printf_core {

LIBC_INLINE int resize_overflow_hook(cpp::string_view new_str, void *target) {
  WriteBuffer<Mode<WriteMode::RESIZE_AND_FILL_BUFF>::value> *wb =
      reinterpret_cast<
          WriteBuffer<Mode<WriteMode::RESIZE_AND_FILL_BUFF>::value> *>(target);
  size_t new_size = new_str.size() + wb->buff_cur;
  const bool isBuffOnStack = (wb->buff == wb->init_buff);
  char *new_buff = static_cast<char *>(
      isBuffOnStack ? malloc(new_size + 1)
                    : realloc(wb->buff, new_size + 1)); // +1 for null
  if (new_buff == nullptr) {
    if (wb->buff != wb->init_buff)
      free(wb->buff);
    return printf_core::ALLOCATION_ERROR;
  }
  if (isBuffOnStack)
    inline_memcpy(new_buff, wb->buff, wb->buff_cur);
  wb->buff = new_buff;
  inline_memcpy(wb->buff + wb->buff_cur, new_str.data(), new_str.size());
  wb->buff_cur = new_size;
  wb->buff_len = new_size;
  return printf_core::WRITE_OK;
}

constexpr size_t DEFAULT_BUFFER_SIZE = 200;

LIBC_INLINE int vasprintf_internal(char **ret, const char *__restrict format,
                                   internal::ArgList args) {
  char init_buff_on_stack[DEFAULT_BUFFER_SIZE];
  printf_core::WriteBuffer<Mode<WriteMode::RESIZE_AND_FILL_BUFF>::value> wb(
      init_buff_on_stack, DEFAULT_BUFFER_SIZE, resize_overflow_hook);
  printf_core::Writer writer(wb);

  auto ret_val = printf_core::printf_main(&writer, format, args);
  if (ret_val < 0) {
    *ret = nullptr;
    return -1;
  }
  if (wb.buff == init_buff_on_stack) {
    *ret = static_cast<char *>(malloc(ret_val + 1));
    if (ret == nullptr)
      return printf_core::ALLOCATION_ERROR;
    inline_memcpy(*ret, wb.buff, ret_val);
  } else {
    *ret = wb.buff;
  }
  (*ret)[ret_val] = '\0';
  return ret_val;
}
} // namespace printf_core
} // namespace LIBC_NAMESPACE_DECL
