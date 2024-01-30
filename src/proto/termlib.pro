/* termlib.c */
int tgetent(char *tbuf, char *term);
int tgetflag(char *id);
int tgetnum(char *id);
char *tgetstr(char *id, char **buf);
char *tgoto(char *cm, int col, int line);
int tputs(char *cp, int affcnt, void (*outc)(unsigned int));
/* vim: set ft=c : */
