// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/rsa.h>

#include <stdlib.h>
#include <string.h>

#include <gtest/gtest.h>

#include <openssl/bn.h>
#include <openssl/bio.h>
#include <openssl/bytestring.h>
#include <openssl/crypto.h>
#include <openssl/evp.h>
#include <openssl/digest.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/nid.h>

#include "../fipsmodule/bn/internal.h"
#include "../fipsmodule/rsa/internal.h"
#include "../internal.h"
#include "../test/test_util.h"

#if defined(OPENSSL_THREADS)
#include <thread>
#include <vector>
#endif


// kPlaintext is a sample plaintext.
static const uint8_t kPlaintext[] = "\x54\x85\x9b\x34\x2c\x49\xea\x2a";
static const size_t kPlaintextLen = sizeof(kPlaintext) - 1;

// kKey1 is a DER-encoded RSAPrivateKey.
static const uint8_t kKey1[] =
    "\x30\x82\x01\x38\x02\x01\x00\x02\x41\x00\xaa\x36\xab\xce\x88\xac\xfd\xff"
    "\x55\x52\x3c\x7f\xc4\x52\x3f\x90\xef\xa0\x0d\xf3\x77\x4a\x25\x9f\x2e\x62"
    "\xb4\xc5\xd9\x9c\xb5\xad\xb3\x00\xa0\x28\x5e\x53\x01\x93\x0e\x0c\x70\xfb"
    "\x68\x76\x93\x9c\xe6\x16\xce\x62\x4a\x11\xe0\x08\x6d\x34\x1e\xbc\xac\xa0"
    "\xa1\xf5\x02\x01\x11\x02\x40\x0a\x03\x37\x48\x62\x64\x87\x69\x5f\x5f\x30"
    "\xbc\x38\xb9\x8b\x44\xc2\xcd\x2d\xff\x43\x40\x98\xcd\x20\xd8\xa1\x38\xd0"
    "\x90\xbf\x64\x79\x7c\x3f\xa7\xa2\xcd\xcb\x3c\xd1\xe0\xbd\xba\x26\x54\xb4"
    "\xf9\xdf\x8e\x8a\xe5\x9d\x73\x3d\x9f\x33\xb3\x01\x62\x4a\xfd\x1d\x51\x02"
    "\x21\x00\xd8\x40\xb4\x16\x66\xb4\x2e\x92\xea\x0d\xa3\xb4\x32\x04\xb5\xcf"
    "\xce\x33\x52\x52\x4d\x04\x16\xa5\xa4\x41\xe7\x00\xaf\x46\x12\x0d\x02\x21"
    "\x00\xc9\x7f\xb1\xf0\x27\xf4\x53\xf6\x34\x12\x33\xea\xaa\xd1\xd9\x35\x3f"
    "\x6c\x42\xd0\x88\x66\xb1\xd0\x5a\x0f\x20\x35\x02\x8b\x9d\x89\x02\x20\x59"
    "\x0b\x95\x72\xa2\xc2\xa9\xc4\x06\x05\x9d\xc2\xab\x2f\x1d\xaf\xeb\x7e\x8b"
    "\x4f\x10\xa7\x54\x9e\x8e\xed\xf5\xb4\xfc\xe0\x9e\x05\x02\x21\x00\x8e\x3c"
    "\x05\x21\xfe\x15\xe0\xea\x06\xa3\x6f\xf0\xf1\x0c\x99\x52\xc3\x5b\x7a\x75"
    "\x14\xfd\x32\x38\xb8\x0a\xad\x52\x98\x62\x8d\x51\x02\x20\x36\x3f\xf7\x18"
    "\x9d\xa8\xe9\x0b\x1d\x34\x1f\x71\xd0\x9b\x76\xa8\xa9\x43\xe1\x1d\x10\xb2"
    "\x4d\x24\x9f\x2d\xea\xfe\xf8\x0c\x18\x26";

// kFIPSKey is a DER-encoded RSAPrivateKey that is FIPS-compliant.
static const uint8_t kFIPSKey[] =
    "\x30\x82\x02\x5c\x02\x01\x00\x02\x81\x81\x00\xa1\x71\x90\x77\x86\x8a\xc7"
    "\xb8\xfc\x2a\x45\x82\x6d\xee\xeb\x35\x3a\x18\x3f\xb6\xb0\x1e\xb1\xd3\x09"
    "\x6b\x05\x4d\xec\x1c\x37\x6f\x09\x31\x32\xda\x21\x8a\x49\x0e\x16\x28\xed"
    "\x9a\x30\xf3\x14\x53\xfd\x5b\xb0\xf6\x4a\x5d\x52\xe1\xda\xe1\x40\x6e\x65"
    "\xbf\xca\x45\xd9\x62\x96\x4a\x1e\x11\xc4\x61\x83\x1f\x58\x8d\x5e\xd0\x12"
    "\xaf\xa5\xec\x9b\x97\x2f\x6c\xb2\x82\x4a\x73\xd0\xd3\x9a\xc9\x69\x6b\x24"
    "\x3c\x82\x6f\xee\x4d\x0c\x7e\xdf\xd7\xae\xea\x3a\xeb\x04\x27\x8d\x43\x81"
    "\x59\xa7\x90\x56\xc1\x69\x42\xb3\xaf\x1c\x8d\x4e\xbf\x02\x03\x01\x00\x01"
    "\x02\x81\x80\x60\x82\xcd\x44\x46\xcf\xeb\xf9\x6f\xf5\xad\x3b\xfd\x90\x18"
    "\x57\xe7\x74\xdb\x91\xd0\xd3\x68\xa6\xaa\x38\xaa\x21\x1d\x06\xf9\x34\x8d"
    "\xa0\x35\xb0\x24\xe0\xd0\x2f\x75\x9b\xdd\xfe\x91\x48\x9f\x5c\x5e\x57\x54"
    "\x00\xc8\x0f\xe6\x1e\x52\x84\xd9\xc9\xa5\x55\xf4\x0a\xbe\x88\x46\x7a\xfb"
    "\x18\x37\x8e\xe6\x6e\xa2\x5f\x80\x48\x34\x3f\x5c\xbe\x0e\x1e\xe8\x2f\x50"
    "\xba\x14\x96\x3c\xea\xfb\xd2\x49\x33\xdc\x12\xb8\xa7\x8a\xb5\x27\xf9\x00"
    "\x4b\xf5\xd2\x2a\xd0\x2c\x1d\x9b\xd5\x6c\x3e\x4b\xb9\x7e\x39\xf7\x3e\x39"
    "\xc9\x47\x5e\xbe\x91\x02\x41\x00\xcd\x33\xcf\x37\x01\xd7\x59\xcc\xbe\xa0"
    "\x1c\xb9\xf5\xe7\x44\x9f\x62\x91\xa7\xa7\x7b\x0c\x52\xcd\x7e\xe6\x31\x11"
    "\x8b\xd8\x2c\x8a\x63\xe1\x07\xc9\xcb\xce\x01\x45\x63\xf5\x5d\x44\xfb\xeb"
    "\x8d\x74\x16\x20\x7d\x3b\xb4\xa1\x61\xb0\xa8\x29\x51\xc9\xef\xb6\xa1\xd5"
    "\x02\x41\x00\xc9\x68\xa6\xd3\x88\xd5\x49\x9d\x6b\x44\x96\xfd\xbf\x66\x27"
    "\xb4\x1f\x90\x76\x86\x2f\xe2\xce\x20\x5d\xee\x9b\xeb\xc4\xb4\x62\x47\x79"
    "\x99\xb1\x99\xbc\xa2\xa6\xb6\x96\x64\xd5\x77\x9b\x45\xd4\xf0\x99\xb5\x9e"
    "\x61\x4d\xf5\x12\xdd\x84\x14\xaf\x1e\xdd\x83\x24\x43\x02\x40\x60\x29\x7f"
    "\x59\xcf\xcb\x13\x92\x17\x63\x01\x13\x44\x61\x74\x8f\x1c\xaa\x15\x5f\x2f"
    "\x12\xbf\x5a\xfd\xb4\xf2\x19\xbe\xe7\x37\x38\x43\x46\x19\x58\x3f\xe1\xf2"
    "\x46\x8a\x69\x59\xa4\x12\x4a\x78\xa7\x86\x17\x03\x99\x0f\x34\xf1\x8a\xcf"
    "\xc3\x4d\x48\xcc\xc5\x51\x61\x02\x41\x00\xc2\x12\xb3\x5d\xf5\xe5\xff\xcf"
    "\x4e\x43\x83\x72\xf2\xf1\x4e\xa4\xc4\x1d\x81\xf7\xff\x40\x7e\xfa\xb5\x48"
    "\x6c\xba\x1c\x8a\xec\x80\x8e\xed\xc8\x32\xa9\x8f\xd9\x30\xeb\x6e\x32\x3b"
    "\xd4\x44\xcf\xd1\x1f\x6b\xe0\x37\x46\xd5\x35\xde\x79\x9d\x2c\xb9\x83\x1d"
    "\x10\xdd\x02\x40\x0f\x14\x95\x96\xa0\xe2\x6c\xd4\x88\xa7\x0b\x82\x14\x10"
    "\xad\x26\x0d\xe4\xa1\x5e\x01\x3d\x21\xd2\xfb\x0e\xf9\x58\xa5\xca\x1e\x21"
    "\xb3\xf5\x9a\x6c\x3d\x5a\x72\xb1\x2d\xfe\xac\x09\x4f\xdd\xe5\x44\xd1\x4e"
    "\xf8\x59\x85\x3a\x65\xe2\xcd\xbc\x27\x1d\x9b\x48\x9f\xb9";

static const uint8_t kFIPSPublicKey[] =
    "\x30\x81\x89\x02\x81\x81\x00\xa1\x71\x90\x77\x86\x8a\xc7\xb8\xfc\x2a\x45"
    "\x82\x6d\xee\xeb\x35\x3a\x18\x3f\xb6\xb0\x1e\xb1\xd3\x09\x6b\x05\x4d\xec"
    "\x1c\x37\x6f\x09\x31\x32\xda\x21\x8a\x49\x0e\x16\x28\xed\x9a\x30\xf3\x14"
    "\x53\xfd\x5b\xb0\xf6\x4a\x5d\x52\xe1\xda\xe1\x40\x6e\x65\xbf\xca\x45\xd9"
    "\x62\x96\x4a\x1e\x11\xc4\x61\x83\x1f\x58\x8d\x5e\xd0\x12\xaf\xa5\xec\x9b"
    "\x97\x2f\x6c\xb2\x82\x4a\x73\xd0\xd3\x9a\xc9\x69\x6b\x24\x3c\x82\x6f\xee"
    "\x4d\x0c\x7e\xdf\xd7\xae\xea\x3a\xeb\x04\x27\x8d\x43\x81\x59\xa7\x90\x56"
    "\xc1\x69\x42\xb3\xaf\x1c\x8d\x4e\xbf\x02\x03\x01\x00\x01";

// kOAEPCiphertext1 is a sample encryption of |kPlaintext| with |kKey1| using
// RSA OAEP.
static const uint8_t kOAEPCiphertext1[] =
    "\x1b\x8f\x05\xf9\xca\x1a\x79\x52\x6e\x53\xf3\xcc\x51\x4f\xdb\x89\x2b\xfb"
    "\x91\x93\x23\x1e\x78\xb9\x92\xe6\x8d\x50\xa4\x80\xcb\x52\x33\x89\x5c\x74"
    "\x95\x8d\x5d\x02\xab\x8c\x0f\xd0\x40\xeb\x58\x44\xb0\x05\xc3\x9e\xd8\x27"
    "\x4a\x9d\xbf\xa8\x06\x71\x40\x94\x39\xd2";

// kKey2 is a DER-encoded RSAPrivateKey.
static const uint8_t kKey2[] =
    "\x30\x81\xfb\x02\x01\x00\x02\x33\x00\xa3\x07\x9a\x90\xdf\x0d\xfd\x72\xac"
    "\x09\x0c\xcc\x2a\x78\xb8\x74\x13\x13\x3e\x40\x75\x9c\x98\xfa\xf8\x20\x4f"
    "\x35\x8a\x0b\x26\x3c\x67\x70\xe7\x83\xa9\x3b\x69\x71\xb7\x37\x79\xd2\x71"
    "\x7b\xe8\x34\x77\xcf\x02\x01\x03\x02\x32\x6c\xaf\xbc\x60\x94\xb3\xfe\x4c"
    "\x72\xb0\xb3\x32\xc6\xfb\x25\xa2\xb7\x62\x29\x80\x4e\x68\x65\xfc\xa4\x5a"
    "\x74\xdf\x0f\x8f\xb8\x41\x3b\x52\xc0\xd0\xe5\x3d\x9b\x59\x0f\xf1\x9b\xe7"
    "\x9f\x49\xdd\x21\xe5\xeb\x02\x1a\x00\xcf\x20\x35\x02\x8b\x9d\x86\x98\x40"
    "\xb4\x16\x66\xb4\x2e\x92\xea\x0d\xa3\xb4\x32\x04\xb5\xcf\xce\x91\x02\x1a"
    "\x00\xc9\x7f\xb1\xf0\x27\xf4\x53\xf6\x34\x12\x33\xea\xaa\xd1\xd9\x35\x3f"
    "\x6c\x42\xd0\x88\x66\xb1\xd0\x5f\x02\x1a\x00\x8a\x15\x78\xac\x5d\x13\xaf"
    "\x10\x2b\x22\xb9\x99\xcd\x74\x61\xf1\x5e\x6d\x22\xcc\x03\x23\xdf\xdf\x0b"
    "\x02\x1a\x00\x86\x55\x21\x4a\xc5\x4d\x8d\x4e\xcd\x61\x77\xf1\xc7\x36\x90"
    "\xce\x2a\x48\x2c\x8b\x05\x99\xcb\xe0\x3f\x02\x1a\x00\x83\xef\xef\xb8\xa9"
    "\xa4\x0d\x1d\xb6\xed\x98\xad\x84\xed\x13\x35\xdc\xc1\x08\xf3\x22\xd0\x57"
    "\xcf\x8d";

// kOAEPCiphertext2 is a sample encryption of |kPlaintext| with |kKey2| using
// RSA OAEP.
static const uint8_t kOAEPCiphertext2[] =
    "\x14\xbd\xdd\x28\xc9\x83\x35\x19\x23\x80\xe8\xe5\x49\xb1\x58\x2a\x8b\x40"
    "\xb4\x48\x6d\x03\xa6\xa5\x31\x1f\x1f\xd5\xf0\xa1\x80\xe4\x17\x53\x03\x29"
    "\xa9\x34\x90\x74\xb1\x52\x13\x54\x29\x08\x24\x52\x62\x51";

// kKey3 is a DER-encoded RSAPrivateKey.
static const uint8_t kKey3[] =
    "\x30\x82\x02\x5b\x02\x01\x00\x02\x81\x81\x00\xbb\xf8\x2f\x09\x06\x82\xce"
    "\x9c\x23\x38\xac\x2b\x9d\xa8\x71\xf7\x36\x8d\x07\xee\xd4\x10\x43\xa4\x40"
    "\xd6\xb6\xf0\x74\x54\xf5\x1f\xb8\xdf\xba\xaf\x03\x5c\x02\xab\x61\xea\x48"
    "\xce\xeb\x6f\xcd\x48\x76\xed\x52\x0d\x60\xe1\xec\x46\x19\x71\x9d\x8a\x5b"
    "\x8b\x80\x7f\xaf\xb8\xe0\xa3\xdf\xc7\x37\x72\x3e\xe6\xb4\xb7\xd9\x3a\x25"
    "\x84\xee\x6a\x64\x9d\x06\x09\x53\x74\x88\x34\xb2\x45\x45\x98\x39\x4e\xe0"
    "\xaa\xb1\x2d\x7b\x61\xa5\x1f\x52\x7a\x9a\x41\xf6\xc1\x68\x7f\xe2\x53\x72"
    "\x98\xca\x2a\x8f\x59\x46\xf8\xe5\xfd\x09\x1d\xbd\xcb\x02\x01\x11\x02\x81"
    "\x81\x00\xa5\xda\xfc\x53\x41\xfa\xf2\x89\xc4\xb9\x88\xdb\x30\xc1\xcd\xf8"
    "\x3f\x31\x25\x1e\x06\x68\xb4\x27\x84\x81\x38\x01\x57\x96\x41\xb2\x94\x10"
    "\xb3\xc7\x99\x8d\x6b\xc4\x65\x74\x5e\x5c\x39\x26\x69\xd6\x87\x0d\xa2\xc0"
    "\x82\xa9\x39\xe3\x7f\xdc\xb8\x2e\xc9\x3e\xda\xc9\x7f\xf3\xad\x59\x50\xac"
    "\xcf\xbc\x11\x1c\x76\xf1\xa9\x52\x94\x44\xe5\x6a\xaf\x68\xc5\x6c\x09\x2c"
    "\xd3\x8d\xc3\xbe\xf5\xd2\x0a\x93\x99\x26\xed\x4f\x74\xa1\x3e\xdd\xfb\xe1"
    "\xa1\xce\xcc\x48\x94\xaf\x94\x28\xc2\xb7\xb8\x88\x3f\xe4\x46\x3a\x4b\xc8"
    "\x5b\x1c\xb3\xc1\x02\x41\x00\xee\xcf\xae\x81\xb1\xb9\xb3\xc9\x08\x81\x0b"
    "\x10\xa1\xb5\x60\x01\x99\xeb\x9f\x44\xae\xf4\xfd\xa4\x93\xb8\x1a\x9e\x3d"
    "\x84\xf6\x32\x12\x4e\xf0\x23\x6e\x5d\x1e\x3b\x7e\x28\xfa\xe7\xaa\x04\x0a"
    "\x2d\x5b\x25\x21\x76\x45\x9d\x1f\x39\x75\x41\xba\x2a\x58\xfb\x65\x99\x02"
    "\x41\x00\xc9\x7f\xb1\xf0\x27\xf4\x53\xf6\x34\x12\x33\xea\xaa\xd1\xd9\x35"
    "\x3f\x6c\x42\xd0\x88\x66\xb1\xd0\x5a\x0f\x20\x35\x02\x8b\x9d\x86\x98\x40"
    "\xb4\x16\x66\xb4\x2e\x92\xea\x0d\xa3\xb4\x32\x04\xb5\xcf\xce\x33\x52\x52"
    "\x4d\x04\x16\xa5\xa4\x41\xe7\x00\xaf\x46\x15\x03\x02\x40\x54\x49\x4c\xa6"
    "\x3e\xba\x03\x37\xe4\xe2\x40\x23\xfc\xd6\x9a\x5a\xeb\x07\xdd\xdc\x01\x83"
    "\xa4\xd0\xac\x9b\x54\xb0\x51\xf2\xb1\x3e\xd9\x49\x09\x75\xea\xb7\x74\x14"
    "\xff\x59\xc1\xf7\x69\x2e\x9a\x2e\x20\x2b\x38\xfc\x91\x0a\x47\x41\x74\xad"
    "\xc9\x3c\x1f\x67\xc9\x81\x02\x40\x47\x1e\x02\x90\xff\x0a\xf0\x75\x03\x51"
    "\xb7\xf8\x78\x86\x4c\xa9\x61\xad\xbd\x3a\x8a\x7e\x99\x1c\x5c\x05\x56\xa9"
    "\x4c\x31\x46\xa7\xf9\x80\x3f\x8f\x6f\x8a\xe3\x42\xe9\x31\xfd\x8a\xe4\x7a"
    "\x22\x0d\x1b\x99\xa4\x95\x84\x98\x07\xfe\x39\xf9\x24\x5a\x98\x36\xda\x3d"
    "\x02\x41\x00\xb0\x6c\x4f\xda\xbb\x63\x01\x19\x8d\x26\x5b\xdb\xae\x94\x23"
    "\xb3\x80\xf2\x71\xf7\x34\x53\x88\x50\x93\x07\x7f\xcd\x39\xe2\x11\x9f\xc9"
    "\x86\x32\x15\x4f\x58\x83\xb1\x67\xa9\x67\xbf\x40\x2b\x4e\x9e\x2e\x0f\x96"
    "\x56\xe6\x98\xea\x36\x66\xed\xfb\x25\x79\x80\x39\xf7";

// kOAEPCiphertext3 is a sample encryption of |kPlaintext| with |kKey3| using
// RSA OAEP.
static const uint8_t kOAEPCiphertext3[] =
    "\xb8\x24\x6b\x56\xa6\xed\x58\x81\xae\xb5\x85\xd9\xa2\x5b\x2a\xd7\x90\xc4"
    "\x17\xe0\x80\x68\x1b\xf1\xac\x2b\xc3\xde\xb6\x9d\x8b\xce\xf0\xc4\x36\x6f"
    "\xec\x40\x0a\xf0\x52\xa7\x2e\x9b\x0e\xff\xb5\xb3\xf2\xf1\x92\xdb\xea\xca"
    "\x03\xc1\x27\x40\x05\x71\x13\xbf\x1f\x06\x69\xac\x22\xe9\xf3\xa7\x85\x2e"
    "\x3c\x15\xd9\x13\xca\xb0\xb8\x86\x3a\x95\xc9\x92\x94\xce\x86\x74\x21\x49"
    "\x54\x61\x03\x46\xf4\xd4\x74\xb2\x6f\x7c\x48\xb4\x2e\xe6\x8e\x1f\x57\x2a"
    "\x1f\xc4\x02\x6a\xc4\x56\xb4\xf5\x9f\x7b\x62\x1e\xa1\xb9\xd8\x8f\x64\x20"
    "\x2f\xb1";

static const uint8_t kTwoPrimeKey[] =
    "\x30\x82\x04\xa1\x02\x01\x00\x02\x82\x01\x01\x00\x93\x3a\x4f\xc9\x6a\x0a"
    "\x6b\x28\x04\xfa\xb7\x05\x56\xdf\xa0\xaa\x4f\xaa\xab\x94\xa0\xa9\x25\xef"
    "\xc5\x96\xd2\xd4\x66\x16\x62\x2c\x13\x7b\x91\xd0\x36\x0a\x10\x11\x6d\x7a"
    "\x91\xb6\xe4\x74\x57\xc1\x3d\x7a\xbe\x24\x05\x3a\x04\x0b\x73\x91\x53\xb1"
    "\x74\x10\xe1\x87\xdc\x91\x28\x9c\x1e\xe5\xf2\xb9\xfc\xa2\x48\x34\xb6\x78"
    "\xed\x6d\x95\xfb\xf2\xc0\x4e\x1c\xa4\x15\x00\x3c\x8a\x68\x2b\xd6\xce\xd5"
    "\xb3\x9f\x66\x02\xa7\x0d\x08\xa3\x23\x9b\xe5\x36\x96\x13\x22\xf9\x69\xa6"
    "\x87\x88\x9b\x85\x3f\x83\x9c\xab\x1a\x1b\x6d\x8d\x16\xf4\x5e\xbd\xee\x4b"
    "\x59\x56\xf8\x9d\x58\xcd\xd2\x83\x85\x59\x43\x84\x63\x4f\xe6\x1a\x86\x66"
    "\x0d\xb5\xa0\x87\x89\xb6\x13\x82\x43\xda\x34\x92\x3b\x68\xc4\x95\x71\x2f"
    "\x15\xc2\xe0\x43\x67\x3c\x08\x00\x36\x10\xc3\xb4\x46\x4c\x4e\x6e\xf5\x44"
    "\xa9\x04\x44\x9d\xce\xc7\x05\x79\xee\x11\xcf\xaf\x2c\xd7\x9a\x32\xd3\xa5"
    "\x30\xd4\x3a\x78\x43\x37\x74\x22\x90\x24\x04\x11\xd7\x95\x08\x52\xa4\x71"
    "\x41\x68\x94\xb0\xa0\xc3\xec\x4e\xd2\xc4\x30\x71\x98\x64\x9c\xe3\x7c\x76"
    "\xef\x33\xa3\x2b\xb1\x87\x63\xd2\x5c\x09\xfc\x90\x2d\x92\xf4\x57\x02\x01"
    "\x03\x02\x82\x01\x00\x62\x26\xdf\xdb\x9c\x06\xf2\x1a\xad\xfc\x7a\x03\x8f"
    "\x3f\xc0\x71\x8a\x71\xc7\xb8\x6b\x1b\x6e\x9f\xd9\x0f\x37\x38\x44\x0e\xec"
    "\x1d\x62\x52\x61\x35\x79\x5c\x0a\xb6\x48\xfc\x61\x24\x98\x4d\x8f\xd6\x28"
    "\xfc\x7e\xc2\xae\x26\xad\x5c\xf7\xb6\x37\xcb\xa2\xb5\xeb\xaf\xe8\x60\xc5"
    "\xbd\x69\xee\xa1\xd1\x53\x16\xda\xcd\xce\xfb\x48\xf3\xb9\x52\xa1\xd5\x89"
    "\x68\x6d\x63\x55\x7d\xb1\x9a\xc7\xe4\x89\xe3\xcd\x14\xee\xac\x6f\x5e\x05"
    "\xc2\x17\xbd\x43\x79\xb9\x62\x17\x50\xf1\x19\xaf\xb0\x67\xae\x2a\x57\xbd"
    "\xc7\x66\xbc\xf3\xb3\x64\xa1\xe3\x16\x74\x9e\xea\x02\x5c\xab\x94\xd8\x97"
    "\x02\x42\x0c\x2c\xba\x54\xb9\xaf\xe0\x45\x93\xad\x7f\xb3\x10\x6a\x96\x50"
    "\x4b\xaf\xcf\xc8\x27\x62\x2d\x83\xe9\x26\xc6\x94\xc1\xef\x5c\x8e\x06\x42"
    "\x53\xe5\x56\xaf\xc2\x99\x01\xaa\x9a\x71\xbc\xe8\x21\x33\x2a\x2d\xa3\x36"
    "\xac\x1b\x86\x19\xf8\xcd\x1f\x80\xa4\x26\x98\xb8\x9f\x62\x62\xd5\x1a\x7f"
    "\xee\xdb\xdf\x81\xd3\x21\xdb\x33\x92\xee\xff\xe2\x2f\x32\x77\x73\x6a\x58"
    "\xab\x21\xf3\xe3\xe1\xbc\x4f\x12\x72\xa6\xb5\xc2\xfb\x27\x9e\xc8\xca\xab"
    "\x64\xa0\x87\x07\x9d\xef\xca\x0f\xdb\x02\x81\x81\x00\xe6\xd3\x4d\xc0\xa1"
    "\x91\x0e\x62\xfd\xb0\xdd\xc6\x30\xb8\x8c\xcb\x14\xc1\x4b\x69\x30\xdd\xcd"
    "\x86\x67\xcb\x37\x14\xc5\x03\xd2\xb4\x69\xab\x3d\xe5\x16\x81\x0f\xe5\x50"
    "\xf4\x18\xb1\xec\xbc\x71\xe9\x80\x99\x06\xe4\xa3\xfe\x44\x84\x4a\x2d\x1e"
    "\x07\x7f\x22\x70\x6d\x4f\xd4\x93\x0b\x8b\x99\xce\x1e\xab\xcd\x4c\xd2\xd3"
    "\x10\x47\x5c\x09\x9f\x6d\x82\xc0\x08\x75\xe3\x3d\x83\xc2\x19\x50\x29\xec"
    "\x1f\x84\x29\xcc\xf1\x56\xee\xbd\x54\x5d\xe6\x19\xdf\x0d\x1c\xa4\xbb\x0a"
    "\xfe\x84\x44\x29\x1d\xf9\x5c\x80\x96\x5b\x24\xb4\xf7\x02\x1b\x02\x81\x81"
    "\x00\xa3\x48\xf1\x9c\x58\xc2\x5f\x38\xfb\xd8\x12\x39\xf1\x8e\x73\xa1\xcf"
    "\x78\x12\xe0\xed\x2a\xbb\xef\xac\x23\xb2\xbf\xd6\x0c\xe9\x6e\x1e\xab\xea"
    "\x3f\x68\x36\xa7\x1f\xe5\xab\xe0\x86\xa5\x76\x32\x98\xdd\x75\xb5\x2b\xbc"
    "\xcb\x8a\x03\x00\x7c\x2e\xca\xf8\xbc\x19\xe4\xe3\xa3\x31\xbd\x1d\x20\x2b"
    "\x09\xad\x6f\x4c\xed\x48\xd4\xdf\x87\xf9\xf0\x46\xb9\x86\x4c\x4b\x71\xe7"
    "\x48\x78\xdc\xed\xc7\x82\x02\x44\xd3\xa6\xb3\x10\x5f\x62\x81\xfc\xb8\xe4"
    "\x0e\xf4\x1a\xdd\xab\x3f\xbc\x63\x79\x5b\x39\x69\x5e\xea\xa9\x15\xfe\x90"
    "\xec\xda\x75\x02\x81\x81\x00\x99\xe2\x33\xd5\xc1\x0b\x5e\xec\xa9\x20\x93"
    "\xd9\x75\xd0\x5d\xdc\xb8\x80\xdc\xf0\xcb\x3e\x89\x04\x45\x32\x24\xb8\x83"
    "\x57\xe1\xcd\x9b\xc7\x7e\x98\xb9\xab\x5f\xee\x35\xf8\x10\x76\x9d\xd2\xf6"
    "\x9b\xab\x10\xaf\x43\x17\xfe\xd8\x58\x31\x73\x69\x5a\x54\xc1\xa0\x48\xdf"
    "\xe3\x0c\xb2\x5d\x11\x34\x14\x72\x88\xdd\xe1\xe2\x0a\xda\x3d\x5b\xbf\x9e"
    "\x57\x2a\xb0\x4e\x97\x7e\x57\xd6\xbb\x8a\xc6\x9d\x6a\x58\x1b\xdd\xf6\x39"
    "\xf4\x7e\x38\x3e\x99\x66\x94\xb3\x68\x6d\xd2\x07\x54\x58\x2d\x70\xbe\xa6"
    "\x3d\xab\x0e\xe7\x6d\xcd\xfa\x01\x67\x02\x81\x80\x6c\xdb\x4b\xbd\x90\x81"
    "\x94\xd0\xa7\xe5\x61\x7b\xf6\x5e\xf7\xc1\x34\xfa\xb7\x40\x9e\x1c\x7d\x4a"
    "\x72\xc2\x77\x2a\x8e\xb3\x46\x49\x69\xc7\xf1\x7f\x9a\xcf\x1a\x15\x43\xc7"
    "\xeb\x04\x6e\x4e\xcc\x65\xe8\xf9\x23\x72\x7d\xdd\x06\xac\xaa\xfd\x74\x87"
    "\x50\x7d\x66\x98\x97\xc2\x21\x28\xbe\x15\x72\x06\x73\x9f\x88\x9e\x30\x8d"
    "\xea\x5a\xa6\xa0\x2f\x26\x59\x88\x32\x4b\xef\x85\xa5\xe8\x9e\x85\x01\x56"
    "\xd8\x8d\x19\xcc\xb5\x94\xec\x56\xa8\x7b\x42\xb4\xa2\xbc\x93\xc7\x7f\xd2"
    "\xec\xfb\x92\x26\x46\x3f\x47\x1b\x63\xff\x0b\x48\x91\xa3\x02\x81\x80\x2c"
    "\x4a\xb9\xa4\x46\x7b\xff\x50\x7e\xbf\x60\x47\x3b\x2b\x66\x82\xdc\x0e\x53"
    "\x65\x71\xe9\xda\x2a\xb8\x32\x93\x42\xb7\xff\xea\x67\x66\xf1\xbc\x87\x28"
    "\x65\x29\x79\xca\xab\x93\x56\xda\x95\xc1\x26\x44\x3d\x27\xc1\x91\xc6\x9b"
    "\xd9\xec\x9d\xb7\x49\xe7\x16\xee\x99\x87\x50\x95\x81\xd4\x5c\x5b\x5a\x5d"
    "\x0a\x43\xa5\xa7\x8f\x5a\x80\x49\xa0\xb7\x10\x85\xc7\xf4\x42\x34\x86\xb6"
    "\x5f\x3f\x88\x9e\xc7\xf5\x59\x29\x39\x68\x48\xf2\xd7\x08\x5b\x92\x8e\x6b"
    "\xea\xa5\x63\x5f\xc0\xfb\xe4\xe1\xb2\x7d\xb7\x40\xe9\x55\x06\xbf\x58\x25"
    "\x6f";

static const uint8_t kTwoPrimeEncryptedMessage[] = {
    0x63, 0x0a, 0x30, 0x45, 0x43, 0x11, 0x45, 0xb7, 0x99, 0x67, 0x90, 0x35,
    0x37, 0x27, 0xff, 0xbc, 0xe0, 0xbf, 0xa6, 0xd1, 0x47, 0x50, 0xbb, 0x6c,
    0x1c, 0xaa, 0x66, 0xf2, 0xff, 0x9d, 0x9a, 0xa6, 0xb4, 0x16, 0x63, 0xb0,
    0xa1, 0x7c, 0x7c, 0x0c, 0xef, 0xb3, 0x66, 0x52, 0x42, 0xd7, 0x5e, 0xf3,
    0xa4, 0x15, 0x33, 0x40, 0x43, 0xe8, 0xb1, 0xfc, 0xe0, 0x42, 0x83, 0x46,
    0x28, 0xce, 0xde, 0x7b, 0x01, 0xeb, 0x28, 0x92, 0x70, 0xdf, 0x8d, 0x54,
    0x9e, 0xed, 0x23, 0xb4, 0x78, 0xc3, 0xca, 0x85, 0x53, 0x48, 0xd6, 0x8a,
    0x87, 0xf7, 0x69, 0xcd, 0x82, 0x8c, 0x4f, 0x5c, 0x05, 0x55, 0xa6, 0x78,
    0x89, 0xab, 0x4c, 0xd8, 0xa9, 0xd6, 0xa5, 0xf4, 0x29, 0x4c, 0x23, 0xc8,
    0xcf, 0xf0, 0x4c, 0x64, 0x6b, 0x4e, 0x02, 0x17, 0x69, 0xd6, 0x47, 0x83,
    0x30, 0x43, 0x02, 0x29, 0xda, 0xda, 0x75, 0x3b, 0xd7, 0xa7, 0x2b, 0x31,
    0xb3, 0xe9, 0x71, 0xa4, 0x41, 0xf7, 0x26, 0x9b, 0xcd, 0x23, 0xfa, 0x45,
    0x3c, 0x9b, 0x7d, 0x28, 0xf7, 0xf9, 0x67, 0x04, 0xba, 0xfc, 0x46, 0x75,
    0x11, 0x3c, 0xd5, 0x27, 0x43, 0x53, 0xb1, 0xb6, 0x9e, 0x18, 0xeb, 0x11,
    0xb4, 0x25, 0x20, 0x30, 0x0b, 0xe0, 0x1c, 0x17, 0x36, 0x22, 0x10, 0x0f,
    0x99, 0xb5, 0x50, 0x14, 0x73, 0x07, 0xf0, 0x2f, 0x5d, 0x4c, 0xe3, 0xf2,
    0x86, 0xc2, 0x05, 0xc8, 0x38, 0xed, 0xeb, 0x2a, 0x4a, 0xab, 0x76, 0xe3,
    0x1a, 0x75, 0x44, 0xf7, 0x6e, 0x94, 0xdc, 0x25, 0x62, 0x7e, 0x31, 0xca,
    0xc2, 0x73, 0x51, 0xb5, 0x03, 0xfb, 0xf9, 0xf6, 0xb5, 0x8d, 0x4e, 0x6c,
    0x21, 0x0e, 0xf9, 0x97, 0x26, 0x57, 0xf3, 0x52, 0x72, 0x07, 0xf8, 0xb4,
    0xcd, 0xb4, 0x39, 0xcf, 0xbf, 0x78, 0xcc, 0xb6, 0x87, 0xf9, 0xb7, 0x8b,
    0x6a, 0xce, 0x9f, 0xc8,
};

// kEstonianRSAKey is an RSAPublicKey encoded with a negative modulus. See
// https://crbug.com/532048.
static const uint8_t kEstonianRSAKey[] = {
    0x30, 0x82, 0x01, 0x09, 0x02, 0x82, 0x01, 0x00, 0x96, 0xa6, 0x2e, 0x9c,
    0x4e, 0x6a, 0xc3, 0xcc, 0xcd, 0x8f, 0x70, 0xc3, 0x55, 0xbf, 0x5e, 0x9c,
    0xd4, 0xf3, 0x17, 0xc3, 0x97, 0x70, 0xae, 0xdf, 0x12, 0x5c, 0x15, 0x80,
    0x03, 0xef, 0x2b, 0x18, 0x9d, 0x6a, 0xcb, 0x52, 0x22, 0xc1, 0x81, 0xb8,
    0x7e, 0x61, 0xe8, 0x0f, 0x79, 0x24, 0x0f, 0x82, 0x70, 0x24, 0x4e, 0x29,
    0x20, 0x05, 0x54, 0xeb, 0xd4, 0xa9, 0x65, 0x59, 0xb6, 0x3c, 0x75, 0x95,
    0x2f, 0x4c, 0xf6, 0x9d, 0xd1, 0xaf, 0x5f, 0x14, 0x14, 0xe7, 0x25, 0xea,
    0xa5, 0x47, 0x5d, 0xc6, 0x3e, 0x28, 0x8d, 0xdc, 0x54, 0x87, 0x2a, 0x7c,
    0x10, 0xe9, 0xc6, 0x76, 0x2d, 0xe7, 0x79, 0xd8, 0x0e, 0xbb, 0xa9, 0xac,
    0xb5, 0x18, 0x98, 0xd6, 0x47, 0x6e, 0x06, 0x70, 0xbf, 0x9e, 0x82, 0x25,
    0x95, 0x4e, 0xfd, 0x70, 0xd7, 0x73, 0x45, 0x2e, 0xc1, 0x1f, 0x7a, 0x9a,
    0x9d, 0x60, 0xc0, 0x1f, 0x67, 0x06, 0x2a, 0x4e, 0x87, 0x3f, 0x19, 0x88,
    0x69, 0x64, 0x4d, 0x9f, 0x75, 0xf5, 0xd3, 0x1a, 0x41, 0x3d, 0x35, 0x17,
    0xb6, 0xd1, 0x44, 0x0d, 0x25, 0x8b, 0xe7, 0x94, 0x39, 0xb0, 0x7c, 0xaf,
    0x3e, 0x6a, 0xfa, 0x8d, 0x90, 0x21, 0x0f, 0x8a, 0x43, 0x94, 0x37, 0x7c,
    0x2a, 0x15, 0x4c, 0xa0, 0xfa, 0xa9, 0x2f, 0x21, 0xa6, 0x6f, 0x8e, 0x2f,
    0x89, 0xbc, 0xbb, 0x33, 0xf8, 0x31, 0xfc, 0xdf, 0xcd, 0x68, 0x9a, 0xbc,
    0x75, 0x06, 0x95, 0xf1, 0x3d, 0xef, 0xca, 0x76, 0x27, 0xd2, 0xba, 0x8e,
    0x0e, 0x1c, 0x43, 0xd7, 0x70, 0xb9, 0xc6, 0x15, 0xca, 0xd5, 0x4d, 0x87,
    0xb9, 0xd1, 0xae, 0xde, 0x69, 0x73, 0x00, 0x2a, 0x97, 0x51, 0x4b, 0x30,
    0x01, 0xc2, 0x85, 0xd0, 0x05, 0xcc, 0x2e, 0xe8, 0xc7, 0x42, 0xe7, 0x94,
    0x51, 0xe3, 0xf5, 0x19, 0x35, 0xdc, 0x57, 0x96, 0xe7, 0xd9, 0xb4, 0x49,
    0x02, 0x03, 0x01, 0x00, 0x01,
};

// kExponent1RSAKey is an RSAPublicKey encoded with an exponent of 1. See
// https://crbug.com/541257
static const uint8_t kExponent1RSAKey[] = {
    0x30, 0x82, 0x01, 0x08, 0x02, 0x82, 0x01, 0x01, 0x00, 0xcf, 0x86, 0x9a,
    0x7d, 0x5c, 0x9f, 0xbd, 0x33, 0xbb, 0xc2, 0xb1, 0x06, 0xa8, 0x3e, 0xc5,
    0x18, 0xf3, 0x01, 0x04, 0xdd, 0x7a, 0x38, 0x0e, 0x8e, 0x8d, 0x10, 0xaa,
    0xf8, 0x64, 0x49, 0x82, 0xa6, 0x16, 0x9d, 0xd9, 0xae, 0x5e, 0x7f, 0x9b,
    0x53, 0xcb, 0xbb, 0x29, 0xda, 0x98, 0x47, 0x26, 0x88, 0x2e, 0x1d, 0x64,
    0xb3, 0xbc, 0x7e, 0x96, 0x3a, 0xa7, 0xd6, 0x87, 0xf6, 0xf5, 0x3f, 0xa7,
    0x3b, 0xd3, 0xc5, 0xd5, 0x61, 0x3c, 0x63, 0x05, 0xf9, 0xbc, 0x64, 0x1d,
    0x71, 0x65, 0xf5, 0xc8, 0xe8, 0x64, 0x41, 0x35, 0x88, 0x81, 0x6b, 0x2a,
    0x24, 0xbb, 0xdd, 0x9f, 0x75, 0x4f, 0xea, 0x35, 0xe5, 0x32, 0x76, 0x5a,
    0x8b, 0x7a, 0xb5, 0x92, 0x65, 0x34, 0xb7, 0x88, 0x42, 0x5d, 0x41, 0x0b,
    0xd1, 0x00, 0x2d, 0x43, 0x47, 0x55, 0x60, 0x3c, 0x0e, 0x60, 0x04, 0x5c,
    0x88, 0x13, 0xc7, 0x42, 0x55, 0x16, 0x31, 0x32, 0x81, 0xba, 0xde, 0xa9,
    0x56, 0xeb, 0xdb, 0x66, 0x7f, 0x31, 0xba, 0xe8, 0x87, 0x1a, 0xcc, 0xad,
    0x90, 0x86, 0x4b, 0xa7, 0x6d, 0xd5, 0xc1, 0xb7, 0xe7, 0x67, 0x56, 0x41,
    0xf7, 0x03, 0xb3, 0x09, 0x61, 0x63, 0xb5, 0xb0, 0x19, 0x7b, 0xc5, 0x91,
    0xc8, 0x96, 0x5b, 0x6a, 0x80, 0xa1, 0x53, 0x0f, 0x9a, 0x47, 0xb5, 0x9a,
    0x44, 0x53, 0xbd, 0x93, 0xe3, 0xe4, 0xce, 0x0c, 0x17, 0x11, 0x51, 0x1d,
    0xfd, 0x6c, 0x74, 0xe4, 0xec, 0x2a, 0xce, 0x57, 0x27, 0xcc, 0x83, 0x98,
    0x08, 0x32, 0x2c, 0xd5, 0x75, 0xa9, 0x27, 0xfe, 0xaa, 0x5e, 0x48, 0xc9,
    0x46, 0x9a, 0x29, 0x3f, 0xe6, 0x01, 0x4d, 0x97, 0x4a, 0x70, 0xd1, 0x5d,
    0xf8, 0xc0, 0x0b, 0x23, 0xcb, 0xbe, 0xf5, 0x70, 0x0b, 0xc2, 0xf2, 0xc0,
    0x33, 0x9c, 0xc4, 0x8b, 0x39, 0x7e, 0x3d, 0xc6, 0x23, 0x39, 0x9a, 0x98,
    0xdd, 0x02, 0x01, 0x01,
};

struct RSAEncryptParam {
  const uint8_t *der;
  size_t der_len;
  const uint8_t *oaep_ciphertext;
  size_t oaep_ciphertext_len;
} kRSAEncryptParams[] = {
    {kKey1, sizeof(kKey1) - 1, kOAEPCiphertext1, sizeof(kOAEPCiphertext1) - 1},
    {kKey2, sizeof(kKey2) - 1, kOAEPCiphertext2, sizeof(kOAEPCiphertext2) - 1},
    {kKey3, sizeof(kKey3) - 1, kOAEPCiphertext3, sizeof(kOAEPCiphertext3) - 1},
};

class RSAEncryptTest : public testing::TestWithParam<RSAEncryptParam> {};

TEST_P(RSAEncryptTest, TestKey) {
  // Construct an RSA key in different ways.
  const auto &param = GetParam();
  bssl::UniquePtr<RSA> parsed(
      RSA_private_key_from_bytes(param.der, param.der_len));

  ASSERT_TRUE(parsed);
  EXPECT_TRUE(RSA_get0_e(parsed.get()));
  EXPECT_TRUE(RSA_get0_d(parsed.get()));

  bssl::UniquePtr<RSA> constructed(RSA_new_private_key(
      RSA_get0_n(parsed.get()), RSA_get0_e(parsed.get()),
      RSA_get0_d(parsed.get()), RSA_get0_p(parsed.get()),
      RSA_get0_q(parsed.get()), RSA_get0_dmp1(parsed.get()),
      RSA_get0_dmq1(parsed.get()), RSA_get0_iqmp(parsed.get())));
  ASSERT_TRUE(constructed);
  EXPECT_TRUE(RSA_get0_e(constructed.get()));
  EXPECT_TRUE(RSA_get0_d(constructed.get()));

  bssl::UniquePtr<RSA> no_crt(RSA_new_private_key_no_crt(
      RSA_get0_n(parsed.get()), RSA_get0_e(parsed.get()),
      RSA_get0_d(parsed.get())));
  ASSERT_TRUE(no_crt);
  EXPECT_TRUE(RSA_get0_e(no_crt.get()));
  EXPECT_TRUE(RSA_get0_d(no_crt.get()));

  bssl::UniquePtr<RSA> no_e(RSA_new_private_key_no_e(RSA_get0_n(parsed.get()),
                                                     RSA_get0_d(parsed.get())));
  ASSERT_TRUE(no_e);
  EXPECT_FALSE(RSA_get0_e(no_e.get()));
  EXPECT_TRUE(RSA_get0_d(no_e.get()));

  bssl::UniquePtr<RSA> pub(
      RSA_new_public_key(RSA_get0_n(parsed.get()), RSA_get0_e(parsed.get())));
  ASSERT_TRUE(pub);
  EXPECT_TRUE(RSA_get0_e(pub.get()));
  EXPECT_FALSE(RSA_get0_d(pub.get()));

  for (RSA *key :
       {parsed.get(), constructed.get(), no_crt.get(), no_e.get(), pub.get()}) {
    EXPECT_TRUE(RSA_check_key(key));
    bssl::UniquePtr<EVP_PKEY> rsa_pkey(EVP_PKEY_new());
    ASSERT_TRUE(rsa_pkey);
    ASSERT_TRUE(EVP_PKEY_set1_RSA(rsa_pkey.get(), key));
    bssl::UniquePtr<EVP_PKEY_CTX> rsa_key_ctx(
            EVP_PKEY_CTX_new(rsa_pkey.get(), NULL));
    ASSERT_TRUE(rsa_key_ctx);
    EXPECT_TRUE(EVP_PKEY_check(rsa_key_ctx.get()));
    EXPECT_TRUE(EVP_PKEY_public_check((rsa_key_ctx.get())));

    uint8_t ciphertext[256], plaintext[256];
    size_t ciphertext_len = 0, plaintext_len = 0;

    if (RSA_get0_e(key) != nullptr) {
      // Test that PKCS#1 v1.5 encryption round-trips.
      ASSERT_TRUE(RSA_encrypt(key, &ciphertext_len, ciphertext,
                              sizeof(ciphertext), kPlaintext, kPlaintextLen,
                              RSA_PKCS1_PADDING));
      EXPECT_EQ(RSA_size(key), ciphertext_len);

      ASSERT_TRUE(RSA_decrypt(parsed.get(), &plaintext_len, plaintext,
                              sizeof(plaintext), ciphertext, ciphertext_len,
                              RSA_PKCS1_PADDING));
      EXPECT_EQ(Bytes(kPlaintext, kPlaintextLen),
                Bytes(plaintext, plaintext_len));

      // Test that OAEP encryption round-trips.
      ciphertext_len = 0;
      ASSERT_TRUE(RSA_encrypt(key, &ciphertext_len, ciphertext,
                              sizeof(ciphertext), kPlaintext, kPlaintextLen,
                              RSA_PKCS1_OAEP_PADDING));
      EXPECT_EQ(RSA_size(key), ciphertext_len);

      plaintext_len = 0;
      ASSERT_TRUE(RSA_decrypt(parsed.get(), &plaintext_len, plaintext,
                              sizeof(plaintext), ciphertext, ciphertext_len,
                              RSA_PKCS1_OAEP_PADDING));
      EXPECT_EQ(Bytes(kPlaintext, kPlaintextLen),
                Bytes(plaintext, plaintext_len));
    }

    if (RSA_get0_d(key) != nullptr) {
      // |oaep_ciphertext| should decrypt to |kPlaintext|.
      plaintext_len = 0;
      ASSERT_TRUE(RSA_decrypt(key, &plaintext_len, plaintext, sizeof(plaintext),
                              param.oaep_ciphertext, param.oaep_ciphertext_len,
                              RSA_PKCS1_OAEP_PADDING));
      EXPECT_EQ(Bytes(kPlaintext, kPlaintextLen),
                Bytes(plaintext, plaintext_len));

      // Try decrypting corrupted ciphertexts.
      OPENSSL_memcpy(ciphertext, param.oaep_ciphertext,
                     param.oaep_ciphertext_len);
      for (size_t i = 0; i < param.oaep_ciphertext_len; i++) {
        SCOPED_TRACE(i);
        ciphertext[i] ^= 1;
        EXPECT_FALSE(RSA_decrypt(
            key, &plaintext_len, plaintext, sizeof(plaintext), ciphertext,
            param.oaep_ciphertext_len, RSA_PKCS1_OAEP_PADDING));
        ERR_clear_error();
        ciphertext[i] ^= 1;
      }

      // Test truncated ciphertexts.
      for (size_t len = 0; len < param.oaep_ciphertext_len; len++) {
        SCOPED_TRACE(len);
        EXPECT_FALSE(RSA_decrypt(key, &plaintext_len, plaintext,
                                 sizeof(plaintext), ciphertext, len,
                                 RSA_PKCS1_OAEP_PADDING));
        ERR_clear_error();
      }
    }
  }
}

INSTANTIATE_TEST_SUITE_P(All, RSAEncryptTest,
                         testing::ValuesIn(kRSAEncryptParams));

TEST(RSATest, TestDecrypt) {
  bssl::UniquePtr<RSA> rsa(
      RSA_private_key_from_bytes(kTwoPrimeKey, sizeof(kTwoPrimeKey) - 1));
  ASSERT_TRUE(rsa);

  EXPECT_TRUE(RSA_check_key(rsa.get()));

  bssl::UniquePtr<EVP_PKEY> rsa_pkey(EVP_PKEY_new());
  ASSERT_TRUE(rsa_pkey);
  ASSERT_TRUE(EVP_PKEY_set1_RSA(rsa_pkey.get(), rsa.get()));
  bssl::UniquePtr<EVP_PKEY_CTX> rsa_key_ctx(
          EVP_PKEY_CTX_new(rsa_pkey.get(), NULL));
  ASSERT_TRUE(rsa_key_ctx);
  EXPECT_TRUE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_TRUE(EVP_PKEY_public_check((rsa_key_ctx.get())));

  uint8_t out[256];
  size_t out_len;
  ASSERT_TRUE(RSA_decrypt(
      rsa.get(), &out_len, out, sizeof(out), kTwoPrimeEncryptedMessage,
      sizeof(kTwoPrimeEncryptedMessage), RSA_PKCS1_PADDING));
  EXPECT_EQ(Bytes("hello world"), Bytes(out, out_len));
}

TEST(RSATest, CheckFIPS) {
  bssl::UniquePtr<RSA> rsa(
      RSA_private_key_from_bytes(kFIPSKey, sizeof(kFIPSKey) - 1));
  ASSERT_TRUE(rsa);
  EXPECT_TRUE(RSA_check_fips(rsa.get()));

  // Check that RSA_check_fips works on a public key.
  bssl::UniquePtr<RSA> pub(
      RSA_public_key_from_bytes(kFIPSPublicKey, sizeof(kFIPSPublicKey) - 1));
  ASSERT_TRUE(pub);
  EXPECT_TRUE(RSA_check_fips(pub.get()));
}

TEST(RSATest, GenerateFIPS) {
  bssl::UniquePtr<RSA> rsa(RSA_new());
  ASSERT_TRUE(rsa);

  // RSA_generate_key_fips may only be used for 2048-, 3072-, and 4096-bit
  // keys.
  EXPECT_FALSE(RSA_generate_key_fips(rsa.get(), 512, nullptr));
  EXPECT_FALSE(RSA_generate_key_fips(rsa.get(), 1024, nullptr));
  EXPECT_FALSE(RSA_generate_key_fips(rsa.get(), 2047, nullptr));
  EXPECT_FALSE(RSA_generate_key_fips(rsa.get(), 2049, nullptr));
  EXPECT_FALSE(RSA_generate_key_fips(rsa.get(), 3071, nullptr));
  EXPECT_FALSE(RSA_generate_key_fips(rsa.get(), 3073, nullptr));
  EXPECT_FALSE(RSA_generate_key_fips(rsa.get(), 4095, nullptr));
  EXPECT_FALSE(RSA_generate_key_fips(rsa.get(), 4097, nullptr));
  ERR_clear_error();

  // Test that we can generate keys of the supported lengths:
  for (const size_t bits : {2048, 3072, 4096}) {
    SCOPED_TRACE(bits);

    rsa.reset(RSA_new());
    ASSERT_TRUE(rsa);
    ASSERT_TRUE(RSA_generate_key_fips(rsa.get(), bits, nullptr));
    EXPECT_EQ(bits, BN_num_bits(rsa->n));
  }
}

TEST(RSATest, BadKey) {
  bssl::UniquePtr<RSA> key(RSA_new());
  bssl::UniquePtr<BIGNUM> e(BN_new());
  ASSERT_TRUE(key);
  ASSERT_TRUE(e);
  ASSERT_TRUE(BN_set_word(e.get(), RSA_F4));

  // Generate a bad key.
  ASSERT_TRUE(RSA_generate_key_ex(key.get(), 2048, e.get(), nullptr));
  ASSERT_TRUE(BN_add(key->p, key->p, BN_value_one()));

  // Bad keys are detected.
  EXPECT_FALSE(RSA_check_key(key.get()));
  EXPECT_FALSE(RSA_check_fips(key.get()));

  bssl::UniquePtr<EVP_PKEY> rsa_pkey(EVP_PKEY_new());
  ASSERT_TRUE(rsa_pkey);
  ASSERT_TRUE(EVP_PKEY_set1_RSA(rsa_pkey.get(), key.get()));
  bssl::UniquePtr<EVP_PKEY_CTX> rsa_key_ctx(
          EVP_PKEY_CTX_new(rsa_pkey.get(), NULL));
  ASSERT_TRUE(rsa_key_ctx);
  EXPECT_FALSE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_FALSE(EVP_PKEY_public_check((rsa_key_ctx.get())));

  // Bad keys may not be parsed.
  uint8_t *der;
  size_t der_len;
  ASSERT_TRUE(RSA_private_key_to_bytes(&der, &der_len, key.get()));
  bssl::UniquePtr<uint8_t> delete_der(der);
  key.reset(RSA_private_key_from_bytes(der, der_len));
  EXPECT_FALSE(key);
}

TEST(RSATest, Set0Key) {
  const int hash_nid = NID_sha256;
  const uint8_t kDummyHash[32] = {0};
  uint8_t sig[256];
  unsigned sig_len = sizeof(sig);

  // Reference RSA key that needs to be reset before each case as
  // RSA_set0_key takes ownership of its parameters
  bssl::UniquePtr<RSA> rsa;

  // Allocate an empty key to imitate JCA keys via calls to RSA_set0_key
  bssl::UniquePtr<RSA> jcaKey(RSA_new());
  ASSERT_TRUE(jcaKey);

  // FULL KEY, BLINDING => OK
  rsa.reset(RSA_private_key_from_bytes(kKey1, sizeof(kKey1) - 1));
  ASSERT_TRUE(rsa);
  jcaKey.reset(RSA_new());
  ASSERT_TRUE(jcaKey);
  EXPECT_TRUE(RSA_set0_key(jcaKey.get(), BN_dup(rsa->n), BN_dup(rsa->e), BN_dup(rsa->d)));
  EXPECT_TRUE(RSA_sign(hash_nid, kDummyHash, sizeof(kDummyHash), sig,
                       &sig_len, jcaKey.get()));
  EXPECT_TRUE(RSA_verify(hash_nid, kDummyHash, sizeof(kDummyHash), sig,
                         sig_len, rsa.get()));

  // NO |e|, BLINDING => ERR
  rsa.reset(RSA_private_key_from_bytes(kKey1, sizeof(kKey1) - 1));
  ASSERT_TRUE(rsa);
  jcaKey.reset(RSA_new());
  ASSERT_TRUE(jcaKey);
  EXPECT_EQ(1, RSA_blinding_on(jcaKey.get(), nullptr));
  EXPECT_TRUE(RSA_set0_key(jcaKey.get(), BN_dup(rsa->n), NULL, BN_dup(rsa->d)));
  EXPECT_FALSE(RSA_sign(hash_nid, kDummyHash, sizeof(kDummyHash), sig,
                        &sig_len, jcaKey.get()));
  uint32_t err = ERR_get_error();
  EXPECT_EQ(ERR_LIB_RSA, ERR_GET_LIB(err));
  EXPECT_EQ(RSA_R_NO_PUBLIC_EXPONENT, ERR_GET_REASON(err));
  ERR_clear_error();

  // NO |e|, NO BLINDING => OK
  rsa.reset(RSA_private_key_from_bytes(kKey1, sizeof(kKey1) - 1));
  ASSERT_TRUE(rsa);
  jcaKey.reset(RSA_new());
  ASSERT_TRUE(jcaKey);
  EXPECT_TRUE(RSA_set0_key(jcaKey.get(), BN_dup(rsa->n), NULL, BN_dup(rsa->d)));
  EXPECT_EQ(1, RSA_blinding_on(jcaKey.get(), nullptr));
  RSA_blinding_off_temp_for_accp_compatibility(jcaKey.get());
  EXPECT_EQ(0, RSA_blinding_on(jcaKey.get(), nullptr));
  EXPECT_TRUE(RSA_sign(hash_nid, kDummyHash, sizeof(kDummyHash), sig,
                       &sig_len, jcaKey.get()));
  EXPECT_TRUE(RSA_verify(hash_nid, kDummyHash, sizeof(kDummyHash), sig,
                         sig_len, rsa.get()));

  // RSA_blinding_on returns 0 for null.
  EXPECT_EQ(0, RSA_blinding_on(nullptr, nullptr));
}

TEST(RSATest, ASN1) {
  // Test that private keys may be decoded.
  bssl::UniquePtr<RSA> rsa(
      RSA_private_key_from_bytes(kKey1, sizeof(kKey1) - 1));
  ASSERT_TRUE(rsa);

  // Test that the serialization round-trips.
  uint8_t *der;
  size_t der_len;
  ASSERT_TRUE(RSA_private_key_to_bytes(&der, &der_len, rsa.get()));
  bssl::UniquePtr<uint8_t> delete_der(der);
  EXPECT_EQ(Bytes(kKey1, sizeof(kKey1) - 1), Bytes(der, der_len));

  // Test that serializing public keys works.
  ASSERT_TRUE(RSA_public_key_to_bytes(&der, &der_len, rsa.get()));
  delete_der.reset(der);

  // Public keys may be parsed back out.
  rsa.reset(RSA_public_key_from_bytes(der, der_len));
  ASSERT_TRUE(rsa);
  EXPECT_FALSE(rsa->p);
  EXPECT_FALSE(rsa->q);

  // Serializing the result round-trips.
  uint8_t *der2;
  size_t der2_len;
  ASSERT_TRUE(RSA_public_key_to_bytes(&der2, &der2_len, rsa.get()));
  bssl::UniquePtr<uint8_t> delete_der2(der2);
  EXPECT_EQ(Bytes(der, der_len), Bytes(der2, der2_len));

  // Public keys cannot be serialized as private keys.
  int ok = RSA_private_key_to_bytes(&der, &der_len, rsa.get());
  if (ok) {
    OPENSSL_free(der);
  }
  EXPECT_FALSE(ok);
  ERR_clear_error();

  // Public keys with negative moduli are invalid.
  rsa.reset(RSA_public_key_from_bytes(kEstonianRSAKey,
                                      sizeof(kEstonianRSAKey)));
  EXPECT_FALSE(rsa);
  ERR_clear_error();
}

TEST(RSATest, BadExponent) {
  bssl::UniquePtr<RSA> rsa(
      RSA_public_key_from_bytes(kExponent1RSAKey, sizeof(kExponent1RSAKey)));
  EXPECT_FALSE(rsa);
  ERR_clear_error();
}

// Attempting to generate an excessively small key should fail.
TEST(RSATest, GenerateSmallKey) {
  bssl::UniquePtr<RSA> rsa(RSA_new());
  ASSERT_TRUE(rsa);
  bssl::UniquePtr<BIGNUM> e(BN_new());
  ASSERT_TRUE(e);
  ASSERT_TRUE(BN_set_word(e.get(), RSA_F4));

  EXPECT_FALSE(RSA_generate_key_ex(rsa.get(), 255, e.get(), nullptr));
  EXPECT_TRUE(
      ErrorEquals(ERR_get_error(), ERR_LIB_RSA, RSA_R_KEY_SIZE_TOO_SMALL));
}

// Attempting to generate an funny RSA key length should round down.
TEST(RSATest, RoundKeyLengths) {
  bssl::UniquePtr<BIGNUM> e(BN_new());
  ASSERT_TRUE(e);
  ASSERT_TRUE(BN_set_word(e.get(), RSA_F4));

  bssl::UniquePtr<RSA> rsa(RSA_new());
  ASSERT_TRUE(rsa);
  ASSERT_TRUE(RSA_generate_key_ex(rsa.get(), 1025, e.get(), nullptr));
  EXPECT_EQ(1024u, BN_num_bits(rsa->n));

  rsa.reset(RSA_new());
  ASSERT_TRUE(rsa);
  ASSERT_TRUE(RSA_generate_key_ex(rsa.get(), 1027, e.get(), nullptr));
  EXPECT_EQ(1024u, BN_num_bits(rsa->n));

  rsa.reset(RSA_new());
  ASSERT_TRUE(rsa);
  ASSERT_TRUE(RSA_generate_key_ex(rsa.get(), 1151, e.get(), nullptr));
  EXPECT_EQ(1024u, BN_num_bits(rsa->n));

  rsa.reset(RSA_new());
  ASSERT_TRUE(rsa);
  ASSERT_TRUE(RSA_generate_key_ex(rsa.get(), 1152, e.get(), nullptr));
  EXPECT_EQ(1152u, BN_num_bits(rsa->n));
}

TEST(RSATest, BlindingDisabled) {
  bssl::UniquePtr<RSA> rsa(
      RSA_private_key_from_bytes(kTwoPrimeKey, sizeof(kTwoPrimeKey) - 1));
  ASSERT_TRUE(rsa);

  rsa->flags |= RSA_FLAG_NO_BLINDING;

  uint8_t sig[256];
  ASSERT_GE(sizeof(sig), RSA_size(rsa.get()));

  static const uint8_t kZeros[32] = {0};
  unsigned sig_len;
  ASSERT_TRUE(
      RSA_sign(NID_sha256, kZeros, sizeof(kZeros), sig, &sig_len, rsa.get()));
  EXPECT_TRUE(
      RSA_verify(NID_sha256, kZeros, sizeof(kZeros), sig, sig_len, rsa.get()));
}

// Test that decrypting with a public key fails gracefully rather than crashing.
TEST(RSATest, DecryptPublic) {
  bssl::UniquePtr<RSA> pub(
      RSA_public_key_from_bytes(kFIPSPublicKey, sizeof(kFIPSPublicKey) - 1));
  ASSERT_TRUE(pub);
  ASSERT_EQ(1024u / 8u, RSA_size(pub.get()));

  size_t len;
  uint8_t in[1024 / 8] = {0}, out[1024 / 8];
  EXPECT_FALSE(RSA_decrypt(pub.get(), &len, out, sizeof(out), in, sizeof(in),
                           RSA_PKCS1_PADDING));
  uint32_t err = ERR_get_error();
  EXPECT_EQ(ERR_LIB_RSA, ERR_GET_LIB(err));
  EXPECT_EQ(RSA_R_VALUE_MISSING, ERR_GET_REASON(err));
}

TEST(RSATest, CheckKey) {
  static const char kN[] =
      "b5a5651bc2e15ce31d789f0984053a2ea0cf8f964a78068c45acfdf078c57fd62d5a287c"
      "32f3baa879f5dfea27d7a3077c9d3a2a728368c3d90164690c3d82f660ffebc7f13fed45"
      "4eb5103df943c10dc32ec60b0d9b6e307bfd7f9b943e0dc3901e42501765365f7286eff2"
      "f1f728774aa6a371e108a3a7dd00d7bcd4c1a186c2865d4b370ea38cc89c0b23b318dbca"
      "fbd872b4f9b833dfb2a4ca7fcc23298020044e8130bfe930adfb3e5cab8d324547adf4b2"
      "ce34d7cea4298f0b613d85f2bf1df03da44aee0784a1a20a15ee0c38a0f8e84962f1f61b"
      "18bd43781c7385f3c2b8e2aebd3c560b4faad208ad3938bad27ddda9ed9e933dba088021"
      "2dd9e28d";
  static const char kE[] = "10001";
  static const char kD[] =
      "fb9c6afd9568ce5ddac8e6a32bb881eb6cdd962bbc639dce5805548bf0fec2214f18ffd3"
      "6a50aa520cfe4477f9507d87355a24e3ff537f9f29ccffe5730b11896ebb9142982ed0df"
      "9c32ba98dddab863f3e5aa764d16ebff4500d3ee11de12fabd7aeca83c7ffa5d242b3ddc"
      "ecc64bcb5220996e79249a6d3f78975dfde769710569812dee59c0f56e4650d02a939d9c"
      "853e2cba9b0c2447a8757951ae9a0336dfa64c3d5476df9b20f200cfb52e3fbd2d4e3f34"
      "200b1171cbac367096f23366e74592025875efb6a7e3b1dd365abb0d86f34ee65ddbfa93"
      "90460da0d346833d6aa6277c0216b20073ba2f18471549c309e82d12e10714d0e0dbf146"
      "6fcd1f1";
  static const char kP[] =
      "edbe476fe8989f3966e72a20348ec6d8e924f44e1d9fa2c3485ea8a2ffd39f68574a5cef"
      "ffbb92d6764789ac0f67149127239c2027fbc55b5268a1dac6588de44e614f3bdce00f0a"
      "56d138800ad772d159a583c6548e37cadbfcf1b4ebfd50d01508986a516f36ed827b94ef"
      "1f9b4e233bf5762b3a903d2dfbbbce1fba30e9f1";
  static const char kQ[] =
      "c398518790a166dbe50498f04940d14c87ded09313fb0f69f69255c688142802ba3d4f9e"
      "f9425dadc462170635593c06a332cfc5fc9e6e1c05281950a5ce3bad4fd7cc83a38bd4ad"
      "6865594275af424f47c64c04af1caab2e261e95b975097c887d587dc8150df34cbeccd7d"
      "0688c392d9f1c617810043c9b93b884bf6ed465d";
  static const char kDMP1[] =
      "b44db5d1fa7e1d6ba44e36d59be6988a132f7294f7c484e543b27e84b82e9fdbbb2feb92"
      "1cc9fe0fe63e54fc07e66e63b3623f5ae7d7fb124a4a8e4de4556eaf327e7c5ff3207e67"
      "a1f624ba7efe6cd6b6fd5f160034a7bd92df9fd44d919d436260556f74793b181ff867b8"
      "7ea9033697978e5a349d05b9250c86c3eb2a8391";
  static const char kDMQ1[] =
      "7c06d9240265264927a6cba80a7b4c7c9fe77d10d669abb38083f85a24adcb55376d6b50"
      "9e34241cecdb5a483889f6132b672bf31aa607a242eed3669d4cf1f08b2186f0ae431bc0"
      "3de38e3f234ad7dc57e1f9103b4e0d3bd36b4cc324671968322207bd9e4e7ecb06c888e0"
      "cfc4e766f646665b3f14c0e7684ac4b98ec1948d";
  static const char kIQMP[] =
      "2887a5cb0c1bf6710e91c25da141dad92134a927431471c2d4a8b78036026d21182990e1"
      "2c1d70635f07ee551383899365a69b33d4db23e5ff7371ff4244d2c3290ce2b91ac11adc"
      "a54bb61ea5e64b9423102933ea100c12dad809fbf9589515e9d28e867f6b95c2d307f792"
      "cac28c6d7d23f441cb5b62798233db29b5cc0348";

  bssl::UniquePtr<RSA> rsa(RSA_new());
  ASSERT_TRUE(rsa);

  bssl::UniquePtr<EVP_PKEY> rsa_pkey(EVP_PKEY_new());
  ASSERT_TRUE(rsa_pkey);
  ASSERT_TRUE(EVP_PKEY_set1_RSA(rsa_pkey.get(), rsa.get()));

  // Missing n or e does not pass.
  ASSERT_TRUE(BN_hex2bn(&rsa->n, kN));
  EXPECT_FALSE(RSA_check_key(rsa.get()));

  bssl::UniquePtr<EVP_PKEY_CTX> rsa_key_ctx(
          EVP_PKEY_CTX_new(rsa_pkey.get(), NULL));
  ASSERT_TRUE(rsa_key_ctx);
  EXPECT_FALSE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_FALSE(EVP_PKEY_public_check((rsa_key_ctx.get())));
  ERR_clear_error();

  BN_free(rsa->n);
  rsa->n = nullptr;
  ASSERT_TRUE(BN_hex2bn(&rsa->e, kE));
  EXPECT_FALSE(RSA_check_key(rsa.get()));
  EXPECT_FALSE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_FALSE(EVP_PKEY_public_check((rsa_key_ctx.get())));
  ERR_clear_error();

  // Public keys pass.
  ASSERT_TRUE(BN_hex2bn(&rsa->n, kN));
  EXPECT_TRUE(RSA_check_key(rsa.get()));
  EXPECT_TRUE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_TRUE(EVP_PKEY_public_check((rsa_key_ctx.get())));

  // Invalid e values (e = 1 or e odd).
  ASSERT_TRUE(BN_hex2bn(&rsa->e, "1"));
  EXPECT_FALSE(RSA_check_key(rsa.get()));
  EXPECT_FALSE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_FALSE(EVP_PKEY_public_check((rsa_key_ctx.get())));

  // Restore the valid public key values.
  ASSERT_TRUE(BN_hex2bn(&rsa->n, kN));
  ASSERT_TRUE(BN_hex2bn(&rsa->e, kE));
  EXPECT_TRUE(RSA_check_key(rsa.get()));
  EXPECT_TRUE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_TRUE(EVP_PKEY_public_check((rsa_key_ctx.get())));

  // Configuring d also passes.
  ASSERT_TRUE(BN_hex2bn(&rsa->d, kD));
  EXPECT_TRUE(RSA_check_key(rsa.get()));
  EXPECT_TRUE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_TRUE(EVP_PKEY_public_check((rsa_key_ctx.get())));

  // p and q must be provided together.
  ASSERT_TRUE(BN_hex2bn(&rsa->p, kP));
  EXPECT_FALSE(RSA_check_key(rsa.get()));
  EXPECT_FALSE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_FALSE(EVP_PKEY_public_check((rsa_key_ctx.get())));
  ERR_clear_error();

  BN_free(rsa->p);
  rsa->p = nullptr;
  ASSERT_TRUE(BN_hex2bn(&rsa->q, kQ));
  EXPECT_FALSE(RSA_check_key(rsa.get()));
  EXPECT_FALSE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_FALSE(EVP_PKEY_public_check((rsa_key_ctx.get())));
  ERR_clear_error();

  // Supplying p and q without CRT parameters passes.
  ASSERT_TRUE(BN_hex2bn(&rsa->p, kP));
  EXPECT_TRUE(RSA_check_key(rsa.get()));
  EXPECT_TRUE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_TRUE(EVP_PKEY_public_check((rsa_key_ctx.get())));

  // With p and q together, it is sufficient to check d against e.
  ASSERT_TRUE(BN_add_word(rsa->d, 1));
  EXPECT_FALSE(RSA_check_key(rsa.get()));
  EXPECT_FALSE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_FALSE(EVP_PKEY_public_check((rsa_key_ctx.get())));
  ERR_clear_error();

  // Test another invalid d. p-1 is divisible by 3, so there is no valid value
  // of d here if e = 111. Set d to what extended GCD would have given if it
  // forgot to check the inverse existed.
  static const char kDBogus[] =
      "140be923edb928cf4340a08ada19f23da680ff20275a81e033825ee8605afc3bf6039b87"
      "f0ddc7ea3b95f214a6fdda1064d0c66b50ac7bfe8cfe6c85d3cd217ae6f5094cd72a39e5"
      "a17a9ce43eae1ba5d7d8c3fb743d8cbcb3bcd74edd0b75fcca23a0b00bcea119864c0243"
      "bf9ab32b25a4d73a1e062482f538055bc2258369353647d4325aec7a28dc1a6798e85fae"
      "85850558868468d60015927cb10b2a893e23aa16b1f9278d4413f64d0a3122218f9000ae"
      "cd8743b8e9e50bd9de81eebc4e0230d1f4f7bffc1e6f903606afba9ee694c2b40022f171"
      "a760e7c63e736e31d7c7ff8b77dc206c2a3aa5afd540073060ebb9050bddce1ff1917630"
      "47fff51d";
  ASSERT_TRUE(BN_set_word(rsa->e, 111));
  ASSERT_TRUE(BN_hex2bn(&rsa->d, kDBogus));
  EXPECT_FALSE(RSA_check_key(rsa.get()));
  EXPECT_FALSE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_FALSE(EVP_PKEY_public_check((rsa_key_ctx.get())));

  ERR_clear_error();
  ASSERT_TRUE(BN_hex2bn(&rsa->e, kE));

  // d computed via the Euler totient rather than the Carmichael totient is also
  // acceptable.
  static const char kDEuler[] =
      "3d231ff6ca0ee41ea50ab62c93bcd6aa5f01bd484e643b7ff6eb94c4dd414c17a0481a1c"
      "4361f94f3f4d5c42098af09a527cf0d8dc96122ae8dd29189a4011d62f2bb40625d2e85f"
      "4d706fb90c2e9bc9b00a0c2a28384a4c134f6d25c62d64a08fdf3f5e89a14d3daee46fda"
      "8b4a2eda87cbb2735fd47290cb37bf65150edef854a28927ce5ac36d36107711cffb8ac3"
      "2b090e409bb822b117744a9aabf878b8b1998d406337ec24cee3877795061c67322ac626"
      "6c675a2cefe0f85f06b4d24eb6ad8e3fae7f218f5bd8ff2fb8bf8176d8527b0dfdaf8490"
      "8f9bfaf3f37dcf8aa0211311bac07b1a478c3ed8a6369e5d5fc42b2afa93f5de8f520981"
      "c62bbe81";
  ASSERT_TRUE(BN_hex2bn(&rsa->d, kDEuler));
  EXPECT_TRUE(RSA_check_key(rsa.get()));
  EXPECT_TRUE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_TRUE(EVP_PKEY_public_check((rsa_key_ctx.get())));

  // If d is out of range, d > n,  but otherwise valid, it is accepted.
  static const char kDgtN[] =
      "f2c885128cf04101c283553617c210d8ffd14cde98dc420c3c9892b55606cbedcda24298"
      "7655b3f7b9433c2c316293a1cf1a2b034f197aeec1de8d81a67d94cc902b9fce1712d5a4"
      "9c257ff705725cd77338d23535d3b87c8f4cecc15a6b72641ffd81aea106839d216b5fcd"
      "7d415751d27255e540dd1638a8389721e9d0807d65d24d7b8c2f60e4b2c0bf250544ce68"
      "b5ddbc1463d5a4638b2816b0f033dacdc0162f329af9e4d142352521fbd2fe14af824ef3"
      "1601fe843c79cc3efbcb8eafd79262bdd25e2bdf21440f774e26d88ed7df938c5cf6982d"
      "e9fa635b8ca36ce5c5fbd579a53cbb0348ceae752d4bc5621c5acc922ca2082494633337"
      "42e770c1";
  ASSERT_TRUE(BN_hex2bn(&rsa->d, kDgtN));
  EXPECT_TRUE(RSA_check_key(rsa.get()));
  EXPECT_TRUE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_TRUE(EVP_PKEY_public_check((rsa_key_ctx.get())));
  ASSERT_TRUE(BN_hex2bn(&rsa->d, kD));

  // CRT value must either all be provided or all missing.
  ASSERT_TRUE(BN_hex2bn(&rsa->dmp1, kDMP1));
  EXPECT_FALSE(RSA_check_key(rsa.get()));
  EXPECT_FALSE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_FALSE(EVP_PKEY_public_check((rsa_key_ctx.get())));
  ERR_clear_error();
  BN_free(rsa->dmp1);
  rsa->dmp1 = nullptr;

  ASSERT_TRUE(BN_hex2bn(&rsa->dmq1, kDMQ1));
  EXPECT_FALSE(RSA_check_key(rsa.get()));
  EXPECT_FALSE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_FALSE(EVP_PKEY_public_check((rsa_key_ctx.get())));
  ERR_clear_error();
  BN_free(rsa->dmq1);
  rsa->dmq1 = nullptr;

  ASSERT_TRUE(BN_hex2bn(&rsa->iqmp, kIQMP));
  EXPECT_FALSE(RSA_check_key(rsa.get()));
  EXPECT_FALSE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_FALSE(EVP_PKEY_public_check((rsa_key_ctx.get())));
  ERR_clear_error();

  // The full key is accepted.
  ASSERT_TRUE(BN_hex2bn(&rsa->dmp1, kDMP1));
  ASSERT_TRUE(BN_hex2bn(&rsa->dmq1, kDMQ1));
  EXPECT_TRUE(RSA_check_key(rsa.get()));
  EXPECT_TRUE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_TRUE(EVP_PKEY_public_check((rsa_key_ctx.get())));

  // Incorrect CRT values are rejected.
  ASSERT_TRUE(BN_add_word(rsa->dmp1, 1));
  EXPECT_FALSE(RSA_check_key(rsa.get()));
  EXPECT_FALSE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_FALSE(EVP_PKEY_public_check((rsa_key_ctx.get())));
  ERR_clear_error();
  ASSERT_TRUE(BN_sub_word(rsa->dmp1, 1));

  ASSERT_TRUE(BN_add_word(rsa->dmq1, 1));
  EXPECT_FALSE(RSA_check_key(rsa.get()));
  EXPECT_FALSE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_FALSE(EVP_PKEY_public_check((rsa_key_ctx.get())));
  ERR_clear_error();
  ASSERT_TRUE(BN_sub_word(rsa->dmq1, 1));

  ASSERT_TRUE(BN_add_word(rsa->iqmp, 1));
  EXPECT_FALSE(RSA_check_key(rsa.get()));
  EXPECT_FALSE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_FALSE(EVP_PKEY_public_check((rsa_key_ctx.get())));
  ERR_clear_error();
  ASSERT_TRUE(BN_sub_word(rsa->iqmp, 1));

  // Non-reduced CRT values are rejected.
  ASSERT_TRUE(BN_add(rsa->dmp1, rsa->dmp1, rsa->p));
  EXPECT_FALSE(RSA_check_key(rsa.get()));
  EXPECT_FALSE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_FALSE(EVP_PKEY_public_check((rsa_key_ctx.get())));
  ERR_clear_error();
  ASSERT_TRUE(BN_sub(rsa->dmp1, rsa->dmp1, rsa->p));

  ASSERT_TRUE(BN_add(rsa->dmq1, rsa->dmq1, rsa->q));
  EXPECT_FALSE(RSA_check_key(rsa.get()));
  EXPECT_FALSE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_FALSE(EVP_PKEY_public_check((rsa_key_ctx.get())));
  ERR_clear_error();
  ASSERT_TRUE(BN_sub(rsa->dmq1, rsa->dmq1, rsa->q));

  ASSERT_TRUE(BN_add(rsa->iqmp, rsa->iqmp, rsa->p));
  EXPECT_FALSE(RSA_check_key(rsa.get()));
  EXPECT_FALSE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_FALSE(EVP_PKEY_public_check((rsa_key_ctx.get())));
  ERR_clear_error();
  ASSERT_TRUE(BN_sub(rsa->iqmp, rsa->iqmp, rsa->p));
}

static int rsa_priv_enc(int max_out, const uint8_t *from, uint8_t *to, RSA *rsa,
                        int padding) {
  RSA_set_ex_data(rsa, 0, (void*)"rsa_priv_enc");
  return 0;
}

static int rsa_priv_dec(int max_out, const uint8_t *from, uint8_t *to, RSA *rsa,
                        int padding) {
  RSA_set_ex_data(rsa, 0, (void*)"rsa_priv_dec");
  return 0;
}

static int rsa_pub_enc(int max_out, const uint8_t *from, uint8_t *to, RSA *rsa,
                        int padding) {
  RSA_set_ex_data(rsa, 0, (void*)"rsa_pub_enc");
  return 0;
}

static int rsa_pub_dec(int max_out, const uint8_t *from, uint8_t *to, RSA *rsa,
                        int padding) {
  RSA_set_ex_data(rsa, 0, (void*)"rsa_pub_dec");
  return 0;
}

static int extkey_rsa_finish (RSA *rsa) {
  const RSA_METHOD *meth = RSA_get_method(rsa);
  RSA_meth_free((RSA_METHOD *)meth);
  return 1;
}

TEST(RSATest, RSAMETHOD) {
  RSA *key = RSA_new();
  bssl::UniquePtr<BIGNUM> e(BN_new());
  ASSERT_TRUE(key);
  ASSERT_TRUE(e);
  ASSERT_TRUE(BN_set_word(e.get(), RSA_F4));

  ASSERT_TRUE(RSA_generate_key_ex(key, 2048, e.get(), nullptr));
  ASSERT_TRUE(RSA_check_key(key));

  // This implementation hides the key material specified by
  // |RSA_METHOD_FLAG_NO_CHECK|
  RSA_METHOD *rsa_meth = RSA_meth_new(NULL, RSA_METHOD_FLAG_NO_CHECK);
  ASSERT_TRUE(rsa_meth);

  ASSERT_TRUE(RSA_meth_set_pub_enc(rsa_meth, rsa_pub_enc));
  ASSERT_TRUE(RSA_meth_set_pub_dec(rsa_meth, rsa_pub_dec));
  ASSERT_TRUE(RSA_meth_set_priv_enc(rsa_meth, rsa_priv_enc));
  ASSERT_TRUE(RSA_meth_set_priv_dec(rsa_meth, rsa_priv_dec));
  ASSERT_TRUE(RSA_meth_set_init(rsa_meth, nullptr));
  ASSERT_TRUE(RSA_meth_set_finish(rsa_meth, extkey_rsa_finish));
  ASSERT_TRUE(RSA_meth_set0_app_data(rsa_meth, nullptr));

  ASSERT_TRUE(rsa_meth->decrypt && rsa_meth->encrypt && rsa_meth->sign_raw &&
  rsa_meth->verify_raw);

  // rsa_meth will now be freed with key when rsa_meth->finish is called
  // in RSA_free
  ASSERT_TRUE(RSA_set_method(key, rsa_meth));
  ASSERT_TRUE(RSA_is_opaque(key));

  bssl::UniquePtr<EVP_PKEY> rsa_key(EVP_PKEY_new());
  ASSERT_TRUE(rsa_key.get());
  // key will now be freed with rsa_key
  EVP_PKEY_assign_RSA(rsa_key.get(), key);
  bssl::UniquePtr<EVP_PKEY_CTX> rsa_key_ctx(EVP_PKEY_CTX_new(rsa_key.get(), NULL));
  ASSERT_TRUE(rsa_key_ctx.get());

  // Encrypt Decrypt Operations (pub_enc & priv_dec)
  size_t out_len = EVP_PKEY_size(rsa_key.get());
  uint8_t in = 0, out = 0;
  ASSERT_TRUE(EVP_PKEY_encrypt_init(rsa_key_ctx.get()));
  ASSERT_TRUE(EVP_PKEY_encrypt(rsa_key_ctx.get(), &out, &out_len, &in, 0));
  // Custom func return 0 since they don't write any data to out
  ASSERT_EQ(out_len, (size_t)0);
  ASSERT_STREQ(static_cast<const char*>(RSA_get_ex_data(key, 0))
  , "rsa_pub_enc");

  // Update before passing into next operation
  out_len = EVP_PKEY_size(rsa_key.get());
  ASSERT_TRUE(EVP_PKEY_decrypt_init(rsa_key_ctx.get()));
  ASSERT_TRUE(EVP_PKEY_decrypt(rsa_key_ctx.get(), &out, &out_len, &in, 0));
  // Custom func return 0 since they don't write any data to out
  ASSERT_EQ(out_len, (size_t)0);
  ASSERT_STREQ(static_cast<const char*>(RSA_get_ex_data(key, 0))
  , "rsa_priv_dec");

  // Update before passing into next operation
  out_len = EVP_PKEY_size(rsa_key.get());
  ASSERT_TRUE(EVP_PKEY_verify_recover_init(rsa_key_ctx.get()));
  ASSERT_TRUE(EVP_PKEY_verify_recover(rsa_key_ctx.get(), &out, &out_len,
                                      nullptr, 0));
  // Custom func return 0 since they don't write any data to out
  ASSERT_EQ(out_len, (size_t)0);
  ASSERT_STREQ(static_cast<const char*>(RSA_get_ex_data(key, 0))
  , "rsa_pub_dec");

  // Update before passing into next operation
  out_len = EVP_PKEY_size(rsa_key.get());
  // This operation is not plumbed through EVP_PKEY API in OpenSSL or AWS-LC
  ASSERT_TRUE(RSA_sign_raw(key, &out_len, &out, 0, nullptr, 0, 0));
  // Custom func return 0 since they don't write any data to out
  ASSERT_EQ(out_len, (size_t)0);
  ASSERT_STREQ(static_cast<const char*>(RSA_get_ex_data(key, 0))
  , "rsa_priv_enc");
}

TEST(RSATest, RSAEngine) {
  ENGINE *engine = ENGINE_new();
  ASSERT_TRUE(engine);
  ASSERT_FALSE(ENGINE_get_RSA(engine));

  RSA_METHOD *eng_funcs = RSA_meth_new(NULL, 0);
  ASSERT_TRUE(eng_funcs);
  ASSERT_TRUE(RSA_meth_set_priv_dec(eng_funcs, rsa_priv_dec));

  ASSERT_TRUE(ENGINE_set_RSA(engine, eng_funcs));
  ASSERT_TRUE(ENGINE_get_RSA(engine));

  RSA *key = RSA_new_method(engine);
  ASSERT_TRUE(key);

  size_t out_len = 16;
  const uint8_t in = 0;
  uint8_t out = 0;
  // Call custom Engine implementation
  ASSERT_TRUE(RSA_decrypt(key, &out_len, &out, out_len, &in, 0, 0));
  ASSERT_EQ(out_len, (size_t)0);
  ASSERT_STREQ(static_cast<const char*>(RSA_get_ex_data(key, 0))
  , "rsa_priv_dec");

  RSA_free(key);
  ENGINE_free(engine);
  RSA_meth_free(eng_funcs);
}

#if !defined(AWSLC_FIPS)

TEST(RSATest, KeygenFail) {
  bssl::UniquePtr<RSA> rsa(RSA_new());
  ASSERT_TRUE(rsa);

  // Cause RSA key generation after a prime has been generated, to test that
  // |rsa| is left alone.
  BN_GENCB cb;
  BN_GENCB_set(&cb,
               [](int event, int, BN_GENCB *) -> int { return event != 3; },
               nullptr);

  bssl::UniquePtr<BIGNUM> e(BN_new());
  ASSERT_TRUE(e);
  ASSERT_TRUE(BN_set_word(e.get(), RSA_F4));

  // Key generation should fail.
  EXPECT_FALSE(RSA_generate_key_ex(rsa.get(), 2048, e.get(), &cb));

  // Failed key generations do not leave garbage in |rsa|.
  EXPECT_FALSE(rsa->n);
  EXPECT_FALSE(rsa->e);
  EXPECT_FALSE(rsa->d);
  EXPECT_FALSE(rsa->p);
  EXPECT_FALSE(rsa->q);
  EXPECT_FALSE(rsa->dmp1);
  EXPECT_FALSE(rsa->dmq1);
  EXPECT_FALSE(rsa->iqmp);
  EXPECT_FALSE(rsa->mont_n);
  EXPECT_FALSE(rsa->mont_p);
  EXPECT_FALSE(rsa->mont_q);
  EXPECT_FALSE(rsa->d_fixed);
  EXPECT_FALSE(rsa->dmp1_fixed);
  EXPECT_FALSE(rsa->dmq1_fixed);
  EXPECT_FALSE(rsa->iqmp_mont);
  EXPECT_FALSE(rsa->private_key_frozen);

  // Failed key generations leave the previous contents alone.
  EXPECT_TRUE(RSA_generate_key_ex(rsa.get(), 2048, e.get(), nullptr));
  uint8_t *der;
  size_t der_len;
  ASSERT_TRUE(RSA_private_key_to_bytes(&der, &der_len, rsa.get()));
  bssl::UniquePtr<uint8_t> delete_der(der);

  EXPECT_FALSE(RSA_generate_key_ex(rsa.get(), 2048, e.get(), &cb));

  uint8_t *der2;
  size_t der2_len;
  ASSERT_TRUE(RSA_private_key_to_bytes(&der2, &der2_len, rsa.get()));
  bssl::UniquePtr<uint8_t> delete_der2(der2);
  EXPECT_EQ(Bytes(der, der_len), Bytes(der2, der2_len));

  // Generating a key over an existing key works, despite any cached state.
  EXPECT_TRUE(RSA_generate_key_ex(rsa.get(), 2048, e.get(), nullptr));
  EXPECT_TRUE(RSA_check_key(rsa.get()));

  bssl::UniquePtr<EVP_PKEY> rsa_pkey(EVP_PKEY_new());
  ASSERT_TRUE(rsa_pkey);
  ASSERT_TRUE(EVP_PKEY_set1_RSA(rsa_pkey.get(), rsa.get()));
  bssl::UniquePtr<EVP_PKEY_CTX> rsa_key_ctx(
          EVP_PKEY_CTX_new(rsa_pkey.get(), NULL));
  ASSERT_TRUE(rsa_key_ctx);
  EXPECT_TRUE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_TRUE(EVP_PKEY_public_check((rsa_key_ctx.get())));

  uint8_t *der3;
  size_t der3_len;
  ASSERT_TRUE(RSA_private_key_to_bytes(&der3, &der3_len, rsa.get()));
  bssl::UniquePtr<uint8_t> delete_der3(der3);
  EXPECT_NE(Bytes(der, der_len), Bytes(der3, der3_len));
}

TEST(RSATest, KeygenFailOnce) {
  bssl::UniquePtr<RSA> rsa(RSA_new());
  ASSERT_TRUE(rsa);

  // Cause only the first iteration of RSA key generation to fail.
  bool failed = false;
  BN_GENCB cb;
  BN_GENCB_set(&cb,
               [](int event, int n, BN_GENCB *cb_ptr) -> int {
                 bool *failed_ptr = static_cast<bool *>(cb_ptr->arg);
                 if (*failed_ptr) {
                   ADD_FAILURE() << "Callback called multiple times.";
                   return 1;
                 }
                 *failed_ptr = true;
                 return 0;
               },
               &failed);

  // Although key generation internally retries, the external behavior of
  // |BN_GENCB| is preserved.
  bssl::UniquePtr<BIGNUM> e(BN_new());
  ASSERT_TRUE(e);
  ASSERT_TRUE(BN_set_word(e.get(), RSA_F4));
  EXPECT_FALSE(RSA_generate_key_ex(rsa.get(), 2048, e.get(), &cb));
}

#else

// AWSLCAndroidTestRunner does not take tests that do |ASSERT_DEATH| very well.
// GTEST issue: https://github.com/google/googletest/issues/1496.
#if !defined(OPENSSL_ANDROID)

// In the case of a FIPS build, expect abort() when PWCTs in
// |RSA_generate_key_fips| fail.
TEST(RSADeathTest, KeygenFailAndDie) {
  const char *const value = getenv("BORINGSSL_FIPS_BREAK_TEST");
  if (!value || strcmp(value, "RSA_PWCT") != 0) {
    GTEST_SKIP() << "Skipping BORINGSSL_FIPS_BREAK_TESTS RSA_PWCT Test.";
  }

  // Test that all supported key lengths abort when PWCTs fail.
  for (const size_t bits : {2048, 3072, 4096}) {
    SCOPED_TRACE(bits);
    bssl::UniquePtr<RSA> rsa(RSA_new());
    ASSERT_TRUE(rsa);

    ASSERT_DEATH_IF_SUPPORTED(RSA_generate_key_fips(rsa.get(), bits, nullptr),
                              "");
  }
}
#endif

#endif


TEST(RSATest, KeygenInternalRetry) {
  bssl::UniquePtr<RSA> rsa(RSA_new());
  ASSERT_TRUE(rsa);

  // Simulate one internal attempt at key generation failing.
  bool failed = false;
  BN_GENCB cb;
  BN_GENCB_set(&cb,
               [](int event, int n, BN_GENCB *cb_ptr) -> int {
                 bool *failed_ptr = static_cast<bool *>(cb_ptr->arg);
                 if (*failed_ptr) {
                   return 1;
                 }
                 *failed_ptr = true;
                 // This test does not test any public API behavior. It is just
                 // a hack to exercise the retry codepath and make sure it
                 // works.
                 OPENSSL_PUT_ERROR(RSA, RSA_R_TOO_MANY_ITERATIONS);
                 return 0;
               },
               &failed);

  // Key generation internally retries on RSA_R_TOO_MANY_ITERATIONS.
  bssl::UniquePtr<BIGNUM> e(BN_new());
  ASSERT_TRUE(e);
  ASSERT_TRUE(BN_set_word(e.get(), RSA_F4));
  EXPECT_TRUE(RSA_generate_key_ex(rsa.get(), 2048, e.get(), &cb));
}

TEST(RSATest, OldCallback) {
  bssl::UniquePtr<RSA> rsa(RSA_new());
  ASSERT_TRUE(rsa);
  BN_GENCB cb;

  int old_callback_call_count = 0;
  void (*old_style_callback)(int, int, void *) = [](int event, int n,
                                                   void *ptr) -> void {
    BN_GENCB *cb_ptr = static_cast<BN_GENCB *>(ptr);
    int *count_ptr = static_cast<int *>(cb_ptr->arg);
    *count_ptr += 1;
  };

  int new_callback_call_count = 0;
  int (*new_style_callback)(int, int, BN_GENCB *cb_ptr) = [](int event, int n,
                                                    BN_GENCB *cb_ptr) -> int {
    int *count_ptr = static_cast<int *>(cb_ptr->arg);
    *count_ptr += 1;
    return 1;
  };

  // Set the new style first, setting the old should overwrite the new callback
  BN_GENCB_set(&cb, new_style_callback, &new_callback_call_count);
  BN_GENCB_set_old(&cb, old_style_callback, &old_callback_call_count);

  bssl::UniquePtr<BIGNUM> e(BN_new());
  ASSERT_TRUE(e);
  ASSERT_TRUE(BN_set_word(e.get(), RSA_F4));
  EXPECT_TRUE(RSA_generate_key_ex(rsa.get(), 2048, e.get(), &cb));
  // The old callback should have been called at least once
  EXPECT_NE(old_callback_call_count, 0);
  // The new callback shouldn't have been called
  EXPECT_EQ(new_callback_call_count, 0);
  int previous_count = old_callback_call_count;

  // Set the new style again and overwrite the old style
  BN_GENCB_set(&cb, new_style_callback, &new_callback_call_count);
  EXPECT_TRUE(RSA_generate_key_ex(rsa.get(), 2048, e.get(), &cb));
  // The old callback should have been overwritten and not called
  EXPECT_EQ(previous_count, old_callback_call_count);
  // The new callback should have been called at least once
  EXPECT_NE(new_callback_call_count, 0);
}

// Test that, after a key has been used, it can still be modified into another
// key.
TEST(RSATest, OverwriteKey) {
  // Make a key and perform public and private key operations with it, so that
  // all derived values are filled in.
  bssl::UniquePtr<RSA> key1(
      RSA_private_key_from_bytes(kKey1, sizeof(kKey1) - 1));
  ASSERT_TRUE(key1);

  ASSERT_TRUE(RSA_check_key(key1.get()));
  bssl::UniquePtr<EVP_PKEY> rsa_pkey(EVP_PKEY_new());
  ASSERT_TRUE(rsa_pkey);
  ASSERT_TRUE(EVP_PKEY_set1_RSA(rsa_pkey.get(), key1.get()));
  bssl::UniquePtr<EVP_PKEY_CTX> rsa_key_ctx(
          EVP_PKEY_CTX_new(rsa_pkey.get(), NULL));
  ASSERT_TRUE(rsa_key_ctx);
  EXPECT_TRUE(EVP_PKEY_check(rsa_key_ctx.get()));
  EXPECT_TRUE(EVP_PKEY_public_check((rsa_key_ctx.get())));

  size_t len;
  std::vector<uint8_t> ciphertext(RSA_size(key1.get()));
  ASSERT_TRUE(RSA_encrypt(key1.get(), &len, ciphertext.data(),
                          ciphertext.size(), kPlaintext, kPlaintextLen,
                          RSA_PKCS1_OAEP_PADDING));
  ciphertext.resize(len);

  std::vector<uint8_t> plaintext(RSA_size(key1.get()));
  ASSERT_TRUE(RSA_decrypt(key1.get(), &len, plaintext.data(),
                          plaintext.size(), ciphertext.data(), ciphertext.size(),
                          RSA_PKCS1_OAEP_PADDING));
  plaintext.resize(len);
  EXPECT_EQ(Bytes(plaintext), Bytes(kPlaintext, kPlaintextLen));

  // Overwrite |key1| with the contents of |key2|.
  bssl::UniquePtr<RSA> key2(
      RSA_private_key_from_bytes(kKey2, sizeof(kKey2) - 1));
  ASSERT_TRUE(key2);

  auto copy_rsa_fields = [](RSA *dst, const RSA *src) {
    bssl::UniquePtr<BIGNUM> n(BN_dup(RSA_get0_n(src)));
    ASSERT_TRUE(n);
    bssl::UniquePtr<BIGNUM> e(BN_dup(RSA_get0_e(src)));
    ASSERT_TRUE(e);
    bssl::UniquePtr<BIGNUM> d(BN_dup(RSA_get0_d(src)));
    ASSERT_TRUE(d);
    bssl::UniquePtr<BIGNUM> p(BN_dup(RSA_get0_p(src)));
    ASSERT_TRUE(p);
    bssl::UniquePtr<BIGNUM> q(BN_dup(RSA_get0_q(src)));
    ASSERT_TRUE(q);
    bssl::UniquePtr<BIGNUM> dmp1(BN_dup(RSA_get0_dmp1(src)));
    ASSERT_TRUE(dmp1);
    bssl::UniquePtr<BIGNUM> dmq1(BN_dup(RSA_get0_dmq1(src)));
    ASSERT_TRUE(dmq1);
    bssl::UniquePtr<BIGNUM> iqmp(BN_dup(RSA_get0_iqmp(src)));
    ASSERT_TRUE(iqmp);
    ASSERT_TRUE(RSA_set0_key(dst, n.release(), e.release(), d.release()));
    ASSERT_TRUE(RSA_set0_factors(dst, p.release(), q.release()));
    ASSERT_TRUE(RSA_set0_crt_params(dst, dmp1.release(), dmq1.release(),
                                    iqmp.release()));
  };
  ASSERT_NO_FATAL_FAILURE(copy_rsa_fields(key1.get(), key2.get()));

  auto check_rsa_compatible = [&](RSA *enc, RSA *dec) {
    ciphertext.resize(RSA_size(enc));
    ASSERT_TRUE(RSA_encrypt(enc, &len, ciphertext.data(),
                            ciphertext.size(), kPlaintext, kPlaintextLen,
                            RSA_PKCS1_OAEP_PADDING));
    ciphertext.resize(len);

    plaintext.resize(RSA_size(dec));
    ASSERT_TRUE(RSA_decrypt(dec, &len, plaintext.data(),
                            plaintext.size(), ciphertext.data(),
                            ciphertext.size(), RSA_PKCS1_OAEP_PADDING));
    plaintext.resize(len);
    EXPECT_EQ(Bytes(plaintext), Bytes(kPlaintext, kPlaintextLen));
  };

  ASSERT_NO_FATAL_FAILURE(
      check_rsa_compatible(/*enc=*/key1.get(), /*dec=*/key2.get()));
  ASSERT_NO_FATAL_FAILURE(
      check_rsa_compatible(/*enc=*/key2.get(), /*dec=*/key1.get()));

  // If we generate a new key on top of |key1|, it should be usable and
  // self-consistent. We test this by making a new key with the same parameters
  // and checking they behave the same.
  ASSERT_TRUE(
      RSA_generate_key_ex(key1.get(), 1024, RSA_get0_e(key2.get()), nullptr));
  EXPECT_NE(0, BN_cmp(RSA_get0_n(key1.get()), RSA_get0_n(key2.get())));

  key2.reset(RSA_new());
  ASSERT_TRUE(key2);
  ASSERT_NO_FATAL_FAILURE(copy_rsa_fields(key2.get(), key1.get()));
  ASSERT_NO_FATAL_FAILURE(
      check_rsa_compatible(/*enc=*/key1.get(), /*dec=*/key2.get()));
  ASSERT_NO_FATAL_FAILURE(
      check_rsa_compatible(/*enc=*/key2.get(), /*dec=*/key1.get()));
}

TEST(RSATest, PrintBio) {
  bssl::UniquePtr<RSA> rsa(
      RSA_private_key_from_bytes(kKey1, sizeof(kKey1) - 1));
  ASSERT_TRUE(rsa);
  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
  ASSERT_TRUE(bio);

  RSA_print(bio.get(), rsa.get(), 4);
  const uint8_t *data;
  size_t len;
  BIO_mem_contents(bio.get(), &data, &len);

  const char *expected = ""
      "    Private-Key: (512 bit, 2 primes)\n"
      "    modulus:\n"
      "        00:aa:36:ab:ce:88:ac:fd:ff:55:52:3c:7f:c4:52:\n"
      "        3f:90:ef:a0:0d:f3:77:4a:25:9f:2e:62:b4:c5:d9:\n"
      "        9c:b5:ad:b3:00:a0:28:5e:53:01:93:0e:0c:70:fb:\n"
      "        68:76:93:9c:e6:16:ce:62:4a:11:e0:08:6d:34:1e:\n"
      "        bc:ac:a0:a1:f5\n"
      "    publicExponent: 17 (0x11)\n"
      "    privateExponent:\n"
      "        0a:03:37:48:62:64:87:69:5f:5f:30:bc:38:b9:8b:\n"
      "        44:c2:cd:2d:ff:43:40:98:cd:20:d8:a1:38:d0:90:\n"
      "        bf:64:79:7c:3f:a7:a2:cd:cb:3c:d1:e0:bd:ba:26:\n"
      "        54:b4:f9:df:8e:8a:e5:9d:73:3d:9f:33:b3:01:62:\n"
      "        4a:fd:1d:51\n"
      "    prime1:\n"
      "        00:d8:40:b4:16:66:b4:2e:92:ea:0d:a3:b4:32:04:\n"
      "        b5:cf:ce:33:52:52:4d:04:16:a5:a4:41:e7:00:af:\n"
      "        46:12:0d\n"
      "    prime2:\n"
      "        00:c9:7f:b1:f0:27:f4:53:f6:34:12:33:ea:aa:d1:\n"
      "        d9:35:3f:6c:42:d0:88:66:b1:d0:5a:0f:20:35:02:\n"
      "        8b:9d:89\n"
      "    exponent1:\n"
      "        59:0b:95:72:a2:c2:a9:c4:06:05:9d:c2:ab:2f:1d:\n"
      "        af:eb:7e:8b:4f:10:a7:54:9e:8e:ed:f5:b4:fc:e0:\n"
      "        9e:05\n"
      "    exponent2:\n"
      "        00:8e:3c:05:21:fe:15:e0:ea:06:a3:6f:f0:f1:0c:\n"
      "        99:52:c3:5b:7a:75:14:fd:32:38:b8:0a:ad:52:98:\n"
      "        62:8d:51\n"
      "    coefficient:\n"
      "        36:3f:f7:18:9d:a8:e9:0b:1d:34:1f:71:d0:9b:76:\n"
      "        a8:a9:43:e1:1d:10:b2:4d:24:9f:2d:ea:fe:f8:0c:\n"
      "        18:26\n";

  ASSERT_EQ(Bytes(expected), Bytes(data, len));

#if !defined(OPENSSL_ANDROID)
  // On Android, when running from an APK, |tmpfile| does not work. See
  // b/36991167#comment8.
  TempFILE tmp = createTempFILE();
  ASSERT_TRUE(tmp);
  ASSERT_TRUE(RSA_print_fp(tmp.get(), rsa.get(), 4));
  fseek(tmp.get(), 0, SEEK_END);
  long fileSize = ftell(tmp.get());
  ASSERT_GT(fileSize, 0);
  rewind(tmp.get());
  std::unique_ptr<uint8_t[]> buf(new uint8_t[fileSize]);
  size_t bytesRead = fread(buf.get(), 1, fileSize, tmp.get());
  ASSERT_EQ(bytesRead, (size_t)fileSize);
  ASSERT_EQ(Bytes(expected), Bytes(buf.get(), fileSize));
#endif
}

// Test that RSA keys do not support operations will cleanly fail them.
TEST(RSATest, MissingParameters) {
  bssl::UniquePtr<RSA> sample(
      RSA_private_key_from_bytes(kKey1, sizeof(kKey1) - 1));
  ASSERT_TRUE(sample);

  // Make a sample signature.
  const uint8_t kZeros[32] = {0};
  std::vector<uint8_t> sig(RSA_size(sample.get()));
  unsigned len_u;
  ASSERT_TRUE(RSA_sign(NID_sha256, kZeros, sizeof(kZeros), sig.data(), &len_u,
                       sample.get()));
  sig.resize(len_u);

  // A public key cannot perform private key operations.
  bssl::UniquePtr<RSA> rsa(
      RSA_new_public_key(RSA_get0_n(sample.get()), RSA_get0_e(sample.get())));
  ASSERT_TRUE(rsa);
  std::vector<uint8_t> out(RSA_size(sample.get()));
  EXPECT_FALSE(RSA_sign(NID_sha256, kZeros, sizeof(kZeros), out.data(), &len_u,
                        rsa.get()));
  EXPECT_TRUE(ErrorEquals(ERR_get_error(), ERR_LIB_RSA, RSA_R_VALUE_MISSING));

  size_t len;
  EXPECT_FALSE(RSA_decrypt(rsa.get(), &len, out.data(), out.size(),
                           kOAEPCiphertext1, sizeof(kOAEPCiphertext1) - 1,
                           RSA_PKCS1_OAEP_PADDING));
  EXPECT_TRUE(ErrorEquals(ERR_get_error(), ERR_LIB_RSA, RSA_R_VALUE_MISSING));

  // A private key without e cannot perform public key operations.
  rsa.reset(RSA_new_private_key_no_e(RSA_get0_n(sample.get()),
                                     RSA_get0_d(sample.get())));
  ASSERT_TRUE(rsa);
  EXPECT_FALSE(RSA_verify(NID_sha256, kZeros, sizeof(kZeros), sig.data(),
                          sig.size(), rsa.get()));
  EXPECT_TRUE(ErrorEquals(ERR_get_error(), ERR_LIB_RSA, RSA_R_VALUE_MISSING));

  EXPECT_FALSE(RSA_encrypt(rsa.get(), &len, out.data(), out.size(), kPlaintext,
                           sizeof(kPlaintext), RSA_PKCS1_OAEP_PADDING));
  EXPECT_TRUE(ErrorEquals(ERR_get_error(), ERR_LIB_RSA, RSA_R_VALUE_MISSING));
}

TEST(RSATest, Negative) {
  auto dup_neg = [](const BIGNUM *bn) -> bssl::UniquePtr<BIGNUM> {
    bssl::UniquePtr<BIGNUM> ret(BN_dup(bn));
    if (!ret) {
      return nullptr;
    }
    BN_set_negative(ret.get(), 1);
    return ret;
  };

  bssl::UniquePtr<RSA> key(
      RSA_private_key_from_bytes(kFIPSKey, sizeof(kFIPSKey) - 1));
  ASSERT_TRUE(key);
  const BIGNUM *n = RSA_get0_n(key.get());
  bssl::UniquePtr<BIGNUM> neg_n = dup_neg(n);
  ASSERT_TRUE(neg_n);
  const BIGNUM *e = RSA_get0_e(key.get());
  bssl::UniquePtr<BIGNUM> neg_e = dup_neg(e);
  ASSERT_TRUE(neg_e);
  const BIGNUM *d = RSA_get0_d(key.get());
  bssl::UniquePtr<BIGNUM> neg_d = dup_neg(d);
  ASSERT_TRUE(neg_d);
  const BIGNUM *p = RSA_get0_p(key.get());
  bssl::UniquePtr<BIGNUM> neg_p = dup_neg(p);
  ASSERT_TRUE(neg_p);
  const BIGNUM *q = RSA_get0_q(key.get());
  bssl::UniquePtr<BIGNUM> neg_q = dup_neg(q);
  ASSERT_TRUE(neg_q);
  const BIGNUM *dmp1 = RSA_get0_dmp1(key.get());
  bssl::UniquePtr<BIGNUM> neg_dmp1 = dup_neg(dmp1);
  ASSERT_TRUE(neg_dmp1);
  const BIGNUM *dmq1 = RSA_get0_dmq1(key.get());
  bssl::UniquePtr<BIGNUM> neg_dmq1 = dup_neg(dmq1);
  ASSERT_TRUE(neg_dmq1);
  const BIGNUM *iqmp = RSA_get0_iqmp(key.get());
  bssl::UniquePtr<BIGNUM> neg_iqmp = dup_neg(iqmp);
  ASSERT_TRUE(neg_iqmp);

  EXPECT_FALSE(RSA_new_public_key(neg_n.get(), e));
  EXPECT_FALSE(RSA_new_public_key(n, neg_e.get()));
  EXPECT_FALSE(RSA_new_private_key(neg_n.get(), e, d, p, q, dmp1, dmq1, iqmp));
  EXPECT_FALSE(RSA_new_private_key(n, neg_e.get(), d, p, q, dmp1, dmq1, iqmp));
  EXPECT_FALSE(RSA_new_private_key(n, e, neg_d.get(), p, q, dmp1, dmq1, iqmp));
  EXPECT_FALSE(RSA_new_private_key(n, e, d, neg_p.get(), q, dmp1, dmq1, iqmp));
  EXPECT_FALSE(RSA_new_private_key(n, e, d, p, neg_q.get(), dmp1, dmq1, iqmp));
  EXPECT_FALSE(RSA_new_private_key(n, e, d, p, q, neg_dmp1.get(), dmq1, iqmp));
  EXPECT_FALSE(RSA_new_private_key(n, e, d, p, q, dmp1, neg_dmq1.get(), iqmp));
  EXPECT_FALSE(RSA_new_private_key(n, e, d, p, q, dmp1, dmq1, neg_iqmp.get()));
}

TEST(RSATest, LargeE) {
  // Test an RSA key with large e by swapping d and e in kFIPSKey. Since e is
  // small, e mod (p-1) and e mod (q-1) will simply be e.
  bssl::UniquePtr<RSA> key(
      RSA_private_key_from_bytes(kFIPSKey, sizeof(kFIPSKey) - 1));
  ASSERT_TRUE(key);
  const BIGNUM *n = RSA_get0_n(key.get());
  const BIGNUM *e = RSA_get0_e(key.get());
  const BIGNUM *d = RSA_get0_d(key.get());
  const BIGNUM *p = RSA_get0_p(key.get());
  const BIGNUM *q = RSA_get0_q(key.get());
  const BIGNUM *iqmp = RSA_get0_iqmp(key.get());

  // By default, the large exponent is not allowed as e.
  bssl::UniquePtr<RSA> pub(RSA_new_public_key(n, /*e=*/d));
  EXPECT_FALSE(pub);
  bssl::UniquePtr<RSA> priv(RSA_new_private_key(n, /*e=*/d, /*d=*/e, p, q,
                                                /*dmp1=*/e, /*dmq1=*/e, iqmp));
  EXPECT_FALSE(priv);

  // But the "large e" APIs tolerate it.
  pub.reset(RSA_new_public_key_large_e(n, /*e=*/d));
  ASSERT_TRUE(pub);
  priv.reset(RSA_new_private_key_large_e(n, /*e=*/d, /*d=*/e, p, q, /*dmp1=*/e,
                                         /*dmq1=*/e, iqmp));
  ASSERT_TRUE(priv);

  // Test that operations work correctly.
  static const uint8_t kDigest[32] = {0};
  std::vector<uint8_t> sig(RSA_size(priv.get()));
  size_t len;
  ASSERT_TRUE(RSA_sign_pss_mgf1(priv.get(), &len, sig.data(), sig.size(),
                                kDigest, sizeof(kDigest), EVP_sha256(),
                                EVP_sha256(), /*salt_len=*/32));
  sig.resize(len);

  EXPECT_TRUE(RSA_verify_pss_mgf1(pub.get(), kDigest, sizeof(kDigest),
                                  EVP_sha256(), EVP_sha256(), /*salt_len=*/32,
                                  sig.data(), sig.size()));

  // e = 1 is still invalid.
  EXPECT_FALSE(RSA_new_public_key_large_e(n, BN_value_one()));

  // e must still be odd.
  bssl::UniquePtr<BIGNUM> bad_e(BN_dup(d));
  ASSERT_TRUE(bad_e);
  ASSERT_TRUE(BN_add_word(bad_e.get(), 1));
  EXPECT_FALSE(RSA_new_public_key_large_e(n, bad_e.get()));

  // e must still be bounded by n.
  bad_e.reset(BN_dup(n));
  ASSERT_TRUE(bad_e);
  ASSERT_TRUE(BN_add_word(bad_e.get(), 2));  // Preserve parity.
  EXPECT_FALSE(RSA_new_public_key_large_e(n, bad_e.get()));
}

#if !defined(BORINGSSL_SHARED_LIBRARY)
TEST(RSATest, SqrtTwo) {
  bssl::UniquePtr<BIGNUM> sqrt(BN_new()), pow2(BN_new());
  bssl::UniquePtr<BN_CTX> ctx(BN_CTX_new());
  ASSERT_TRUE(sqrt);
  ASSERT_TRUE(pow2);
  ASSERT_TRUE(ctx);

  size_t bits = kBoringSSLRSASqrtTwoLen * BN_BITS2;
  ASSERT_TRUE(BN_one(pow2.get()));
  ASSERT_TRUE(BN_lshift(pow2.get(), pow2.get(), 2 * bits - 1));

  // Check that sqrt² < pow2.
  ASSERT_TRUE(
      bn_set_words(sqrt.get(), kBoringSSLRSASqrtTwo, kBoringSSLRSASqrtTwoLen));
  ASSERT_TRUE(BN_sqr(sqrt.get(), sqrt.get(), ctx.get()));
  EXPECT_LT(BN_cmp(sqrt.get(), pow2.get()), 0);

  // Check that pow2 < (sqrt + 1)².
  ASSERT_TRUE(
      bn_set_words(sqrt.get(), kBoringSSLRSASqrtTwo, kBoringSSLRSASqrtTwoLen));
  ASSERT_TRUE(BN_add_word(sqrt.get(), 1));
  ASSERT_TRUE(BN_sqr(sqrt.get(), sqrt.get(), ctx.get()));
  EXPECT_LT(BN_cmp(pow2.get(), sqrt.get()), 0);

  // Check the kBoringSSLRSASqrtTwo is sized for a 4096-bit RSA key.
  EXPECT_EQ(4096u / 2u, bits);
}
#endif  // !BORINGSSL_SHARED_LIBRARY

#if defined(OPENSSL_THREADS)
TEST(RSATest, Threads) {
  bssl::UniquePtr<RSA> rsa_template(
      RSA_private_key_from_bytes(kKey1, sizeof(kKey1) - 1));
  ASSERT_TRUE(rsa_template);

  const uint8_t kDummyHash[32] = {0};
  uint8_t sig[256];
  unsigned sig_len = sizeof(sig);
  ASSERT_LE(RSA_size(rsa_template.get()), sizeof(sig));
  EXPECT_TRUE(RSA_sign(NID_sha256, kDummyHash, sizeof(kDummyHash), sig,
                       &sig_len, rsa_template.get()));

  // RSA keys may be assembled piece-meal and then used in parallel between
  // threads, which requires internal locking to create some derived properties.
  bssl::UniquePtr<RSA> rsa(RSA_new());
  rsa->n = BN_dup(rsa_template->n);
  ASSERT_TRUE(rsa->n);
  rsa->e = BN_dup(rsa_template->e);
  ASSERT_TRUE(rsa->e);
  rsa->d = BN_dup(rsa_template->d);
  ASSERT_TRUE(rsa->d);
  rsa->p = BN_dup(rsa_template->p);
  ASSERT_TRUE(rsa->p);
  rsa->q = BN_dup(rsa_template->q);
  ASSERT_TRUE(rsa->q);
  rsa->dmp1 = BN_dup(rsa_template->dmp1);
  ASSERT_TRUE(rsa->dmp1);
  rsa->dmq1 = BN_dup(rsa_template->dmq1);
  ASSERT_TRUE(rsa->dmq1);
  rsa->iqmp = BN_dup(rsa_template->iqmp);
  ASSERT_TRUE(rsa->iqmp);

  // Each of these operations must be safe to do concurrently on different
  // threads.
  auto raw_access = [&] { EXPECT_EQ(0, BN_cmp(rsa->d, rsa_template->d)); };
  auto getter = [&] {
    const BIGNUM *d;
    RSA_get0_key(rsa.get(), nullptr, nullptr, &d);
    EXPECT_EQ(0, BN_cmp(d, rsa_template->d));
  };
  auto sign = [&] {
    uint8_t sig2[256];
    unsigned sig2_len = sizeof(sig2);
    ASSERT_LE(RSA_size(rsa.get()), sizeof(sig2));
    EXPECT_TRUE(RSA_sign(NID_sha256, kDummyHash, sizeof(kDummyHash), sig2,
                         &sig2_len, rsa.get()));
    // RSASSA-PKCS1-v1_5 is deterministic.
    EXPECT_EQ(Bytes(sig, sig_len), Bytes(sig2, sig2_len));
  };
  auto verify = [&] {
    EXPECT_TRUE(RSA_verify(NID_sha256, kDummyHash, sizeof(kDummyHash), sig,
                           sig_len, rsa.get()));
  };

  std::vector<std::thread> threads;
  threads.emplace_back(raw_access);
  threads.emplace_back(raw_access);
  threads.emplace_back(getter);
  threads.emplace_back(getter);
  threads.emplace_back(sign);
  threads.emplace_back(sign);
  threads.emplace_back(verify);
  threads.emplace_back(verify);
  for (auto &thread : threads) {
    thread.join();
  }
}

// This test might be excessively slow on slower CPUs or platforms that do not
// expect server workloads. It is disabled by default and reenabled on some
// platforms when running tests standalone via all_tests.go.
//
// Additionally, even when running disabled tests standalone, limit this to
// x86_64. On other platforms, this test hits resource limits or is too slow. We
// also disable on FreeBSD. See https://crbug.com/boringssl/603.
#if defined(OPENSSL_TSAN) || \
    (defined(OPENSSL_X86_64) && !defined(OPENSSL_FREEBSD))
TEST(RSATest, DISABLED_BlindingCacheConcurrency) {
  bssl::UniquePtr<RSA> rsa(
      RSA_private_key_from_bytes(kKey1, sizeof(kKey1) - 1));
  ASSERT_TRUE(rsa);

#if defined(OPENSSL_TSAN)
  constexpr size_t kSignaturesPerThread = 10;
  constexpr size_t kNumThreads = 10;
#else
  constexpr size_t kSignaturesPerThread = 100;
  constexpr size_t kNumThreads = 2048;
#endif
  // On some platforms, the number of threads should be reduced because resources are limited.
  // e.g. Travis CI MacOS has 2 cores and 4 GB memories.
  size_t numOfThreads = kNumThreads;
  const char* rsaThreadsLimit = getenv("RSA_TEST_THREADS_LIMIT");
  if (rsaThreadsLimit != nullptr) {
    numOfThreads = std::stoul(std::string(rsaThreadsLimit), nullptr);
  }
  const uint8_t kDummyHash[32] = {0};
  auto worker = [&] {
    uint8_t sig[256];
    ASSERT_LE(RSA_size(rsa.get()), sizeof(sig));

    for (size_t i = 0; i < kSignaturesPerThread; i++) {
      unsigned sig_len = sizeof(sig);
      EXPECT_TRUE(RSA_sign(NID_sha256, kDummyHash, sizeof(kDummyHash), sig,
                           &sig_len, rsa.get()));
    }
  };

  std::vector<std::thread> threads;
  threads.reserve(numOfThreads);
  for (size_t i = 0; i < numOfThreads; i++) {
    threads.emplace_back(worker);
  }
  for (auto &thread : threads) {
    thread.join();
  }
}
#endif  // TSAN || (X86_64 && !FREEBSD)

#endif  // THREADS
