// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_FILE_READER_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_FILE_READER_DATA_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/fileapi/file_read_type.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer/array_buffer_contents.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_array_buffer.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

// FileReaderData is a convenience class to help users convert the data
// received from FileReaderLoader to the different FileReadType types. This
// object works with move semantics, which means it must not be converted
// multiple times to the desired type.
// Example usage:
// DidFinishLoading(FileReaderData contents) {
//   DOMArrayBuffer* buffer = std::move(contents).AsDOMArrayBuffer();
//   ...
// }
class CORE_EXPORT FileReaderData {
 public:
  FileReaderData() = default;
  explicit FileReaderData(ArrayBufferContents raw_data)
      : raw_data_(std::move(raw_data)) {
    CHECK(raw_data_.IsValid());
  }
  FileReaderData(const FileReaderData&) = delete;
  FileReaderData(FileReaderData&& o) = default;
  FileReaderData& operator=(const FileReaderData& o) = delete;
  FileReaderData& operator=(FileReaderData&& o) = default;

  // AsArrayBufferContents directly returns the underlying stored
  // ArrayBufferContents.
  ArrayBufferContents AsArrayBufferContents() &&;
  // AsDOMArrayBuffer converts the underlying ArrayBufferContents to a
  // DOMArrayBuffer.
  DOMArrayBuffer* AsDOMArrayBuffer() &&;
  // AsBinaryString converts the underlying ArrayBufferContents to a binary
  // string representation.
  WTF::String AsBinaryString() &&;
  // AsText converts the underlying ArrayBufferContents to text.
  WTF::String AsText(const WTF::String& encoding) &&;
  // AsDataURL converts the underlying ArrayBufferContents to a data URL
  // representation.
  WTF::String AsDataURL(const WTF::String& data_type) &&;
  // AsString is a convenience method that calls either AsBinaryString, AsText
  // or AsDataURL depending on the passed FileReadType. Depending on that type,
  // encoding or data_type must be set accordingly.
  WTF::String AsString(FileReadType read_type,
                       const WTF::String& encoding,
                       const WTF::String& data_type) &&;

 private:
  ArrayBufferContents raw_data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_FILE_READER_DATA_H_
