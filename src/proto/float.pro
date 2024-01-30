/* float.c */
int string2float(char_u *text, float_T *value, int skip_quotes);
void f_abs(typval_T *argvars, typval_T *rettv);
void f_acos(typval_T *argvars, typval_T *rettv);
void f_asin(typval_T *argvars, typval_T *rettv);
void f_atan(typval_T *argvars, typval_T *rettv);
void f_atan2(typval_T *argvars, typval_T *rettv);
void f_ceil(typval_T *argvars, typval_T *rettv);
void f_cos(typval_T *argvars, typval_T *rettv);
void f_cosh(typval_T *argvars, typval_T *rettv);
void f_exp(typval_T *argvars, typval_T *rettv);
void f_float2nr(typval_T *argvars, typval_T *rettv);
void f_floor(typval_T *argvars, typval_T *rettv);
void f_fmod(typval_T *argvars, typval_T *rettv);
void f_isinf(typval_T *argvars, typval_T *rettv);
void f_isnan(typval_T *argvars, typval_T *rettv);
void f_log(typval_T *argvars, typval_T *rettv);
void f_log10(typval_T *argvars, typval_T *rettv);
void f_pow(typval_T *argvars, typval_T *rettv);
float_T vim_round(float_T f);
void f_round(typval_T *argvars, typval_T *rettv);
void f_sin(typval_T *argvars, typval_T *rettv);
void f_sinh(typval_T *argvars, typval_T *rettv);
void f_sqrt(typval_T *argvars, typval_T *rettv);
void f_str2float(typval_T *argvars, typval_T *rettv);
void f_tan(typval_T *argvars, typval_T *rettv);
void f_tanh(typval_T *argvars, typval_T *rettv);
void f_trunc(typval_T *argvars, typval_T *rettv);
/* vim: set ft=c : */
