// Copyright 2010 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file was adapted from GreenBorder's Code.
// To understand what this class is about (for other than well known functions
// as GetProcAddress), a good starting point is "An In-Depth Look into the
// Win32 Portable Executable File Format" by Matt Pietrek:
// http://msdn.microsoft.com/msdnmag/issues/02/02/PE/default.aspx

#ifndef BASE_WIN_PE_IMAGE_H_
#define BASE_WIN_PE_IMAGE_H_

#include <windows.h>

#include <delayimp.h>
#include <stdint.h>

namespace base {
namespace win {

// This class is a wrapper for the Portable Executable File Format (PE).
// Its main purpose is to provide an easy way to work with imports and exports
// from a file, mapped in memory as image. A PEImage object is constructed from
// a loaded PE file by passing the HMODULE to the constructor. Loading a PE file
// as an image will execute code and should only be done with trusted images.
// Parsing of untrusted PE files is better done with PeImageReader.
// PEImage can only parse PE files that match the bitness of the process.
// See also PEImageAsData.
class PEImage {
 public:
  // Callback to enumerate sections.
  // cookie is the value passed to the enumerate method.
  // Returns true to continue the enumeration.
  using EnumSectionsFunction =
      bool (*)(const PEImage&, PIMAGE_SECTION_HEADER, PVOID, DWORD, PVOID);

  // Callback to enumerate exports.
  // function is the actual address of the symbol. If forward is not null, it
  // contains the dll and symbol to forward this export to. cookie is the value
  // passed to the enumerate method.
  // Returns true to continue the enumeration.
  using EnumExportsFunction =
      bool (*)(const PEImage&, DWORD, DWORD, LPCSTR, PVOID, LPCSTR, PVOID);

  // Callback to enumerate import blocks.
  // name_table and iat point to the imports name table and address table for
  // this block. cookie is the value passed to the enumerate method.
  // Returns true to continue the enumeration.
  using EnumImportChunksFunction = bool (*)(const PEImage&,
                                            LPCSTR,
                                            PIMAGE_THUNK_DATA,
                                            PIMAGE_THUNK_DATA,
                                            PVOID);

  // Callback to enumerate imports.
  // module is the dll that exports this symbol. cookie is the value passed to
  // the enumerate method.
  // Returns true to continue the enumeration.
  using EnumImportsFunction = bool (*)(const PEImage&,
                                       LPCSTR,
                                       DWORD,
                                       LPCSTR,
                                       DWORD,
                                       PIMAGE_THUNK_DATA,
                                       PVOID);

  // Callback to enumerate delayed import blocks.
  // module is the dll that exports this block of symbols. cookie is the value
  // passed to the enumerate method.
  // Returns true to continue the enumeration.
  using EnumDelayImportChunksFunction = bool (*)(const PEImage&,
                                                 PImgDelayDescr,
                                                 LPCSTR,
                                                 PIMAGE_THUNK_DATA,
                                                 PIMAGE_THUNK_DATA,
                                                 PVOID);

  // Callback to enumerate relocations.
  // cookie is the value passed to the enumerate method.
  // Returns true to continue the enumeration.
  using EnumRelocsFunction = bool (*)(const PEImage&, WORD, PVOID, PVOID);

  explicit PEImage(HMODULE module) : module_(module) {}
  explicit PEImage(const void* module) {
    module_ = reinterpret_cast<HMODULE>(const_cast<void*>(module));
  }

  virtual ~PEImage() = default;

  // Gets the HMODULE for this object.
  HMODULE module() const;

  // Sets this object's HMODULE.
  void set_module(HMODULE module);

  // Checks if this symbol is actually an ordinal.
  static bool IsOrdinal(LPCSTR name);

  // Converts a named symbol to the corresponding ordinal.
  static WORD ToOrdinal(LPCSTR name);

  // Returns the DOS_HEADER for this PE.
  PIMAGE_DOS_HEADER GetDosHeader() const;

  // Returns the NT_HEADER for this PE.
  PIMAGE_NT_HEADERS GetNTHeaders() const;

  // Returns number of sections of this PE.
  WORD GetNumSections() const;

  // Returns the header for a given section.
  // returns NULL if there is no such section.
  PIMAGE_SECTION_HEADER GetSectionHeader(WORD section) const;

  // Returns the size of a given directory entry or 0 if |directory| is out of
  // bounds.
  DWORD GetImageDirectoryEntrySize(UINT directory) const;

  // Returns the address of a given directory entry or NULL if |directory| is
  // out of bounds.
  PVOID GetImageDirectoryEntryAddr(UINT directory) const;

  // Returns the section header for a given address.
  // Use: s = image.GetImageSectionFromAddr(a);
  // Post: 's' is the section header of the section that contains 'a'
  //       or NULL if there is no such section.
  PIMAGE_SECTION_HEADER GetImageSectionFromAddr(PVOID address) const;

  // Returns the section header for a given section.
  PIMAGE_SECTION_HEADER GetImageSectionHeaderByName(LPCSTR section_name) const;

  // Returns the first block of imports.
  PIMAGE_IMPORT_DESCRIPTOR GetFirstImportChunk() const;

  // Returns the exports directory.
  PIMAGE_EXPORT_DIRECTORY GetExportDirectory() const;

  // Retrieves the contents of the image's CodeView debug entry, returning true
  // if such an entry is found and is within a section mapped into the current
  // process's memory. |guid|, |age|, and |pdb_filename| are each optional and
  // may be NULL. |pdb_filename_length| is mandatory if |pdb_filename| is not
  // NULL, as the latter is populated with a direct reference to a string in the
  // image that is is not guaranteed to be terminated (note: informal
  // documentation indicates that it should be terminated, but the data is
  // untrusted). Furthermore, owing to its nature of being a string in the
  // image, it is only valid while the image is mapped into the process, and the
  // caller is not responsible for freeing it. |pdb_filename_length| is
  // populated with the string length of |pdb_filename| (not including a
  // terminator) and must be used rather than relying on |pdb_filename| being
  // properly terminated.
  bool GetDebugId(LPGUID guid,
                  LPDWORD age,
                  LPCSTR* pdb_filename,
                  size_t* pdb_filename_length) const;

  // Returns a given export entry.
  // Use: e = image.GetExportEntry(f);
  // Pre: 'f' is either a zero terminated string or ordinal
  // Post: 'e' is a pointer to the export directory entry
  //       that contains 'f's export RVA, or NULL if 'f'
  //       is not exported from this image
  PDWORD GetExportEntry(LPCSTR name) const;

  // Returns the address for a given exported symbol.
  // Use: p = image.GetProcAddress(f);
  // Pre: 'f' is either a zero terminated string or ordinal.
  // Post: if 'f' is a non-forwarded export from image, 'p' is
  //       the exported function. If 'f' is a forwarded export
  //       then p is the special value -1. In this case
  //       RVAToAddr(*GetExportEntry) can be used to resolve
  //       the string that describes the forward.
  FARPROC GetProcAddress(LPCSTR function_name) const;

  // Retrieves the ordinal for a given exported symbol.
  // Returns true if the symbol was found.
  bool GetProcOrdinal(LPCSTR function_name, WORD* ordinal) const;

  // Enumerates PE sections.
  // cookie is a generic cookie to pass to the callback.
  // Returns true on success.
  bool EnumSections(EnumSectionsFunction callback, PVOID cookie) const;

  // Enumerates PE exports.
  // cookie is a generic cookie to pass to the callback.
  // Returns true on success.
  bool EnumExports(EnumExportsFunction callback, PVOID cookie) const;

  // Enumerates PE imports.
  // cookie is a generic cookie to pass to the callback.
  // Returns true on success.
  // Use |target_module_name| to ensure the callback is only invoked for the
  // specified module.
  bool EnumAllImports(EnumImportsFunction callback,
                      PVOID cookie,
                      LPCSTR target_module_name) const;

  // Enumerates PE import blocks.
  // cookie is a generic cookie to pass to the callback.
  // Returns true on success.
  // Use |target_module_name| to ensure the callback is only invoked for the
  // specified module.
  bool EnumImportChunks(EnumImportChunksFunction callback,
                        PVOID cookie,
                        LPCSTR target_module_name) const;

  // Enumerates the imports from a single PE import block.
  // cookie is a generic cookie to pass to the callback.
  // Returns true on success.
  bool EnumOneImportChunk(EnumImportsFunction callback,
                          LPCSTR module_name,
                          PIMAGE_THUNK_DATA name_table,
                          PIMAGE_THUNK_DATA iat,
                          PVOID cookie) const;

  // Enumerates PE delay imports.
  // cookie is a generic cookie to pass to the callback.
  // Returns true on success.
  // Use |target_module_name| to ensure the callback is only invoked for the
  // specified module. If this parameter is non-null then all delayloaded
  // imports are resolved when the target module is found.
  bool EnumAllDelayImports(EnumImportsFunction callback,
                           PVOID cookie,
                           LPCSTR target_module_name) const;

  // Enumerates PE delay import blocks.
  // cookie is a generic cookie to pass to the callback.
  // Returns true on success.
  // Use |target_module_name| to ensure the callback is only invoked for the
  // specified module. If this parameter is non-null then all delayloaded
  // imports are resolved when the target module is found.
  bool EnumDelayImportChunks(EnumDelayImportChunksFunction callback,
                             PVOID cookie,
                             LPCSTR target_module_name) const;

  // Enumerates imports from a single PE delay import block.
  // cookie is a generic cookie to pass to the callback.
  // Returns true on success.
  bool EnumOneDelayImportChunk(EnumImportsFunction callback,
                               PImgDelayDescr delay_descriptor,
                               LPCSTR module_name,
                               PIMAGE_THUNK_DATA name_table,
                               PIMAGE_THUNK_DATA iat,
                               PVOID cookie) const;

  // Enumerates PE relocation entries.
  // cookie is a generic cookie to pass to the callback.
  // Returns true on success.
  bool EnumRelocs(EnumRelocsFunction callback, PVOID cookie) const;

  // Verifies the magic values on the PE file.
  // Returns true if all values are correct.
  bool VerifyMagic() const;

  // Converts an rva value to the appropriate address.
  virtual PVOID RVAToAddr(uintptr_t rva) const;

  // Converts an rva value to an offset on disk.
  // Returns true on success.
  bool ImageRVAToOnDiskOffset(uintptr_t rva, DWORD* on_disk_offset) const;

  // Converts an address to an offset on disk.
  // Returns true on success.
  bool ImageAddrToOnDiskOffset(LPVOID address, DWORD* on_disk_offset) const;

 private:
  // Returns a pointer to a data directory, or NULL if |directory| is out of
  // range.
  const IMAGE_DATA_DIRECTORY* GetDataDirectory(UINT directory) const;

  HMODULE module_;
};

// This class is an extension to the PEImage class that allows working with PE
// files mapped as data instead of as image file.
class PEImageAsData : public PEImage {
 public:
  explicit PEImageAsData(HMODULE hModule) : PEImage(hModule) {}

  PVOID RVAToAddr(uintptr_t rva) const override;
};

inline bool PEImage::IsOrdinal(LPCSTR name) {
  return reinterpret_cast<uintptr_t>(name) <= 0xFFFF;
}

inline WORD PEImage::ToOrdinal(LPCSTR name) {
  return static_cast<WORD>(reinterpret_cast<intptr_t>(name));
}

inline HMODULE PEImage::module() const {
  return module_;
}

inline PIMAGE_IMPORT_DESCRIPTOR PEImage::GetFirstImportChunk() const {
  return reinterpret_cast<PIMAGE_IMPORT_DESCRIPTOR>(
      GetImageDirectoryEntryAddr(IMAGE_DIRECTORY_ENTRY_IMPORT));
}

inline PIMAGE_EXPORT_DIRECTORY PEImage::GetExportDirectory() const {
  return reinterpret_cast<PIMAGE_EXPORT_DIRECTORY>(
      GetImageDirectoryEntryAddr(IMAGE_DIRECTORY_ENTRY_EXPORT));
}

}  // namespace win
}  // namespace base

#endif  // BASE_WIN_PE_IMAGE_H_
