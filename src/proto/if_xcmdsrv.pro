/* if_xcmdsrv.c */
int serverRegisterName(Display *dpy, char_u *name);
void serverChangeRegisteredWindow(Display *dpy, Window newwin);
int serverSendToVim(Display *dpy, char_u *name, char_u *cmd, char_u **result, Window *server, int asExpr, int timeout, int localLoop, int silent);
char_u *serverGetVimNames(Display *dpy);
Window serverStrToWin(char_u *str);
int serverSendReply(char_u *name, char_u *str);
int serverReadReply(Display *dpy, Window win, char_u **str, int localLoop, int timeout);
int serverPeekReply(Display *dpy, Window win, char_u **str);
void serverEventProc(Display *dpy, XEvent *eventPtr, int immediate);
void server_parse_messages(void);
int server_waiting(void);
/* vim: set ft=c : */
