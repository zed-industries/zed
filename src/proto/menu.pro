/* menu.c */
int winbar_height(win_T *wp);
void ex_menu(exarg_T *eap);
void remove_winbar(win_T *wp);
char_u *set_context_in_menu_cmd(expand_T *xp, char_u *cmd, char_u *arg, int forceit);
char_u *get_menu_name(expand_T *xp, int idx);
char_u *get_menu_names(expand_T *xp, int idx);
int get_menu_index(vimmenu_T *menu, int state);
int menu_is_menubar(char_u *name);
int menu_is_popup(char_u *name);
int menu_is_child_of_popup(vimmenu_T *menu);
int menu_is_toolbar(char_u *name);
int menu_is_separator(char_u *name);
int get_menu_mode_flag(void);
void show_popupmenu(void);
int check_menu_pointer(vimmenu_T *root, vimmenu_T *menu_to_check);
void gui_create_initial_menus(vimmenu_T *menu);
void gui_update_menus(int modes);
int gui_is_menu_shortcut(int key);
void gui_mch_toggle_tearoffs(int enable);
void execute_menu(exarg_T *eap, vimmenu_T *menu, int mode_idx);
void ex_emenu(exarg_T *eap);
void winbar_click(win_T *wp, int col);
vimmenu_T *gui_find_menu(char_u *path_name);
void ex_menutranslate(exarg_T *eap);
void f_menu_info(typval_T *argvars, typval_T *rettv);
/* vim: set ft=c : */
