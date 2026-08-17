/*
 *  Header file for the T.85 "light" version of the portable
 *  JBIG image compression library
 *
 *  Copyright 1995-2014 -- Markus Kuhn -- http://www.cl.cam.ac.uk/~mgk25/
 */

#ifndef JBG85_H
#define JBG85_H

#include <stddef.h>
#include "jbig_ar.h"

/*
 * JBIG-KIT version number
 */

#define JBG85_VERSION    "2.1"
#define JBG85_VERSION_MAJOR 2
#define JBG85_VERSION_MINOR 1

/*
 * JBIG-KIT licence agreement reference code:
 * If you use JBIG-KIT under a commercial licence, please replace
 * below the letters GPL with the reference code that you received
 * with your licence agreement. (This code is typically a letter "A"
 * followed by four decimal digits, e.g. "A1234".)
 */

#define JBG85_LICENCE    "GPL"

/*
 * Maximum number of ATMOVEs per stripe that decoder can handle
 */

#define JBG85_ATMOVES_MAX  1

#ifndef JBG_LRLTWO

/*
 * Option and order flags
 */

#define JBG_LRLTWO     0x40
#define JBG_VLENGTH    0x20
#define JBG_TPBON      0x08

/*
 * Possible error code return values
 */

#define JBG_EOK        (0 << 4)
#define JBG_EOK_INTR   (1 << 4)
#define JBG_EAGAIN     (2 << 4)
#define JBG_ENOMEM     (3 << 4)
#define JBG_EABORT     (4 << 4)
#define JBG_EMARKER    (5 << 4)
#define JBG_EINVAL     (6 << 4)
#define JBG_EIMPL      (7 << 4)

#endif

/*
 * Status of a JBIG encoder
 */

struct jbg85_enc_state {
  unsigned long x0, y0;                         /* size of the input image */
  unsigned long l0;                          /* number of lines per stripe */
  int options;                                      /* encoding parameters */
  int newlen;     /* 0 = jbg85_enc_newlen() has not yet been called
                     1 = jbg85_enc_newlen() has updated y0, NEWLEN pending
                     2 = NEWLEN has already been output                    */
  unsigned mx;                               /* maximum ATMOVE window size */
  unsigned long y;                       /* next line number to be encoded */
  unsigned long i;            /* next per-stripe line number to be encoded */
  int tx;                           /* x-offset of adaptive template pixel */
  unsigned long c_all, c[128];    /* adaptive template algorithm variables */
  int new_tx;            /* -1 = no ATMOVE pending, otherwise new TX value */
  int ltp_old;                           /* true if line y-1 was "typical" */
  struct jbg_arenc_state s;                   /* arithmetic encoder status */
  void (*data_out)(unsigned char *start, size_t len, void *file);
                                                    /* data write callback */
  void *file;                            /* parameter passed to data_out() */
  unsigned char *comment; /* content of comment marker segment to be added
                             at next opportunity (will be reset to NULL
                             as soon as comment has been written)          */
  unsigned long comment_len;       /* length of data pointed to by comment */
};


/*
 * Status of a JBIG decoder
 */

struct jbg85_dec_state {
  /* data from BIH */
  unsigned long x0, y0;                          /* size of the full image */
  unsigned long l0;                          /* number of lines per stripe */
  int options;                                      /* encoding parameters */
  int mx;                                    /* maximum ATMOVE window size */
  /* image data */
  int p[3];    /* curr. line starts at linebuf+bpl*p[0], prev. line starts
                * at linebuf+bpl*p[1], its predecessor at linebuf+bpl*p[2] */
  unsigned char *linebuf;              /* buffer region provided by caller */
  size_t linebuf_len;
  size_t bpl;                                            /* bytes per line */
  /* status information */
  int tx;                                         /*  x-offset of AT pixel */
  struct jbg_ardec_state s;                   /* arithmetic decoder status */
  unsigned long bie_len;                    /* number of bytes read so far */
  unsigned char buffer[20]; /* used to store BIH or marker segments fragm. */
  int buf_len;                                /* number of bytes in buffer */
  unsigned long comment_skip;      /* remaining bytes of a COMMENT segment */
  unsigned long x;                             /* x position of next pixel */
  unsigned long stripe;                                  /* current stripe */
  unsigned long y;                      /* line in image (first line is 0) */ 
  unsigned long i;   /* line in current stripe (first line of stripe is 0) */ 
  int at_moves;                /* number of AT moves in the current stripe */
  unsigned long at_line[JBG85_ATMOVES_MAX];         /* lines at which an   *
					             * AT move will happen */
  int at_tx[JBG85_ATMOVES_MAX];      /* ATMOVE x-offsets in current stripe */
  unsigned long line_h1, line_h2, line_h3;     /* variables of decode_pscd */
  int pseudo;         /* flag for TPBON/TPDON:  next pixel is pseudo pixel */
  int lntp;                            /* flag for TP: line is not typical */
  int (*line_out)(const struct jbg85_dec_state *s,
		  unsigned char *start, size_t len,
		  unsigned long y, void *file);
                                                    /* data write callback */
  void *file;                            /* parameter passed to data_out() */
  int intr;                      /* flag that line_out requested interrupt */
  int end_of_bie;       /* flag that the end of the BIE has been signalled */
};


/* function prototypes */

void jbg85_enc_init(struct jbg85_enc_state *s,
		    unsigned long x0, unsigned long y0,
		    void (*data_out)(unsigned char *start, size_t len,
				     void *file),
		    void *file);
void jbg85_enc_options(struct jbg85_enc_state *s, int options,
		       unsigned long l0, int mx);
void jbg85_enc_lineout(struct jbg85_enc_state *s, unsigned char *line,
		       unsigned char *prevline, unsigned char *prevprevline);
void jbg85_enc_newlen(struct jbg85_enc_state *s, unsigned long y0);
void jbg85_enc_abort(struct jbg85_enc_state *s);

void jbg85_dec_init(struct jbg85_dec_state *s,
		    unsigned char *buf, size_t buflen,
		    int (*line_out)(const struct jbg85_dec_state *s,
				    unsigned char *start, size_t len,
				    unsigned long y, void *file),
		    void *file);
int  jbg85_dec_in(struct jbg85_dec_state *s, unsigned char *data, size_t len,
		  size_t *cnt);
int  jbg85_dec_end(struct jbg85_dec_state *s);
const char *jbg85_strerror(int errnum);

/* some macros for examining decoder state */

#define jbg85_dec_finished(s)    ((s)->bie_len == 20 && (s)->y >= (s)->y0)
/* enquire about image size */
#define jbg85_dec_getwidth(s)    ((s)->x0)
#define jbg85_dec_getheight(s)   ((s)->y0)
/* enquire about validity of image-size results */
#define jbg85_dec_validwidth(s)  ((s)->bie_len == 20)
#define jbg85_dec_finalheight(s) ((s)->bie_len == 20 &&			\
				  ((((s)->options & JBG_VLENGHT) == 0) || \
				   ((s)->y >= (s)->y0)))

#endif /* JBG85_H */
