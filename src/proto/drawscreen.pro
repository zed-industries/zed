/* drawscreen.c */
int update_screen(int type_arg);
int statusline_row(win_T *wp);
void win_redr_status(win_T *wp, int ignore_pum);
void showruler(int always);
void win_redr_ruler(win_T *wp, int always, int ignore_pum);
void after_updating_screen(int may_resize_shell);
void update_curbuf(int type);
void update_debug_sign(buf_T *buf, linenr_T lnum);
void updateWindow(win_T *wp);
int redraw_asap(int type);
void redraw_after_callback(int call_update_screen, int do_message);
void redraw_later(int type);
void redraw_win_later(win_T *wp, int type);
void redraw_later_clear(void);
void redraw_all_later(int type);
void set_must_redraw(int type);
void redraw_curbuf_later(int type);
void redraw_buf_later(buf_T *buf, int type);
void redraw_buf_line_later(buf_T *buf, linenr_T lnum);
void redraw_buf_and_status_later(buf_T *buf, int type);
void status_redraw_all(void);
void status_redraw_curbuf(void);
void redraw_statuslines(void);
void win_redraw_last_status(frame_T *frp);
void redrawWinline(win_T *wp, linenr_T lnum);
/* vim: set ft=c : */
