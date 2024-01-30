/* blowfish.c */
void crypt_blowfish_encode(cryptstate_T *state, char_u *from, size_t len, char_u *to, int last);
void crypt_blowfish_decode(cryptstate_T *state, char_u *from, size_t len, char_u *to, int last);
int crypt_blowfish_init(cryptstate_T *state, char_u *key, crypt_arg_T *arg);
int blowfish_self_test(void);
/* vim: set ft=c : */
