/*
 * Copyright (C) 2020 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_EXT_BASE_ENDIAN_H_
#define INCLUDE_PERFETTO_EXT_BASE_ENDIAN_H_

#include <stdint.h>
#include <stdlib.h>  // For MSVC

#include "perfetto/base/build_config.h"
#include "perfetto/base/compiler.h"

#if !PERFETTO_IS_LITTLE_ENDIAN()
#error "endian.h supports only little-endian archs"
#endif

namespace perfetto {
namespace base {

#if PERFETTO_BUILDFLAG(PERFETTO_COMPILER_MSVC)
inline uint16_t HostToBE16(uint16_t x) {
  return _byteswap_ushort(x);
}
inline uint32_t HostToBE32(uint32_t x) {
  return _byteswap_ulong(x);
}
inline uint64_t HostToBE64(uint64_t x) {
  return _byteswap_uint64(x);
}
#else
inline uint16_t HostToBE16(uint16_t x) {
  return __builtin_bswap16(x);
}
inline uint32_t HostToBE32(uint32_t x) {
  return __builtin_bswap32(x);
}
inline uint64_t HostToBE64(uint64_t x) {
  return __builtin_bswap64(x);
}
#endif

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_ENDIAN_H_
