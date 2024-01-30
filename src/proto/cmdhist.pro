/* cmdhist.c */
int get_hislen(void);
histentry_T *get_histentry(int hist_type);
void set_histentry(int hist_type, histentry_T *entry);
int *get_hisidx(int hist_type);
int *get_hisnum(int hist_type);
int hist_char2type(int c);
char_u *get_history_arg(expand_T *xp, int idx);
void init_history(void);
void clear_hist_entry(histentry_T *hisptr);
int in_history(int type, char_u *str, int move_to_front, int sep, int writing);
void add_to_history(int histype, char_u *new_entry, int in_map, int sep);
void f_histadd(typval_T *argvars, typval_T *rettv);
void f_histdel(typval_T *argvars, typval_T *rettv);
void f_histget(typval_T *argvars, typval_T *rettv);
void f_histnr(typval_T *argvars, typval_T *rettv);
void remove_key_from_history(void);
void ex_history(exarg_T *eap);
/* vim: set ft=c : */
