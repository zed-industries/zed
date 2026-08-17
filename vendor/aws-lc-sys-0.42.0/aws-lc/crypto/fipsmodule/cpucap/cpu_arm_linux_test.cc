// Copyright (c) 2018, Google Inc.
// SPDX-License-Identifier: ISC

#include "cpu_arm_linux.h"

#include <string.h>

#include <gtest/gtest.h>


TEST(ARMLinuxTest, CPUInfo) {
  struct CPUInfoTest {
    const char *cpuinfo;
    unsigned long hwcap2;
  } kTests[] = {
      // Nexus 4 from https://crbug.com/341598#c43
      {
          "Processor       : ARMv7 Processor rev 2 (v7l)\n"
          "processor       : 0\n"
          "BogoMIPS        : 13.53\n"
          "\n"
          "processor       : 1\n"
          "BogoMIPS        : 13.53\n"
          "\n"
          "processor       : 2\n"
          "BogoMIPS        : 13.53\n"
          "\n"
          "processor       : 3\n"
          "BogoMIPS        : 13.53\n"
          "\n"
          "Features        : swp half thumb fastmult vfp edsp neon vfpv3 tls "
          "vfpv4 \n"
          "CPU implementer : 0x51\n"
          "CPU architecture: 7\n"
          "CPU variant     : 0x0\n"
          "CPU part        : 0x06f\n"
          "CPU revision    : 2\n"
          "\n"
          "Hardware        : QCT APQ8064 MAKO\n"
          "Revision        : 000b\n"
          "Serial          : 0000000000000000\n",
          0,
      },
      // Pixel 2 (truncated slightly)
      {
          "Processor       : AArch64 Processor rev 1 (aarch64)\n"
          "processor       : 0\n"
          "BogoMIPS        : 38.00\n"
          "Features        : fp asimd evtstrm aes pmull sha1 sha2 crc32\n"
          "CPU implementer : 0x51\n"
          "CPU architecture: 8\n"
          "CPU variant     : 0xa\n"
          "CPU part        : 0x801\n"
          "CPU revision    : 4\n"
          "\n"
          "processor       : 1\n"
          "BogoMIPS        : 38.00\n"
          "Features        : fp asimd evtstrm aes pmull sha1 sha2 crc32\n"
          "CPU implementer : 0x51\n"
          "CPU architecture: 8\n"
          "CPU variant     : 0xa\n"
          "CPU part        : 0x801\n"
          "CPU revision    : 4\n"
          "\n"
          "processor       : 2\n"
          "BogoMIPS        : 38.00\n"
          "Features        : fp asimd evtstrm aes pmull sha1 sha2 crc32\n"
          "CPU implementer : 0x51\n"
          "CPU architecture: 8\n"
          "CPU variant     : 0xa\n"
          "CPU part        : 0x801\n"
          "CPU revision    : 4\n"
          "\n"
          "processor       : 3\n"
          "BogoMIPS        : 38.00\n"
          "Features        : fp asimd evtstrm aes pmull sha1 sha2 crc32\n"
          "CPU implementer : 0x51\n"
          "CPU architecture: 8\n"
          "CPU variant     : 0xa\n"
          "CPU part        : 0x801\n"
          "CPU revision    : 4\n"
          // (Extra processors omitted.)
          "\n"
          "Hardware        : Qualcomm Technologies, Inc MSM8998\n",
          HWCAP2_AES | HWCAP2_PMULL | HWCAP2_SHA1 | HWCAP2_SHA2,
      },
      // Garbage should be tolerated.
      {
          "Blah blah blah this is definitely an ARM CPU",
          0,
      },
      // A hypothetical ARMv8 CPU without crc32 (and thus no trailing space
      // after the last crypto entry).
      {
          "Features        : aes pmull sha1 sha2\n"
          "CPU architecture: 8\n",
          HWCAP2_AES | HWCAP2_PMULL | HWCAP2_SHA1 | HWCAP2_SHA2,
      },
      // Various combinations of ARMv8 flags.
      {
          "Features        : aes sha1 sha2\n"
          "CPU architecture: 8\n",
          HWCAP2_AES | HWCAP2_SHA1 | HWCAP2_SHA2,
      },
      {
          "Features        : pmull sha2\n"
          "CPU architecture: 8\n",
          HWCAP2_PMULL | HWCAP2_SHA2,
      },
      {
          "Features        : aes aes   aes not_aes aes aes \n"
          "CPU architecture: 8\n",
          HWCAP2_AES,
      },
      {
          "Features        : \n"
          "CPU architecture: 8\n",
          0,
      },
      {
          "Features        : nothing\n"
          "CPU architecture: 8\n",
          0,
      },
      // If opening /proc/cpuinfo fails, we process the empty string.
      {
          "",
          0,
      },
  };

  for (const auto &t : kTests) {
    SCOPED_TRACE(t.cpuinfo);
    STRING_PIECE sp = {t.cpuinfo, strlen(t.cpuinfo)};
    EXPECT_EQ(t.hwcap2, crypto_get_arm_hwcap2_from_cpuinfo(&sp));
  }
}
