/*    fakesdio.h
 *
 *    Copyright (C) 2000, by Larry Wall and others
 *
 *    You may distribute under the terms of either the GNU General Public
 *    License or the Artistic License, as specified in the README file.
 *
 */

/*
 * This is "source level" stdio compatibility mode.
 * We try and #define stdio functions in terms of PerlIO.
 */
#define _CANNOT "CANNOT"
#undef FILE
#define FILE			PerlIO
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

/* printf used to live in perl.h like this - more sophisticated 
   than the rest 
 */
#if defined(__GNUC__) && !defined(__STRICT_ANSI__) && !defined(PERL_GCC_PEDANTIC)
#define printf(fmt,args...) PerlIO_stdoutf(fmt,##args)
#else
#define printf PerlIO_stdoutf
#endif

#define fprintf			PerlIO_printf
#define stdin			PerlIO_stdin()
#define stdout			PerlIO_stdout()
#define stderr			PerlIO_stderr()
#define tmpfile()		PerlIO_tmpfile()
#define fclose(f)		PerlIO_close(f)
#define fflush(f)		PerlIO_flush(f)
#define fopen(p,m)		PerlIO_open(p,m)
#define vfprintf(f,fmt,a)	PerlIO_vprintf(f,fmt,a)
#define fgetc(f)		PerlIO_getc(f)
#define fputc(c,f)		PerlIO_putc(f,c)
#define fputs(s,f)		PerlIO_puts(f,s)
#define getc(f)			PerlIO_getc(f)
#define getc_unlocked(f)	PerlIO_getc(f)
#define putc(c,f)		PerlIO_putc(f,c)
#define putc_unlocked(c,f)	PerlIO_putc(c,f)
#define ungetc(c,f)		PerlIO_ungetc(f,c)
#if 0
/* return values of read/write need work */
#define fread(b,s,c,f)		PerlIO_read(f,b,(s*c))
#define fwrite(b,s,c,f)		PerlIO_write(f,b,(s*c))
#else
#define fread(b,s,c,f)		_CANNOT fread
#define fwrite(b,s,c,f)		_CANNOT fwrite
#endif
#define fseek(f,o,w)		PerlIO_seek(f,o,w)
#define ftell(f)		PerlIO_tell(f)
#define rewind(f)		PerlIO_rewind(f)
#define clearerr(f)		PerlIO_clearerr(f)
#define feof(f)			PerlIO_eof(f)
#define ferror(f)		PerlIO_error(f)
#define fdopen(fd,p)		PerlIO_fdopen(fd,p)
#define fileno(f)		PerlIO_fileno(f)
#define popen(c,m)		my_popen(c,m)
#define pclose(f)		my_pclose(f)

#define fsetpos(f,p)		_CANNOT _fsetpos_
#define fgetpos(f,p)		_CANNOT _fgetpos_

#define __filbuf(f)		_CANNOT __filbuf_
#define _filbuf(f)		_CANNOT _filbuf_
#define __flsbuf(c,f)		_CANNOT __flsbuf_
#define _flsbuf(c,f)		_CANNOT _flsbuf_
#define getw(f)			_CANNOT _getw_
#define putw(v,f)		_CANNOT _putw_
#if SFIO_VERSION < 20000101L
#define flockfile(f)		_CANNOT _flockfile_
#define ftrylockfile(f)		_CANNOT _ftrylockfile_
#define funlockfile(f)		_CANNOT _funlockfile_
#endif
#define freopen(p,m,f)		_CANNOT _freopen_
#define setbuf(f,b)		_CANNOT _setbuf_
#define setvbuf(f,b,x,s)	_CANNOT _setvbuf_
#define fscanf			_CANNOT _fscanf_
#define fgets(s,n,f)		_CANNOT _fgets_

/*
 * ex: set ts=8 sts=4 sw=4 et:
 */
