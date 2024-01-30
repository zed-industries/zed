/* winclip.c */
int utf8_to_utf16(char_u *instr, int inlen, short_u *outstr, int *unconvlenp);
int utf16_to_utf8(short_u *instr, int inlen, char_u *outstr);
void MultiByteToWideChar_alloc(UINT cp, DWORD flags, LPCSTR in, int inlen, LPWSTR *out, int *outlen);
void WideCharToMultiByte_alloc(UINT cp, DWORD flags, LPCWSTR in, int inlen, LPSTR *out, int *outlen, LPCSTR def, LPBOOL useddef);
void win_clip_init(void);
int clip_mch_own_selection(Clipboard_T *cbd);
void clip_mch_lose_selection(Clipboard_T *cbd);
void clip_mch_request_selection(Clipboard_T *cbd);
void clip_mch_set_selection(Clipboard_T *cbd);
short_u *enc_to_utf16(char_u *str, int *lenp);
char_u *utf16_to_enc(short_u *str, int *lenp);
void acp_to_enc(char_u *str, int str_size, char_u **out, int *outlen);
void enc_to_acp(char_u *str, int str_size, char_u **out, int *outlen);
/* vim: set ft=c : */
