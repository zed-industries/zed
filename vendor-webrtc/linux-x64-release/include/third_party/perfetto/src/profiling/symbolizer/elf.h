/*
 * Copyright (C) 2021 The Android Open Source Project
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

#ifndef SRC_PROFILING_SYMBOLIZER_ELF_H_
#define SRC_PROFILING_SYMBOLIZER_ELF_H_

#include <stddef.h>
#include <cinttypes>

#include "perfetto/base/build_config.h"

// We cannot just include elf.h, as that only exists on Linux, and we want to
// allow symbolization on other platforms as well. As we only need a small
// subset, it is easiest to define the constants and structs ourselves.

namespace perfetto {
namespace profiling {

constexpr auto PT_LOAD = 1;
constexpr auto PF_X = 1;
constexpr auto SHT_NOTE = 7;
constexpr auto NT_GNU_BUILD_ID = 3;
constexpr auto ELFCLASS32 = 1;
constexpr auto ELFCLASS64 = 2;
constexpr auto ELFMAG0 = 0x7f;
constexpr auto ELFMAG1 = 'E';
constexpr auto ELFMAG2 = 'L';
constexpr auto ELFMAG3 = 'F';
constexpr auto ELFDATA2LSB = 1;
constexpr auto EV_CURRENT = 1;
constexpr auto EI_MAG0 = 0;
constexpr auto EI_MAG1 = 1;
constexpr auto EI_MAG2 = 2;
constexpr auto EI_MAG3 = 3;
constexpr auto EI_CLASS = 4;
constexpr auto EI_DATA = 5;
constexpr auto EI_VERSION = 6;

struct Elf32 {
  using Addr = uint32_t;
  using Half = uint16_t;
  using Off = uint32_t;
  using Sword = int32_t;
  using Word = uint32_t;
  struct Ehdr {
    unsigned char e_ident[16];
    Half e_type;
    Half e_machine;
    Word e_version;
    Addr e_entry;
    Off e_phoff;
    Off e_shoff;
    Word e_flags;
    Half e_ehsize;
    Half e_phentsize;
    Half e_phnum;
    Half e_shentsize;
    Half e_shnum;
    Half e_shstrndx;
  };
  struct Shdr {
    Word sh_name;
    Word sh_type;
    Word sh_flags;
    Addr sh_addr;
    Off sh_offset;
    Word sh_size;
    Word sh_link;
    Word sh_info;
    Word sh_addralign;
    Word sh_entsize;
  };
  struct Nhdr {
    Word n_namesz;
    Word n_descsz;
    Word n_type;
  };
  struct Phdr {
    uint32_t p_type;
    Off p_offset;
    Addr p_vaddr;
    Addr p_paddr;
    uint32_t p_filesz;
    uint32_t p_memsz;
    uint32_t p_flags;
    uint32_t p_align;
  };
};

struct Elf64 {
  using Addr = uint64_t;
  using Half = uint16_t;
  using SHalf = int16_t;
  using Off = uint64_t;
  using Sword = int32_t;
  using Word = uint32_t;
  using Xword = uint64_t;
  using Sxword = int64_t;
  struct Ehdr {
    unsigned char e_ident[16];
    Half e_type;
    Half e_machine;
    Word e_version;
    Addr e_entry;
    Off e_phoff;
    Off e_shoff;
    Word e_flags;
    Half e_ehsize;
    Half e_phentsize;
    Half e_phnum;
    Half e_shentsize;
    Half e_shnum;
    Half e_shstrndx;
  };
  struct Shdr {
    Word sh_name;
    Word sh_type;
    Xword sh_flags;
    Addr sh_addr;
    Off sh_offset;
    Xword sh_size;
    Word sh_link;
    Word sh_info;
    Xword sh_addralign;
    Xword sh_entsize;
  };
  struct Nhdr {
    Word n_namesz;
    Word n_descsz;
    Word n_type;
  };
  struct Phdr {
    uint32_t p_type;
    uint32_t p_flags;
    Off p_offset;
    Addr p_vaddr;
    Addr p_paddr;
    uint64_t p_filesz;
    uint64_t p_memsz;
    uint64_t p_align;
  };
};

template <typename E>
typename E::Shdr* GetShdr(void* mem, const typename E::Ehdr* ehdr, size_t i) {
  return reinterpret_cast<typename E::Shdr*>(
      static_cast<char*>(mem) + ehdr->e_shoff + i * sizeof(typename E::Shdr));
}

template <typename E>
typename E::Phdr* GetPhdr(void* mem, const typename E::Ehdr* ehdr, size_t i) {
  return reinterpret_cast<typename E::Phdr*>(
      static_cast<char*>(mem) + ehdr->e_phoff + i * sizeof(typename E::Phdr));
}

}  // namespace profiling
}  // namespace perfetto

#endif  // SRC_PROFILING_SYMBOLIZER_ELF_H_
