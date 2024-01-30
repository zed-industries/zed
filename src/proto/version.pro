/* version.c */
void init_longVersion(void);
int highest_patch(void);
int has_patch(int n);
void ex_version(exarg_T *eap);
void list_in_columns(char_u **items, int size, int current);
void list_version(void);
void maybe_intro_message(void);
void ex_intro(exarg_T *eap);
/* vim: set ft=c : */
