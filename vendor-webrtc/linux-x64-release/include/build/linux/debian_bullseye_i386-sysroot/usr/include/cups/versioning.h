/*
 * API versioning definitions for CUPS.
 *
 * Copyright © 2007-2019 by Apple Inc.
 *
 * Licensed under Apache License v2.0.  See the file "LICENSE" for more
 * information.
 */

#ifndef _CUPS_VERSIONING_H_
#  define _CUPS_VERSIONING_H_

/*
 * This header defines several macros that add compiler-specific attributes for
 * functions:
 *
 *   - _CUPS_API_major_minor[_patch]: Specifies when an API became available by
 *     CUPS version.
 *   - _CUPS_DEPRECATED: Function is deprecated with no replacement.
 *   - _CUPS_DEPRECATED_MSG("message"): Function is deprecated and has a
 *     replacement.
 *   - _CUPS_FORMAT(format-index, additional-args-index): Function has a
 *     printf-style format argument followed by zero or more additional
 *     arguments.  Indices start at 1.
 *   - _CUPS_INTERNAL: Function is internal with no replacement API.
 *   - _CUPS_INTERNAL_MSG("msg"): Function is internal - use specified API
 *     instead.
 *   - _CUPS_NONNULL((arg list)): Specifies the comma-separated argument indices
 *     are assumed non-NULL.  Indices start at 1.
 *   - _CUPS_NORETURN: Specifies the function does not return.
 *   - _CUPS_PRIVATE: Specifies the function is private to CUPS.
 *   - _CUPS_PUBLIC: Specifies the function is public API.
 */

/*
 * Determine which compiler is being used and what annotation features are
 * available...
 */

#  ifdef __APPLE__
#    include <os/availability.h>
#  endif /* __APPLE__ */

#  ifdef __has_extension		/* Clang */
#    define _CUPS_HAS_DEPRECATED
#    define _CUPS_HAS_FORMAT
#    define _CUPS_HAS_NORETURN
#    define _CUPS_HAS_VISIBILITY
#    if __has_extension(attribute_deprecated_with_message)
#      define _CUPS_HAS_DEPRECATED_WITH_MESSAGE
#    endif
#    if __has_extension(attribute_unavailable_with_message)
#      define _CUPS_HAS_UNAVAILABLE_WITH_MESSAGE
#    endif
#  elif defined(__GNUC__)		/* GCC and compatible */
#    if __GNUC__ >= 3			/* GCC 3.0 or higher */
#      define _CUPS_HAS_DEPRECATED
#      define _CUPS_HAS_FORMAT
#      define _CUPS_HAS_NORETURN
#      define _CUPS_HAS_VISIBILITY
#    endif /* __GNUC__ >= 3 */
#    if __GNUC__ >= 5			/* GCC 5.x */
#      define _CUPS_HAS_DEPRECATED_WITH_MESSAGE
#    elif __GNUC__ == 4 && __GNUC_MINOR__ >= 5
					/* GCC 4.5 or higher */
#      define _CUPS_HAS_DEPRECATED_WITH_MESSAGE
#    endif /* __GNUC__ >= 5 */
#  elif defined(_WIN32)
#    define __attribute__(...)
#  endif /* __has_extension */


/*
 * Define _CUPS_INTERNAL, _CUPS_PRIVATE, and _CUPS_PUBLIC visibilty macros for
 * internal/private/public functions...
 */

#  ifdef _CUPS_HAS_VISIBILITY
#    define _CUPS_INTERNAL	__attribute__ ((visibility("hidden")))
#    define _CUPS_PRIVATE	__attribute__ ((visibility("default")))
#    define _CUPS_PUBLIC	__attribute__ ((visibility("default")))
#  elif defined(_WIN32) && defined(LIBCUPS2_EXPORTS) && 0
#    define _CUPS_INTERNAL
#    define _CUPS_PRIVATE	__declspec(dllexport)
#    define _CUPS_PUBLIC	__declspec(dllexport)
#  else
#    define _CUPS_INTERNAL
#    define _CUPS_PRIVATE
#    define _CUPS_PUBLIC
#  endif /* _CUPS_HAS_VISIBILITY */


/*
 * Define _CUPS_API_major_minor[_patch] availability macros for CUPS.
 *
 * Note: Using any of the _CUPS_API macros automatically adds _CUPS_PUBLIC.
 */

#  if defined(__APPLE__) && !defined(_CUPS_SOURCE) && TARGET_OS_OSX
/*
 * On Apple operating systems, the _CUPS_API_* constants are defined using the
 * API_ macros in <os/availability.h>.
 *
 * On iOS, we don't actually have libcups available directly, but the supplied
 * libcups_static target in the Xcode project supports building on iOS 11.0 and
 * later.
 */
#    define _CUPS_API_1_1_19 API_AVAILABLE(macos(10.3), ios(11.0)) _CUPS_PUBLIC
#    define _CUPS_API_1_1_20 API_AVAILABLE(macos(10.4), ios(11.0)) _CUPS_PUBLIC
#    define _CUPS_API_1_1_21 API_AVAILABLE(macos(10.4), ios(11.0)) _CUPS_PUBLIC
#    define _CUPS_API_1_2 API_AVAILABLE(macos(10.5), ios(11.0)) _CUPS_PUBLIC
#    define _CUPS_API_1_3 API_AVAILABLE(macos(10.5), ios(11.0)) _CUPS_PUBLIC
#    define _CUPS_API_1_4 API_AVAILABLE(macos(10.6), ios(11.0)) _CUPS_PUBLIC
#    define _CUPS_API_1_5 API_AVAILABLE(macos(10.7), ios(11.0)) _CUPS_PUBLIC
#    define _CUPS_API_1_6 API_AVAILABLE(macos(10.8), ios(11.0)) _CUPS_PUBLIC
#    define _CUPS_API_1_7 API_AVAILABLE(macos(10.9), ios(11.0)) _CUPS_PUBLIC
#    define _CUPS_API_2_0 API_AVAILABLE(macos(10.10), ios(11.0)) _CUPS_PUBLIC
#    define _CUPS_API_2_2 API_AVAILABLE(macos(10.12), ios(11.0)) _CUPS_PUBLIC
#    define _CUPS_API_2_2_4 API_AVAILABLE(macos(10.13), ios(12.0)) _CUPS_PUBLIC
#    define _CUPS_API_2_2_7 API_AVAILABLE(macos(10.14), ios(13.0)) _CUPS_PUBLIC
#    define _CUPS_API_2_3 API_AVAILABLE(macos(10.14), ios(13.0)) _CUPS_PUBLIC
#  else
#    define _CUPS_API_1_1_19 _CUPS_PUBLIC
#    define _CUPS_API_1_1_20 _CUPS_PUBLIC
#    define _CUPS_API_1_1_21 _CUPS_PUBLIC
#    define _CUPS_API_1_2 _CUPS_PUBLIC
#    define _CUPS_API_1_3 _CUPS_PUBLIC
#    define _CUPS_API_1_4 _CUPS_PUBLIC
#    define _CUPS_API_1_5 _CUPS_PUBLIC
#    define _CUPS_API_1_6 _CUPS_PUBLIC
#    define _CUPS_API_1_7 _CUPS_PUBLIC
#    define _CUPS_API_2_0 _CUPS_PUBLIC
#    define _CUPS_API_2_2 _CUPS_PUBLIC
#    define _CUPS_API_2_2_4 _CUPS_PUBLIC
#    define _CUPS_API_2_2_7 _CUPS_PUBLIC
#    define _CUPS_API_2_3 _CUPS_PUBLIC
#  endif /* __APPLE__ && !_CUPS_SOURCE */


/*
 * Define _CUPS_DEPRECATED and _CUPS_INTERNAL macros to mark old APIs as
 * "deprecated" or "unavailable" with messages so you get warnings/errors are
 * compile-time...
 *
 * Note: Using any of the _CUPS_DEPRECATED macros automatically adds
 * _CUPS_PUBLIC.
 */

#  if !defined(_CUPS_HAS_DEPRECATED) || (defined(_CUPS_SOURCE) && !defined(_CUPS_NO_DEPRECATED))
    /*
     * Don't mark functions deprecated if the compiler doesn't support it
     * or we are building CUPS source that doesn't care.
     */
#    define _CUPS_DEPRECATED _CUPS_PUBLIC
#    define _CUPS_DEPRECATED_MSG(m) _CUPS_PUBLIC
#    define _CUPS_DEPRECATED_1_2_MSG(m) _CUPS_PUBLIC
#    define _CUPS_DEPRECATED_1_6_MSG(m) _CUPS_PUBLIC
#    define _CUPS_DEPRECATED_1_7_MSG(m) _CUPS_PUBLIC
#    define _CUPS_DEPRECATED_2_2_MSG(m) _CUPS_PUBLIC
#  elif defined(__APPLE__) && defined(_CUPS_NO_DEPRECATED)
    /*
     * Compiler supports the unavailable attribute, so use it when the code
     * wants to exclude the use of deprecated API.
     */
#    define _CUPS_DEPRECATED __attribute__ ((unavailable)) _CUPS_PUBLIC
#    define _CUPS_DEPRECATED_MSG(m) __attribute__ ((unavailable(m))) _CUPS_PUBLIC
#    define _CUPS_DEPRECATED_1_2_MSG(m) API_DEPRECATED(m, macos(10.2,10.5), ios(11.0,11.0)) _CUPS_PUBLIC
#    define _CUPS_DEPRECATED_1_6_MSG(m) API_DEPRECATED(m, macos(10.2,10.8), ios(11.0,11.0)) _CUPS_PUBLIC
#    define _CUPS_DEPRECATED_1_7_MSG(m) API_DEPRECATED(m, macos(10.2,10.9), ios(11.0,11.0)) _CUPS_PUBLIC
#    define _CUPS_DEPRECATED_2_2_MSG(m) API_DEPRECATED(m, macos(10.2,10.12), ios(11.0,11.0)) _CUPS_PUBLIC

#  elif defined(__APPLE__)
    /*
     * Just mark things as deprecated...
     */
#    define _CUPS_DEPRECATED __attribute__ ((deprecated)) _CUPS_PUBLIC
#    define _CUPS_DEPRECATED_MSG(m) __attribute__ ((deprecated(m))) _CUPS_PUBLIC
#    define _CUPS_DEPRECATED_1_2_MSG(m) API_DEPRECATED(m, macos(10.2,10.5), ios(11.0,11.0)) _CUPS_PUBLIC
#    define _CUPS_DEPRECATED_1_6_MSG(m) API_DEPRECATED(m, macos(10.2,10.8), ios(11.0,11.0)) _CUPS_PUBLIC
#    define _CUPS_DEPRECATED_1_7_MSG(m) API_DEPRECATED(m, macos(10.2,10.9), ios(11.0,11.0)) _CUPS_PUBLIC
#    define _CUPS_DEPRECATED_2_2_MSG(m) API_DEPRECATED(m, macos(10.2,10.12), ios(11.0,11.0)) _CUPS_PUBLIC

#  elif defined(_CUPS_HAS_UNAVAILABLE_WITH_MESSAGE) && defined(_CUPS_NO_DEPRECATED)
    /*
     * Compiler supports the unavailable attribute, so use it when the code
     * wants to exclude the use of deprecated API.
     */
#    define _CUPS_DEPRECATED __attribute__ ((unavailable)) _CUPS_PUBLIC
#    define _CUPS_DEPRECATED_MSG(m) __attribute__ ((unavailable(m))) _CUPS_PUBLIC
#    define _CUPS_DEPRECATED_1_2_MSG(m) __attribute__ ((unavailable(m))) _CUPS_PUBLIC
#    define _CUPS_DEPRECATED_1_6_MSG(m) __attribute__ ((unavailable(m))) _CUPS_PUBLIC
#    define _CUPS_DEPRECATED_1_7_MSG(m) __attribute__ ((unavailable(m))) _CUPS_PUBLIC
#    define _CUPS_DEPRECATED_2_2_MSG(m) __attribute__ ((unavailable(m))) _CUPS_PUBLIC
#  else
    /*
     * Compiler supports the deprecated attribute, so use it.
     */
#    define _CUPS_DEPRECATED __attribute__ ((deprecated)) _CUPS_PUBLIC
#    ifdef _CUPS_HAS_DEPRECATED_WITH_MESSAGE
#      define _CUPS_DEPRECATED_MSG(m) __attribute__ ((deprecated(m))) _CUPS_PUBLIC
#      define _CUPS_DEPRECATED_1_2_MSG(m) __attribute__ ((deprecated(m))) _CUPS_PUBLIC
#      define _CUPS_DEPRECATED_1_6_MSG(m) __attribute__ ((deprecated(m))) _CUPS_PUBLIC
#      define _CUPS_DEPRECATED_1_7_MSG(m) __attribute__ ((deprecated(m))) _CUPS_PUBLIC
#      define _CUPS_DEPRECATED_2_2_MSG(m) __attribute__ ((deprecated(m))) _CUPS_PUBLIC
#    else
#      define _CUPS_DEPRECATED_MSG(m) __attribute__ ((deprecated)) _CUPS_PUBLIC
#      define _CUPS_DEPRECATED_1_2_MSG(m) __attribute__ ((deprecated)) _CUPS_PUBLIC
#      define _CUPS_DEPRECATED_1_6_MSG(m) __attribute__ ((deprecated)) _CUPS_PUBLIC
#      define _CUPS_DEPRECATED_1_7_MSG(m) __attribute__ ((deprecated)) _CUPS_PUBLIC
#      define _CUPS_DEPRECATED_2_2_MSG(m) __attribute__ ((deprecated)) _CUPS_PUBLIC
#    endif /* _CUPS_HAS_DEPRECATED_WITH_MESSAGE */
#  endif /* !_CUPS_HAS_DEPRECATED || (_CUPS_SOURCE && !_CUPS_NO_DEPRECATED) */


/*
 * Define _CUPS_FORMAT macro for printf-style functions...
 */

#  ifdef _CUPS_HAS_FORMAT
#    define _CUPS_FORMAT(a,b) __attribute__ ((__format__(__printf__, a,b)))
#  else
#    define _CUPS_FORMAT(a,b)
#  endif /* _CUPS_HAS_FORMAT */


/*
 * Define _CUPS_INTERNAL_MSG macro for private APIs that have (historical)
 * public visibility.
 *
 * Note: Using the _CUPS_INTERNAL_MSG macro automatically adds _CUPS_PUBLIC.
 */

#  ifdef _CUPS_SOURCE
#    define _CUPS_INTERNAL_MSG(m) _CUPS_PUBLIC
#  elif defined(_CUPS_HAS_UNAVAILABLE_WITH_MESSAGE)
#    define _CUPS_INTERNAL_MSG(m) __attribute__ ((unavailable(m))) _CUPS_PUBLIC
#  elif defined(_CUPS_HAS_DEPRECATED_WITH_MESSAGE)
#    define _CUPS_INTERNAL_MSG(m) __attribute__ ((deprecated(m))) _CUPS_PUBLIC
#  else
#    define _CUPS_INTERNAL_MSG(m) __attribute__ ((deprecated)) _CUPS_PUBLIC
#  endif /* _CUPS_SOURCE */


/*
 * Define _CUPS_NONNULL macro for functions that don't expect non-null
 * arguments...
 */

#  ifdef _CUPS_HAS_NONNULL
#    define _CUPS_NONNULL(...) __attribute__ ((nonnull(__VA_ARGS__)))
#  else
#    define _CUPS_NONNULL(...)
#  endif /* _CUPS_HAS_FORMAT */


/*
 * Define _CUPS_NORETURN macro for functions that don't return.
 */

#  ifdef _CUPS_HAS_NORETURN
#    define _CUPS_NORETURN	__attribute__ ((noreturn))
#  else
#    define _CUPS_NORETURN
#  endif /* _CUPS_HAS_NORETURN */


#endif /* !_CUPS_VERSIONING_H_ */
