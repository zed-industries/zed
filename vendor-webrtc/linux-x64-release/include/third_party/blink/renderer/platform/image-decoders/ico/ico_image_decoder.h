// Copyright 2008 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_ICO_ICO_IMAGE_DECODER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_ICO_ICO_IMAGE_DECODER_H_

#include <memory>

#include "third_party/blink/renderer/platform/image-decoders/bmp/bmp_image_reader.h"
#include "third_party/blink/renderer/platform/image-decoders/fast_shared_buffer_reader.h"

namespace blink {

// This class decodes the ICO and CUR image formats.
class PLATFORM_EXPORT ICOImageDecoder final : public ImageDecoder {
 public:
  ICOImageDecoder(AlphaOption, ColorBehavior, wtf_size_t max_decoded_bytes);
  ICOImageDecoder(const ICOImageDecoder&) = delete;
  ICOImageDecoder& operator=(const ICOImageDecoder&) = delete;
  ~ICOImageDecoder() override;

  // ImageDecoder:
  String FilenameExtension() const override;
  const AtomicString& MimeType() const override;
  void OnSetData(scoped_refptr<SegmentReader>) override;
  gfx::Size Size() const override;
  gfx::Size FrameSizeAtIndex(wtf_size_t) const override;
  bool SetSize(unsigned width, unsigned height) override;
  bool FrameIsReceivedAtIndex(wtf_size_t) const override;
  // CAUTION: SetFailed() deletes all readers and decoders.  Be careful to
  // avoid accessing deleted memory, especially when calling this from
  // inside BMPImageReader!
  bool SetFailed() override;
  bool HotSpot(gfx::Point&) const override;

 private:
  enum ImageType {
    kUnknown,
    BMP,
    PNG,
  };

  enum FileType {
    ICON = 1,
    CURSOR = 2,
  };

  struct IconDirectoryEntry {
    DISALLOW_NEW();
    gfx::Size size_;
    uint16_t bit_count_;
    gfx::Point hot_spot_;
    uint32_t image_offset_;
    uint32_t byte_size_;
  };

  // Returns true if |a| is a preferable icon entry to |b|.
  // Larger sizes, or greater bitdepths at the same size, are preferable.
  static bool CompareEntries(const IconDirectoryEntry& a,
                             const IconDirectoryEntry& b);

  // ImageDecoder:
  void DecodeSize() override;
  wtf_size_t DecodeFrameCount() override;
  void Decode(wtf_size_t index) override;

  // TODO (scroggo): These functions are identical to functions in
  // BMPImageReader. Share code?
  inline uint8_t ReadUint8(wtf_size_t offset) const {
    return fast_reader_.GetOneByte(decoded_offset_ + offset);
  }

  inline uint16_t ReadUint16(int offset) const {
    char buffer[2];
    const char* data =
        fast_reader_.GetConsecutiveData(decoded_offset_ + offset, 2, buffer);
    return BMPImageReader::ReadUint16(data);
  }

  inline uint32_t ReadUint32(int offset) const {
    char buffer[4];
    const char* data =
        fast_reader_.GetConsecutiveData(decoded_offset_ + offset, 4, buffer);
    return BMPImageReader::ReadUint32(data);
  }

  // If the desired PNGImageDecoder exists, gives it the appropriate data.
  void SetDataForPNGDecoderAtIndex(wtf_size_t);

  // Decodes the entry at |index|.  If |only_size| is true, stops decoding
  // after calculating the image size.  If decoding fails but there is no
  // more data coming, sets the "decode failure" flag.
  void Decode(wtf_size_t index, bool only_size);

  // Decodes the directory and directory entries at the beginning of the
  // data, and initializes members.  Returns true if all decoding
  // succeeded.  Once this returns true, all entries' sizes are known.
  bool DecodeDirectory();

  // Decodes the specified entry.
  bool DecodeAtIndex(wtf_size_t);

  // Processes the ICONDIR at the beginning of the data.  Returns true if
  // the directory could be decoded.
  bool ProcessDirectory();

  // Processes the ICONDIRENTRY records after the directory.  Keeps the
  // "best" entry as the one we'll decode.  Returns true if the entries
  // could be decoded.
  bool ProcessDirectoryEntries();

  // Stores the hot-spot for |index| in |hot_spot| and returns true,
  // or returns false if there is none.
  bool HotSpotAtIndex(wtf_size_t index, gfx::Point& hot_spot) const;

  // Reads and returns a directory entry from the current offset into
  // |data|.
  IconDirectoryEntry ReadDirectoryEntry();

  // Determines whether the desired entry is a BMP or PNG.  Returns true
  // if the type could be determined.
  ImageType ImageTypeAtIndex(wtf_size_t);

  FastSharedBufferReader fast_reader_{nullptr};

  // An index into |data_| representing how much we've already decoded.
  // Note that this only tracks data _this_ class decodes; once the
  // BMPImageReader takes over this will not be updated further.
  wtf_size_t decoded_offset_ = 0;

  // Which type of file (ICO/CUR) this is.
  FileType file_type_;

  // The headers for the ICO.
  typedef Vector<IconDirectoryEntry> IconDirectoryEntries;
  IconDirectoryEntries dir_entries_;

  // Count of directory entries is parsed from header before initializing
  // dir_entries_. dir_entries_ is populated only when full header
  // information including directory entries is available.
  wtf_size_t dir_entries_count_ = 0;

  // The image decoders for the various frames.
  typedef Vector<std::unique_ptr<BMPImageReader>> BMPReaders;
  BMPReaders bmp_readers_;
  typedef Vector<std::unique_ptr<ImageDecoder>> PNGDecoders;
  PNGDecoders png_decoders_;

  // Valid only while a BMPImageReader is decoding, this holds the size
  // for the particular entry being decoded.
  gfx::Size frame_size_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_ICO_ICO_IMAGE_DECODER_H_
