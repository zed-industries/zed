/* Copyright 2014 Google LLC

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are
met:

 * Redistributions of source code must retain the above copyright
notice, this list of conditions and the following disclaimer.
 * Redistributions in binary form must reproduce the above
copyright notice, this list of conditions and the following disclaimer
in the documentation and/or other materials provided with the
distribution.
 * Neither the name of Google LLC nor the names of its
contributors may be used to endorse or promote products derived from
this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
"AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

#include <mach-o/arch.h>
#include <mach-o/loader.h>
#include <mach/machine.h>

// Go/Cgo does not support #define constants, so turn them into symbols
// that are reachable from Go.

#ifndef CPU_TYPE_ARM64
#define CPU_TYPE_ARM64 (CPU_TYPE_ARM | CPU_ARCH_ABI64)
#endif

#ifndef CPU_SUBTYPE_ARM64_ALL
#define CPU_SUBTYPE_ARM64_ALL 0
#endif

#ifndef CPU_SUBTYPE_ARM64_E
#define CPU_SUBTYPE_ARM64_E 2
#endif

const cpu_type_t kCPU_TYPE_ARM = CPU_TYPE_ARM;
const cpu_type_t kCPU_TYPE_ARM64 = CPU_TYPE_ARM64;

const cpu_subtype_t kCPU_SUBTYPE_ARM64_ALL = CPU_SUBTYPE_ARM64_ALL;
const cpu_subtype_t kCPU_SUBTYPE_ARM64_E = CPU_SUBTYPE_ARM64_E;
const cpu_subtype_t kCPU_SUBTYPE_ARM_V7S = CPU_SUBTYPE_ARM_V7S;

const char* GetNXArchInfoName(cpu_type_t cpu_type, cpu_subtype_t cpu_subtype) {
  const NXArchInfo* arch_info = NXGetArchInfoFromCpuType(cpu_type, cpu_subtype);
  if (!arch_info)
    return 0;
  return arch_info->name;
}

const uint32_t kMachHeaderFtypeDylib = MH_DYLIB;
const uint32_t kMachHeaderFtypeBundle = MH_BUNDLE;
const uint32_t kMachHeaderFtypeExe = MH_EXECUTE;
const uint32_t kMachHeaderFtypeDylinker = MH_DYLINKER;
