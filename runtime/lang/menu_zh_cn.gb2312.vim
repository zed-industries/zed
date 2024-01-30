" Menu Translations:    Simplified Chinese
" Maintainer:           Shun Bai <baishunde@gmail.com>
" Previous Maintainer:  Yuheng Xie <elephant@linux.net.cn>
" Last Change:          2019-09-09

" This causes trouble for a broken iconv (symptom: last character is always
" ??).  Without this it works fine anyway, because gbk/cp936 is a superset of
" gb2312. (suggested by Edward L. Fox)
" scriptencoding gb2312

" As mentioned above, gbk/cp936 is a superset of (and backward compatible with)
" gb2312, then source the translation encoded in cp936 should be ok. -- Shun
source <sfile>:p:h/menu_zh_cn.cp936.vim
