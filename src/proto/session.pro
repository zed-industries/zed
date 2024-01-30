/* session.c */
void ex_loadview(exarg_T *eap);
int write_session_file(char_u *filename);
void ex_mkrc(exarg_T *eap);
var_flavour_T var_flavour(char_u *varname);
int put_eol(FILE *fd);
int put_line(FILE *fd, char *s);
/* vim: set ft=c : */
