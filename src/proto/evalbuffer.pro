/* evalbuffer.c */
int set_ref_in_buffers(int copyID);
buf_T *buflist_find_by_name(char_u *name, int curtab_only);
buf_T *find_buffer(typval_T *avar);
void f_append(typval_T *argvars, typval_T *rettv);
void f_appendbufline(typval_T *argvars, typval_T *rettv);
void f_bufadd(typval_T *argvars, typval_T *rettv);
void f_bufexists(typval_T *argvars, typval_T *rettv);
void f_buflisted(typval_T *argvars, typval_T *rettv);
void f_bufload(typval_T *argvars, typval_T *rettv);
void f_bufloaded(typval_T *argvars, typval_T *rettv);
void f_bufname(typval_T *argvars, typval_T *rettv);
void f_bufnr(typval_T *argvars, typval_T *rettv);
void f_bufwinid(typval_T *argvars, typval_T *rettv);
void f_bufwinnr(typval_T *argvars, typval_T *rettv);
void f_deletebufline(typval_T *argvars, typval_T *rettv);
void f_getbufinfo(typval_T *argvars, typval_T *rettv);
void f_getbufline(typval_T *argvars, typval_T *rettv);
void f_getbufoneline(typval_T *argvars, typval_T *rettv);
void f_getline(typval_T *argvars, typval_T *rettv);
void f_setbufline(typval_T *argvars, typval_T *rettv);
void f_setline(typval_T *argvars, typval_T *rettv);
void switch_buffer(bufref_T *save_curbuf, buf_T *buf);
void restore_buffer(bufref_T *save_curbuf);
void switch_to_win_for_buf(buf_T *buf, switchwin_T *switchwin, bufref_T *save_curbuf);
void restore_win_for_buf(switchwin_T *switchwin, bufref_T *save_curbuf);
/* vim: set ft=c : */
