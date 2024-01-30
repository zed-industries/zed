/* bufwrite.c */
char *new_file_message(void);
int buf_write(buf_T *buf, char_u *fname, char_u *sfname, linenr_T start, linenr_T end, exarg_T *eap, int append, int forceit, int reset_changed, int filtering);
/* vim: set ft=c : */
