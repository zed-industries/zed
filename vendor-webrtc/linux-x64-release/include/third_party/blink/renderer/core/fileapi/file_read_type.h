// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_FILE_READ_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_FILE_READ_TYPE_H_

namespace blink {

enum class FileReadType {
  kReadAsArrayBuffer,
  kReadAsBinaryString,
  kReadAsText,
  kReadAsDataURL
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_FILE_READ_TYPE_H_
