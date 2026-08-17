// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_PE_IMAGE_READER_H_
#define BASE_WIN_PE_IMAGE_READER_H_

#include <windows.h>

#include <stddef.h>
#include <stdint.h>

#include <memory>

#include "base/base_export.h"
#include "base/containers/span.h"
#include "base/memory/raw_span.h"
#include "base/numerics/safe_math.h"

namespace base {
namespace win {

// Parses headers and various data from a PE image. This parser is safe for use
// on untrusted data and works on PE files with different bitness from the
// current process. The PeImageReader is initialized after construction by
// passing the address and size of a PE file that has been read into memory -
// not loaded by the OS as an image. Parsing of a PE file that has been loaded
// as an image can be done with PEImage.
class BASE_EXPORT PeImageReader {
 public:
  enum WordSize {
    WORD_SIZE_32,
    WORD_SIZE_64,
  };

  // A callback invoked by EnumCertificates once for each attribute certificate
  // entry in the image's attribute certificate table. |revision| and
  // |certificate_type| identify the contents of |certificate_data| (which is of
  // |certificate_data_size| bytes). |context| is the value provided by the
  // caller to EnumCertificates(). Implementations must return true to continue
  // the enumeration, or false to abort.
  using EnumCertificatesCallback =
      bool (*)(uint16_t revision,
               uint16_t certificate_type,
               base::span<const uint8_t> certificate_data,
               void* context);

  PeImageReader();

  PeImageReader(const PeImageReader&) = delete;
  PeImageReader& operator=(const PeImageReader&) = delete;

  ~PeImageReader();

  // Returns false if the given data does not appear to be a valid PE image.
  bool Initialize(span<const uint8_t> image_data);

  // Returns the machine word size for the image.
  WordSize GetWordSize();

  const IMAGE_DOS_HEADER* GetDosHeader();
  const IMAGE_FILE_HEADER* GetCoffFileHeader();

  // Returns the optional header data.
  span<const uint8_t> GetOptionalHeaderData();
  size_t GetNumberOfSections();
  const IMAGE_SECTION_HEADER* GetSectionHeaderAt(size_t index);

  // Returns the image's export data (.edata) section, or an empty span if the
  // section is not present.
  span<const uint8_t> GetExportSection();

  size_t GetNumberOfDebugEntries();
  // Returns a pointer to the |index|'th debug directory entry, or nullptr if
  // |index| is out of bounds. |raw_data| is an out-param which will be filled
  // with the corresponding raw data.
  const IMAGE_DEBUG_DIRECTORY* GetDebugEntry(size_t index,
                                             span<const uint8_t>& raw_data);

  // Invokes |callback| once per attribute certificate entry. |context| is a
  // caller-specific value that is passed to |callback|. Returns true if all
  // certificate entries are visited (even if there are no such entries) and
  // |callback| returns true for each. Conversely, returns |false| if |callback|
  // returns false or if the image is malformed in any way.
  bool EnumCertificates(EnumCertificatesCallback callback, void* context);

  // Returns the size of the image file.
  DWORD GetSizeOfImage();

 private:
  // Bits indicating what portions of the image have been validated.
  enum ValidationStages {
    VALID_DOS_HEADER = 1 << 0,
    VALID_PE_SIGNATURE = 1 << 1,
    VALID_COFF_FILE_HEADER = 1 << 2,
    VALID_OPTIONAL_HEADER = 1 << 3,
    VALID_SECTION_HEADERS = 1 << 4,
  };

  // An interface to an image's optional header.
  class OptionalHeader {
   public:
    virtual ~OptionalHeader() = default;

    virtual WordSize GetWordSize() = 0;

    // Returns the offset of the DataDirectory member relative to the start of
    // the optional header.
    virtual size_t GetDataDirectoryOffset() = 0;

    // Returns the number of entries in the data directory.
    virtual DWORD GetDataDirectorySize() = 0;

    // Returns a pointer to the first data directory entry.
    virtual const IMAGE_DATA_DIRECTORY* GetDataDirectoryEntries() = 0;

    // Returns the size of the image file.
    virtual DWORD GetSizeOfImage() = 0;
  };

  template <class OPTIONAL_HEADER_TYPE>
  class OptionalHeaderImpl;

  void Clear();
  bool ValidateDosHeader();
  bool ValidatePeSignature();
  bool ValidateCoffFileHeader();
  bool ValidateOptionalHeader();
  bool ValidateSectionHeaders();

  // Return a pointer to the first byte of the image's optional header.
  const uint8_t* GetOptionalHeaderStart();
  size_t GetOptionalHeaderSize();

  // Returns the desired directory entry, or nullptr if |index| is out of
  // bounds.
  const IMAGE_DATA_DIRECTORY* GetDataDirectoryEntryAt(size_t index);

  // Returns the header for the section that contains the given address, or
  // nullptr if the address is out of bounds or the image does not contain the
  // section.
  const IMAGE_SECTION_HEADER* FindSectionFromRva(uint32_t relative_address);

  // Returns the bytes referenced by the |index|'th data directory entry.
  span<const uint8_t> GetImageData(size_t index);

  // Populates |structure| with a pointer to a desired structure of type T at
  // the given offset if the image is sufficiently large to contain it. Returns
  // false if the structure does not fully fit within the image at the given
  // offset.
  template <typename T>
  bool GetStructureAt(size_t offset, const T** structure) {
    return GetStructureAt(offset, sizeof(**structure), structure);
  }

  // Populates |structure| with a pointer to a desired structure of type T at
  // the given offset if the image is sufficiently large to contain
  // |structure_size| bytes. Returns false if the structure does not fully fit
  // within the image at the given offset.
  template <typename T>
  bool GetStructureAt(size_t offset,
                      size_t structure_size,
                      const T** structure) {
    size_t remaining_bytes = 0;
    if (!CheckSub(image_data_.size(), offset).AssignIfValid(&remaining_bytes)) {
      return false;
    }
    if (structure_size > remaining_bytes) {
      return false;
    }
    *structure = reinterpret_cast<const T*>(image_data_.subspan(offset).data());
    return true;
  }

  raw_span<const uint8_t> image_data_;
  uint32_t validation_state_ = 0;
  std::unique_ptr<OptionalHeader> optional_header_;
};

}  // namespace win
}  // namespace base

#endif  // BASE_WIN_PE_IMAGE_READER_H_
