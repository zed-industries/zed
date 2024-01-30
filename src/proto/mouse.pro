/* mouse.c */
void mouse_set_vert_scroll_step(long step);
void mouse_set_hor_scroll_step(long step);
int do_mouse(oparg_T *oap, int c, int dir, long count, int fixindent);
void ins_mouse(int c);
void ins_mousescroll(int dir);
int is_mouse_key(int c);
int get_mouse_button(int code, int *is_click, int *is_drag);
int get_pseudo_mouse_code(int button, int is_click, int is_drag);
void set_mouse_termcode(int n, char_u *s);
void del_mouse_termcode(int n);
void setmouse(void);
int mouse_has(int c);
int mouse_model_popup(void);
void reset_dragwin(void);
int jump_to_mouse(int flags, int *inclusive, int which_button);
int do_mousescroll_horiz(long_u leftcol);
void nv_mousescroll(cmdarg_T *cap);
void nv_mouse(cmdarg_T *cap);
void reset_held_button(void);
int check_termcode_mouse(char_u *tp, int *slen, char_u *key_name, char_u *modifiers_start, int idx, int *modifiers);
int mouse_comp_pos(win_T *win, int *rowp, int *colp, linenr_T *lnump, int *plines_cache);
win_T *mouse_find_win(int *rowp, int *colp, mouse_find_T popup);
int vcol2col(win_T *wp, linenr_T lnum, int vcol, colnr_T *coladdp);
void f_getmousepos(typval_T *argvars, typval_T *rettv);
/* vim: set ft=c : */
