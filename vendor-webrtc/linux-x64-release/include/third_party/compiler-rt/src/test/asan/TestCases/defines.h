#ifndef SANITIZER_TEST_DEFINES_H
#define SANITIZER_TEST_DEFINES_H

#if defined(_MSC_VER) && !defined(__clang__)
#  include <intrin.h>

#  define ATTRIBUTE_NOINLINE __declspec(noinline)
#  define ATTRIBUTE_ALIGNED(x) __declspec(align(x))
#  define ATTRIBUTE_NO_SANITIZE_ADDRESS __declspec(no_sanitize_address)
#  define ATTRIBUTE_USED /* FIXME: Is there a __declspec used? */
#  define ATTRIBUTE_ALWAYS_INLINE __forceinline
#  define VOLATILE volatile
#  define EXTRACT_RETURN_ADDRESS _ReturnAddress()
#  define ASM_CAUSE_SIDE_EFFECT(dest) __asm { mov eax, dest}
#  define MULTIPLE_ATTRIBUTE_DECL(a, b) __declspec(a b)

#else

#  define ATTRIBUTE_NOINLINE __attribute__((noinline))
#  define ATTRIBUTE_ALIGNED(x) __attribute__((aligned(x)))
#  define ATTRIBUTE_NO_SANITIZE_ADDRESS __attribute__((no_sanitize_address))
#  define ATTRIBUTE_USED __attribute__((used))
#  define ATTRIBUTE_ALWAYS_INLINE __attribute__((always_inline))
#  define INLINE_ASM(x) __asm__(x)
#  define VOLATILE __volatile__
#  define EXTRACT_RETURN_ADDRESS                                               \
    __builtin_extract_return_addr(__builtin_return_address(0))
#  define ASM_CAUSE_SIDE_EFFECT(dest)                                          \
    __asm__ __volatile__("" : : "r"(dest) : "memory");
#  define MULTIPLE_ATTRIBUTE_DECL(a, b) __attribute__((a, b))

#endif // _MSC_VER

#endif // SANITIZER_TEST_DEFINES
