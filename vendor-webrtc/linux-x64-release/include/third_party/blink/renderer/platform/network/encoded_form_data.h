/*
 * Copyright (C) 2004, 2006, 2008, 2011 Apple Inc. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB. If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_ENCODED_FORM_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_ENCODED_FORM_DATA_H_

// This file is required via encoded_form_data.typemap. To avoid build
// circularity issues, it should not transitively include anything that is
// generated from a mojom_blink target.
//
// This requires some gymnastics below, to explicitly forward-declare the
// required types without reference to the generator output headers.

#include <optional>

#include "base/time/time.h"
#include "mojo/public/cpp/bindings/struct_traits.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

namespace mojom {
class FetchAPIRequestBodyDataView;
}  // namespace mojom

class BlobDataHandle;
class ResourceRequestBody;
class WrappedDataPipeGetter;

class PLATFORM_EXPORT FormDataElement final {
  DISALLOW_NEW();

 public:
  FormDataElement();
  explicit FormDataElement(const Vector<char>&);
  explicit FormDataElement(Vector<char>&&);
  FormDataElement(
      const String& filename,
      int64_t file_start,
      int64_t file_length,
      const std::optional<base::Time>& expected_file_modification_time);
  explicit FormDataElement(scoped_refptr<BlobDataHandle> handle);
  explicit FormDataElement(scoped_refptr<WrappedDataPipeGetter>);

  FormDataElement(const FormDataElement&);
  FormDataElement(FormDataElement&&);

  ~FormDataElement();

  FormDataElement& operator=(const FormDataElement&);
  FormDataElement& operator=(FormDataElement&&);

  enum Type { kData, kEncodedFile, kEncodedBlob, kDataPipe } type_;
  Vector<char> data_;
  String filename_;
  // Non-null IFF the type is kEncodedBlob.
  scoped_refptr<BlobDataHandle> blob_data_handle_;
  int64_t file_start_;
  int64_t file_length_;
  std::optional<base::Time> expected_file_modification_time_;
  scoped_refptr<WrappedDataPipeGetter> data_pipe_getter_;
};

// Only used as a test helper.
PLATFORM_EXPORT bool operator==(const FormDataElement& a,
                                const FormDataElement& b);

class PLATFORM_EXPORT EncodedFormData : public RefCounted<EncodedFormData> {
  USING_FAST_MALLOC(EncodedFormData);

 public:
  enum EncodingType {
    kFormURLEncoded,    // for application/x-www-form-urlencoded
    kTextPlain,         // for text/plain
    kMultipartFormData  // for multipart/form-data
  };

  static scoped_refptr<EncodedFormData> Create();
  static scoped_refptr<EncodedFormData> Create(base::span<const uint8_t>);
  static scoped_refptr<EncodedFormData> Create(base::span<const char> chars) {
    return Create(base::as_bytes(chars));
  }
  static scoped_refptr<EncodedFormData> Create(SegmentedBuffer&&);
  scoped_refptr<EncodedFormData> Copy() const;
  scoped_refptr<EncodedFormData> DeepCopy() const;
  ~EncodedFormData();

  void AppendData(base::span<const uint8_t> data);
  void AppendData(base::span<const char> data) {
    AppendData(base::as_bytes(data));
  }
  void AppendData(SegmentedBuffer&&);
  void AppendFile(const String& file_path,
                  const std::optional<base::Time>& expected_modification_time);
  void AppendFileRange(
      const String& filename,
      int64_t start,
      int64_t length,
      const std::optional<base::Time>& expected_modification_time);
  void AppendBlob(scoped_refptr<BlobDataHandle> handle);
  void AppendDataPipe(scoped_refptr<WrappedDataPipeGetter> handle);

  void Flatten(Vector<char>&) const;  // omits files
  String FlattenToString() const;     // omits files

  bool IsEmpty() const { return elements_.empty(); }
  const Vector<FormDataElement>& Elements() const { return elements_; }
  Vector<FormDataElement>& MutableElements() { return elements_; }

  const Vector<char>& Boundary() const { return boundary_; }
  void SetBoundary(Vector<char> boundary) { boundary_ = boundary; }

  // Identifies a particular form submission instance.  A value of 0 is used
  // to indicate an unspecified identifier.
  void SetIdentifier(int64_t identifier) { identifier_ = identifier; }
  int64_t Identifier() const { return identifier_; }

  bool ContainsPasswordData() const { return contains_password_data_; }
  void SetContainsPasswordData(bool contains_password_data) {
    contains_password_data_ = contains_password_data;
  }

  static EncodingType ParseEncodingType(const String& type) {
    if (EqualIgnoringASCIICase(type, "text/plain"))
      return kTextPlain;
    if (EqualIgnoringASCIICase(type, "multipart/form-data"))
      return kMultipartFormData;
    return kFormURLEncoded;
  }

  // Size of the elements making up the EncodedFormData.
  uint64_t SizeInBytes() const;

  bool IsSafeToSendToAnotherThread() const;

 private:
  friend struct mojo::StructTraits<mojom::FetchAPIRequestBodyDataView,
                                   ResourceRequestBody>;
  EncodedFormData();
  EncodedFormData(const EncodedFormData&);

  Vector<FormDataElement> elements_;

  int64_t identifier_;
  Vector<char> boundary_;
  bool contains_password_data_;
};

// Only used as a test helper.
inline bool operator==(const EncodedFormData& a, const EncodedFormData& b) {
  return a.Elements() == b.Elements();
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_ENCODED_FORM_DATA_H_
