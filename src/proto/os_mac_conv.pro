/* os_mac_conv.c */
char_u *mac_string_convert(char_u *ptr, int len, int *lenp, int fail_on_error, int from_enc, int to_enc, int *unconvlenp);
int macroman2enc(char_u *ptr, long *sizep, long real_size);
int enc2macroman(char_u *from, size_t fromlen, char_u *to, int *tolenp, int maxtolen, char_u *rest, int *restlenp);
void mac_conv_init(void);
void mac_conv_cleanup(void);
char_u *mac_utf16_to_enc(unsigned short *from, size_t fromLen, size_t *actualLen);
unsigned short *mac_enc_to_utf16(char_u *from, size_t fromLen, size_t *actualLen);
void *mac_enc_to_cfstring(char_u *from, size_t fromLen);
char_u *mac_precompose_path(char_u *decompPath, size_t decompLen, size_t *precompLen);
void mac_lang_init(void);
/* vim: set ft=c : */
