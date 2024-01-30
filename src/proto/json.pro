/* json.c */
char_u *json_encode(typval_T *val, int options);
char_u *json_encode_nr_expr(int nr, typval_T *val, int options);
char_u *json_encode_lsp_msg(typval_T *val);
int json_decode(js_read_T *reader, typval_T *res, int options);
int json_find_end(js_read_T *reader, int options);
void f_js_decode(typval_T *argvars, typval_T *rettv);
void f_js_encode(typval_T *argvars, typval_T *rettv);
void f_json_decode(typval_T *argvars, typval_T *rettv);
void f_json_encode(typval_T *argvars, typval_T *rettv);
/* vim: set ft=c : */
