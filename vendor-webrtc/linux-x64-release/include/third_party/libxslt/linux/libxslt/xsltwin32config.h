/*
 * Summary: compile-time version information for the XSLT engine 
 *          when compiled on windows
 * Description: compile-time version information for the XSLT engine
 *              when compiled on windows. This file is generated.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */

#ifndef __XML_XSLTWIN32CONFIG_H__
#define __XML_XSLTWIN32CONFIG_H__

#include "win32config.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * LIBXSLT_DOTTED_VERSION:
 *
 * the version string like "1.2.3"
 */
#define LIBXSLT_DOTTED_VERSION "1.1.34"

/**
 * LIBXSLT_VERSION:
 *
 * the version number: 1.2.3 value is 1002003
 */
#define LIBXSLT_VERSION 10134

/**
 * LIBXSLT_VERSION_STRING:
 *
 * the version number string, 1.2.3 value is "1002003"
 */
#define LIBXSLT_VERSION_STRING "10134"

/**
 * LIBXSLT_VERSION_EXTRA:
 *
 * extra version information, used to show a CVS compilation
 */
#define LIBXSLT_VERSION_EXTRA "-win32"

/**
 * WITH_XSLT_DEBUG:
 *
 * Activate the compilation of the debug reporting. Speed penalty
 * is insignifiant and being able to run xsltpoc -v is useful. On
 * by default
 */
#if 1
#define WITH_XSLT_DEBUG
#endif

/**
 * WITH_MODULES:
 *
 * Whether module support is configured into libxslt
 */
#if 0
#ifndef WITH_MODULES
#define WITH_MODULES
#endif
#define LIBXSLT_PLUGINS_PATH() getenv("LIBXSLT_PLUGINS_PATH")
#endif

#if 0
/**
 * DEBUG_MEMORY:
 *
 * should be activated only when debugging libxslt. It replaces the
 * allocator with a collect and debug shell to the libc allocator.
 * Use configure --with-mem-debug to activate it on both library
 */
#define DEBUG_MEMORY

/**
 * DEBUG_MEMORY_LOCATION:
 *
 * should be activated only when debugging libxslt.
 * DEBUG_MEMORY_LOCATION should be activated only when libxml has
 * been configured with --with-debug-mem too
 */
#define DEBUG_MEMORY_LOCATION
#endif

/**
 * ATTRIBUTE_UNUSED:
 *
 * This macro is used to flag unused function parameters to GCC, useless here
 */
#ifndef ATTRIBUTE_UNUSED
#define ATTRIBUTE_UNUSED
#endif

#ifdef __cplusplus
}
#endif

#endif /* __XML_XSLTWIN32CONFIG_H__ */
