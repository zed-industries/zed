// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_BASE_PATHS_WIN_H_
#define BASE_BASE_PATHS_WIN_H_

// This file declares windows-specific path keys for the base module.
// These can be used with the PathService to access various special
// directories and files.

namespace base {

enum {
  PATH_WIN_START = 100,

  DIR_WINDOWS,  // Windows directory, usually "c:\windows"
  DIR_SYSTEM,   // Usually c:\windows\system32"
  //                         32-bit     32-bit on 64-bit   64-bit on 64-bit
  // DIR_PROGRAM_FILES         1               2                  1
  // DIR_PROGRAM_FILESX86      1               2                  2
  // DIR_PROGRAM_FILES6432     1               1                  1
  // 1 - C:\Program Files   2 - C:\Program Files (x86)
  DIR_PROGRAM_FILES,      // See table above.
  DIR_PROGRAM_FILESX86,   // See table above.
  DIR_PROGRAM_FILES6432,  // See table above.

  DIR_IE_INTERNET_CACHE,       // Temporary Internet Files directory.
  DIR_COMMON_START_MENU,       // Usually "C:\ProgramData\Microsoft\Windows\
                               // Start Menu\Programs"
  DIR_START_MENU,              // Usually "C:\Users\<user>\AppData\Roaming\
                               // Microsoft\Windows\Start Menu\Programs"
  DIR_COMMON_STARTUP,          // Usually "C:\ProgramData\Microsoft\Windows\
                               // Start Menu\Programs\Startup"
  DIR_USER_STARTUP,            // Usually "C:\Users\<user>\AppData\Roaming\
                               // Microsoft\Windows\Start Menu\Programs\Startup"
  DIR_ROAMING_APP_DATA,        // Roaming Application Data directory under the
                               // user profile.
                               // Usually "C:\Users\<user>\AppData\Roaming".
                               // Data for specific applications is stored in
                               // subdirectories.
  DIR_LOCAL_APP_DATA,          // Local Application Data directory under the
                               // user profile.
                               // Usually "C:\Users\<user>\AppData\Local".
                               // Data for specific applications is stored in
                               // subdirectories.
  DIR_COMMON_APP_DATA,         // Usually "C:\ProgramData". Data for specific
                               // applications is stored in subdirectories.
  DIR_APP_SHORTCUTS,           // Where tiles on the start screen are stored,
                               // only for Windows 8. Maps to "Local\AppData\
                               // Microsoft\Windows\Application Shortcuts\".
  DIR_COMMON_DESKTOP,          // Directory for the common desktop (visible
                               // on all user's Desktop).
  DIR_USER_QUICK_LAUNCH,       // Directory for the quick launch shortcuts.
  DIR_TASKBAR_PINS,            // Directory for the shortcuts pinned to taskbar.
  DIR_IMPLICIT_APP_SHORTCUTS,  // The implicit user pinned shortcut directory.
  DIR_WINDOWS_FONTS,           // Usually C:\Windows\Fonts.
  DIR_SYSTEM_TEMP,             // %SystemRoot%\SystemTemp or %ProgramFiles%;
                               // only for admin processes.
  DIR_ONE_DRIVE,               // The synced personal OneDrive directory.
  PATH_WIN_END
};

}  // namespace base

#endif  // BASE_BASE_PATHS_WIN_H_
