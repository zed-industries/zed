#ifndef FLATBUFFERS_BASE_H_
#define FLATBUFFERS_BASE_H_

// clang-format off

// If activate should be declared and included first.
#if defined(FLATBUFFERS_MEMORY_LEAK_TRACKING) && \
    defined(_MSC_VER) && defined(_DEBUG)
  // The _CRTDBG_MAP_ALLOC inside <crtdbg.h> will replace
  // calloc/free (etc) to its debug version using #define directives.
  #define _CRTDBG_MAP_ALLOC
  #include <stdlib.h>
  #include <crtdbg.h>
  // Replace operator new by trace-enabled version.
  #define DEBUG_NEW new(_NORMAL_BLOCK, __FILE__, __LINE__)
  #define new DEBUG_NEW
#endif

#if !defined(FLATBUFFERS_ASSERT)
#include <assert.h>
#define FLATBUFFERS_ASSERT assert
#elif defined(FLATBUFFERS_ASSERT_INCLUDE)
// Include file with forward declaration
#include FLATBUFFERS_ASSERT_INCLUDE
#endif

#ifndef ARDUINO
#include <cstdint>
#endif

#include <cstddef>
#include <cstdlib>
#include <cstring>

#if defined(ARDUINO) && !defined(ARDUINOSTL_M_H) && defined(__AVR__)
  #include <utility.h>
#else
  #include <utility>
#endif

#include <string>
#include <type_traits>
#include <vector>
#include <set>
#include <algorithm>
#include <limits>
#include <iterator>
#include <memory>

#if defined(__unix__) && !defined(FLATBUFFERS_LOCALE_INDEPENDENT)
  #include <unistd.h>
#endif

#ifdef __ANDROID__
  #include <android/api-level.h>
#endif

#if defined(__ICCARM__)
#include <intrinsics.h>
#endif

// Note the __clang__ check is needed, because clang presents itself
// as an older GNUC compiler (4.2).
// Clang 3.3 and later implement all of the ISO C++ 2011 standard.
// Clang 3.4 and later implement all of the ISO C++ 2014 standard.
// http://clang.llvm.org/cxx_status.html

// Note the MSVC value '__cplusplus' may be incorrect:
// The '__cplusplus' predefined macro in the MSVC stuck at the value 199711L,
// indicating (erroneously!) that the compiler conformed to the C++98 Standard.
// This value should be correct starting from MSVC2017-15.7-Preview-3.
// The '__cplusplus' will be valid only if MSVC2017-15.7-P3 and the `/Zc:__cplusplus` switch is set.
// Workaround (for details see MSDN):
// Use the _MSC_VER and _MSVC_LANG definition instead of the __cplusplus  for compatibility.
// The _MSVC_LANG macro reports the Standard version regardless of the '/Zc:__cplusplus' switch.

#if defined(__GNUC__) && !defined(__clang__)
  #define FLATBUFFERS_GCC (__GNUC__ * 10000 + __GNUC_MINOR__ * 100 + __GNUC_PATCHLEVEL__)
#else
  #define FLATBUFFERS_GCC 0
#endif

#if defined(__clang__)
  #define FLATBUFFERS_CLANG (__clang_major__ * 10000 + __clang_minor__ * 100 + __clang_patchlevel__)
#else
  #define FLATBUFFERS_CLANG 0
#endif

/// @cond FLATBUFFERS_INTERNAL
#if __cplusplus <= 199711L && \
    (!defined(_MSC_VER) || _MSC_VER < 1600) && \
    (!defined(__GNUC__) || \
      (__GNUC__ * 10000 + __GNUC_MINOR__ * 100 + __GNUC_PATCHLEVEL__ < 40400))
  #error A C++11 compatible compiler with support for the auto typing is \
         required for FlatBuffers.
  #error __cplusplus _MSC_VER __GNUC__  __GNUC_MINOR__  __GNUC_PATCHLEVEL__
#endif

#if !defined(__clang__) && \
    defined(__GNUC__) && \
    (__GNUC__ * 10000 + __GNUC_MINOR__ * 100 + __GNUC_PATCHLEVEL__ < 40600)
  // Backwards compatibility for g++ 4.4, and 4.5 which don't have the nullptr
  // and constexpr keywords. Note the __clang__ check is needed, because clang
  // presents itself as an older GNUC compiler.
  #ifndef nullptr_t
    const class nullptr_t {
    public:
      template<class T> inline operator T*() const { return 0; }
    private:
      void operator&() const;
    } nullptr = {};
  #endif
  #ifndef constexpr
    #define constexpr const
  #endif
#endif

// The wire format uses a little endian encoding (since that's efficient for
// the common platforms).
#if defined(__s390x__)
  #define FLATBUFFERS_LITTLEENDIAN 0
#endif // __s390x__
#if !defined(FLATBUFFERS_LITTLEENDIAN)
  #if defined(__GNUC__) || defined(__clang__) || defined(__ICCARM__)
    #if (defined(__BIG_ENDIAN__) || \
         (defined(__BYTE_ORDER__) && __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__))
      #define FLATBUFFERS_LITTLEENDIAN 0
    #else
      #define FLATBUFFERS_LITTLEENDIAN 1
    #endif // __BIG_ENDIAN__
  #elif defined(_MSC_VER)
    #if defined(_M_PPC)
      #define FLATBUFFERS_LITTLEENDIAN 0
    #else
      #define FLATBUFFERS_LITTLEENDIAN 1
    #endif
  #else
    #error Unable to determine endianness, define FLATBUFFERS_LITTLEENDIAN.
  #endif
#endif // !defined(FLATBUFFERS_LITTLEENDIAN)

#define FLATBUFFERS_VERSION_MAJOR 24
#define FLATBUFFERS_VERSION_MINOR 3
#define FLATBUFFERS_VERSION_REVISION 25
#define FLATBUFFERS_STRING_EXPAND(X) #X
#define FLATBUFFERS_STRING(X) FLATBUFFERS_STRING_EXPAND(X)
namespace flatbuffers {
  // Returns version as string  "MAJOR.MINOR.REVISION".
  const char* FLATBUFFERS_VERSION();
}

#if (!defined(_MSC_VER) || _MSC_VER > 1600) && \
    (!defined(__GNUC__) || (__GNUC__ * 100 + __GNUC_MINOR__ >= 407)) || \
    defined(__clang__)
  #define FLATBUFFERS_FINAL_CLASS final
  #define FLATBUFFERS_OVERRIDE override
  #define FLATBUFFERS_EXPLICIT_CPP11 explicit
  #define FLATBUFFERS_VTABLE_UNDERLYING_TYPE : ::flatbuffers::voffset_t
#else
  #define FLATBUFFERS_FINAL_CLASS
  #define FLATBUFFERS_OVERRIDE
  #define FLATBUFFERS_EXPLICIT_CPP11
  #define FLATBUFFERS_VTABLE_UNDERLYING_TYPE
#endif

#if (!defined(_MSC_VER) || _MSC_VER >= 1900) && \
    (!defined(__GNUC__) || (__GNUC__ * 100 + __GNUC_MINOR__ >= 406)) || \
    (defined(__cpp_constexpr) && __cpp_constexpr >= 200704)
  #define FLATBUFFERS_CONSTEXPR constexpr
  #define FLATBUFFERS_CONSTEXPR_CPP11 constexpr
  #define FLATBUFFERS_CONSTEXPR_DEFINED
#else
  #define FLATBUFFERS_CONSTEXPR const
  #define FLATBUFFERS_CONSTEXPR_CPP11
#endif

#if (defined(__cplusplus) && __cplusplus >= 201402L) || \
    (defined(__cpp_constexpr) && __cpp_constexpr >= 201304)
  #define FLATBUFFERS_CONSTEXPR_CPP14 FLATBUFFERS_CONSTEXPR_CPP11
#else
  #define FLATBUFFERS_CONSTEXPR_CPP14
#endif

#if (defined(__GXX_EXPERIMENTAL_CXX0X__) && (__GNUC__ * 100 + __GNUC_MINOR__ >= 406)) || \
    (defined(_MSC_FULL_VER) && (_MSC_FULL_VER >= 190023026)) || \
    defined(__clang__)
  #define FLATBUFFERS_NOEXCEPT noexcept
#else
  #define FLATBUFFERS_NOEXCEPT
#endif

// NOTE: the FLATBUFFERS_DELETE_FUNC macro may change the access mode to
// private, so be sure to put it at the end or reset access mode explicitly.
#if (!defined(_MSC_VER) || _MSC_FULL_VER >= 180020827) && \
    (!defined(__GNUC__) || (__GNUC__ * 100 + __GNUC_MINOR__ >= 404)) || \
    defined(__clang__)
  #define FLATBUFFERS_DELETE_FUNC(func) func = delete
#else
  #define FLATBUFFERS_DELETE_FUNC(func) private: func
#endif

#if (!defined(_MSC_VER) || _MSC_VER >= 1900) && \
    (!defined(__GNUC__) || (__GNUC__ * 100 + __GNUC_MINOR__ >= 409)) || \
    defined(__clang__)
  #define FLATBUFFERS_DEFAULT_DECLARATION
#endif

// Check if we can use template aliases
// Not possible if Microsoft Compiler before 2012
// Possible is the language feature __cpp_alias_templates is defined well
// Or possible if the C++ std is C+11 or newer
#if (defined(_MSC_VER) && _MSC_VER > 1700 /* MSVC2012 */) \
    || (defined(__cpp_alias_templates) && __cpp_alias_templates >= 200704) \
    || (defined(__cplusplus) && __cplusplus >= 201103L)
  #define FLATBUFFERS_TEMPLATES_ALIASES
#endif

#ifndef FLATBUFFERS_HAS_STRING_VIEW
  // Only provide flatbuffers::string_view if __has_include can be used
  // to detect a header that provides an implementation
  #if defined(__has_include)
    // Check for std::string_view (in c++17)
    #if __has_include(<string_view>) && (__cplusplus >= 201606 || (defined(_HAS_CXX17) && _HAS_CXX17))
      #include <string_view>
      namespace flatbuffers {
        typedef std::string_view string_view;
      }
      #define FLATBUFFERS_HAS_STRING_VIEW 1
    // Check for std::experimental::string_view (in c++14, compiler-dependent)
    #elif __has_include(<experimental/string_view>) && (__cplusplus >= 201411)
      #include <experimental/string_view>
      namespace flatbuffers {
        typedef std::experimental::string_view string_view;
      }
      #define FLATBUFFERS_HAS_STRING_VIEW 1
    // Check for absl::string_view
    #elif __has_include("absl/strings/string_view.h") && \
          __has_include("absl/base/config.h") && \
          (__cplusplus >= 201411)
      #include "absl/base/config.h"
      #if !defined(ABSL_USES_STD_STRING_VIEW)
        #include "absl/strings/string_view.h"
        namespace flatbuffers {
          typedef absl::string_view string_view;
        }
        #define FLATBUFFERS_HAS_STRING_VIEW 1
      #endif
    #endif
  #endif // __has_include
#endif // !FLATBUFFERS_HAS_STRING_VIEW

#ifndef FLATBUFFERS_GENERAL_HEAP_ALLOC_OK
  // Allow heap allocations to be used
  #define FLATBUFFERS_GENERAL_HEAP_ALLOC_OK 1
#endif // !FLATBUFFERS_GENERAL_HEAP_ALLOC_OK

#ifndef FLATBUFFERS_HAS_NEW_STRTOD
  // Modern (C++11) strtod and strtof functions are available for use.
  // 1) nan/inf strings as argument of strtod;
  // 2) hex-float  as argument of  strtod/strtof.
  #if (defined(_MSC_VER) && _MSC_VER >= 1900) || \
      (defined(__GNUC__) && (__GNUC__ * 100 + __GNUC_MINOR__ >= 409)) || \
      (defined(__clang__))
    #define FLATBUFFERS_HAS_NEW_STRTOD 1
  #endif
#endif // !FLATBUFFERS_HAS_NEW_STRTOD

#ifndef FLATBUFFERS_LOCALE_INDEPENDENT
  // Enable locale independent functions {strtof_l, strtod_l,strtoll_l,
  // strtoull_l}.
  #if (defined(_MSC_VER) && _MSC_VER >= 1800) || \
      (defined(__ANDROID_API__) && __ANDROID_API__>= 21) || \
      (defined(_XOPEN_VERSION) && (_XOPEN_VERSION >= 700)) && \
        (!defined(__Fuchsia__) && !defined(__ANDROID_API__))
    #define FLATBUFFERS_LOCALE_INDEPENDENT 1
  #else
    #define FLATBUFFERS_LOCALE_INDEPENDENT 0
  #endif
#endif  // !FLATBUFFERS_LOCALE_INDEPENDENT

// Suppress Undefined Behavior Sanitizer (recoverable only). Usage:
// - FLATBUFFERS_SUPPRESS_UBSAN("undefined")
// - FLATBUFFERS_SUPPRESS_UBSAN("signed-integer-overflow")
#if defined(__clang__) && (__clang_major__ > 3 || (__clang_major__ == 3 && __clang_minor__ >=7))
  #define FLATBUFFERS_SUPPRESS_UBSAN(type) __attribute__((no_sanitize(type)))
#elif defined(__GNUC__) && (__GNUC__ * 100 + __GNUC_MINOR__ >= 409)
  #define FLATBUFFERS_SUPPRESS_UBSAN(type) __attribute__((no_sanitize_undefined))
#else
  #define FLATBUFFERS_SUPPRESS_UBSAN(type)
#endif

namespace flatbuffers {
  // This is constexpr function used for checking compile-time constants.
  // Avoid `#pragma warning(disable: 4127) // C4127: expression is constant`.
  template<typename T> FLATBUFFERS_CONSTEXPR inline bool IsConstTrue(T t) {
    return !!t;
  }
}

// Enable C++ attribute [[]] if std:c++17 or higher.
#if ((__cplusplus >= 201703L) \
    || (defined(_MSVC_LANG) &&  (_MSVC_LANG >= 201703L)))
  // All attributes unknown to an implementation are ignored without causing an error.
  #define FLATBUFFERS_ATTRIBUTE(attr) attr

  #define FLATBUFFERS_FALLTHROUGH() [[fallthrough]]
#else
  #define FLATBUFFERS_ATTRIBUTE(attr)

  #if FLATBUFFERS_CLANG >= 30800
    #define FLATBUFFERS_FALLTHROUGH() [[clang::fallthrough]]
  #elif FLATBUFFERS_GCC >= 70300
    #define FLATBUFFERS_FALLTHROUGH() [[gnu::fallthrough]]
  #else
    #define FLATBUFFERS_FALLTHROUGH()
  #endif
#endif

/// @endcond

/// @file
namespace flatbuffers {

/// @cond FLATBUFFERS_INTERNAL
// Our default offset / size type, 32bit on purpose on 64bit systems.
// Also, using a consistent offset type maintains compatibility of serialized
// offset values between 32bit and 64bit systems.
typedef uint32_t uoffset_t;
typedef uint64_t uoffset64_t;

// Signed offsets for references that can go in both directions.
typedef int32_t soffset_t;
typedef int64_t soffset64_t;

// Offset/index used in v-tables, can be changed to uint8_t in
// format forks to save a bit of space if desired.
typedef uint16_t voffset_t;

typedef uintmax_t largest_scalar_t;

// In 32bits, this evaluates to 2GB - 1
#define FLATBUFFERS_MAX_BUFFER_SIZE (std::numeric_limits<::flatbuffers::soffset_t>::max)()
#define FLATBUFFERS_MAX_64_BUFFER_SIZE (std::numeric_limits<::flatbuffers::soffset64_t>::max)()

// The minimum size buffer that can be a valid flatbuffer.
// Includes the offset to the root table (uoffset_t), the offset to the vtable
// of the root table (soffset_t), the size of the vtable (uint16_t), and the
// size of the referring table (uint16_t).
#define FLATBUFFERS_MIN_BUFFER_SIZE sizeof(::flatbuffers::uoffset_t) + \
  sizeof(::flatbuffers::soffset_t) + sizeof(uint16_t) + sizeof(uint16_t)

// We support aligning the contents of buffers up to this size.
#ifndef FLATBUFFERS_MAX_ALIGNMENT
  #define FLATBUFFERS_MAX_ALIGNMENT 32
#endif

/// @brief The length of a FlatBuffer file header.
static const size_t kFileIdentifierLength = 4;

inline bool VerifyAlignmentRequirements(size_t align, size_t min_align = 1) {
  return (min_align <= align) && (align <= (FLATBUFFERS_MAX_ALIGNMENT)) &&
         (align & (align - 1)) == 0;  // must be power of 2
}

#if defined(_MSC_VER)
  #pragma warning(push)
  #pragma warning(disable: 4127) // C4127: conditional expression is constant
#endif

template<typename T> T EndianSwap(T t) {
  #if defined(_MSC_VER)
    #define FLATBUFFERS_BYTESWAP16 _byteswap_ushort
    #define FLATBUFFERS_BYTESWAP32 _byteswap_ulong
    #define FLATBUFFERS_BYTESWAP64 _byteswap_uint64
  #elif defined(__ICCARM__)
    #define FLATBUFFERS_BYTESWAP16 __REV16
    #define FLATBUFFERS_BYTESWAP32 __REV
    #define FLATBUFFERS_BYTESWAP64(x) \
       ((__REV(static_cast<uint32_t>(x >> 32U))) | (static_cast<uint64_t>(__REV(static_cast<uint32_t>(x)))) << 32U)
  #else
    #if defined(__GNUC__) && __GNUC__ * 100 + __GNUC_MINOR__ < 408 && !defined(__clang__)
      // __builtin_bswap16 was missing prior to GCC 4.8.
      #define FLATBUFFERS_BYTESWAP16(x) \
        static_cast<uint16_t>(__builtin_bswap32(static_cast<uint32_t>(x) << 16))
    #else
      #define FLATBUFFERS_BYTESWAP16 __builtin_bswap16
    #endif
    #define FLATBUFFERS_BYTESWAP32 __builtin_bswap32
    #define FLATBUFFERS_BYTESWAP64 __builtin_bswap64
  #endif
  if (sizeof(T) == 1) {   // Compile-time if-then's.
    return t;
  } else if (sizeof(T) == 2) {
    union { T t; uint16_t i; } u = { t };
    u.i = FLATBUFFERS_BYTESWAP16(u.i);
    return u.t;
  } else if (sizeof(T) == 4) {
    union { T t; uint32_t i; } u = { t };
    u.i = FLATBUFFERS_BYTESWAP32(u.i);
    return u.t;
  } else if (sizeof(T) == 8) {
    union { T t; uint64_t i; } u = { t };
    u.i = FLATBUFFERS_BYTESWAP64(u.i);
    return u.t;
  } else {
    FLATBUFFERS_ASSERT(0);
    return t;
  }
}

#if defined(_MSC_VER)
  #pragma warning(pop)
#endif


template<typename T> T EndianScalar(T t) {
  #if FLATBUFFERS_LITTLEENDIAN
    return t;
  #else
    return EndianSwap(t);
  #endif
}

template<typename T>
// UBSAN: C++ aliasing type rules, see std::bit_cast<> for details.
FLATBUFFERS_SUPPRESS_UBSAN("alignment")
T ReadScalar(const void *p) {
  return EndianScalar(*reinterpret_cast<const T *>(p));
}

// See https://github.com/google/flatbuffers/issues/5950

#if (FLATBUFFERS_GCC >= 100000) && (FLATBUFFERS_GCC < 110000)
  #pragma GCC diagnostic push
  #pragma GCC diagnostic ignored "-Wstringop-overflow"
#endif

template<typename T>
// UBSAN: C++ aliasing type rules, see std::bit_cast<> for details.
FLATBUFFERS_SUPPRESS_UBSAN("alignment")
void WriteScalar(void *p, T t) {
  *reinterpret_cast<T *>(p) = EndianScalar(t);
}

template<typename T> struct Offset;
template<typename T> FLATBUFFERS_SUPPRESS_UBSAN("alignment") void WriteScalar(void *p, Offset<T> t) {
  *reinterpret_cast<uoffset_t *>(p) = EndianScalar(t.o);
}

#if (FLATBUFFERS_GCC >= 100000) && (FLATBUFFERS_GCC < 110000)
  #pragma GCC diagnostic pop
#endif

// Computes how many bytes you'd have to pad to be able to write an
// "scalar_size" scalar if the buffer had grown to "buf_size" (downwards in
// memory).
FLATBUFFERS_SUPPRESS_UBSAN("unsigned-integer-overflow")
inline size_t PaddingBytes(size_t buf_size, size_t scalar_size) {
  return ((~buf_size) + 1) & (scalar_size - 1);
}

#if !defined(_MSC_VER)
  #pragma GCC diagnostic push
  #pragma GCC diagnostic ignored "-Wfloat-equal"
#endif
// Generic 'operator==' with conditional specialisations.
// T e - new value of a scalar field.
// T def - default of scalar (is known at compile-time).
template<typename T> inline bool IsTheSameAs(T e, T def) { return e == def; }
#if !defined(_MSC_VER)
  #pragma GCC diagnostic pop
#endif

#if defined(FLATBUFFERS_NAN_DEFAULTS) && \
    defined(FLATBUFFERS_HAS_NEW_STRTOD) && (FLATBUFFERS_HAS_NEW_STRTOD > 0)
// Like `operator==(e, def)` with weak NaN if T=(float|double).
template<typename T> inline bool IsFloatTheSameAs(T e, T def) {
  return (e == def) || ((def != def) && (e != e));
}
template<> inline bool IsTheSameAs<float>(float e, float def) {
  return IsFloatTheSameAs(e, def);
}
template<> inline bool IsTheSameAs<double>(double e, double def) {
  return IsFloatTheSameAs(e, def);
}
#endif

// Check 'v' is out of closed range [low; high].
// Workaround for GCC warning [-Werror=type-limits]:
// comparison is always true due to limited range of data type.
template<typename T>
inline bool IsOutRange(const T &v, const T &low, const T &high) {
  return (v < low) || (high < v);
}

// Check 'v' is in closed range [low; high].
template<typename T>
inline bool IsInRange(const T &v, const T &low, const T &high) {
  return !IsOutRange(v, low, high);
}

}  // namespace flatbuffers
#endif  // FLATBUFFERS_BASE_H_
