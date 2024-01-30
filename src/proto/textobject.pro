/* textobject.c */
int findsent(int dir, long count);
int findpar(int *pincl, int dir, long count, int what, int both);
int startPS(linenr_T lnum, int para, int both);
int fwd_word(long count, int bigword, int eol);
int bck_word(long count, int bigword, int stop);
int end_word(long count, int bigword, int stop, int empty);
int bckend_word(long count, int bigword, int eol);
int current_word(oparg_T *oap, long count, int include, int bigword);
int current_sent(oparg_T *oap, long count, int include);
int current_block(oparg_T *oap, long count, int include, int what, int other);
int current_tagblock(oparg_T *oap, long count_arg, int include);
int current_par(oparg_T *oap, long count, int include, int type);
int current_quote(oparg_T *oap, long count, int include, int quotechar);
/* vim: set ft=c : */
