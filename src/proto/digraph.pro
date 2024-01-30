/* digraph.c */
int do_digraph(int c);
char_u *get_digraph_for_char(int val_arg);
int get_digraph(int cmdline);
int digraph_get(int char1, int char2, int meta_char);
void putdigraph(char_u *str);
void listdigraphs(int use_headers);
void f_digraph_get(typval_T *argvars, typval_T *rettv);
void f_digraph_getlist(typval_T *argvars, typval_T *rettv);
void f_digraph_set(typval_T *argvars, typval_T *rettv);
void f_digraph_setlist(typval_T *argvars, typval_T *rettv);
char *keymap_init(void);
void ex_loadkeymap(exarg_T *eap);
void keymap_clear(garray_T *kmap);
/* vim: set ft=c : */
