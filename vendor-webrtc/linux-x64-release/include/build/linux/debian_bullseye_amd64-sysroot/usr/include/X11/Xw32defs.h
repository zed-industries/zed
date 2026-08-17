#ifndef _XW32DEFS_H
# define  _XW32DEFS_H

# ifdef __GNUC__ /* mingw is more close to unix than msvc */
#  if !defined(__daddr_t_defined)
typedef char *caddr_t;
#  endif
#  define lstat stat

# else
typedef char *caddr_t;

#  define access	   _access
#  define alloca	   _alloca
#  define chdir	_chdir
#  define chmod	   _chmod
#  define close	   _close
#  define creat	   _creat
#  define dup	   _dup
#  define dup2	   _dup2
#  define environ     _environ
#  define execl	 _execl
#  define execle	 _execle
#  define execlp	 _execlp
#  define execlpe  _execlpe
#  define execv	 _execv
#  define execve	 _execve
#  define execvp	 _execvp
#  define execvpe  _execvpe
#  define fdopen	  _fdopen
#  define fileno	  _fileno
#  define fstat	 _fstat
#  define getcwd	_getcwd
#  define getpid	 _getpid
#  define hypot		_hypot
#  define isascii __isascii
#  define isatty	   _isatty
#  define lseek	   _lseek
#  define mkdir	_mkdir
#  define mktemp	   _mktemp
#  define open	   _open
#  define putenv	    _putenv
#  define read	   _read
#  define rmdir	_rmdir
#  define sleep(x) Sleep((x) * 1000)
#  define stat	 _stat
#  define sys_errlist _sys_errlist
#  define sys_nerr    _sys_nerr
#  define umask	   _umask
#  define unlink	   _unlink
#  define write	   _write
#  define random   rand
#  define srandom  srand

#  define O_RDONLY    _O_RDONLY
#  define O_WRONLY    _O_WRONLY
#  define O_RDWR	    _O_RDWR
#  define O_APPEND    _O_APPEND
#  define O_CREAT     _O_CREAT
#  define O_TRUNC     _O_TRUNC
#  define O_EXCL	    _O_EXCL
#  define O_TEXT	    _O_TEXT
#  define O_BINARY    _O_BINARY
#  define O_RAW	    _O_BINARY

#  define S_IFMT	 _S_IFMT
#  define S_IFDIR  _S_IFDIR
#  define S_IFCHR  _S_IFCHR
#  define S_IFREG  _S_IFREG
#  define S_IREAD  _S_IREAD
#  define S_IWRITE _S_IWRITE
#  define S_IEXEC  _S_IEXEC

#  define	F_OK	0
#  define	X_OK	1
#  define	W_OK	2
#  define	R_OK	4
# endif /* __GNUC__ */
#endif
