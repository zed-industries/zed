/* pty.c */
int setup_slavepty(int fd);
int mch_openpty(char **ttyn);
int mch_isatty(int fd);
/* vim: set ft=c : */
