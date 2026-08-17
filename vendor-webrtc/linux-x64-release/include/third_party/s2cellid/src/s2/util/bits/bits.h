// Copyright 2002 Google Inc. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS-IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

#ifndef S2_UTIL_BITS_BITS_H_
#define S2_UTIL_BITS_BITS_H_

//
// Various bit-twiddling functions, all of which are static members of the Bits
// class (making it effectively a namespace). Operands are unsigned integers.
// Munging bits in _signed_ integers is fraught with peril! For example,
// -5 << n has undefined behavior (for some values of n).
//
// Bits provide the following:
//
//   * Log2(Floor|Ceiling)(NonZero)?.* - The NonZero variants have undefined
//     behavior if argument is 0.
//
// The only other thing is BitPattern, which is a trait class template (not in
// Bits) containing a few bit patterns (which vary based on value of template
// parameter).

#if defined(__i386__) || defined(__x86_64__)
#include <x86intrin.h>
#endif

#include "base/logging.h"

class Bits {
 public:
  // Return floor(log2(n)) for positive integer n.  Returns -1 iff n == 0.
  static int Log2Floor(uint32_t n);
  static int Log2Floor64(uint64_t n);

  // Potentially faster version of Log2Floor() that returns an
  // undefined value if n == 0
  static int Log2FloorNonZero(uint32_t n);
  static int Log2FloorNonZero64(uint64_t n);

  // Return the first set least / most significant bit, 0-indexed.  Returns an
  // undefined value if n == 0.  FindLSBSetNonZero() is similar to ffs() except
  // that it's 0-indexed, while FindMSBSetNonZero() is the same as
  // Log2FloorNonZero().
  static int FindLSBSetNonZero(uint32_t n);
  static int FindLSBSetNonZero64(uint64_t n);
  static int FindMSBSetNonZero(uint32_t n) { return Log2FloorNonZero(n); }
  static int FindMSBSetNonZero64(uint64_t n) { return Log2FloorNonZero64(n); }

 private:
  // Portable implementations.
  static int Log2Floor_Portable(uint32_t n);
  static int Log2Floor64_Portable(uint64_t n);
  static int Log2FloorNonZero_Portable(uint32_t n);
  static int Log2FloorNonZero64_Portable(uint64_t n);
  static int FindLSBSetNonZero_Portable(uint32_t n);
  static int FindLSBSetNonZero64_Portable(uint64_t n);

  Bits(Bits const&) = delete;
  void operator=(Bits const&) = delete;
};

// ------------------------------------------------------------------------
// Implementation details follow
// ------------------------------------------------------------------------

#if defined(__GNUC__)

inline int Bits::Log2Floor(uint32_t n) {
  return n == 0 ? -1 : 31 ^ __builtin_clz(n);
}

inline int Bits::Log2FloorNonZero(uint32_t n) {
  return 31 ^ __builtin_clz(n);
}

inline int Bits::FindLSBSetNonZero(uint32_t n) {
  return __builtin_ctz(n);
}

inline int Bits::Log2Floor64(uint64_t n) {
  return n == 0 ? -1 : 63 ^ __builtin_clzll(n);
}

inline int Bits::Log2FloorNonZero64(uint64_t n) {
  return 63 ^ __builtin_clzll(n);
}

inline int Bits::FindLSBSetNonZero64(uint64_t n) {
  return __builtin_ctzll(n);
}

#elif defined(_MSC_VER)

inline int Bits::FindLSBSetNonZero(uint32_t n) {
  return Bits::FindLSBSetNonZero_Portable(n);
}

inline int Bits::FindLSBSetNonZero64(uint64_t n) {
  return Bits::FindLSBSetNonZero64_Portable(n);
}

inline int Bits::Log2FloorNonZero(uint32_t n) {
#ifdef _M_IX86
  _asm {
    bsr ebx, n
    mov n, ebx
  }
  return n;
#else
  return Bits::Log2FloorNonZero_Portable(n);
#endif
}

inline int Bits::Log2Floor(uint32_t n) {
#ifdef _M_IX86
  _asm {
    xor ebx, ebx
    mov eax, n
    and eax, eax
    jz return_ebx
    bsr ebx, eax
return_ebx:
    mov n, ebx
  }
  return n;
#else
  return Bits::Log2Floor_Portable(n);
#endif
}

inline int Bits::Log2Floor64(uint64_t n) {
  return Bits::Log2Floor64_Portable(n);
}

inline int Bits::Log2FloorNonZero64(uint64_t n) {
  return Bits::Log2FloorNonZero64_Portable(n);
}

#else  // !__GNUC__ && !_MSC_VER

inline int Bits::Log2Floor(uint32_t n) {
  return Bits::Log2Floor_Portable(n);
}

inline int Bits::Log2FloorNonZero(uint32_t n) {
  return Bits::Log2FloorNonZero_Portable(n);
}

inline int Bits::FindLSBSetNonZero(uint32_t n) {
  return Bits::FindLSBSetNonZero_Portable(n);
}

inline int Bits::Log2Floor64(uint64_t n) {
  return Bits::Log2Floor64_Portable(n);
}

inline int Bits::Log2FloorNonZero64(uint64_t n) {
  return Bits::Log2FloorNonZero64_Portable(n);
}

inline int Bits::FindLSBSetNonZero64(uint64_t n) {
  return Bits::FindLSBSetNonZero64_Portable(n);
}

#endif

inline int Bits::Log2Floor_Portable(uint32_t n) {
  if (n == 0)
    return -1;
  int log = 0;
  uint32_t value = n;
  for (int i = 4; i >= 0; --i) {
    int shift = (1 << i);
    uint32_t x = value >> shift;
    if (x != 0) {
      value = x;
      log += shift;
    }
  }
  assert(value == 1);
  return log;
}

inline int Bits::Log2FloorNonZero_Portable(uint32_t n) {
  // Just use the common routine
  return Log2Floor(n);
}

// Log2Floor64() is defined in terms of Log2Floor32(), Log2FloorNonZero32()
inline int Bits::Log2Floor64_Portable(uint64_t n) {
  const uint32_t topbits = static_cast<uint32_t>(n >> 32);
  if (topbits == 0) {
    // Top bits are zero, so scan in bottom bits
    return Log2Floor(static_cast<uint32_t>(n));
  } else {
    return 32 + Log2FloorNonZero(topbits);
  }
}

// Log2FloorNonZero64() is defined in terms of Log2FloorNonZero32()
inline int Bits::Log2FloorNonZero64_Portable(uint64_t n) {
  const uint32_t topbits = static_cast<uint32_t>(n >> 32);
  if (topbits == 0) {
    // Top bits are zero, so scan in bottom bits
    return Log2FloorNonZero(static_cast<uint32_t>(n));
  } else {
    return 32 + Log2FloorNonZero(topbits);
  }
}

inline int Bits::FindLSBSetNonZero_Portable(uint32_t n) {
  int rc = 31;
  for (int i = 4, shift = 1 << 4; i >= 0; --i) {
    const uint32_t x = n << shift;
    if (x != 0) {
      n = x;
      rc -= shift;
    }
    shift >>= 1;
  }
  return rc;
}

// FindLSBSetNonZero64() is defined in terms of FindLSBSetNonZero()
inline int Bits::FindLSBSetNonZero64_Portable(uint64_t n) {
  const uint32_t bottombits = static_cast<uint32_t>(n);
  if (bottombits == 0) {
    // Bottom bits are zero, so scan in top bits
    return 32 + FindLSBSetNonZero(static_cast<uint32_t>(n >> 32));
  } else {
    return FindLSBSetNonZero(bottombits);
  }
}

#endif  // S2_UTIL_BITS_BITS_H_
