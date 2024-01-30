/* gui_beval.c */
BalloonEval *gui_mch_create_beval_area(void *target, char_u *mesg, void (*mesgCB)(BalloonEval *, int), void *clientData);
void gui_mch_destroy_beval_area(BalloonEval *beval);
void gui_mch_enable_beval_area(BalloonEval *beval);
void gui_mch_disable_beval_area(BalloonEval *beval);
BalloonEval *gui_mch_currently_showing_beval(void);
void gui_mch_post_balloon(BalloonEval *beval, char_u *mesg);
void gui_mch_unpost_balloon(BalloonEval *beval);
/* vim: set ft=c : */
