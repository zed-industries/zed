/* ex_cmds2.c */
int autowrite(buf_T *buf, int forceit);
void autowrite_all(void);
int check_changed(buf_T *buf, int flags);
void browse_save_fname(buf_T *buf);
void dialog_changed(buf_T *buf, int checkall);
int can_abandon(buf_T *buf, int forceit);
int check_changed_any(int hidden, int unload);
int check_fname(void);
int buf_write_all(buf_T *buf, int forceit);
void ex_listdo(exarg_T *eap);
void ex_compiler(exarg_T *eap);
void init_pyxversion(void);
void ex_pyxfile(exarg_T *eap);
void ex_pyx(exarg_T *eap);
void ex_pyxdo(exarg_T *eap);
void ex_checktime(exarg_T *eap);
/* vim: set ft=c : */
