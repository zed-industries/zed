/* match.c */
void clear_matches(win_T *wp);
void init_search_hl(win_T *wp, match_T *search_hl);
void prepare_search_hl(win_T *wp, match_T *search_hl, linenr_T lnum);
int prepare_search_hl_line(win_T *wp, linenr_T lnum, colnr_T mincol, char_u **line, match_T *search_hl, int *search_attr);
int update_search_hl(win_T *wp, linenr_T lnum, colnr_T col, char_u **line, match_T *search_hl, int *has_match_conc, int *match_conc, int did_line_attr, int lcs_eol_one, int *on_last_col);
int get_prevcol_hl_flag(win_T *wp, match_T *search_hl, long curcol);
void get_search_match_hl(win_T *wp, match_T *search_hl, long col, int *char_attr);
void f_clearmatches(typval_T *argvars, typval_T *rettv);
void f_getmatches(typval_T *argvars, typval_T *rettv);
void f_setmatches(typval_T *argvars, typval_T *rettv);
void f_matchadd(typval_T *argvars, typval_T *rettv);
void f_matchaddpos(typval_T *argvars, typval_T *rettv);
void f_matcharg(typval_T *argvars, typval_T *rettv);
void f_matchdelete(typval_T *argvars, typval_T *rettv);
void ex_match(exarg_T *eap);
/* vim: set ft=c : */
