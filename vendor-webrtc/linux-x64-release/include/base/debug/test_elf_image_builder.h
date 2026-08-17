// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_DEBUG_TEST_ELF_IMAGE_BUILDER_H_
#define BASE_DEBUG_TEST_ELF_IMAGE_BUILDER_H_

#include <elf.h>

#include <cstdint>
#include <optional>
#include <string>
#include <string_view>
#include <vector>

#include "base/containers/span.h"
#include "base/memory/raw_ptr.h"

#if __SIZEOF_POINTER__ == 4
using Addr = Elf32_Addr;
using Ehdr = Elf32_Ehdr;
using Half = Elf32_Half;
using Off = Elf32_Off;
using Phdr = Elf32_Phdr;
using Word = Elf32_Word;
#else
using Addr = Elf64_Addr;
using Ehdr = Elf64_Ehdr;
using Half = Elf64_Half;
using Off = Elf64_Off;
using Phdr = Elf64_Phdr;
using Word = Elf64_Word;
#endif

namespace base {

// In-memory ELF image constructed by TestElfImageBuilder.
class TestElfImage {
 public:
  // |buffer| is a memory buffer containing the ELF image. |elf_start| is the
  // start address of the ELF image within the buffer.
  TestElfImage(std::vector<uint8_t> buffer, const void* elf_start);
  ~TestElfImage();

  TestElfImage(TestElfImage&&);
  TestElfImage& operator=(TestElfImage&&);

  // The start address of the ELF image.
  const void* elf_start() const { return elf_start_; }

 private:
  std::vector<uint8_t> buffer_;
  raw_ptr<const void> elf_start_;
};

// Builds an in-memory image of an ELF file for testing.
class TestElfImageBuilder {
 public:
  // The type of mapping to use for virtual addresses in the ELF file.
  enum MappingType {
    RELOCATABLE,            // Virtual address == file offset.
    RELOCATABLE_WITH_BIAS,  // Virtual address == file offset + load bias.
    NON_RELOCATABLE,        // Virtual address == mapped address.
  };

  // The load bias to use for RELOCATABLE_WITH_BIAS. 0xc000 is a commonly used
  // load bias for Android system ELF images.
  static constexpr size_t kLoadBias = 0xc000;

  explicit TestElfImageBuilder(MappingType mapping_type);
  ~TestElfImageBuilder();

  TestElfImageBuilder(const TestElfImageBuilder&) = delete;
  TestElfImageBuilder& operator=(const TestElfImageBuilder&) = delete;

  // Add a PT_LOAD segment with the specified rwx |flags|. The contents will be
  // filled with |size| bytes of zeros.
  TestElfImageBuilder& AddLoadSegment(Word flags, size_t size);

  // Add a PT_NOTE segment with the specified state.
  TestElfImageBuilder& AddNoteSegment(Word type,
                                      std::string_view name,
                                      span<const uint8_t> desc);

  // Adds a DT_SONAME dynamic section and the necessary state to support it. May
  // be invoked at most once.
  TestElfImageBuilder& AddSoName(std::string_view soname);

  TestElfImage Build();

 private:
  // Properties of a load segment to create.
  struct LoadSegment;

  // Computed sizing state for parts of the ELF image.
  struct ImageMeasures;

  // Gets the 'virtual address' corresponding to |offset| to write into the
  // image, according to |mapping_type_|. Relocatable ELF images have virtual
  // addresses equal to the offset with a possible constant load bias.
  // Non-relocatable ELF images have virtual addresses equal to the actual
  // memory address.
  Addr GetVirtualAddressForOffset(Off offset, const uint8_t* elf_start) const;

  // Measures sizes/start offset of segments in the image.
  ImageMeasures MeasureSizesAndOffsets() const;

  // Appends a header of type |T| at |loc|, a memory address within the ELF
  // image being constructed, and returns the address past the header.
  template <typename T>
  static uint8_t* AppendHdr(const T& hdr, uint8_t* loc);

  Ehdr CreateEhdr(Half phnum);
  Phdr CreatePhdr(Word type,
                  Word flags,
                  size_t align,
                  Off offset,
                  Addr vaddr,
                  size_t size);

  const MappingType mapping_type_;
  std::vector<std::vector<uint8_t>> note_contents_;
  std::vector<LoadSegment> load_segments_;
  std::optional<std::string> soname_;
};

}  // namespace base

#endif  // BASE_DEBUG_TEST_ELF_IMAGE_BUILDER_H_
