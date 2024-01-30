/* spellsuggest.c */
int spell_check_sps(void);
void spell_suggest(int count);
void spell_suggest_list(garray_T *gap, char_u *word, int maxcount, int need_cap, int interactive);
/* vim: set ft=c : */
