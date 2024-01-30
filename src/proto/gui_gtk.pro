/* gui_gtk.c */
void gui_gtk_register_stock_icons(void);
void gui_mch_add_menu(vimmenu_T *menu, int idx);
void gui_mch_add_menu_item(vimmenu_T *menu, int idx);
void gui_mch_set_text_area_pos(int x, int y, int w, int h);
void gui_gtk_set_mnemonics(int enable);
void gui_mch_toggle_tearoffs(int enable);
void gui_mch_menu_set_tip(vimmenu_T *menu);
void gui_mch_destroy_menu(vimmenu_T *menu);
void gui_mch_set_scrollbar_thumb(scrollbar_T *sb, long val, long size, long max);
void gui_mch_set_scrollbar_pos(scrollbar_T *sb, int x, int y, int w, int h);
int gui_mch_get_scrollbar_xpadding(void);
int gui_mch_get_scrollbar_ypadding(void);
void gui_mch_create_scrollbar(scrollbar_T *sb, int orient);
void gui_mch_destroy_scrollbar(scrollbar_T *sb);
char_u *gui_mch_browse(int saving, char_u *title, char_u *dflt, char_u *ext, char_u *initdir, char_u *filter);
char_u *gui_mch_browsedir(char_u *title, char_u *initdir);
int gui_mch_dialog(int type, char_u *title, char_u *message, char_u *buttons, int def_but, char_u *textfield, int ex_cmd);
void gui_mch_show_popupmenu(vimmenu_T *menu);
void gui_make_popup(char_u *path_name, int mouse_pos);
void gui_mch_find_dialog(exarg_T *eap);
void gui_mch_replace_dialog(exarg_T *eap);
void ex_helpfind(exarg_T *eap);
/* vim: set ft=c : */
