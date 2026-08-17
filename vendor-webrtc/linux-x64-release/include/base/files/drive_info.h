// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FILES_DRIVE_INFO_H_
#define BASE_FILES_DRIVE_INFO_H_

#include <stdint.h>
#include <time.h>

#include <optional>
#include <string>
#include <string_view>

#include "base/base_export.h"
#include "base/files/file_path.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_MAC)
#include <IOKit/IOKitLib.h>
#endif

namespace base {

// Used to hold information about either a drive, or of a combination of a
// partition residing on a drive and the drive itself, depending on how the
// object was constructed. In general, when calling GetFileDriveInfo(), this
// latter case is the one which should be considered. On macOS, whole media can
// be queried by using by calling GetIOObjectDriveInfo() with an `IOObject`
// obtained via IOServiceGetMatchingService() with `kIOMediaWholeKey` set to
// `true`.
//
// Each of these parameters can fail to be retrieved from the OS, and so they
// can each be empty.
//
// If you add more fields to this structure (platform-specific fields are OK),
// make sure to update all functions that use it in
// file_util_{win|posix|mac|ios}.cc, too.
struct BASE_EXPORT DriveInfo {
  DriveInfo();
  DriveInfo(const DriveInfo&) = delete;
  DriveInfo(DriveInfo&&);
  DriveInfo& operator=(const DriveInfo&) = delete;
  DriveInfo& operator=(DriveInfo&&);
  ~DriveInfo();

  // Whether the drive has a seek penalty (i.e. is or is not a spinning disk).
  std::optional<bool> has_seek_penalty;

#if BUILDFLAG(IS_WIN) || BUILDFLAG(IS_MAC) || BUILDFLAG(IS_LINUX) || \
    BUILDFLAG(IS_CHROMEOS)
  // Whether the drive is a "removable" drive.
  //
  // In macOS's IOKit API, a drive is "removable" if "the media is removable
  // from the drive mechanism" (e.g. DVD media), and can be further marked as
  // "ejectable" if it can be "eject[ed] from the drive mechanism under software
  // control" (also e.g. DVD media). If a drive is marked as being "removable"
  // as per IOKit, then `is_removable` is set to true.
  //
  // However, in the Finder, all drives connected via external I/O busses are
  // marked with an ⏏ icon to allow the user to initiate an unmount on the drive
  // in preparation for disconnection. Because the Finder offers that ⏏ action,
  // on macOS, such drives also have `is_removable` set to true.
  //
  // However, on Windows, drives in similar situations are not marked as
  // "ejectable" in Explorer, and thus `is_removable` is set to false in those
  // cases. For Windows, `is_removable` is a strict reflection of the
  // `RemovableMedia` flag in `STORAGE_DEVICE_DESCRIPTOR`.
  std::optional<bool> is_removable;

  // The size of the media, in bytes.
  std::optional<int64_t> size_bytes;
#endif
#if BUILDFLAG(IS_WIN) || BUILDFLAG(IS_MAC)
  // Whether the drive is connected over USB.
  std::optional<bool> is_usb;
#endif
#if BUILDFLAG(IS_MAC)
  // Whether the drive is a CoreStorage volume.
  std::optional<bool> is_core_storage;

  // Whether the drive is an APFS container.
  std::optional<bool> is_apfs;

  // Whether the drive can be written to.
  std::optional<bool> is_writable;

  // The BSD name is the filename at which the drive is found under /dev. For
  // example, the 3rd partition of the 3rd disk is "disk3s3".
  std::optional<std::string> bsd_name;
#endif
};

// Given a path to a file (following symlinks), returns information about the
// drive upon which sits that file. Returns nullopt if the file doesn't exist or
// if there is another error in looking up the drive.
BASE_EXPORT std::optional<DriveInfo> GetFileDriveInfo(
    const FilePath& file_path);

#if BUILDFLAG(IS_MAC)
// Given an IOObject of a drive's media, returns information about that drive.
// Returns nullopt if the IOObject does not conform to kIOMediaClass.
BASE_EXPORT std::optional<DriveInfo> GetIOObjectDriveInfo(
    io_object_t io_object);
#endif

}  // namespace base

#endif  // BASE_FILES_DRIVE_INFO_H_
