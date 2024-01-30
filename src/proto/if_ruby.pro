/* if_ruby.c */
int ruby_enabled(int verbose);
void ruby_end(void);
void ex_ruby(exarg_T *eap);
void ex_rubydo(exarg_T *eap);
void ex_rubyfile(exarg_T *eap);
void ruby_buffer_free(buf_T *buf);
void ruby_window_free(win_T *win);
void vim_ruby_init(void *stack_start);
void do_rubyeval(char_u *str, typval_T *rettv);
/* vim: set ft=c : */
