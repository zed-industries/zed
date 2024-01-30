/* spellfile.c */
slang_T *spell_load_file(char_u *fname, char_u *lang, slang_T *old_lp, int silent);
void suggest_load_files(void);
int spell_check_msm(void);
void ex_mkspell(exarg_T *eap);
void mkspell(int fcount, char_u **fnames, int ascii, int over_write, int added_word);
void ex_spell(exarg_T *eap);
void spell_add_word(char_u *word, int len, int what, int idx, int undo);
/* vim: set ft=c : */
