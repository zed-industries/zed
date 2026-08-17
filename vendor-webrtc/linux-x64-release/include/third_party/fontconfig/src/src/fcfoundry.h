/*
  Copyright © 2002-2003 by Juliusz Chroboczek

  Permission is hereby granted, free of charge, to any person obtaining a copy
  of this software and associated documentation files (the "Software"), to deal
  in the Software without restriction, including without limitation the rights
  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
  copies of the Software, and to permit persons to whom the Software is
  furnished to do so, subject to the following conditions:

  The above copyright notice and this permission notice shall be included in
  all copies or substantial portions of the Software.

  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL THE
  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
  THE SOFTWARE.
*/

/* Order is significant.  For example, some B&H fonts are hinted by
   URW++, and both strings appear in the notice. */

static const char *FcNoticeFoundries[][2] =
    {
     {"Adobe", "adobe"},
     {"Bigelow", "b&h"},
     {"Bitstream", "bitstream"},
     {"Gnat", "culmus"},
     {"Iorsh", "culmus"},
     {"HanYang System", "hanyang"},
     {"Font21", "hwan"},
     {"IBM", "ibm"},
     {"International Typeface Corporation", "itc"},
     {"Linotype", "linotype"},
     {"LINOTYPE-HELL", "linotype"},
     {"Microsoft", "microsoft"},
     {"Monotype", "monotype"},
     {"Omega", "omega"},
     {"Tiro Typeworks", "tiro"},
     {"URW", "urw"},
     {"XFree86", "xfree86"},
     {"Xorg", "xorg"},
};

#define NUM_NOTICE_FOUNDRIES	(int) (sizeof (FcNoticeFoundries) / sizeof (FcNoticeFoundries[0]))
