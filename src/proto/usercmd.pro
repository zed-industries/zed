/* usercmd.c */
char_u *find_ucmd(exarg_T *eap, char_u *p, int *full, expand_T *xp, int *complp);
char_u *set_context_in_user_cmd(expand_T *xp, char_u *arg_in);
char_u *set_context_in_user_cmdarg(char_u *cmd, char_u *arg, long argt, int context, expand_T *xp, int forceit);
char_u *expand_user_command_name(int idx);
char_u *get_user_commands(expand_T *xp, int idx);
char_u *get_user_command_name(int idx, int cmdidx);
char_u *get_user_cmd_addr_type(expand_T *xp, int idx);
char_u *get_user_cmd_flags(expand_T *xp, int idx);
char_u *get_user_cmd_nargs(expand_T *xp, int idx);
char_u *get_user_cmd_complete(expand_T *xp, int idx);
char_u *cmdcomplete_type_to_str(int expand);
int cmdcomplete_str_to_type(char_u *complete_str);
char *uc_fun_cmd(void);
int parse_compl_arg(char_u *value, int vallen, int *complp, long *argt, char_u **compl_arg);
char_u *may_get_cmd_block(exarg_T *eap, char_u *p, char_u **tofree, int *flags);
void ex_command(exarg_T *eap);
void ex_comclear(exarg_T *eap);
void uc_clear(garray_T *gap);
void ex_delcommand(exarg_T *eap);
size_t add_win_cmd_modifiers(char_u *buf, cmdmod_T *cmod, int *multi_mods);
size_t produce_cmdmods(char_u *buf, cmdmod_T *cmod, int quote);
void do_ucmd(exarg_T *eap);
/* vim: set ft=c : */
