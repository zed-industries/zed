/* regexp.c */
void init_regexp_timeout(long msec);
void disable_regexp_timeout(void);
void save_timeout_for_debugging(void);
void restore_timeout_for_debugging(void);
int re_multiline(regprog_T *prog);
char_u *skip_regexp(char_u *startp, int delim, int magic);
char_u *skip_regexp_err(char_u *startp, int delim, int magic);
char_u *skip_regexp_ex(char_u *startp, int dirc, int magic, char_u **newp, int *dropped, magic_T *magic_val);
reg_extmatch_T *ref_extmatch(reg_extmatch_T *em);
void unref_extmatch(reg_extmatch_T *em);
char_u *regtilde(char_u *source, int magic);
int vim_regsub(regmatch_T *rmp, char_u *source, typval_T *expr, char_u *dest, int destlen, int flags);
int vim_regsub_multi(regmmatch_T *rmp, linenr_T lnum, char_u *source, char_u *dest, int destlen, int flags);
void free_resub_eval_result(void);
char_u *reg_submatch(int no);
list_T *reg_submatch_list(int no);
int vim_regcomp_had_eol(void);
regprog_T *vim_regcomp(char_u *expr_arg, int re_flags);
void vim_regfree(regprog_T *prog);
void free_regexp_stuff(void);
int regprog_in_use(regprog_T *prog);
int vim_regexec_prog(regprog_T **prog, int ignore_case, char_u *line, colnr_T col);
int vim_regexec(regmatch_T *rmp, char_u *line, colnr_T col);
int vim_regexec_nl(regmatch_T *rmp, char_u *line, colnr_T col);
long vim_regexec_multi(regmmatch_T *rmp, win_T *win, buf_T *buf, linenr_T lnum, colnr_T col, int *timed_out);
/* vim: set ft=c : */
