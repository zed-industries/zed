/* logfile.c */
void ch_logfile(char_u *fname, char_u *opt);
int ch_log_active(void);
void ch_log_literal(char *lead, channel_T *ch, ch_part_T part, char_u *buf, int len);
void f_ch_log(typval_T *argvars, typval_T *rettv);
void f_ch_logfile(typval_T *argvars, typval_T *rettv);
/* vim: set ft=c : */
