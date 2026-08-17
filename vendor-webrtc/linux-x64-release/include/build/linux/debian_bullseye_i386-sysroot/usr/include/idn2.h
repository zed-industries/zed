/* idn2.h - header file for idn2
   Copyright (C) 2011-2017 Simon Josefsson

   Libidn2 is free software: you can redistribute it and/or modify it
   under the terms of either:

     * the GNU Lesser General Public License as published by the Free
       Software Foundation; either version 3 of the License, or (at
       your option) any later version.

   or

     * the GNU General Public License as published by the Free
       Software Foundation; either version 2 of the License, or (at
       your option) any later version.

   or both in parallel, as here.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received copies of the GNU General Public License and
   the GNU Lesser General Public License along with this program.  If
   not, see <http://www.gnu.org/licenses/>.
*/

#ifndef IDN2_H
#define IDN2_H

/* *INDENT-OFF* */
/* see https://www.gnu.org/software/gnulib/manual/html_node/Exported-Symbols-of-Shared-Libraries.html */
#ifndef _IDN2_API
# if defined IDN2_BUILDING && defined HAVE_VISIBILITY && HAVE_VISIBILITY
#  define _IDN2_API __attribute__((__visibility__("default")))
# elif defined IDN2_BUILDING && defined _MSC_VER && ! defined IDN2_STATIC
#  define _IDN2_API __declspec(dllexport)
# elif defined _MSC_VER && ! defined IDN2_STATIC
#  define _IDN2_API __declspec(dllimport)
# else
#  define _IDN2_API
# endif
#endif
/* *INDENT-ON* */

#include <stdint.h>		/* uint32_t */
#include <string.h>		/* size_t */

#ifdef __cplusplus
extern "C"
{
#endif

/**
 * GCC_VERSION_AT_LEAST
 * @major: gcc major version number to compare with
 * @minor: gcc minor version number to compare with
 *
 * Pre-processor symbol to check the gcc version.
 */
#if defined __GNUC__ && defined __GNUC_MINOR__
# define GCC_VERSION_AT_LEAST(major, minor) ((__GNUC__ > (major)) || (__GNUC__ == (major) && __GNUC_MINOR__ >= (minor)))
#else
# define GCC_VERSION_AT_LEAST(major, minor) 0
#endif

/* the following G_GNUC_ prefixes are for gtk-doc to recognize the attributes */

/**
 * G_GNUC_IDN2_ATTRIBUTE_PURE
 *
 * Function attribute: Function is a pure function.
 */
#if GCC_VERSION_AT_LEAST(2,96)
#	define G_GNUC_IDN2_ATTRIBUTE_PURE __attribute__ ((pure))
#else
#	define G_GNUC_IDN2_ATTRIBUTE_PURE
#endif

/**
 * G_GNUC_IDN2_ATTRIBUTE_CONST
 *
 * Function attribute: Function is a const function.
 */
#if GCC_VERSION_AT_LEAST(2,5)
# define G_GNUC_IDN2_ATTRIBUTE_CONST __attribute__ ((const))
#else
# define G_GNUC_IDN2_ATTRIBUTE_CONST
#endif

/**
 * G_GNUC_DEPRECATED
 *
 * Function attribute: Function is deprecated.
 */
#if GCC_VERSION_AT_LEAST(3,1)
# define G_GNUC_DEPRECATED __attribute__((deprecated))
#elif defined(_MSC_VER)
# define G_GNUC_DEPRECATED __declspec(deprecated)
#else
# define G_GNUC_DEPRECATED /* empty */
#endif

/**
 * G_GNUC_UNUSED
 *
 * Parameter attribute: Parameter is not used.
 */
#if GCC_VERSION_AT_LEAST(2,95)
# define G_GNUC_UNUSED __attribute__ ((__unused__))
#else
# define G_GNUC_UNUSED /* empty */
#endif


/**
 * IDN2_VERSION
 *
 * Pre-processor symbol with a string that describe the header file
 * version number.  Used together with idn2_check_version() to verify
 * header file and run-time library consistency.
 */
#define IDN2_VERSION "2.3.0"

/**
 * IDN2_VERSION_NUMBER
 *
 * Pre-processor symbol with a hexadecimal value describing the header
 * file version number.  For example, when the header version is
 * 1.2.4711 this symbol will have the value 0x01021267.  The last four
 * digits are used to enumerate development snapshots, but for all
 * public releases they will be 0000.
 */
#define IDN2_VERSION_NUMBER 0x02030000

/**
 * IDN2_VERSION_MAJOR
 *
 * Pre-processor symbol for the major version number (decimal).
 * The version scheme is major.minor.patchlevel.
 */
#define IDN2_VERSION_MAJOR 2

/**
 * IDN2_VERSION_MINOR
 *
 * Pre-processor symbol for the minor version number (decimal).
 * The version scheme is major.minor.patchlevel.
 */
#define IDN2_VERSION_MINOR 3

/**
 * IDN2_VERSION_PATCH
 *
 * Pre-processor symbol for the patch level number (decimal).
 * The version scheme is major.minor.patchlevel.
 */
#define IDN2_VERSION_PATCH 0

/**
 * IDN2_LABEL_MAX_LENGTH
 *
 * Constant specifying the maximum length of a DNS label to 63
 * characters, as specified in RFC 1034.
 */
#define IDN2_LABEL_MAX_LENGTH 63

/**
 * IDN2_DOMAIN_MAX_LENGTH
 *
 * Constant specifying the maximum size of the wire encoding of a DNS
 * domain to 255 characters, as specified in RFC 1034.  Note that the
 * usual printed representation of a domain name is limited to 253
 * characters if it does not end with a period, or 254 characters if
 * it ends with a period.
 */
#define IDN2_DOMAIN_MAX_LENGTH 255

/**
 * idn2_flags:
 * @IDN2_NFC_INPUT: Normalize input string using normalization form C.
 * @IDN2_ALABEL_ROUNDTRIP: Perform optional IDNA2008 lookup roundtrip check (default).
 * @IDN2_NO_ALABEL_ROUNDTRIP: Disable ALabel lookup roundtrip check.
 * @IDN2_NO_TR46: Disable Unicode TR46 processing.
 * @IDN2_TRANSITIONAL: Perform Unicode TR46 transitional processing.
 * @IDN2_NONTRANSITIONAL: Perform Unicode TR46 non-transitional processing (default).
 * @IDN2_ALLOW_UNASSIGNED: Libidn compatibility flag, unused.
 * @IDN2_USE_STD3_ASCII_RULES: Use STD3 ASCII rules.
 * This is a #TR46 only flag, and will be ignored when set without either
 * @IDN2_TRANSITIONAL or @IDN2_NONTRANSITIONAL.
 *
 * Flags to IDNA2008 functions, to be binary or:ed together.  Specify
 * only 0 if you want the default behaviour.
 */
  typedef enum
  {
    IDN2_NFC_INPUT = 1,
    IDN2_ALABEL_ROUNDTRIP = 2,
    IDN2_TRANSITIONAL = 4,
    IDN2_NONTRANSITIONAL = 8,
    IDN2_ALLOW_UNASSIGNED = 16,
    IDN2_USE_STD3_ASCII_RULES = 32,
    IDN2_NO_TR46 = 64,
    IDN2_NO_ALABEL_ROUNDTRIP = 128
  } idn2_flags;

/* IDNA2008 with UTF-8 encoded inputs. */

  extern _IDN2_API int
    idn2_lookup_u8 (const uint8_t * src, uint8_t ** lookupname, int flags);

  extern _IDN2_API int
    idn2_register_u8 (const uint8_t * ulabel, const uint8_t * alabel,
		      uint8_t ** insertname, int flags);

/* IDNA2008 with locale encoded inputs. */

  extern _IDN2_API int
    idn2_lookup_ul (const char *src, char **lookupname, int flags);

  extern _IDN2_API int
    idn2_register_ul (const char *ulabel, const char *alabel,
		      char **insertname, int flags);

/**
 * idn2_rc:
 * @IDN2_OK: Successful return.
 * @IDN2_MALLOC: Memory allocation error.
 * @IDN2_NO_CODESET: Could not determine locale string encoding format.
 * @IDN2_ICONV_FAIL: Could not transcode locale string to UTF-8.
 * @IDN2_ENCODING_ERROR: Unicode data encoding error.
 * @IDN2_NFC: Error normalizing string.
 * @IDN2_PUNYCODE_BAD_INPUT: Punycode invalid input.
 * @IDN2_PUNYCODE_BIG_OUTPUT: Punycode output buffer too small.
 * @IDN2_PUNYCODE_OVERFLOW: Punycode conversion would overflow.
 * @IDN2_TOO_BIG_DOMAIN: Domain name longer than 255 characters.
 * @IDN2_TOO_BIG_LABEL: Domain label longer than 63 characters.
 * @IDN2_INVALID_ALABEL: Input A-label is not valid.
 * @IDN2_UALABEL_MISMATCH: Input A-label and U-label does not match.
 * @IDN2_INVALID_FLAGS: Invalid combination of flags.
 * @IDN2_NOT_NFC: String is not NFC.
 * @IDN2_2HYPHEN: String has forbidden two hyphens.
 * @IDN2_HYPHEN_STARTEND: String has forbidden starting/ending hyphen.
 * @IDN2_LEADING_COMBINING: String has forbidden leading combining character.
 * @IDN2_DISALLOWED: String has disallowed character.
 * @IDN2_CONTEXTJ: String has forbidden context-j character.
 * @IDN2_CONTEXTJ_NO_RULE: String has context-j character with no rull.
 * @IDN2_CONTEXTO: String has forbidden context-o character.
 * @IDN2_CONTEXTO_NO_RULE: String has context-o character with no rull.
 * @IDN2_UNASSIGNED: String has forbidden unassigned character.
 * @IDN2_BIDI: String has forbidden bi-directional properties.
 * @IDN2_DOT_IN_LABEL: Label has forbidden dot (TR46).
 * @IDN2_INVALID_TRANSITIONAL: Label has character forbidden in transitional mode (TR46).
 * @IDN2_INVALID_NONTRANSITIONAL: Label has character forbidden in non-transitional mode (TR46).
 * @IDN2_ALABEL_ROUNDTRIP_FAILED: ALabel -> Ulabel -> ALabel result differs from input.
 *
 * Return codes for IDN2 functions.  All return codes are negative
 * except for the successful code IDN2_OK which are guaranteed to be
 * 0.  Positive values are reserved for non-error return codes.
 *
 * Note that the #idn2_rc enumeration may be extended at a later date
 * to include new return codes.
 */
  typedef enum
  {
    IDN2_OK = 0,
    IDN2_MALLOC = -100,
    IDN2_NO_CODESET = -101,
    IDN2_ICONV_FAIL = -102,
    IDN2_ENCODING_ERROR = -200,
    IDN2_NFC = -201,
    IDN2_PUNYCODE_BAD_INPUT = -202,
    IDN2_PUNYCODE_BIG_OUTPUT = -203,
    IDN2_PUNYCODE_OVERFLOW = -204,
    IDN2_TOO_BIG_DOMAIN = -205,
    IDN2_TOO_BIG_LABEL = -206,
    IDN2_INVALID_ALABEL = -207,
    IDN2_UALABEL_MISMATCH = -208,
    IDN2_INVALID_FLAGS = -209,
    IDN2_NOT_NFC = -300,
    IDN2_2HYPHEN = -301,
    IDN2_HYPHEN_STARTEND = -302,
    IDN2_LEADING_COMBINING = -303,
    IDN2_DISALLOWED = -304,
    IDN2_CONTEXTJ = -305,
    IDN2_CONTEXTJ_NO_RULE = -306,
    IDN2_CONTEXTO = -307,
    IDN2_CONTEXTO_NO_RULE = -308,
    IDN2_UNASSIGNED = -309,
    IDN2_BIDI = -310,
    IDN2_DOT_IN_LABEL = -311,
    IDN2_INVALID_TRANSITIONAL = -312,
    IDN2_INVALID_NONTRANSITIONAL = -313,
    IDN2_ALABEL_ROUNDTRIP_FAILED = -314,
  } idn2_rc;

/* Auxiliary functions. */

  extern _IDN2_API G_GNUC_DEPRECATED int
    idn2_to_ascii_4i (const uint32_t * input, size_t inlen, char * output, int flags);
  extern _IDN2_API int
    idn2_to_ascii_4i2 (const uint32_t * input, size_t inlen, char ** output, int flags);
  extern _IDN2_API int
    idn2_to_ascii_4z (const uint32_t * input, char ** output, int flags);
  extern _IDN2_API int
    idn2_to_ascii_8z (const char * input, char ** output, int flags);
  extern _IDN2_API int
    idn2_to_ascii_lz (const char * input, char ** output, int flags);

  extern _IDN2_API int
    idn2_to_unicode_8z4z (const char * input, uint32_t ** output, int flags G_GNUC_UNUSED);
  extern _IDN2_API int
    idn2_to_unicode_4z4z (const uint32_t * input, uint32_t ** output, int flags);
  extern _IDN2_API int
    idn2_to_unicode_44i (const uint32_t * in, size_t inlen, uint32_t * out, size_t * outlen, int flags);
  extern _IDN2_API int
    idn2_to_unicode_8z8z (const char * input, char ** output, int flags);
  extern _IDN2_API int
    idn2_to_unicode_8zlz (const char * input, char ** output, int flags);
  extern _IDN2_API int
    idn2_to_unicode_lzlz (const char * input, char ** output, int flags);

  extern _IDN2_API G_GNUC_IDN2_ATTRIBUTE_CONST const char *
    idn2_strerror (int rc);
  extern _IDN2_API G_GNUC_IDN2_ATTRIBUTE_CONST const char *
    idn2_strerror_name (int rc);

  extern _IDN2_API G_GNUC_IDN2_ATTRIBUTE_PURE const char *
    idn2_check_version (const char *req_version);

  extern _IDN2_API void
    idn2_free (void *ptr);


/*** libidn compatibility layer ***/
#if !defined IDNA_H && !defined IDN2_SKIP_LIBIDN_COMPAT

/**
 * Idna_rc:
 * @IDNA_SUCCESS: Same as %IDN2_OK
 * @IDNA_STRINGPREP_ERROR: Same as %IDN2_ENCODING_ERROR
 * @IDNA_PUNYCODE_ERROR: Same as %IDN2_PUNYCODE_BAD_INPUT
 * @IDNA_CONTAINS_NON_LDH: Same as %IDN2_ENCODING_ERROR
 * @IDNA_CONTAINS_LDH: Same as %IDNA_CONTAINS_NON_LDH
 * @IDNA_CONTAINS_MINUS: Same as %IDN2_ENCODING_ERROR
 * @IDNA_INVALID_LENGTH: Same as %IDN2_DISALLOWED
 * @IDNA_NO_ACE_PREFIX: Same as %IDN2_ENCODING_ERROR
 * @IDNA_ROUNDTRIP_VERIFY_ERROR: Same as %IDN2_ENCODING_ERROR
 * @IDNA_CONTAINS_ACE_PREFIX: Same as %IDN2_ENCODING_ERROR
 * @IDNA_ICONV_ERROR: Same as %IDN2_ENCODING_ERROR
 * @IDNA_MALLOC_ERROR: Same as %IDN2_MALLOC
 * @IDNA_DLOPEN_ERROR: Same as %IDN2_MALLOC
 *
 * Return codes for transition to / compatibility with libidn2.
 *
 * Please be aware that return codes from idna_ functions might be unexpected
 * when linked / built with libidn2.
 */
  typedef enum
  {
    IDNA_SUCCESS = IDN2_OK,
    IDNA_STRINGPREP_ERROR = IDN2_ENCODING_ERROR,
    IDNA_PUNYCODE_ERROR = IDN2_PUNYCODE_BAD_INPUT,
    IDNA_CONTAINS_NON_LDH = IDN2_ENCODING_ERROR,
    IDNA_CONTAINS_LDH = IDNA_CONTAINS_NON_LDH,
    IDNA_CONTAINS_MINUS = IDN2_ENCODING_ERROR,
    IDNA_INVALID_LENGTH = IDN2_DISALLOWED,
    IDNA_NO_ACE_PREFIX = IDN2_ENCODING_ERROR,
    IDNA_ROUNDTRIP_VERIFY_ERROR = IDN2_ENCODING_ERROR,
    IDNA_CONTAINS_ACE_PREFIX = IDN2_ENCODING_ERROR,
    IDNA_ICONV_ERROR = IDN2_ENCODING_ERROR,
    IDNA_MALLOC_ERROR = IDN2_MALLOC,
    IDNA_DLOPEN_ERROR = IDN2_MALLOC
  } Idna_rc;

/**
 * Idna_flags:
 * @IDNA_ALLOW_UNASSIGNED: Same as %IDN2_ALLOW_UNASSIGNED
 * @IDNA_USE_STD3_ASCII_RULES: Same as %IDN2_USE_STD3_ASCII_RULES
 *
 * Flags for transition to / compatibility with libidn2.
 */
  typedef enum
  {
    IDNA_ALLOW_UNASSIGNED = IDN2_ALLOW_UNASSIGNED,
    IDNA_USE_STD3_ASCII_RULES = IDN2_USE_STD3_ASCII_RULES
  } Idna_flags;

  #define idna_to_ascii_4i(i,l,o,f)  idn2_to_ascii_4i(i,l,o,f|IDN2_NFC_INPUT|IDN2_NONTRANSITIONAL)
  #define idna_to_ascii_4z(i,o,f)  idn2_to_ascii_4z(i,o,f|IDN2_NFC_INPUT|IDN2_NONTRANSITIONAL)
  #define idna_to_ascii_8z(i,o,f)  idn2_to_ascii_8z(i,o,f|IDN2_NFC_INPUT|IDN2_NONTRANSITIONAL)
  #define idna_to_ascii_lz(i,o,f)  idn2_to_ascii_lz(i,o,f|IDN2_NFC_INPUT|IDN2_NONTRANSITIONAL)

  #define idna_to_unicode_8z4z  idn2_to_unicode_8z4z
  #define idna_to_unicode_4z4z  idn2_to_unicode_4z4z
  #define idna_to_unicode_44i   idn2_to_unicode_44i
  #define idna_to_unicode_8z8z  idn2_to_unicode_8z8z
  #define idna_to_unicode_8zlz  idn2_to_unicode_8zlz
  #define idna_to_unicode_lzlz  idn2_to_unicode_lzlz

  #define idna_strerror         idn2_strerror
  #define idn_free              idn2_free

#endif /* IDNA_H */


#ifdef __cplusplus
}
#endif

#endif				/* IDN2_H */
