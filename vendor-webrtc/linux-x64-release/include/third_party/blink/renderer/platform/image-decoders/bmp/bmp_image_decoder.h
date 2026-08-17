// Copyright 2008 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_BMP_BMP_IMAGE_DECODER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_BMP_BMP_IMAGE_DECODER_H_

#include <memory>

#include "third_party/blink/renderer/platform/image-decoders/image_decoder.h"

namespace blink {

class BMPImageReader;
class FastSharedBufferReader;

// This class decodes the BMP image format.
class PLATFORM_EXPORT BMPImageDecoder final : public ImageDecoder {
 public:
  BMPImageDecoder(AlphaOption, ColorBehavior, wtf_size_t max_decoded_bytes);

  ~BMPImageDecoder() override;

  // ImageDecoder:
  String FilenameExtension() const override;
  const AtomicString& MimeType() const override;
  void OnSetData(scoped_refptr<SegmentReader> data) override;
  // CAUTION: SetFailed() deletes |reader_|.  Be careful to avoid
  // accessing deleted memory, especially when calling this from inside
  // BMPImageReader!
  bool SetFailed() override;

 private:
  // ImageDecoder:
  void DecodeSize() override;
  void Decode(wtf_size_t) override;

  // Decodes the image.  If |only_size| is true, stops decoding after
  // calculating the image size. If decoding fails but there is no more
  // data coming, sets the "decode failure" flag.
  void Decode(bool only_size);

  // Decodes the image.  If |only_size| is true, stops decoding after
  // calculating the image size. Returns whether decoding succeeded.
  bool DecodeHelper(bool only_size);

  // Processes the file header at the beginning of the data.  Sets
  // |img_data_offset| based on the header contents. Returns true if the
  // file header could be decoded.
  bool ProcessFileHeader(wtf_size_t& img_data_offset);

  // Uses |fast_reader| and |buffer| to read the file header into |file_header|.
  // Computes |file_type| from the file header.  Returns whether there was
  // sufficient data available to read the header.
  bool GetFileType(const FastSharedBufferReader& fast_reader,
                   char* buffer,
                   const char*& file_header,
                   uint16_t& file_type) const;

  // An index into |data_| representing how much we've already decoded.
  // Note that this only tracks data _this_ class decodes; once the
  // BMPImageReader takes over this will not be updated further.
  wtf_size_t decoded_offset_;

  // The reader used to do most of the BMP decoding.
  std::unique_ptr<BMPImageReader> reader_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_BMP_BMP_IMAGE_DECODER_H_
