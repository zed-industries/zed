// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FILE_VERSION_INFO_WIN_H_
#define BASE_FILE_VERSION_INFO_WIN_H_

#include <windows.h>

#include <stdint.h>

#include <memory>
#include <string>
#include <vector>

#include "base/base_export.h"
#include "base/file_version_info.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/raw_ref.h"
#include "base/version.h"

struct tagVS_FIXEDFILEINFO;
typedef tagVS_FIXEDFILEINFO VS_FIXEDFILEINFO;

class BASE_EXPORT FileVersionInfoWin : public FileVersionInfo {
 public:
  FileVersionInfoWin(const FileVersionInfoWin&) = delete;
  FileVersionInfoWin& operator=(const FileVersionInfoWin&) = delete;
  ~FileVersionInfoWin() override;

  // Accessors to the different version properties.
  // Returns an empty string if the property is not found.
  std::u16string company_name() override;
  std::u16string company_short_name() override;
  std::u16string product_name() override;
  std::u16string product_short_name() override;
  std::u16string internal_name() override;
  std::u16string product_version() override;
  std::u16string special_build() override;
  std::u16string original_filename() override;
  std::u16string file_description() override;
  std::u16string file_version() override;

  // Lets you access other properties not covered above. |value| is only
  // modified if GetValue() returns true.
  bool GetValue(const char16_t* name, std::u16string* value) const;

  // Similar to GetValue but returns a std::u16string (empty string if the
  // property does not exist).
  std::u16string GetStringValue(const char16_t* name) const;

  // Get file version number in dotted version format.
  base::Version GetFileVersion() const;

  // Behaves like CreateFileVersionInfo, but returns a FileVersionInfoWin.
  static std::unique_ptr<FileVersionInfoWin> CreateFileVersionInfoWin(
      const base::FilePath& file_path);

 private:
  friend FileVersionInfo;

  // |data| is a VS_VERSION_INFO resource. |language| and |code_page| are
  // extracted from the \VarFileInfo\Translation value of |data|.
  FileVersionInfoWin(std::vector<uint8_t>&& data,
                     WORD language,
                     WORD code_page);
  FileVersionInfoWin(void* data, WORD language, WORD code_page);

  const std::vector<uint8_t> owned_data_;
  const raw_ptr<const void> data_;
  const WORD language_;
  const WORD code_page_;

  // This is a reference for a portion of |data_|.
  const raw_ref<const VS_FIXEDFILEINFO> fixed_file_info_;
};

#endif  // BASE_FILE_VERSION_INFO_WIN_H_
