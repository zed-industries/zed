/* if_python3.c */
int python3_enabled(int verbose);
void python3_end(void);
int python3_loaded(void);
void ex_py3(exarg_T *eap);
void ex_py3file(exarg_T *eap);
void ex_py3do(exarg_T *eap);
void python3_buffer_free(buf_T *buf);
void python3_window_free(win_T *win);
void python3_tabpage_free(tabpage_T *tab);
void do_py3eval(char_u *str, typval_T *rettv);
int set_ref_in_python3(int copyID);
int python3_version(void);
/* vim: set ft=c : */
