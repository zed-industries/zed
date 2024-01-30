/* netbeans.c */
void netbeans_parse_messages(void);
int isNetbeansBuffer(buf_T *bufp);
int isNetbeansModified(buf_T *bufp);
void netbeans_end(void);
void ex_nbclose(exarg_T *eap);
void ex_nbkey(exarg_T *eap);
void ex_nbstart(exarg_T *eap);
void netbeans_beval_cb(BalloonEval *beval, int state);
int netbeans_active(void);
void netbeans_open(char *params, int doabort);
void netbeans_send_disconnect(void);
int set_ref_in_nb_channel(int copyID);
void netbeans_frame_moved(int new_x, int new_y);
void netbeans_file_activated(buf_T *bufp);
void netbeans_file_opened(buf_T *bufp);
void netbeans_file_killed(buf_T *bufp);
void netbeans_inserted(buf_T *bufp, linenr_T linenr, colnr_T col, char_u *txt, int newlen);
void netbeans_removed(buf_T *bufp, linenr_T linenr, colnr_T col, long len);
void netbeans_unmodified(buf_T *bufp);
void netbeans_button_release(int button);
int netbeans_keycommand(int key);
void netbeans_save_buffer(buf_T *bufp);
void netbeans_deleted_all_lines(buf_T *bufp);
int netbeans_is_guarded(linenr_T top, linenr_T bot);
void netbeans_draw_multisign_indicator(int row);
void netbeans_gutter_click(linenr_T lnum);
/* vim: set ft=c : */
