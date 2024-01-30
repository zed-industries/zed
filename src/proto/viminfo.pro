/* viminfo.c */
int get_viminfo_parameter(int type);
int buf_compare(const void *s1, const void *s2);
void check_marks_read(void);
int read_viminfo(char_u *file, int flags);
void write_viminfo(char_u *file, int forceit);
void ex_viminfo(exarg_T *eap);
/* vim: set ft=c : */
