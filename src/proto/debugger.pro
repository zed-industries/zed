/* debugger.c */
int has_watchexpr(void);
void do_debug(char_u *cmd);
void ex_debug(exarg_T *eap);
void dbg_check_breakpoint(exarg_T *eap);
int dbg_check_skipped(exarg_T *eap);
void ex_breakadd(exarg_T *eap);
void ex_debuggreedy(exarg_T *eap);
int debug_has_expr_breakpoint(void);
void ex_breakdel(exarg_T *eap);
void ex_breaklist(exarg_T *eap);
linenr_T dbg_find_breakpoint(int file, char_u *fname, linenr_T after);
int has_profiling(int file, char_u *fname, int *fp, hash_T *hashp);
void dbg_breakpoint(char_u *name, linenr_T lnum);
/* vim: set ft=c : */
