/* locale.c */
char_u *get_mess_lang(void);
void set_lang_var(void);
void init_locale(void);
void ex_language(exarg_T *eap);
void free_locales(void);
char_u *get_lang_arg(expand_T *xp, int idx);
char_u *get_locales(expand_T *xp, int idx);
/* vim: set ft=c : */
