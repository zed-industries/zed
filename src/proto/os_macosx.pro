/* os_macosx.m */
void process_cfrunloop(void);
bool sound_mch_play(const char_u* event, long sound_id, soundcb_T *callback, bool playfile);
void sound_mch_stop(long sound_id);
void sound_mch_clear(void);
void sound_mch_free(void);
/* vim: set ft=c : */
