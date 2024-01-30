/* help.c */
void ex_help(exarg_T *eap);
void ex_helpclose(exarg_T *eap);
char_u *check_help_lang(char_u *arg);
int help_heuristic(char_u *matched_string, int offset, int wrong_case);
int find_help_tags(char_u *arg, int *num_matches, char_u ***matches, int keep_lang);
void cleanup_help_tags(int num_file, char_u **file);
void prepare_help_buffer(void);
void fix_help_buffer(void);
void ex_exusage(exarg_T *eap);
void ex_viusage(exarg_T *eap);
void ex_helptags(exarg_T *eap);
/* vim: set ft=c : */
