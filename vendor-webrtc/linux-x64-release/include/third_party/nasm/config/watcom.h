/* ----------------------------------------------------------------------- *
 *
 *   Copyright 2016 The NASM Authors - All Rights Reserved
 *   See the file AUTHORS included with the NASM distribution for
 *   the specific copyright holders.
 *
 *   Redistribution and use in source and binary forms, with or without
 *   modification, are permitted provided that the following
 *   conditions are met:
 *
 *   * Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above
 *     copyright notice, this list of conditions and the following
 *     disclaimer in the documentation and/or other materials provided
 *     with the distribution.
 *
 *     THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND
 *     CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
 *     INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 *     MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 *     DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
 *     CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 *     SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 *     NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 *     LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 *     HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 *     CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
 *     OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE,
 *     EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * ----------------------------------------------------------------------- */

/*
 * config/watcom.h
 *
 * Compiler definitions for OpenWatcom instead of config.h.in.
 *  See config.h.in for the variables which can be defined here.
 *
 * This was taken from openwcom.mak and needs to be actually validated.
 */

#ifndef NASM_CONFIG_WATCOM_H
#define NASM_CONFIG_WATCOM_H

#define HAVE_DECL_STRCASECMP 1
#define HAVE_DECL_STRICMP 1
#define HAVE_DECL_STRLCPY 1
#define HAVE_DECL_STRNCASECMP 1
#define HAVE_DECL_STRNICMP 1
#ifndef __LINUX__
#define HAVE_IO_H 1
#endif
#define HAVE_LIMITS_H 1
#define HAVE_MEMORY_H 1
#define HAVE_SNPRINTF 1
#if (__WATCOMC__ >= 1230)
#undef HAVE__BOOL /* need stdbool.h */
#define HAVE_STDBOOL_H 1
#define HAVE_INTTYPES_H 1
#define HAVE_STDINT_H 1
#define HAVE_UINTPTR_T 1
#endif
#define HAVE_STDLIB_H 1
#define HAVE_STRCSPN 1
#define HAVE_STRICMP 1
#define HAVE_STRNICMP 1
#define HAVE_STRSPN 1
#define HAVE_STRING_H 1
#if (__WATCOMC__ >= 1240)
#define HAVE_STRCASECMP 1
#define HAVE_STRNCASECMP 1
#define HAVE_STRLCPY 1
#define HAVE_STRINGS_H 1
#endif
#define HAVE_SYS_STAT_H 1
#define HAVE_SYS_TYPES_H 1
#define HAVE_FCNTL_H 1
#define HAVE_UNISTD_H 1
#define HAVE_VSNPRINTF 1
#define STDC_HEADERS 1

#define HAVE__FULLPATH 1
#define HAVE_ACCESS
#define HAVE_STRUCT_STAT
#define HAVE_STAT
#define HAVE_FSTAT
#define HAVE_FILENO
#ifdef __LINUX__
#define HAVE_FTRUNCATE
#else
#define HAVE_CHSIZE
#define HAVE__CHSIZE
#endif
#define HAVE_ISASCII
#define HAVE_ISCNTRL

#if (__WATCOMC__ >= 1250)
#define restrict __restrict
#else
#define restrict
#endif
#define inline __inline

#endif /* NASM_CONFIG_WATCOM_H */
