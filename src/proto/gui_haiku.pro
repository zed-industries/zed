/* gui_haiku.cc - hand crafted */

void gui_mch_prepare(int *argc, char **argv);
int gui_mch_init(void);
int gui_mch_open(void);
void gui_mch_exit(int vim_exitcode);
int gui_mch_init_check(void);
void gui_mch_flush(void);
int gui_mch_is_blink_off(void);
void gui_mch_new_colors(void);
void gui_mch_set_bg_color(guicolor_T color);
void gui_mch_set_fg_color(guicolor_T color);
void gui_mch_set_sp_color(guicolor_T color);
guicolor_T gui_mch_get_rgb(guicolor_T pixel);
guicolor_T gui_mch_get_rgb_color(int r, int g, int b);
guicolor_T gui_mch_get_color(char_u *name);

GuiFont gui_mch_get_font(char_u *name, int giveErrorIfMissing);
void gui_mch_set_font(GuiFont font);
int gui_mch_init_font(char_u *font_name, int fontset);
void gui_mch_free_font(GuiFont font);
char_u *gui_mch_get_fontname(GuiFont font, char_u *name);

void gui_mch_set_winpos(int x, int y);
int gui_mch_get_winpos(int *x, int *y);
void gui_mch_set_shellsize(int w, int h, int m_w, int m_h, int b_w, int b_h, int d);
void gui_mch_get_screen_dimensions(int* screen_w, int* screen_h);
void gui_mch_set_text_area_pos(int x, int y, int w, int h);

void gui_mch_enable_scrollbar(scrollbar_T *sb, int flag);

//void gui_mch_set_scrollbar_thumb __ARGS((scrollbar_T *sb,int val, int size, int max));
void gui_mch_set_scrollbar_thumb(scrollbar_T *sb, int val, int size, int max);

void gui_mch_set_scrollbar_pos(scrollbar_T *sb, int x, int y, int w, int h);
int gui_mch_get_scrollbar_xpadding(void);
int gui_mch_get_scrollbar_ypadding(void);
void gui_mch_create_scrollbar(scrollbar_T *sb, int orient);
void gui_mch_destroy_scrollbar(scrollbar_T *sb);

void gui_mch_set_blinking(long waittime, long on, long off);
void gui_mch_stop_blink(int may_call_gui_update_cursor);
void gui_mch_start_blink(void);

int gui_mch_adjust_charheight(void);
void gui_mch_draw_string(int row, int col, char_u *s, int len, int flags);
int gui_mch_haskey(char_u *name);
void gui_mch_beep(void);
void gui_mch_flash(int msec);
void gui_mch_invert_rectangle(int r, int c, int nr, int nc);
void gui_mch_iconify(void);
void gui_mch_set_foreground(void);
void gui_mch_settitle(char_u *title, char_u *icon);
void gui_mch_draw_hollow_cursor(guicolor_T color);
void gui_mch_draw_part_cursor(int w, int h, guicolor_T color);
void gui_mch_update(void);
int gui_mch_wait_for_chars(int wtime);
void gui_mch_clear_block(int row1, int col1, int row2, int col2);
void gui_mch_clear_all(void);
void gui_mch_delete_lines(int row, int num_lines);
void gui_mch_insert_lines(int row, int num_lines);

void gui_mch_getmouse(int *x, int *y);
void gui_mch_setmouse(int x, int y);
void gui_mch_mousehide(int hide);

void gui_mch_enable_menu(int flag);
void gui_mch_set_menu_pos(int x, int y, int w, int h);
void gui_mch_add_menu(vimmenu_T *menu, int idx);
void gui_mch_add_menu_item(vimmenu_T *menu, int idx);
void gui_mch_destroy_menu(vimmenu_T *menu);
void gui_mch_menu_grey(vimmenu_T *menu, int grey);
void gui_mch_menu_hidden(vimmenu_T *menu, int hidden);
void gui_mch_draw_menubar(void);
void gui_mch_show_popupmenu(vimmenu_T *menu);
void gui_mch_toggle_tearoffs(int enable);

void clip_mch_request_selection(Clipboard_T *cbd);
void clip_mch_set_selection(Clipboard_T *cbd);
void clip_mch_lose_selection(Clipboard_T *cbd);
int clip_mch_own_selection(Clipboard_T *cbd);

char_u *gui_mch_browse(int saving, char_u *title, char_u *dflt, char_u *ext, char_u *initdir, char_u *filter);
int gui_mch_dialog(int type, char_u *title, char_u *message, char_u *buttons, int dfltbutton, char_u *textfield, int ex_cmd);

void im_set_position(int row, int col);
void im_set_active(int activate);
int im_get_status(void);

void gui_mch_show_toolbar(int showit);
void gui_mch_set_toolbar_pos(int x, int y, int w, int h);

void gui_mch_show_tabline(int showit);
void gui_mch_set_tabline_pos(int x, int y, int w, int h);
int gui_mch_showing_tabline(void);
void gui_mch_update_tabline(void);
void gui_mch_set_curtab(int nr);
