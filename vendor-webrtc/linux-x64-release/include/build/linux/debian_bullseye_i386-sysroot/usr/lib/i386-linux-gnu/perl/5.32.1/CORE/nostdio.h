/*    nostdio.h
 *
 *    Copyright (C) 1996, 2000, 2001, 2005, by Larry Wall and others
 *
 *    You may distribute under the terms of either the GNU General Public
 *    License or the Artistic License, as specified in the README file.
 *
 */

/*
 * Strong denial of stdio - make all stdio calls (we can think of) errors
 */
/* This is a 1st attempt to stop other include files pulling
   in real <stdio.h>.
   A more ambitious set of possible symbols can be found in
   sfio.h (inside an _cplusplus gard).
   It is completely pointless as we have already included it ourselves.
*/

#if !defined(_STDIO_H) && !defined(FILE) && !defined(_STDIO_INCLUDED) && !defined(__STDIO_LOADED)
#define _STDIO_H
#define _STDIO_INCLUDED
#define __STDIO_LOADED
struct _FILE;
#define FILE struct _FILE
#endif

#ifndef EBCDIC

#define _CANNOT "CANNOT"

#undef clearerr
#undef fclose
#undef fdopen
#undef feof
#undef ferror
#undef fflush
#undef fgetc
#undef fgetpos
#undef fgets
#undef fileno
#undef flockfile
#undef fopen
#undef fprintf
#undef fputc
#undef fputs
#undef fread
#undef freopen
#undef fscanf
#undef fseek
#undef fsetpos
#undef ftell
#undef ftrylockfile
#undef funlockfile
#undef fwrite
#undef getc
#undef getc_unlocked
#undef getw
#undef pclose
#undef popen
#undef putc
#undef putc_unlocked
#undef putw
#undef rewind
#undef setbuf
#undef setvbuf
#undef stderr
#undef stdin
#undef stdout
#undef tmpfile
#undef ungetc
#undef vfprintf
#undef printf

#define fprintf    _CANNOT _fprintf_
#define printf     _CANNOT _printf_
#define stdin      _CANNOT _stdin_
#define stdout     _CANNOT _stdout_
#define stderr     _CANNOT _stderr_
#ifndef OS2
#define tmpfile()  _CANNOT _tmpfile_
#endif
#define fclose(f)  _CANNOT _fclose_
#define fflush(f)  _CANNOT _fflush_
#define fopen(p,m)  _CANNOT _fopen_
#define freopen(p,m,f)  _CANNOT _freopen_
#define setbuf(f,b)  _CANNOT _setbuf_
#define setvbuf(f,b,x,s)  _CANNOT _setvbuf_
#define fscanf  _CANNOT _fscanf_
#define vfprintf(f,fmt,a)  _CANNOT _vfprintf_
#define fgetc(f)  _CANNOT _fgetc_
#define fgets(s,n,f)  _CANNOT _fgets_
#define fputc(c,f)  _CANNOT _fputc_
#define fputs(s,f)  _CANNOT _fputs_
#define getc(f)  _CANNOT _getc_
#define putc(c,f)  _CANNOT _putc_
#ifndef OS2
#define ungetc(c,f)  _CANNOT _ungetc_
#endif
#define fread(b,s,c,f)  _CANNOT _fread_
#define fwrite(b,s,c,f)  _CANNOT _fwrite_
#define fgetpos(f,p)  _CANNOT _fgetpos_
#define fseek(f,o,w)  _CANNOT _fseek_
#define fsetpos(f,p)  _CANNOT _fsetpos_
#define ftell(f)  _CANNOT _ftell_
#define rewind(f)  _CANNOT _rewind_
#define clearerr(f)  _CANNOT _clearerr_
#define feof(f)  _CANNOT _feof_
#define ferror(f)  _CANNOT _ferror_
#define __filbuf(f)  _CANNOT __filbuf_
#define __flsbuf(c,f)  _CANNOT __flsbuf_
#define _filbuf(f)  _CANNOT _filbuf_
#define _flsbuf(c,f)  _CANNOT _flsbuf_
#define fdopen(fd,p)  _CANNOT _fdopen_
#define fileno(f)  _CANNOT _fileno_
#if defined(SFIO_VERSION) && SFIO_VERSION < 20000101L
#define flockfile(f)  _CANNOT _flockfile_
#define ftrylockfile(f)  _CANNOT _ftrylockfile_
#define funlockfile(f)  _CANNOT _funlockfile_
#endif
#define getc_unlocked(f)  _CANNOT _getc_unlocked_
#define putc_unlocked(c,f)  _CANNOT _putc_unlocked_
#define popen(c,m)  _CANNOT _popen_
#define getw(f)  _CANNOT _getw_
#define putw(v,f)  _CANNOT _putw_
#ifndef OS2
#define pclose(f)  _CANNOT _pclose_
#endif

#endif /*not define EBCDIC */

/*
 * ex: set ts=8 sts=4 sw=4 et:
 */
