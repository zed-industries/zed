/* crypt_zip.c */
int crypt_zip_init(cryptstate_T *state, char_u *key, crypt_arg_T *arg);
void crypt_zip_encode(cryptstate_T *state, char_u *from, size_t len, char_u *to, int last);
void crypt_zip_decode(cryptstate_T *state, char_u *from, size_t len, char_u *to, int last);
/* vim: set ft=c : */
