/* vim9expr.c */
int generate_ppconst(cctx_T *cctx, ppconst_T *ppconst);
void clear_ppconst(ppconst_T *ppconst);
int compile_member(int is_slice, int *keeping_dict, cctx_T *cctx);
int compile_load_scriptvar(cctx_T *cctx, char_u *name, char_u *start, char_u **end);
int compile_load(char_u **arg, char_u *end_arg, cctx_T *cctx, int is_expr, int error);
int compile_arguments(char_u **arg, cctx_T *cctx, int *argcount, ca_special_T special_fn);
char_u *to_name_end(char_u *arg, int use_namespace);
char_u *to_name_const_end(char_u *arg);
int get_lambda_tv_and_compile(char_u **arg, typval_T *rettv, int types_optional, evalarg_T *evalarg);
exprtype_T get_compare_type(char_u *p, int *len, int *type_is);
void skip_expr_cctx(char_u **arg, cctx_T *cctx);
int bool_on_stack(cctx_T *cctx);
void error_white_both(char_u *op, int len);
int compile_expr1(char_u **arg, cctx_T *cctx, ppconst_T *ppconst);
int compile_expr0_ext(char_u **arg, cctx_T *cctx, int *is_const);
int compile_expr0(char_u **arg, cctx_T *cctx);
/* vim: set ft=c : */
