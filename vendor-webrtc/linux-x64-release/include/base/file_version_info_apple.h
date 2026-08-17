// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FILE_VERSION_INFO_APPLE_H_
#define BASE_FILE_VERSION_INFO_APPLE_H_

#include <CoreFoundation/CoreFoundation.h>

#include <string>

#include "base/file_version_info.h"

@class NSBundle;

class FileVersionInfoApple : public FileVersionInfo {
 public:
  explicit FileVersionInfoApple(NSBundle* bundle);
  FileVersionInfoApple(const FileVersionInfoApple&) = delete;
  FileVersionInfoApple& operator=(const FileVersionInfoApple&) = delete;
  ~FileVersionInfoApple() override;

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

 private:
  // Returns a std::u16string value for a property name.
  // Returns the empty string if the property does not exist.
  std::u16string GetString16Value(CFStringRef name);

  NSBundle* __strong bundle_;
};

#endif  // BASE_FILE_VERSION_INFO_APPLE_H_
