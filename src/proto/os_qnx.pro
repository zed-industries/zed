/* os_qnx.c */
void qnx_init(void);
void qnx_clip_init (void);
int clip_mch_own_selection(Clipboard_T *cbd);
void clip_mch_lose_selection(Clipboard_T *cbd);
void clip_mch_request_selection(Clipboard_T *cbd);
void clip_mch_set_selection(Clipboard_T *cbd);
/* vim: set ft=c : */
