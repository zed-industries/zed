// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_I18N_ICU_UTIL_H_
#define BASE_I18N_ICU_UTIL_H_

#include <stdint.h>

#include "base/files/memory_mapped_file.h"
#include "base/i18n/base_i18n_export.h"
#include "build/build_config.h"

#define ICU_UTIL_DATA_FILE 0
#define ICU_UTIL_DATA_STATIC 1

namespace base::i18n {

#if !BUILDFLAG(IS_NACL)
// Call this function to load ICU's data tables for the current process.  This
// function should be called before ICU is used.
BASE_I18N_EXPORT bool InitializeICU();

#if ICU_UTIL_DATA_IMPL == ICU_UTIL_DATA_FILE

// Returns the PlatformFile and Region that was initialized by InitializeICU().
// Use with InitializeICUWithFileDescriptor().
BASE_I18N_EXPORT PlatformFile
GetIcuDataFileHandle(MemoryMappedFile::Region* out_region);

// Loads ICU data file from file descriptor passed by browser process to
// initialize ICU in render processes.
BASE_I18N_EXPORT bool InitializeICUWithFileDescriptor(
    PlatformFile data_fd,
    const MemoryMappedFile::Region& data_region);

// Calls `u_cleanup()` to reset the ICU library, and clears global state,
// notably releasing the mapped ICU data file, and handle.
BASE_I18N_EXPORT void ResetGlobalsForTesting();

#if BUILDFLAG(IS_FUCHSIA)
// Overrides the directory used by ICU for external time zone data.
BASE_I18N_EXPORT void SetIcuTimeZoneDataDirForTesting(const char* dir);
#endif  // BUILDFLAG(IS_FUCHSIA)
#endif  // ICU_UTIL_DATA_IMPL == ICU_UTIL_DATA_FILE

// In a test binary, initialize functions might be called twice.
BASE_I18N_EXPORT void AllowMultipleInitializeCallsForTesting();
#endif  // !BUILDFLAG(IS_NACL)

}  // namespace base::i18n

#endif  // BASE_I18N_ICU_UTIL_H_
