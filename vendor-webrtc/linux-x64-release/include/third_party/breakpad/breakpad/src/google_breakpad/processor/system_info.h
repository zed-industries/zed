// Copyright 2006 Google LLC
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//     * Neither the name of Google LLC nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

// system_info.h: Information about the system that was running a program
// when a crash report was produced.
//
// Author: Mark Mentovai

#ifndef GOOGLE_BREAKPAD_PROCESSOR_SYSTEM_INFO_H__
#define GOOGLE_BREAKPAD_PROCESSOR_SYSTEM_INFO_H__

#include <string>

#include "common/using_std_string.h"

namespace google_breakpad {

struct SystemInfo {
 public:
  SystemInfo() : os(), os_short(), os_version(), cpu(), cpu_info(),
    cpu_count(0), gl_version(), gl_vendor(), gl_renderer() {}

  // Resets the SystemInfo object to its default values.
  void Clear() {
    os.clear();
    os_short.clear();
    os_version.clear();
    cpu.clear();
    cpu_info.clear();
    cpu_count = 0;
    gl_version.clear();
    gl_vendor.clear();
    gl_renderer.clear();
  }

  // A string identifying the operating system, such as "Windows NT",
  // "Mac OS X", or "Linux".  If the information is present in the dump but
  // its value is unknown, this field will contain a numeric value.  If
  // the information is not present in the dump, this field will be empty.
  string os;

  // A short form of the os string, using lowercase letters and no spaces,
  // suitable for use in a filesystem.  Possible values include "windows",
  // "mac", "linux" and "nacl".  Empty if the information is not present
  // in the dump or if the OS given by the dump is unknown.  The values
  // stored in this field should match those used by
  // MinidumpSystemInfo::GetOS.
  string os_short;

  // A string identifying the version of the operating system, such as
  // "5.1.2600 Service Pack 2" or "10.4.8 8L2127".  If the dump does not
  // contain this information, this field will be empty.
  string os_version;

  // A string identifying the basic CPU family, such as "x86" or "ppc".
  // If this information is present in the dump but its value is unknown,
  // this field will contain a numeric value.  If the information is not
  // present in the dump, this field will be empty.  The values stored in
  // this field should match those used by MinidumpSystemInfo::GetCPU.
  string cpu;

  // A string further identifying the specific CPU, such as
  // "GenuineIntel level 6 model 13 stepping 8".  If the information is not
  // present in the dump, or additional identifying information is not
  // defined for the CPU family, this field will be empty.
  string cpu_info;

  // The number of processors in the system.  Will be greater than one for
  // multi-core systems.
  int cpu_count;

  // The GPU information. Currently only populated in microdumps.
  string gl_version;
  string gl_vendor;
  string gl_renderer;
};

}  // namespace google_breakpad

#endif  // GOOGLE_BREAKPAD_PROCESSOR_SYSTEM_INFO_H__
