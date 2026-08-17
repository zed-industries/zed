/*
 * Copyright 2020 The Chromium Authors. All Rights Reserved.
 *
 * This software is provided 'as-is', without any express or implied
 * warranty.  In no event will the authors be held liable for any damages
 * arising from the use of this software.
 *
 * Permission is granted to anyone to use this software for any purpose,
 * including commercial applications, and to alter it and redistribute it
 * freely, subject to the following restrictions:
 *
 * 1. The origin of this software must not be misrepresented; you must not
 *    claim that you wrote the original software. If you use this software
 *    in a product, an acknowledgment in the product documentation would be
 *    appreciated but is not required.
 * 2. Altered source versions must be plainly marked as such, and must not be
 *    misrepresented as being the original software.
 * 3. This notice may not be removed or altered from any source distribution.
 */

#ifndef THIRD_PARTY_LIBJPEG_TURBO_GTEST_UTILS_H_
#define THIRD_PARTY_LIBJPEG_TURBO_GTEST_UTILS_H_

#include "base/files/file.h"
#include "base/files/file_util.h"
#include "base/hash/md5.h"
#include "base/path_service.h"
#include "gtest-utils.h"

#include <gtest/gtest.h>
#include <string>

// Returns the absolute path of the test output directory as a string.
// On Android this path is /sdcard; on all other platforms it is the current
// directory.
#if defined(OS_WIN)
std::wstring GetTargetDirectory();
#else
std::string GetTargetDirectory();
#endif

// Files used as input for libjpeg-turbo unit tests are stored in
// <chromium>/src/third_party/libjpeg_turbo/testimages.
// Given a test file |filename|, this function will set |path| to
// <chromium>/src/third_party/libjpeg_turbo/testimages/filename
// If the file does not exit an assertion will fail.
void GetTestFilePath(base::FilePath* path,
                     const std::string filename);

// Returns true if the MD5 sum of the file at the given |path| matches that of
// the |expected_md5|, false otherwise.
bool CompareFileAndMD5(const base::FilePath& path,
                       const std::string expected_md5);

#endif // THIRD_PARTY_LIBJPEG_TURBO_GTEST_UTILS_H_
