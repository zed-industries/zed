/* if_lua.c */
int lua_enabled(int verbose);
void lua_end(void);
void ex_lua(exarg_T *eap);
void ex_luado(exarg_T *eap);
void ex_luafile(exarg_T *eap);
void lua_buffer_free(buf_T *o);
void lua_window_free(win_T *o);
void do_luaeval(char_u *str, typval_T *arg, typval_T *rettv);
int set_ref_in_lua(int copyID);
void update_package_paths_in_lua(void);
/* vim: set ft=c : */
