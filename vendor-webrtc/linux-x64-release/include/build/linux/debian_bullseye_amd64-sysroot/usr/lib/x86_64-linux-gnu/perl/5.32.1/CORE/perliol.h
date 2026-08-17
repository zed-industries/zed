#ifndef PERLIOL_H_
#define PERLIOL_H_

typedef struct {
    PerlIO_funcs *funcs;
    SV *arg;
} PerlIO_pair_t;

struct PerlIO_list_s {
    IV refcnt;
    IV cur;
    IV len;
    PerlIO_pair_t *array;
};

struct _PerlIO_funcs {
    Size_t fsize;
    const char *name;
    Size_t size;
    U32 kind;
    IV (*Pushed) (pTHX_ PerlIO *f, const char *mode, SV *arg, PerlIO_funcs *tab);
    IV (*Popped) (pTHX_ PerlIO *f);
    PerlIO *(*Open) (pTHX_ PerlIO_funcs *tab,
		     PerlIO_list_t *layers, IV n,
		     const char *mode,
		     int fd, int imode, int perm,
		     PerlIO *old, int narg, SV **args);
    IV (*Binmode)(pTHX_ PerlIO *f);
    SV *(*Getarg) (pTHX_ PerlIO *f, CLONE_PARAMS *param, int flags);
    IV (*Fileno) (pTHX_ PerlIO *f);
    PerlIO *(*Dup) (pTHX_ PerlIO *f, PerlIO *o, CLONE_PARAMS *param, int flags);
    /* Unix-like functions - cf sfio line disciplines */
     SSize_t(*Read) (pTHX_ PerlIO *f, void *vbuf, Size_t count);
     SSize_t(*Unread) (pTHX_ PerlIO *f, const void *vbuf, Size_t count);
     SSize_t(*Write) (pTHX_ PerlIO *f, const void *vbuf, Size_t count);
    IV (*Seek) (pTHX_ PerlIO *f, Off_t offset, int whence);
     Off_t(*Tell) (pTHX_ PerlIO *f);
    IV (*Close) (pTHX_ PerlIO *f);
    /* Stdio-like buffered IO functions */
    IV (*Flush) (pTHX_ PerlIO *f);
    IV (*Fill) (pTHX_ PerlIO *f);
    IV (*Eof) (pTHX_ PerlIO *f);
    IV (*Error) (pTHX_ PerlIO *f);
    void (*Clearerr) (pTHX_ PerlIO *f);
    void (*Setlinebuf) (pTHX_ PerlIO *f);
    /* Perl's snooping functions */
    STDCHAR *(*Get_base) (pTHX_ PerlIO *f);
     Size_t(*Get_bufsiz) (pTHX_ PerlIO *f);
    STDCHAR *(*Get_ptr) (pTHX_ PerlIO *f);
     SSize_t(*Get_cnt) (pTHX_ PerlIO *f);
    void (*Set_ptrcnt) (pTHX_ PerlIO *f, STDCHAR * ptr, SSize_t cnt);
};

/*--------------------------------------------------------------------------------------*/
/* Kind values */
#define PERLIO_K_RAW		0x00000001
#define PERLIO_K_BUFFERED	0x00000002
#define PERLIO_K_CANCRLF	0x00000004
#define PERLIO_K_FASTGETS	0x00000008
#define PERLIO_K_DUMMY		0x00000010
#define PERLIO_K_UTF8		0x00008000
#define PERLIO_K_DESTRUCT	0x00010000
#define PERLIO_K_MULTIARG	0x00020000

/*--------------------------------------------------------------------------------------*/
struct _PerlIO {
    PerlIOl *next;		/* Lower layer */
    PerlIO_funcs *tab;		/* Functions for this layer */
    U32 flags;			/* Various flags for state */
    int err;			/* Saved errno value */
#ifdef VMS
    unsigned os_err;		/* Saved vaxc$errno value */
#elif defined (OS2)
    unsigned long os_err;
#elif defined (WIN32)
    DWORD os_err;		/* Saved GetLastError() value */
#endif
    PerlIOl *head;		/* our ultimate parent pointer */
};

/*--------------------------------------------------------------------------------------*/

/* Flag values */
#define PERLIO_F_EOF		0x00000100
#define PERLIO_F_CANWRITE	0x00000200
#define PERLIO_F_CANREAD	0x00000400
#define PERLIO_F_ERROR		0x00000800
#define PERLIO_F_TRUNCATE	0x00001000
#define PERLIO_F_APPEND		0x00002000
#define PERLIO_F_CRLF		0x00004000
#define PERLIO_F_UTF8		0x00008000
#define PERLIO_F_UNBUF		0x00010000
#define PERLIO_F_WRBUF		0x00020000
#define PERLIO_F_RDBUF		0x00040000
#define PERLIO_F_LINEBUF	0x00080000
#define PERLIO_F_TEMP		0x00100000
#define PERLIO_F_OPEN		0x00200000
#define PERLIO_F_FASTGETS	0x00400000
#define PERLIO_F_TTY		0x00800000
#define PERLIO_F_NOTREG         0x01000000   
#define PERLIO_F_CLEARED        0x02000000 /* layer cleared but not freed */

#define PerlIOBase(f)      (*(f))
#define PerlIOSelf(f,type) ((type *)PerlIOBase(f))
#define PerlIONext(f)      (&(PerlIOBase(f)->next))
#define PerlIOValid(f)     ((f) && *(f))

/*--------------------------------------------------------------------------------------*/
/* Data exports - EXTCONST rather than extern is needed for Cygwin */
#undef EXTPERLIO 
#ifdef PERLIO_FUNCS_CONST
#define EXTPERLIO EXTCONST
#else
#define EXTPERLIO EXT
#endif
EXTPERLIO PerlIO_funcs PerlIO_unix;
EXTPERLIO PerlIO_funcs PerlIO_perlio;
EXTPERLIO PerlIO_funcs PerlIO_stdio;
EXTPERLIO PerlIO_funcs PerlIO_crlf;
EXTPERLIO PerlIO_funcs PerlIO_utf8;
EXTPERLIO PerlIO_funcs PerlIO_byte;
EXTPERLIO PerlIO_funcs PerlIO_raw;
EXTPERLIO PerlIO_funcs PerlIO_pending;
#ifdef WIN32
EXTPERLIO PerlIO_funcs PerlIO_win32;
#endif
PERL_CALLCONV PerlIO *PerlIO_allocate(pTHX);
PERL_CALLCONV SV *PerlIO_arg_fetch(PerlIO_list_t *av, IV n);
#define PerlIOArg PerlIO_arg_fetch(layers,n)

#ifdef PERLIO_USING_CRLF
#define PERLIO_STDTEXT "t"
#else
#define PERLIO_STDTEXT ""
#endif

/*--------------------------------------------------------------------------------------*/
/* perlio buffer layer
   As this is reasonably generic its struct and "methods" are declared here
   so they can be used to "inherit" from it.
*/

typedef struct {
    struct _PerlIO base;	/* Base "class" info */
    STDCHAR *buf;		/* Start of buffer */
    STDCHAR *end;		/* End of valid part of buffer */
    STDCHAR *ptr;		/* Current position in buffer */
    Off_t posn;			/* Offset of buf into the file */
    Size_t bufsiz;		/* Real size of buffer */
    IV oneword;			/* Emergency buffer */
} PerlIOBuf;

PERL_CALLCONV int PerlIO_apply_layera(pTHX_ PerlIO *f, const char *mode,
		    PerlIO_list_t *layers, IV n, IV max);
PERL_CALLCONV int PerlIO_parse_layers(pTHX_ PerlIO_list_t *av, const char *names);
PERL_CALLCONV PerlIO_funcs *PerlIO_layer_fetch(pTHX_ PerlIO_list_t *av, IV n, PerlIO_funcs *def);


PERL_CALLCONV SV *PerlIO_sv_dup(pTHX_ SV *arg, CLONE_PARAMS *param);
PERL_CALLCONV void PerlIO_cleantable(pTHX_ PerlIOl **tablep);
PERL_CALLCONV SV * PerlIO_tab_sv(pTHX_ PerlIO_funcs *tab);
PERL_CALLCONV void PerlIO_default_buffer(pTHX_ PerlIO_list_t *av);
PERL_CALLCONV void PerlIO_stdstreams(pTHX);
PERL_CALLCONV int PerlIO__close(pTHX_ PerlIO *f);
PERL_CALLCONV PerlIO_list_t * PerlIO_resolve_layers(pTHX_ const char *layers, const char *mode, int narg, SV **args);
PERL_CALLCONV PerlIO_funcs * PerlIO_default_layer(pTHX_ I32 n);
PERL_CALLCONV PerlIO_list_t * PerlIO_default_layers(pTHX);
PERL_CALLCONV PerlIO * PerlIO_reopen(const char *path, const char *mode, PerlIO *f);

PERL_CALLCONV PerlIO_list_t *PerlIO_list_alloc(pTHX);
PERL_CALLCONV PerlIO_list_t *PerlIO_clone_list(pTHX_ PerlIO_list_t *proto, CLONE_PARAMS *param);
PERL_CALLCONV void PerlIO_list_free(pTHX_ PerlIO_list_t *list);
PERL_CALLCONV void PerlIO_list_push(pTHX_ PerlIO_list_t *list, PerlIO_funcs *funcs, SV *arg);
PERL_CALLCONV void PerlIO_list_free(pTHX_ PerlIO_list_t *list);

/* PerlIO_teardown doesn't need exporting, but the EXTERN_C is needed
 * for compiling as C++.  Must also match with what perl.h says. */
EXTERN_C void PerlIO_teardown(void);

/*--------------------------------------------------------------------------------------*/
/* Generic, or stub layer functions */

PERL_CALLCONV IV        PerlIOBase_binmode(pTHX_ PerlIO *f);
PERL_CALLCONV void      PerlIOBase_clearerr(pTHX_ PerlIO *f);
PERL_CALLCONV IV        PerlIOBase_close(pTHX_ PerlIO *f);
PERL_CALLCONV PerlIO *  PerlIOBase_dup(pTHX_ PerlIO *f, PerlIO *o, CLONE_PARAMS *param, int flags);
PERL_CALLCONV IV        PerlIOBase_eof(pTHX_ PerlIO *f);
PERL_CALLCONV IV        PerlIOBase_error(pTHX_ PerlIO *f);
PERL_CALLCONV IV        PerlIOBase_fileno(pTHX_ PerlIO *f);
PERL_CALLCONV void      PerlIOBase_flush_linebuf(pTHX);
PERL_CALLCONV IV        PerlIOBase_noop_fail(pTHX_ PerlIO *f);
PERL_CALLCONV IV        PerlIOBase_noop_ok(pTHX_ PerlIO *f);
PERL_CALLCONV IV        PerlIOBase_popped(pTHX_ PerlIO *f);
PERL_CALLCONV IV        PerlIOBase_pushed(pTHX_ PerlIO *f, const char *mode, SV *arg, PerlIO_funcs *tab);
PERL_CALLCONV PerlIO *  PerlIOBase_open(pTHX_ PerlIO_funcs *self, PerlIO_list_t *layers, IV n, const char *mode, int fd, int imode, int perm, PerlIO *old, int narg, SV **args);
PERL_CALLCONV SSize_t   PerlIOBase_read(pTHX_ PerlIO *f, void *vbuf, Size_t count);
PERL_CALLCONV void      PerlIOBase_setlinebuf(pTHX_ PerlIO *f);
PERL_CALLCONV SSize_t   PerlIOBase_unread(pTHX_ PerlIO *f, const void *vbuf, Size_t count);

/* Buf */
PERL_CALLCONV Size_t    PerlIOBuf_bufsiz(pTHX_ PerlIO *f);
PERL_CALLCONV IV        PerlIOBuf_close(pTHX_ PerlIO *f);
PERL_CALLCONV PerlIO *  PerlIOBuf_dup(pTHX_ PerlIO *f, PerlIO *o, CLONE_PARAMS *param, int flags);
PERL_CALLCONV IV        PerlIOBuf_fill(pTHX_ PerlIO *f);
PERL_CALLCONV IV        PerlIOBuf_flush(pTHX_ PerlIO *f);
PERL_CALLCONV STDCHAR * PerlIOBuf_get_base(pTHX_ PerlIO *f);
PERL_CALLCONV SSize_t   PerlIOBuf_get_cnt(pTHX_ PerlIO *f);
PERL_CALLCONV STDCHAR * PerlIOBuf_get_ptr(pTHX_ PerlIO *f);
PERL_CALLCONV PerlIO *  PerlIOBuf_open(pTHX_ PerlIO_funcs *self, PerlIO_list_t *layers, IV n, const char *mode, int fd, int imode, int perm, PerlIO *old, int narg, SV **args);
PERL_CALLCONV IV        PerlIOBuf_popped(pTHX_ PerlIO *f);
PERL_CALLCONV IV        PerlIOBuf_pushed(pTHX_ PerlIO *f, const char *mode, SV *arg, PerlIO_funcs *tab);
PERL_CALLCONV SSize_t   PerlIOBuf_read(pTHX_ PerlIO *f, void *vbuf, Size_t count);
PERL_CALLCONV IV        PerlIOBuf_seek(pTHX_ PerlIO *f, Off_t offset, int whence);
PERL_CALLCONV void      PerlIOBuf_set_ptrcnt(pTHX_ PerlIO *f, STDCHAR * ptr, SSize_t cnt);
PERL_CALLCONV Off_t     PerlIOBuf_tell(pTHX_ PerlIO *f);
PERL_CALLCONV SSize_t   PerlIOBuf_unread(pTHX_ PerlIO *f, const void *vbuf, Size_t count);
PERL_CALLCONV SSize_t   PerlIOBuf_write(pTHX_ PerlIO *f, const void *vbuf, Size_t count);

/* Crlf */
PERL_CALLCONV IV        PerlIOCrlf_binmode(pTHX_ PerlIO *f);
PERL_CALLCONV IV        PerlIOCrlf_flush(pTHX_ PerlIO *f);
PERL_CALLCONV SSize_t   PerlIOCrlf_get_cnt(pTHX_ PerlIO *f);
PERL_CALLCONV IV        PerlIOCrlf_pushed(pTHX_ PerlIO *f, const char *mode, SV *arg, PerlIO_funcs *tab);
PERL_CALLCONV void      PerlIOCrlf_set_ptrcnt(pTHX_ PerlIO *f, STDCHAR * ptr, SSize_t cnt);
PERL_CALLCONV SSize_t   PerlIOCrlf_unread(pTHX_ PerlIO *f, const void *vbuf, Size_t count);
PERL_CALLCONV SSize_t   PerlIOCrlf_write(pTHX_ PerlIO *f, const void *vbuf, Size_t count);

/* Pending */
PERL_CALLCONV IV        PerlIOPending_close(pTHX_ PerlIO *f);
PERL_CALLCONV IV        PerlIOPending_fill(pTHX_ PerlIO *f);
PERL_CALLCONV IV        PerlIOPending_flush(pTHX_ PerlIO *f);
PERL_CALLCONV IV        PerlIOPending_pushed(pTHX_ PerlIO *f, const char *mode, SV *arg, PerlIO_funcs *tab);
PERL_CALLCONV SSize_t   PerlIOPending_read(pTHX_ PerlIO *f, void *vbuf, Size_t count);
PERL_CALLCONV IV        PerlIOPending_seek(pTHX_ PerlIO *f, Off_t offset, int whence);
PERL_CALLCONV void      PerlIOPending_set_ptrcnt(pTHX_ PerlIO *f, STDCHAR * ptr, SSize_t cnt);

/* Pop */
PERL_CALLCONV IV        PerlIOPop_pushed(pTHX_ PerlIO *f, const char *mode, SV *arg, PerlIO_funcs *tab);

/* Raw */
PERL_CALLCONV IV        PerlIORaw_pushed(pTHX_ PerlIO *f, const char *mode, SV *arg, PerlIO_funcs *tab);

/* Stdio */
PERL_CALLCONV void      PerlIOStdio_clearerr(pTHX_ PerlIO *f);
PERL_CALLCONV IV        PerlIOStdio_close(pTHX_ PerlIO *f);
PERL_CALLCONV PerlIO *  PerlIOStdio_dup(pTHX_ PerlIO *f, PerlIO *o, CLONE_PARAMS *param, int flags);
PERL_CALLCONV IV        PerlIOStdio_eof(pTHX_ PerlIO *f);
PERL_CALLCONV IV        PerlIOStdio_error(pTHX_ PerlIO *f);
PERL_CALLCONV IV        PerlIOStdio_fileno(pTHX_ PerlIO *f);
#ifdef USE_STDIO_PTR
PERL_CALLCONV STDCHAR * PerlIOStdio_get_ptr(pTHX_ PerlIO *f);
PERL_CALLCONV SSize_t   PerlIOStdio_get_cnt(pTHX_ PerlIO *f);
PERL_CALLCONV void      PerlIOStdio_set_ptrcnt(pTHX_ PerlIO *f, STDCHAR * ptr, SSize_t cnt);
#endif
PERL_CALLCONV IV        PerlIOStdio_fill(pTHX_ PerlIO *f);
PERL_CALLCONV IV        PerlIOStdio_flush(pTHX_ PerlIO *f);
#ifdef FILE_base
PERL_CALLCONV STDCHAR * PerlIOStdio_get_base(pTHX_ PerlIO *f);
PERL_CALLCONV Size_t    PerlIOStdio_get_bufsiz(pTHX_ PerlIO *f);
#endif
PERL_CALLCONV char *    PerlIOStdio_mode(const char *mode, char *tmode);
PERL_CALLCONV PerlIO *  PerlIOStdio_open(pTHX_ PerlIO_funcs *self, PerlIO_list_t *layers, IV n, const char *mode, int fd, int imode, int perm, PerlIO *f, int narg, SV **args);
PERL_CALLCONV IV        PerlIOStdio_pushed(pTHX_ PerlIO *f, const char *mode, SV *arg, PerlIO_funcs *tab);
PERL_CALLCONV SSize_t   PerlIOStdio_read(pTHX_ PerlIO *f, void *vbuf, Size_t count);
PERL_CALLCONV IV        PerlIOStdio_seek(pTHX_ PerlIO *f, Off_t offset, int whence);
PERL_CALLCONV void      PerlIOStdio_setlinebuf(pTHX_ PerlIO *f);
PERL_CALLCONV Off_t     PerlIOStdio_tell(pTHX_ PerlIO *f);
PERL_CALLCONV SSize_t   PerlIOStdio_unread(pTHX_ PerlIO *f, const void *vbuf, Size_t count);
PERL_CALLCONV SSize_t   PerlIOStdio_write(pTHX_ PerlIO *f, const void *vbuf, Size_t count);

/* Unix */
PERL_CALLCONV IV        PerlIOUnix_close(pTHX_ PerlIO *f);
PERL_CALLCONV PerlIO *  PerlIOUnix_dup(pTHX_ PerlIO *f, PerlIO *o, CLONE_PARAMS *param, int flags);
PERL_CALLCONV IV        PerlIOUnix_fileno(pTHX_ PerlIO *f);
PERL_CALLCONV int       PerlIOUnix_oflags(const char *mode);
PERL_CALLCONV PerlIO *  PerlIOUnix_open(pTHX_ PerlIO_funcs *self, PerlIO_list_t *layers, IV n, const char *mode, int fd, int imode, int perm, PerlIO *f, int narg, SV **args);
PERL_CALLCONV IV        PerlIOUnix_pushed(pTHX_ PerlIO *f, const char *mode, SV *arg, PerlIO_funcs *tab);
PERL_CALLCONV SSize_t   PerlIOUnix_read(pTHX_ PerlIO *f, void *vbuf, Size_t count);
PERL_CALLCONV int       PerlIOUnix_refcnt_dec(int fd);
PERL_CALLCONV void      PerlIOUnix_refcnt_inc(int fd);
PERL_CALLCONV int       PerlIOUnix_refcnt(int fd);
PERL_CALLCONV IV        PerlIOUnix_seek(pTHX_ PerlIO *f, Off_t offset, int whence);
PERL_CALLCONV Off_t     PerlIOUnix_tell(pTHX_ PerlIO *f);
PERL_CALLCONV SSize_t   PerlIOUnix_write(pTHX_ PerlIO *f, const void *vbuf, Size_t count);

/* Utf8 */
PERL_CALLCONV IV        PerlIOUtf8_pushed(pTHX_ PerlIO *f, const char *mode, SV *arg, PerlIO_funcs *tab);

#endif				/* PERLIOL_H_ */

/*
 * ex: set ts=8 sts=4 sw=4 et:
 */
