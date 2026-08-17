/*
 * Define a macro for compiler attributes. Use either gcc
 * syntax if __GNUC__ is defined, or try to look for the
 * modern standard [[x]] attributes.
 *
 * Unfortunately [[x]] doesn't always work when it comes to
 * GNUC-specific attributes, and some compilers support GCC
 * syntax without __attribute__ just to be confusing.
 * Therefore, this also needs an autoconf module to test
 * the validity.
 *
 * Use #ifdef and not defined() here; some compilers do the wrong
 * thing in the latter case.
 */

#ifndef ATTRIBUTE
# define MODERN_ATTRIBUTE(x) [[x]]
# ifndef __GNUC__
#  ifdef __cplusplus
#   ifdef __has_cpp_attribute
#    define ATTRIBUTE(x) MODERN_ATTRIBUTE(x)
#   endif
#  endif
#  ifndef ATTRIBUTE
#   ifdef __has_c_attribute
#    define ATTRIBUTE(x) MODERN_ATTRIBUTE(x)
#   endif
#  endif
#  ifndef ATTRIBUTE
#   ifdef __has_attribute
#    define ATTRIBUTE(x) MODERN_ATTRIBUTE(x)
#   endif
#  endif
# endif
# ifndef ATTRIBUTE
#  define ATTRIBUTE(x) __attribute__((x))
# endif
#endif
