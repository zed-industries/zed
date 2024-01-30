/* vim9script.c */
int in_vim9script(void);
int in_old_script(int max_version);
int current_script_is_vim9(void);
void clear_vim9_scriptlocal_vars(int sid);
void ex_vim9script(exarg_T *eap);
int not_in_vim9(exarg_T *eap);
int vim9_bad_comment(char_u *p);
int vim9_comment_start(char_u *p);
void ex_incdec(exarg_T *eap);
void ex_export(exarg_T *eap);
void free_imports_and_script_vars(int sid);
void mark_imports_for_reload(int sid);
void ex_import(exarg_T *eap);
void import_check_sourced_sid(int *sid);
int find_exported(int sid, char_u *name, ufunc_T **ufunc, type_T **type, cctx_T *cctx, cstack_T *cstack, int verbose);
char_u *vim9_declare_scriptvar(exarg_T *eap, char_u *arg);
void update_vim9_script_var(int create, dictitem_T *di, char_u *name, int flags, typval_T *tv, type_T **type, int do_member);
void hide_script_var(scriptitem_T *si, int idx, int func_defined);
svar_T *find_typval_in_script(typval_T *dest, scid_T sid, int must_find);
int check_script_var_type(svar_T *sv, typval_T *value, char_u *name, where_T where);
int check_reserved_name(char_u *name, int is_objm_access);
/* vim: set ft=c : */
