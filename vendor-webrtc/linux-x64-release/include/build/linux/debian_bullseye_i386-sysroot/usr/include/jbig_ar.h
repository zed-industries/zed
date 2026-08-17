/*
 *  Header file for the arithmetic encoder and decoder of
 *  the portable JBIG compression library
 *
 *  Markus Kuhn -- http://www.cl.cam.ac.uk/~mgk25/jbigkit/
 */

#ifndef JBG_AR_H
#define JBG_AR_H

/*
 * Status of arithmetic encoder
 */

struct jbg_arenc_state {
  unsigned char st[4096];    /* probability status for contexts, MSB = MPS */
  unsigned long c;                /* register C: base of coding intervall, *
                                   * layout as in Table 23                 */
  unsigned long a;       /* register A: normalized size of coding interval */
  long sc;     /* number of buffered 0xff values that might still overflow */
  int ct;  /* bit shift counter, determines when next byte will be written */
  int buffer;                /* buffer for most recent output byte != 0xff */
  void (*byte_out)(int, void *);  /* function that receives all PSCD bytes */
  void *file;                              /* parameter passed to byte_out */
};

/*
 * Status of arithmetic decoder
 */

struct jbg_ardec_state {
  unsigned char st[4096];    /* probability status for contexts, MSB = MPS */
  unsigned long c;                /* register C: base of coding intervall, *
                                   * layout as in Table 25                 */
  unsigned long a;       /* register A: normalized size of coding interval */
  unsigned char *pscd_ptr;               /* pointer to next PSCD data byte */
  unsigned char *pscd_end;                   /* pointer to byte after PSCD */
  int ct;    /* bit-shift counter, determines when next byte will be read;
              * special value -1 signals that zero-padding has started     */
  int startup;          /* boolean flag that controls initial fill of s->c */
  int nopadding;        /* boolean flag that triggers return -2 between
			 * reaching PSCD end and decoding the first symbol
			 * that might never have been encoded in the first
			 * place */
};

void arith_encode_init(struct jbg_arenc_state *s, int reuse_st);
void arith_encode_flush(struct jbg_arenc_state *s);
void arith_encode(struct jbg_arenc_state *s, int cx, int pix);
void arith_decode_init(struct jbg_ardec_state *s, int reuse_st);
int  arith_decode(struct jbg_ardec_state *s, int cx);

#endif /* JBG_AR_H */
