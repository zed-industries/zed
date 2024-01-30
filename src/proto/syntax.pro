/* syntax.c */
void syntax_start(win_T *wp, linenr_T lnum);
void syn_stack_free_all(synblock_T *block);
void syn_stack_apply_changes(buf_T *buf);
void syntax_end_parsing(win_T *wp, linenr_T lnum);
int syntax_check_changed(linenr_T lnum);
int get_syntax_attr(colnr_T col, int *can_spell, int keep_state);
void syntax_clear(synblock_T *block);
void reset_synblock(win_T *wp);
void ex_syntax(exarg_T *eap);
void ex_ownsyntax(exarg_T *eap);
int syntax_present(win_T *win);
void reset_expand_highlight(void);
void set_context_in_echohl_cmd(expand_T *xp, char_u *arg);
void set_context_in_syntax_cmd(expand_T *xp, char_u *arg);
char_u *get_syntax_name(expand_T *xp, int idx);
int syn_get_id(win_T *wp, long lnum, colnr_T col, int trans, int *spellp, int keep_state);
int get_syntax_info(int *seqnrp);
int syn_get_sub_char(void);
int syn_get_stack_item(int i);
int syn_get_foldlevel(win_T *wp, long lnum);
void ex_syntime(exarg_T *eap);
char_u *get_syntime_arg(expand_T *xp, int idx);
/* vim: set ft=c : */
