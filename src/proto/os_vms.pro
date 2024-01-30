/* os_vms.c */
void mch_settmode(tmode_T tmode);
int mch_get_shellsize(void);
void mch_set_shellsize(void);
char_u *mch_getenv(char_u *lognam);
int mch_setenv(char *var, char *value, int x);
int vms_sys(char *cmd, char *out, char *inp);
char *vms_tolower(char *name);
int vms_sys_status(int status);
int vms_read(char *inbuf, size_t nbytes);
int mch_expand_wildcards(int num_pat, char_u **pat, int *num_file, char_u ***file, int flags);
int mch_expandpath(garray_T *gap, char_u *path, int flags);
void *vms_fixfilename(void *instring);
void vms_remove_version(void *fname);
int RealWaitForChar(int fd, long msec, int *check_for_gpm, int *interrupted);
/* vim: set ft=c : */
