/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _plstr_h
#define _plstr_h

/*
 * plstr.h
 *
 * This header file exports the API to the NSPR portable library or string-
 * handling functions.
 *
 * This API was not designed as an "optimal" or "ideal" string library; it
 * was based on the good ol' unix string.3 functions, and was written to
 *
 *  1) replace the libc functions, for cross-platform consistency,
 *  2) complete the API on platforms lacking common functions (e.g.,
 *     strcase*), and
 *  3) to implement some obvious "closure" functions that I've seen
 *     people hacking around in our code.
 *
 * Point number three largely means that most functions have an "strn"
 * limited-length version, and all comparison routines have a non-case-
 * sensitive version available.
 */

#include "prtypes.h"

PR_BEGIN_EXTERN_C
/*
 * PL_strlen
 *
 * Returns the length of the provided string, not including the trailing '\0'.
 */

PR_EXTERN(PRUint32)
PL_strlen(const char *str);

/*
 * PL_strnlen
 *
 * Returns the length of the provided string, not including the trailing '\0',
 * up to the indicated maximum.  The string will not be examined beyond the
 * maximum; if no terminating '\0' is found, the maximum will be returned.
 */

PR_EXTERN(PRUint32)
PL_strnlen(const char *str, PRUint32 max);

/*
 * PL_strcpy
 *
 * Copies the source string, up to and including the trailing '\0', into the
 * destination buffer.  It does not (can not) verify that the destination
 * buffer is large enough.  It returns the "dest" argument.
 */

PR_EXTERN(char *)
PL_strcpy(char *dest, const char *src);

/*
 * PL_strncpy
 *
 * Copies the source string into the destination buffer, up to and including
 * the trailing '\0' or up to and including the max'th character, whichever
 * comes first.  It does not (can not) verify that the destination buffer is
 * large enough.  If the source string is longer than the maximum length,
 * the result will *not* be null-terminated (JLRU).
 */

PR_EXTERN(char *)
PL_strncpy(char *dest, const char *src, PRUint32 max);

/*
 * PL_strncpyz
 *
 * Copies the source string into the destination buffer, up to and including
 * the trailing '\0' or up but not including the max'th character, whichever
 * comes first.  It does not (can not) verify that the destination buffer is
 * large enough.  The destination string is always terminated with a '\0',
 * unlike the traditional libc implementation.  It returns the "dest" argument.
 *
 * NOTE: If you call this with a source "abcdefg" and a max of 5, the
 * destination will end up with "abcd\0" (i.e., its strlen length will be 4)!
 *
 * This means you can do this:
 *
 *     char buffer[ SOME_SIZE ];
 *     PL_strncpyz(buffer, src, sizeof(buffer));
 *
 * and the result will be properly terminated.
 */

PR_EXTERN(char *)
PL_strncpyz(char *dest, const char *src, PRUint32 max);

/*
 * PL_strdup
 *
 * Returns a pointer to a malloc'd extent of memory containing a duplicate
 * of the argument string.  The size of the allocated extent is one greater
 * than the length of the argument string, because of the terminator.  A
 * null argument, like a zero-length argument, will result in a pointer to
 * a one-byte extent containing the null value.  This routine returns null
 * upon malloc failure.
 */

PR_EXTERN(char *)
PL_strdup(const char *s);

/*
 * PL_strfree
 *
 * Free memory allocated by PL_strdup
 */

PR_EXTERN(void)
PL_strfree(char *s);

/*
 * PL_strndup
 *
 * Returns a pointer to a malloc'd extent of memory containing a duplicate
 * of the argument string, up to the maximum specified.  If the argument
 * string has a length greater than the value of the specified maximum, the
 * return value will be a pointer to an extent of memory of length one
 * greater than the maximum specified.  A null string, a zero-length string,
 * or a zero maximum will all result in a pointer to a one-byte extent
 * containing the null value.  This routine returns null upon malloc failure.
 */

PR_EXTERN(char *)
PL_strndup(const char *s, PRUint32 max);

/*
 * PL_strcat
 *
 * Appends a copy of the string pointed to by the second argument to the
 * end of the string pointed to by the first.  The destination buffer is
 * not (can not be) checked for sufficient size.  A null destination
 * argument returns null; otherwise, the first argument is returned.
 */

PR_EXTERN(char *)
PL_strcat(char *dst, const char *src);

/*
 * PL_strncat
 *
 * Appends a copy of the string pointed to by the second argument, up to
 * the maximum size specified, to the end of the string pointed to by the
 * first.  The destination buffer is not (can not be) checked for sufficient
 * size.  A null destination argument returns null; otherwise, the first
 * argument is returned.  If the maximum size limits the copy, then the
 * result will *not* be null-terminated (JLRU).  A null destination
 * returns null; otherwise, the destination argument is returned.
 */

PR_EXTERN(char *)
PL_strncat(char *dst, const char *src, PRUint32 max);

/*
 * PL_strcatn
 *
 * Appends a copy of the string pointed to by the third argument, to the
 * end of the string pointed to by the first.  The second argument specifies
 * the maximum size of the destination buffer, including the null termination.
 * If the existing string in dst is longer than the max, no action is taken.
 * The resulting string will be null-terminated.  A null destination returns
 * null; otherwise, the destination argument is returned.
 */

PR_EXTERN(char *)
PL_strcatn(char *dst, PRUint32 max, const char *src);

/*
 * PL_strcmp
 *
 * Returns an integer, the sign of which -- positive, zero, or negative --
 * reflects the lexical sorting order of the two strings indicated.  The
 * result is positive if the first string comes after the second.  The
 * NSPR implementation is not i18n.
 */

PR_EXTERN(PRIntn)
PL_strcmp(const char *a, const char *b);

/*
 * PL_strncmp
 *
 * Returns an integer, the sign of which -- positive, zero, or negative --
 * reflects the lexical sorting order of the two strings indicated, up to
 * the maximum specified.  The result is positive if the first string comes
 * after the second.  The NSPR implementation is not i18n.  If the maximum
 * is zero, only the existance or non-existance (pointer is null) of the
 * strings is compared.
 */

PR_EXTERN(PRIntn)
PL_strncmp(const char *a, const char *b, PRUint32 max);

/*
 * PL_strcasecmp
 *
 * Returns an integer, the sign of which -- positive, zero or negative --
 * reflects the case-insensitive lexical sorting order of the two strings
 * indicated.  The result is positive if the first string comes after the
 * second.  The NSPR implementation is not i18n.
 */

PR_EXTERN(PRIntn)
PL_strcasecmp(const char *a, const char *b);

/*
 * PL_strncasecmp
 *
 * Returns an integer, the sign of which -- positive, zero or negative --
 * reflects the case-insensitive lexical sorting order of the first n characters
 * of the two strings indicated.  The result is positive if the first string comes
 * after the second.  The NSPR implementation is not i18n.
 */

PR_EXTERN(PRIntn)
PL_strncasecmp(const char *a, const char *b, PRUint32 max);

/*
 * PL_strchr
 *
 * Returns a pointer to the first instance of the specified character in the
 * provided string.  It returns null if the character is not found, or if the
 * provided string is null.  The character may be the null character.
 */

PR_EXTERN(char *)
PL_strchr(const char *s, char c);

/*
 * PL_strrchr
 *
 * Returns a pointer to the last instance of the specified character in the
 * provided string.  It returns null if the character is not found, or if the
 * provided string is null.  The character may be the null character.
 */

PR_EXTERN(char *)
PL_strrchr(const char *s, char c);

/*
 * PL_strnchr
 *
 * Returns a pointer to the first instance of the specified character within the
 * first n characters of the provided string.  It returns null if the character
 * is not found, or if the provided string is null.  The character may be the
 * null character.
 */

PR_EXTERN(char *)
PL_strnchr(const char *s, char c, PRUint32 n);

/*
 * PL_strnrchr
 *
 * Returns a pointer to the last instance of the specified character within the
 * first n characters of the provided string.  It returns null if the character is
 * not found, or if the provided string is null.  The character may be the null
 * character.
 */

PR_EXTERN(char *)
PL_strnrchr(const char *s, char c, PRUint32 n);

/*
 * NOTE: Looking for strcasechr, strcaserchr, strncasechr, or strncaserchr?
 * Use strpbrk, strprbrk, strnpbrk or strnprbrk.
 */

/*
 * PL_strpbrk
 *
 * Returns a pointer to the first instance in the first string of any character
 * (not including the terminating null character) of the second string.  It returns
 * null if either string is null.
 */

PR_EXTERN(char *)
PL_strpbrk(const char *s, const char *list);

/*
 * PL_strprbrk
 *
 * Returns a pointer to the last instance in the first string of any character
 * (not including the terminating null character) of the second string.  It returns
 * null if either string is null.
 */

PR_EXTERN(char *)
PL_strprbrk(const char *s, const char *list);

/*
 * PL_strnpbrk
 *
 * Returns a pointer to the first instance (within the first n characters) of any
 * character (not including the terminating null character) of the second string.
 * It returns null if either string is null.
 */

PR_EXTERN(char *)
PL_strnpbrk(const char *s, const char *list, PRUint32 n);

/*
 * PL_strnprbrk
 *
 * Returns a pointer to the last instance (within the first n characters) of any
 * character (not including the terminating null character) of the second string.
 * It returns null if either string is null.
 */

PR_EXTERN(char *)
PL_strnprbrk(const char *s, const char *list, PRUint32 n);

/*
 * PL_strstr
 *
 * Returns a pointer to the first instance of the little string within the
 * big one.  It returns null if either string is null.
 */

PR_EXTERN(char *)
PL_strstr(const char *big, const char *little);

/*
 * PL_strrstr
 *
 * Returns a pointer to the last instance of the little string within the big one.
 * It returns null if either string is null.
 */

PR_EXTERN(char *)
PL_strrstr(const char *big, const char *little);

/*
 * PL_strnstr
 *
 * Returns a pointer to the first instance of the little string within the first
 * n characters of the big one.  It returns null if either string is null.  It
 * returns null if the length of the little string is greater than n.
 */

PR_EXTERN(char *)
PL_strnstr(const char *big, const char *little, PRUint32 n);

/*
 * PL_strnrstr
 *
 * Returns a pointer to the last instance of the little string within the first
 * n characters of the big one.  It returns null if either string is null.  It
 * returns null if the length of the little string is greater than n.
 */

PR_EXTERN(char *)
PL_strnrstr(const char *big, const char *little, PRUint32 max);

/*
 * PL_strcasestr
 *
 * Returns a pointer to the first instance of the little string within the big one,
 * ignoring case.  It returns null if either string is null.
 */

PR_EXTERN(char *)
PL_strcasestr(const char *big, const char *little);

/*
 * PL_strcaserstr
 *
 * Returns a pointer to the last instance of the little string within the big one,
 * ignoring case.  It returns null if either string is null.
 */

PR_EXTERN(char *)
PL_strcaserstr(const char *big, const char *little);

/*
 * PL_strncasestr
 *
 * Returns a pointer to the first instance of the little string within the first
 * n characters of the big one, ignoring case.  It returns null if either string is
 * null.  It returns null if the length of the little string is greater than n.
 */

PR_EXTERN(char *)
PL_strncasestr(const char *big, const char *little, PRUint32 max);

/*
 * PL_strncaserstr
 *
 * Returns a pointer to the last instance of the little string within the first
 * n characters of the big one, ignoring case.  It returns null if either string is
 * null.  It returns null if the length of the little string is greater than n.
 */

PR_EXTERN(char *)
PL_strncaserstr(const char *big, const char *little, PRUint32 max);

/*
 * PL_strtok_r
 *
 * Splits the string s1 into tokens, separated by one or more characters
 * from the separator string s2.  The argument lasts points to a
 * user-supplied char * pointer in which PL_strtok_r stores information
 * for it to continue scanning the same string.
 *
 * In the first call to PL_strtok_r, s1 points to a string and the value
 * of *lasts is ignored.  PL_strtok_r returns a pointer to the first
 * token, writes '\0' into the character following the first token, and
 * updates *lasts.
 *
 * In subsequent calls, s1 is null and lasts must stay unchanged from the
 * previous call.  The separator string s2 may be different from call to
 * call.  PL_strtok_r returns a pointer to the next token in s1.  When no
 * token remains in s1, PL_strtok_r returns null.
 */

PR_EXTERN(char *)
PL_strtok_r(char *s1, const char *s2, char **lasts);

/*
 * Things not (yet?) included: strspn/strcspn, strsep.
 * memchr, memcmp, memcpy, memccpy, index, rindex, bcmp, bcopy, bzero.
 * Any and all i18n/l10n stuff.
 */

PR_END_EXTERN_C

#endif /* _plstr_h */
