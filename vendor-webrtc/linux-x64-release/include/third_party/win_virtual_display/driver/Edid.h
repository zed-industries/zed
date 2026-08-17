// Copyright (c) Microsoft Corporation

#ifndef THIRD_PARTY_WIN_VIRTUAL_DISPLAY_DRIVER_EDID_H_
#define THIRD_PARTY_WIN_VIRTUAL_DISPLAY_DRIVER_EDID_H_

#include <algorithm>
#include <array>

namespace display::test {

// Bytes 38-53 of an EDID (v1.4) blob contains timing information as a list of 8
// 2-byte structures. The following structure represents a single record which
// consists of: X resolution, aspect ratio and vertical frequency. See:
// https://en.wikipedia.org/wiki/Extended_Display_Identification_Data
class EdidTimingEntry {
 public:
  // Sets the mode of this timing entry. Returns true on success, false
  // otherwise. Based on EDID restrictions the following constraints exist:
  //  `width` should be between 256 and 2288.
  //  `height` should maintain an aspect ratio (width:height) of 16:10, 4:3,
  //  5:4, 16:9.
  //  `freq` should be between 60 and 123.
  bool SetMode(unsigned short width,
               unsigned short height,
               unsigned short freq);
  // Get the width (in pixels) of the currently set mode.
  int GetWidth();
  // Get the height (in pixels) of the currently set mode.
  int GetHeight();
  // Get the vertical frequency (in pixels) of the currently set mode.
  int GetVerticalFrequency();

 private:
  unsigned char x_pixels;
  unsigned char aspect_ratio : 2;
  unsigned char vertical_frequency : 6;
};

// Wrapper for a 128 byte EDID block with some basic support for extracting
// (Standard) timing information structures.
class Edid {
 public:
  // EDID block size (based on version 1.4).
  static constexpr size_t kBlockSize = 128;

  explicit Edid(const Edid& other) = delete;
  // Copying an existing blob of data.
  explicit Edid(unsigned char edidData[kBlockSize]) {
    std::copy_n(edidData, kBlockSize, edidBlock.begin());
  }

  const std::array<unsigned char, kBlockSize>& getEdidBlock() const {
    return edidBlock;
  }

  // Return a specified timing entry (Standard timing information; EDID 1.4).
  // There are 8 entries. `entry` should be in in the range [0,8).
  EdidTimingEntry* GetTimingEntry(int entry);

  // Set the 16-bit manufacturer product code (EDID 1.4).
  void SetProductCode(uint16_t code);

  // Set the 32-bit serial number (EDID 1.4).
  void SetSerialNumber(uint32_t serial);

  // Updates the checksum byte to be valid.
  void UpdateChecksum();

 private:
  std::array<unsigned char, kBlockSize> edidBlock;
};

}  // namespace display::test

#endif  // THIRD_PARTY_WIN_VIRTUAL_DISPLAY_DRIVER_EDID_H_
