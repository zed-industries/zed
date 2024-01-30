/* clientserver.c */
void server_to_input_buf(char_u *str);
char_u *eval_client_expr_to_string(char_u *expr);
int sendToLocalVim(char_u *cmd, int asExpr, char_u **result);
char_u *serverConvert(char_u *client_enc, char_u *data, char_u **tofree);
void exec_on_server(mparm_T *parmp);
void prepare_server(mparm_T *parmp);
void f_remote_expr(typval_T *argvars, typval_T *rettv);
void f_remote_foreground(typval_T *argvars, typval_T *rettv);
void f_remote_peek(typval_T *argvars, typval_T *rettv);
void f_remote_read(typval_T *argvars, typval_T *rettv);
void f_remote_send(typval_T *argvars, typval_T *rettv);
void f_remote_startserver(typval_T *argvars, typval_T *rettv);
void f_server2client(typval_T *argvars, typval_T *rettv);
void f_serverlist(typval_T *argvars, typval_T *rettv);
/* vim: set ft=c : */
