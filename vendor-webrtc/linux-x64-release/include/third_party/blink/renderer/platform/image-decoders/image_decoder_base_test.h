// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_IMAGE_DECODER_BASE_TEST_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_IMAGE_DECODER_BASE_TEST_H_

#include <stdint.h>

#include "base/compiler_specific.h"
#include "base/files/file_path.h"
#include "testing/gtest/include/gtest/gtest.h"
#include "third_party/blink/renderer/platform/image-decoders/image_decoder.h"

namespace blink {

// Decodes a handful of image files and compares their MD5 sums to the stored
// sums on disk.  To recalculate the MD5 sums, uncomment the CALCULATE_MD5_SUMS
// #define in the .cc file.
//
// The image files and corresponding MD5 sums live in the directory
// webkit/data/*_decoder (where "*" is the format being tested).
//
// Note: The MD5 sums calculated in this test by little- and big-endian systems
// will differ, since no endianness correction is done.  If we start compiling
// for big endian machines this should be fixed.

class ImageDecoderBaseTest : public testing::Test {
 public:
  explicit ImageDecoderBaseTest(const String& format) : format_(format) {}
  ImageDecoderBaseTest(const ImageDecoderBaseTest&) = delete;
  ImageDecoderBaseTest& operator=(const ImageDecoderBaseTest&) = delete;

  enum class FileSelection {
    kAll,
    kSmaller,
    kBigger,
  };

 protected:
  void SetUp() override;

  // Returns the path the decoded data is saved at.
  base::FilePath GetMD5SumPath(const base::FilePath& path);

  // Returns the vector of image files for testing.
  Vector<base::FilePath> GetImageFiles() const;

  // Returns true if the image is bogus and should not be successfully decoded.
  bool ShouldImageFail(const base::FilePath& path) const;

  // Tests if decoder decodes image at image_path with underlying frame at
  // index desired_frame_index. The md5_sum_path is needed if the test is not
  // asked to generate one, i.e. if #define CALCULATE_MD5_SUMS is not set.
  void TestImageDecoder(const base::FilePath& image_path,
                        const base::FilePath& md5_sum_path,
                        int desired_frame_index) const;

  // Verifies each of the test image files is decoded correctly and matches the
  // expected state. |file_selection| and |threshold| can be used to select
  // files to test based on file size.
  // If just the MD5 sum is wanted, this skips chunking.
  void TestDecoding(FileSelection file_selection, const int64_t threshold);

  void TestDecoding() { TestDecoding(FileSelection::kAll, 0); }

  // Creates decoder.
  virtual std::unique_ptr<ImageDecoder> CreateImageDecoder() const = 0;

  // The format to be decoded, like "bmp" or "ico".
  String format_;

 protected:
  const base::FilePath& data_dir() const { return data_dir_; }

 private:
  // Path to the test files.
  base::FilePath data_dir_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_IMAGE_DECODER_BASE_TEST_H_
