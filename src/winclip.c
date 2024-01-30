/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * winclip.c
 *
 * Routines for Win32 clipboard handling.
 * Also used by Cygwin, using os_unix.c.
 */

#include "vim.h"

/*
 * Compile only the clipboard handling features when compiling for cygwin
 * posix environment.
 */
#ifdef FEAT_CYGWIN_WIN32_CLIPBOARD
# define WIN32_LEAN_AND_MEAN
# include <windows.h>
# include "winclip.pro"
#endif

/*
 * When generating prototypes for Win32 on Unix, these lines make the syntax
 * errors disappear.  They do not need to be correct.
 */
#ifdef PROTO
#define WINAPI
#define WINBASEAPI
typedef int DWORD;
typedef int LPBOOL;
typedef int LPCSTR;
typedef int LPCWSTR;
typedef int LPSTR;
typedef int LPWSTR;
typedef int UINT;
#endif

/*
 * Convert an UTF-8 string to UTF-16.
 * "instr[inlen]" is the input.  "inlen" is in bytes.
 * When "outstr" is NULL only return the number of UTF-16 words produced.
 * Otherwise "outstr" must be a buffer of sufficient size.
 * Returns the number of UTF-16 words produced.
 */
    int
utf8_to_utf16(char_u *instr, int inlen, short_u *outstr, int *unconvlenp)
{
    int		outlen = 0;
    char_u	*p = instr;
    int		todo = inlen;
    int		l;
    int		ch;

    while (todo > 0)
    {
	// Only convert if we have a complete sequence.
	l = utf_ptr2len_len(p, todo);
	if (l > todo)
	{
	    // Return length of incomplete sequence.
	    if (unconvlenp != NULL)
		*unconvlenp = todo;
	    break;
	}

	ch = utf_ptr2char(p);
	if (ch >= 0x10000)
	{
	    // non-BMP character, encoding with surrogate pairs
	    ++outlen;
	    if (outstr != NULL)
	    {
		*outstr++ = (0xD800 - (0x10000 >> 10)) + (ch >> 10);
		*outstr++ = 0xDC00 | (ch & 0x3FF);
	    }
	}
	else if (outstr != NULL)
	    *outstr++ = ch;
	++outlen;
	p += l;
	todo -= l;
    }

    return outlen;
}

/*
 * Convert an UTF-16 string to UTF-8.
 * The input is "instr[inlen]" with "inlen" in number of UTF-16 words.
 * When "outstr" is NULL only return the required number of bytes.
 * Otherwise "outstr" must be a buffer of sufficient size.
 * Return the number of bytes produced.
 */
    int
utf16_to_utf8(short_u *instr, int inlen, char_u *outstr)
{
    int		outlen = 0;
    int		todo = inlen;
    short_u	*p = instr;
    int		l;
    int		ch, ch2;

    while (todo > 0)
    {
	ch = *p;
	if (ch >= 0xD800 && ch <= 0xDBFF && todo > 1)
	{
	    // surrogate pairs handling
	    ch2 = p[1];
	    if (ch2 >= 0xDC00 && ch2 <= 0xDFFF)
	    {
		ch = ((ch - 0xD800) << 10) + (ch2 & 0x3FF) + 0x10000;
		++p;
		--todo;
	    }
	}
	if (outstr != NULL)
	{
	    l = utf_char2bytes(ch, outstr);
	    outstr += l;
	}
	else
	    l = utf_char2len(ch);
	++p;
	outlen += l;
	--todo;
    }

    return outlen;
}

/*
 * Call MultiByteToWideChar() and allocate memory for the result.
 * Returns the result in "*out[*outlen]" with an extra zero appended.
 * "outlen" is in words.
 */
    void
MultiByteToWideChar_alloc(UINT cp, DWORD flags,
	LPCSTR in, int inlen,
	LPWSTR *out, int *outlen)
{
    *outlen = MultiByteToWideChar(cp, flags, in, inlen, 0, 0);
    // Add one word to avoid a zero-length alloc().
    *out = ALLOC_MULT(WCHAR, *outlen + 1);
    if (*out == NULL)
	return;

    MultiByteToWideChar(cp, flags, in, inlen, *out, *outlen);
    (*out)[*outlen] = 0;
}

/*
 * Call WideCharToMultiByte() and allocate memory for the result.
 * Returns the result in "*out[*outlen]" with an extra NUL appended.
 */
    void
WideCharToMultiByte_alloc(UINT cp, DWORD flags,
	LPCWSTR in, int inlen,
	LPSTR *out, int *outlen,
	LPCSTR def, LPBOOL useddef)
{
    *outlen = WideCharToMultiByte(cp, flags, in, inlen, NULL, 0, def, useddef);
    // Add one byte to avoid a zero-length alloc().
    *out = alloc(*outlen + 1);
    if (*out == NULL)
	return;
    WideCharToMultiByte(cp, flags, in, inlen, *out, *outlen, def, useddef);
    (*out)[*outlen] = 0;
}


#ifdef FEAT_CLIPBOARD
/*
 * Clipboard stuff, for cutting and pasting text to other windows.
 */

    void
win_clip_init(void)
{
    clip_init(TRUE);

    /*
     * Vim's own clipboard format recognises whether the text is char, line,
     * or rectangular block.  Only useful for copying between two Vims.
     * "Clipboard_T" was used for previous versions, using the first
     * character to specify MCHAR, MLINE or MBLOCK.
     */
    clip_star.format = RegisterClipboardFormat("VimClipboard2");
    clip_star.format_raw = RegisterClipboardFormat("VimRawBytes");
}

// Type used for the clipboard type of Vim's data.
typedef struct
{
    int type;		// MCHAR, MBLOCK or MLINE
    int txtlen;		// length of CF_TEXT in bytes
    int ucslen;		// length of CF_UNICODETEXT in words
    int rawlen;		// length of clip_star.format_raw, including encoding,
			// excluding terminating NUL
} VimClipType_t;

/*
 * Make vim the owner of the current selection.  Return OK upon success.
 */
    int
clip_mch_own_selection(Clipboard_T *cbd UNUSED)
{
    /*
     * Never actually own the clipboard.  If another application sets the
     * clipboard, we don't want to think that we still own it.
     */
    return FAIL;
}

/*
 * Make vim NOT the owner of the current selection.
 */
    void
clip_mch_lose_selection(Clipboard_T *cbd UNUSED)
{
    // Nothing needs to be done here
}

/*
 * Copy "str[*size]" into allocated memory, changing CR-NL to NL.
 * Return the allocated result and the size in "*size".
 * Returns NULL when out of memory.
 */
    static char_u *
crnl_to_nl(const char_u *str, int *size)
{
    int		pos = 0;
    int		str_len = *size;
    char_u	*ret;
    char_u	*retp;

    // Avoid allocating zero bytes, it generates an error message.
    ret = alloc(str_len == 0 ? 1 : str_len);
    if (ret != NULL)
    {
	retp = ret;
	for (pos = 0; pos < str_len; ++pos)
	{
	    if (str[pos] == '\r' && str[pos + 1] == '\n')
	    {
		++pos;
		--(*size);
	    }
	    *retp++ = str[pos];
	}
    }

    return ret;
}

/*
 * Wait for another process to Close the Clipboard.
 * Returns TRUE for success.
 */
    static int
vim_open_clipboard(void)
{
    int delay = 10;

    while (!OpenClipboard(NULL))
    {
	if (delay > 500)
	    return FALSE;  // waited too long, give up
	Sleep(delay);
	delay *= 2;	// wait for 10, 20, 40, 80, etc. msec
    }
    return TRUE;
}

/*
 * Get the current selection and put it in the clipboard register.
 *
 * NOTE: Must use GlobalLock/Unlock here to ensure Win32s compatibility.
 * On NT/W95 the clipboard data is a fixed global memory object and
 * so its handle = its pointer.
 * On Win32s, however, co-operation with the Win16 system means that
 * the clipboard data is moveable and its handle is not a pointer at all,
 * so we can't just cast the return value of GetClipboardData to (char_u*).
 * <VN>
 */
    void
clip_mch_request_selection(Clipboard_T *cbd)
{
    VimClipType_t	metadata = { -1, -1, -1, -1 };
    HGLOBAL		hMem = NULL;
    char_u		*str = NULL;
    char_u		*to_free = NULL;
    HGLOBAL		rawh = NULL;
    int			str_size = 0;
    int			maxlen;
    size_t		n;

    /*
     * Don't pass GetActiveWindow() as an argument to OpenClipboard() because
     * then we can't paste back into the same window for some reason - webb.
     */
    if (!vim_open_clipboard())
	return;

    // Check for vim's own clipboard format first.  This only gets the type of
    // the data, still need to use CF_UNICODETEXT or CF_TEXT for the text.
    if (IsClipboardFormatAvailable(cbd->format))
    {
	VimClipType_t	*meta_p;
	HGLOBAL		meta_h;

	// We have metadata on the clipboard; try to get it.
	if ((meta_h = GetClipboardData(cbd->format)) != NULL
		&& (meta_p = (VimClipType_t *)GlobalLock(meta_h)) != NULL)
	{
	    // The size of "VimClipType_t" changed, "rawlen" was added later.
	    // Only copy what is available for backwards compatibility.
	    n = sizeof(VimClipType_t);
	    if (GlobalSize(meta_h) < n)
		n = GlobalSize(meta_h);
	    memcpy(&metadata, meta_p, n);
	    GlobalUnlock(meta_h);
	}
    }

    // Check for Vim's raw clipboard format first.  This is used without
    // conversion, but only if 'encoding' matches.
    if (IsClipboardFormatAvailable(cbd->format_raw)
				      && metadata.rawlen > (int)STRLEN(p_enc))
    {
	// We have raw data on the clipboard; try to get it.
	if ((rawh = GetClipboardData(cbd->format_raw)) != NULL)
	{
	    char_u	*rawp;

	    rawp = (char_u *)GlobalLock(rawh);
	    if (rawp != NULL && STRCMP(p_enc, rawp) == 0)
	    {
		n = STRLEN(p_enc) + 1;
		str = rawp + n;
		str_size = (int)(metadata.rawlen - n);
	    }
	    else
	    {
		GlobalUnlock(rawh);
		rawh = NULL;
	    }
	}
    }
    if (str == NULL)
    {
	// Try to get the clipboard in Unicode if it's not an empty string.
	if (IsClipboardFormatAvailable(CF_UNICODETEXT) && metadata.ucslen != 0)
	{
	    HGLOBAL hMemW;

	    if ((hMemW = GetClipboardData(CF_UNICODETEXT)) != NULL)
	    {
		WCHAR *hMemWstr = (WCHAR *)GlobalLock(hMemW);

		// Use the length of our metadata if possible, but limit it to
		// the GlobalSize() for safety.
		maxlen = (int)(GlobalSize(hMemW) / sizeof(WCHAR));
		if (metadata.ucslen >= 0)
		{
		    if (metadata.ucslen > maxlen)
			str_size = maxlen;
		    else
			str_size = metadata.ucslen;
		}
		else
		{
		    for (str_size = 0; str_size < maxlen; ++str_size)
			if (hMemWstr[str_size] == NUL)
			    break;
		}
		to_free = str = utf16_to_enc((short_u *)hMemWstr, &str_size);
		GlobalUnlock(hMemW);
	    }
	}
	// Get the clipboard in the Active codepage.
	else if (IsClipboardFormatAvailable(CF_TEXT))
	{
	    if ((hMem = GetClipboardData(CF_TEXT)) != NULL)
	    {
		str = (char_u *)GlobalLock(hMem);

		// The length is either what our metadata says or the strlen().
		// But limit it to the GlobalSize() for safety.
		maxlen = (int)GlobalSize(hMem);
		if (metadata.txtlen >= 0)
		{
		    if (metadata.txtlen > maxlen)
			str_size = maxlen;
		    else
			str_size = metadata.txtlen;
		}
		else
		{
		    for (str_size = 0; str_size < maxlen; ++str_size)
			if (str[str_size] == NUL)
			    break;
		}

		// The text is in the active codepage.  Convert to
		// 'encoding', going through UTF-16.
		acp_to_enc(str, str_size, &to_free, &maxlen);
		if (to_free != NULL)
		{
		    str_size = maxlen;
		    str = to_free;
		}
	    }
	}
    }

    if (str != NULL && metadata.txtlen != 0)
    {
	char_u *temp_clipboard;

	// If the type is not known detect it.
	if (metadata.type == -1)
	    metadata.type = MAUTO;

	// Translate <CR><NL> into <NL>.
	temp_clipboard = crnl_to_nl(str, &str_size);
	if (temp_clipboard != NULL)
	{
	    clip_yank_selection(metadata.type, temp_clipboard, str_size, cbd);
	    vim_free(temp_clipboard);
	}
    }

    // unlock the global object
    if (hMem != NULL)
	GlobalUnlock(hMem);
    if (rawh != NULL)
	GlobalUnlock(rawh);
    CloseClipboard();
    vim_free(to_free);
}

/*
 * Send the current selection to the clipboard.
 */
    void
clip_mch_set_selection(Clipboard_T *cbd)
{
    char_u		*str = NULL;
    VimClipType_t	metadata;
    long_u		txtlen;
    HGLOBAL		hMemRaw = NULL;
    HGLOBAL		hMem = NULL;
    HGLOBAL		hMemVim = NULL;
    HGLOBAL		hMemW = NULL;

    // If the '*' register isn't already filled in, fill it in now
    cbd->owned = TRUE;
    clip_get_selection(cbd);
    cbd->owned = FALSE;

    // Get the text to be put on the clipboard, with CR-LF.
    metadata.type = clip_convert_selection(&str, &txtlen, cbd);
    if (metadata.type < 0)
	return;
    metadata.txtlen = (int)txtlen;
    metadata.ucslen = 0;
    metadata.rawlen = 0;

    // Always set the raw bytes: 'encoding', NUL and the text.  This is used
    // when copy/paste from/to Vim with the same 'encoding', so that illegal
    // bytes can also be copied and no conversion is needed.
    {
	LPSTR lpszMemRaw;

	metadata.rawlen = (int)(txtlen + STRLEN(p_enc) + 1);
	hMemRaw = (LPSTR)GlobalAlloc(GMEM_MOVEABLE | GMEM_DDESHARE,
							 metadata.rawlen + 1);
	lpszMemRaw = (LPSTR)GlobalLock(hMemRaw);
	if (lpszMemRaw != NULL)
	{
	    STRCPY(lpszMemRaw, p_enc);
	    memcpy(lpszMemRaw + STRLEN(p_enc) + 1, str, txtlen + 1);
	    GlobalUnlock(hMemRaw);
	}
	else
	    metadata.rawlen = 0;
    }

    {
	WCHAR		*out;
	int		len = metadata.txtlen;

	// Convert the text to UTF-16. This is put on the clipboard as
	// CF_UNICODETEXT.
	out = (WCHAR *)enc_to_utf16(str, &len);
	if (out != NULL)
	{
	    WCHAR *lpszMemW;

	    // Convert the text for CF_TEXT to Active codepage. Otherwise it's
	    // p_enc, which has no relation to the Active codepage.
	    metadata.txtlen = WideCharToMultiByte(GetACP(), 0, out, len,
							       NULL, 0, 0, 0);
	    vim_free(str);
	    str = alloc(metadata.txtlen == 0 ? 1 : metadata.txtlen);
	    if (str == NULL)
	    {
		vim_free(out);
		return;		// out of memory
	    }
	    WideCharToMultiByte(GetACP(), 0, out, len,
					   (LPSTR)str, metadata.txtlen, 0, 0);

	    // Allocate memory for the UTF-16 text, add one NUL word to
	    // terminate the string.
	    hMemW = (LPSTR)GlobalAlloc(GMEM_MOVEABLE | GMEM_DDESHARE,
						   (len + 1) * sizeof(WCHAR));
	    lpszMemW = (WCHAR *)GlobalLock(hMemW);
	    if (lpszMemW != NULL)
	    {
		memcpy(lpszMemW, out, len * sizeof(WCHAR));
		lpszMemW[len] = NUL;
		GlobalUnlock(hMemW);
	    }
	    vim_free(out);
	    metadata.ucslen = len;
	}
    }

    // Allocate memory for the text, add one NUL byte to terminate the string.
    hMem = GlobalAlloc(GMEM_MOVEABLE | GMEM_DDESHARE, metadata.txtlen + 1);
    {
	LPSTR lpszMem = (LPSTR)GlobalLock(hMem);

	if (lpszMem)
	{
	    mch_memmove((char_u *)lpszMem, str, metadata.txtlen);
	    GlobalUnlock(hMem);
	}
    }

    // Set up metadata:
    {
	VimClipType_t *lpszMemVim = NULL;

	hMemVim = GlobalAlloc(GMEM_MOVEABLE|GMEM_DDESHARE,
						       sizeof(VimClipType_t));
	lpszMemVim = (VimClipType_t *)GlobalLock(hMemVim);
	memcpy(lpszMemVim, &metadata, sizeof(metadata));
	GlobalUnlock(hMemVim);
    }

    /*
     * Open the clipboard, clear it and put our text on it.
     * Always set our Vim format.  Put Unicode and plain text on it.
     *
     * Don't pass GetActiveWindow() as an argument to OpenClipboard()
     * because then we can't paste back into the same window for some
     * reason - webb.
     */
    if (vim_open_clipboard())
    {
	if (EmptyClipboard())
	{
	    SetClipboardData(cbd->format, hMemVim);
	    hMemVim = 0;
	    if (hMemW != NULL)
	    {
		if (SetClipboardData(CF_UNICODETEXT, hMemW) != NULL)
		    hMemW = NULL;
	    }
	    // Always use CF_TEXT.  On Win98 Notepad won't obtain the
	    // CF_UNICODETEXT text, only CF_TEXT.
	    SetClipboardData(CF_TEXT, hMem);
	    hMem = 0;
	}
	CloseClipboard();
    }

    vim_free(str);
    // Free any allocations we didn't give to the clipboard:
    if (hMemRaw)
	GlobalFree(hMemRaw);
    if (hMem)
	GlobalFree(hMem);
    if (hMemW)
	GlobalFree(hMemW);
    if (hMemVim)
	GlobalFree(hMemVim);
}

#endif // FEAT_CLIPBOARD

/*
 * Note: the following two functions are only guaranteed to work when using
 * valid MS-Windows codepages or when iconv() is available.
 */

/*
 * Convert "str" from 'encoding' to UTF-16.
 * Input in "str" with length "*lenp".  When "lenp" is NULL, use strlen().
 * Output is returned as an allocated string.  "*lenp" is set to the length of
 * the result.  A trailing NUL is always added.
 * Returns NULL when out of memory.
 */
    short_u *
enc_to_utf16(char_u *str, int *lenp)
{
    vimconv_T	conv;
    WCHAR	*ret;
    char_u	*allocbuf = NULL;
    int		len_loc;
    int		length;

    if (lenp == NULL)
    {
	len_loc = (int)STRLEN(str) + 1;
	lenp = &len_loc;
    }

    if (enc_codepage > 0)
    {
	// We can do any CP### -> UTF-16 in one pass, and we can do it
	// without iconv() (convert_* may need iconv).
	MultiByteToWideChar_alloc(enc_codepage, 0, (LPCSTR)str, *lenp,
							       &ret, &length);
    }
    else
    {
	// Use "latin1" by default, we might be called before we have p_enc
	// set up.  Convert to utf-8 first, works better with iconv().  Does
	// nothing if 'encoding' is "utf-8".
	conv.vc_type = CONV_NONE;
	if (convert_setup(&conv, p_enc ? p_enc : (char_u *)"latin1",
						   (char_u *)"utf-8") == FAIL)
	    return NULL;
	if (conv.vc_type != CONV_NONE)
	{
	    str = allocbuf = string_convert(&conv, str, lenp);
	    if (str == NULL)
		return NULL;
	}
	convert_setup(&conv, NULL, NULL);

	length = utf8_to_utf16(str, *lenp, NULL, NULL);
	ret = ALLOC_MULT(WCHAR, length + 1);
	if (ret != NULL)
	{
	    utf8_to_utf16(str, *lenp, (short_u *)ret, NULL);
	    ret[length] = 0;
	}

	vim_free(allocbuf);
    }

    *lenp = length;
    return (short_u *)ret;
}

/*
 * Convert an UTF-16 string to 'encoding'.
 * Input in "str" with length (counted in wide characters) "*lenp".  When
 * "lenp" is NULL, use wcslen().
 * Output is returned as an allocated string.  If "*lenp" is not NULL it is
 * set to the length of the result.
 * Returns NULL when out of memory.
 */
    char_u *
utf16_to_enc(short_u *str, int *lenp)
{
    vimconv_T	conv;
    char_u	*utf8_str = NULL, *enc_str = NULL;
    int		len_loc;

    if (lenp == NULL)
    {
	len_loc = (int)wcslen(str) + 1;
	lenp = &len_loc;
    }

    if (enc_codepage > 0)
    {
	// We can do any UTF-16 -> CP### in one pass.
	int length;

	WideCharToMultiByte_alloc(enc_codepage, 0, str, *lenp,
					    (LPSTR *)&enc_str, &length, 0, 0);
	*lenp = length;
	return enc_str;
    }

    // Avoid allocating zero bytes, it generates an error message.
    utf8_str = alloc(utf16_to_utf8(str, *lenp == 0 ? 1 : *lenp, NULL));
    if (utf8_str != NULL)
    {
	*lenp = utf16_to_utf8(str, *lenp, utf8_str);

	// We might be called before we have p_enc set up.
	conv.vc_type = CONV_NONE;
	convert_setup(&conv, (char_u *)"utf-8",
					    p_enc? p_enc: (char_u *)"latin1");
	if (conv.vc_type == CONV_NONE)
	{
	    // p_enc is utf-8, so we're done.
	    enc_str = utf8_str;
	}
	else
	{
	    enc_str = string_convert(&conv, utf8_str, lenp);
	    vim_free(utf8_str);
	}

	convert_setup(&conv, NULL, NULL);
    }

    return enc_str;
}

/*
 * Convert from the active codepage to 'encoding'.
 * Input is "str[str_size]".
 * The result is in allocated memory: "out[outlen]".  "outlen" includes the
 * terminating NUL.
 */
    void
acp_to_enc(
    char_u	*str,
    int		str_size,
    char_u	**out,
    int		*outlen)

{
    LPWSTR	widestr;

    MultiByteToWideChar_alloc(GetACP(), 0, (LPCSTR)str, str_size,
							    &widestr, outlen);
    if (widestr == NULL)
	return;
    ++*outlen;	// Include the 0 after the string
    *out = utf16_to_enc((short_u *)widestr, outlen);
    vim_free(widestr);
}

/*
 * Convert from 'encoding' to the active codepage.
 * Input is "str[str_size]".
 * The result is in allocated memory: "out[outlen]".  With terminating NUL.
 */
    void
enc_to_acp(
    char_u	*str,
    int		str_size,
    char_u	**out,
    int		*outlen)

{
    LPWSTR	widestr;
    int		len = str_size;

    widestr = (WCHAR *)enc_to_utf16(str, &len);
    if (widestr == NULL)
	return;
    WideCharToMultiByte_alloc(GetACP(), 0, widestr, len,
	    (LPSTR *)out, outlen, 0, 0);
    vim_free(widestr);
}
