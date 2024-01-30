/* if_python.c */
int python_enabled(int verbose);
void python_end(void);
int python_loaded(void);
void ex_python(exarg_T *eap);
void ex_pyfile(exarg_T *eap);
void ex_pydo(exarg_T *eap);
void python_buffer_free(buf_T *buf);
void python_window_free(win_T *win);
void python_tabpage_free(tabpage_T *tab);
void do_pyeval(char_u *str, typval_T *rettv);
int set_ref_in_python(int copyID);
/* vim: set ft=c : */
