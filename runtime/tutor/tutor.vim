" Vim tutor support file
" Author:	Eduardo F. Amatria <eferna1@platea.pntic.mec.es>
" Maintainer:	The·Vim·Project·<https://github.com/vim/vim>
" Last Change:	2023 Aug 13

" This Vim script is used for detecting if a translation of the
" tutor file exist, i.e., a tutor.xx file, where xx is the language.
" If the translation does not exist, or no extension is given,
" it defaults to the English version.

" It is invoked by the vimtutor shell script.

" 1. Build the extension of the file, if any:
let s:ext = ""
if strlen($xx) > 1
  let s:ext = "." . $xx
else
  let s:lang = ""
  " Check that a potential value has at least two letters.
  " Ignore "1043" and "C".
  if exists("v:lang") && v:lang =~ '\a\a'
    let s:lang = v:lang
  elseif $LC_ALL =~ '\a\a'
    let s:lang = $LC_ALL
  elseif $LC_MESSAGES =~ '\a\a' || $LC_MESSAGES ==# "C"
    " LC_MESSAGES=C can be used to explicitly ask for English messages while
    " keeping LANG non-English; don't set s:lang then.
    if $LC_MESSAGES =~ '\a\a'
      let s:lang = $LC_MESSAGES
    endif
  elseif $LANG =~ '\a\a'
    let s:lang = $LANG
  endif
  if s:lang != ""
    " Remove "@euro" (ignoring case), it may be at the end
    let s:lang = substitute(s:lang, '\c@euro', '', '')
    " On MS-Windows it may be German_Germany.1252 or Polish_Poland.1250.  How
    " about other languages?
    if s:lang =~ "German"
      let s:ext = ".de"
    elseif s:lang =~ "Polish"
      let s:ext = ".pl"
    elseif s:lang =~ "Slovak"
      let s:ext = ".sk"
    elseif s:lang =~ "Serbian"
      let s:ext = ".sr"
    elseif s:lang =~ "Czech"
      let s:ext = ".cs"
    elseif s:lang =~ "Dutch"
      let s:ext = ".nl"
    elseif s:lang =~ "Bulgarian"
      let s:ext = ".bg"
    else
      let s:ext = "." . strpart(s:lang, 0, 2)
    endif
  endif
endif

" Somehow ".ge" (Germany) is sometimes used for ".de" (Deutsch).
if s:ext =~? '\.ge'
  let s:ext = ".de"
endif

if s:ext =~? '\.en'
  let s:ext = ""
endif

" The Japanese tutor is available in three encodings, guess which one to use
" The "sjis" one is actually "cp932", it doesn't matter for this text.
if s:ext =~? '\.ja'
  if &enc =~ "euc"
    let s:ext = ".ja.euc"
  elseif &enc != "utf-8"
    let s:ext = ".ja.sjis"
  endif
endif

" The Korean tutor is available in two encodings, guess which one to use
if s:ext =~? '\.ko'
  if &enc != "utf-8"
    let s:ext = ".ko.euc"
  endif
endif

" The Chinese tutor is available in three encodings, guess which one to use
" This segment is from the above lines and modified by
" Mendel L Chan <beos@turbolinux.com.cn> for Chinese vim tutorial
" When 'encoding' is utf-8, choose between China (simplified) and Taiwan
" (traditional) based on the language, suggested by Alick Zhao.
if s:ext =~? '\.zh'
  if &enc =~ 'big5\|cp950'
    let s:ext = ".zh.big5"
  elseif &enc != 'utf-8'
    let s:ext = ".zh.euc"
  elseif s:ext =~? 'zh_tw' || (exists("s:lang") && s:lang =~? 'zh_tw')
    let s:ext = ".zh_tw"
  else
    let s:ext = ".zh_cn"
  endif
endif

" The Polish tutor is available in two encodings, guess which one to use.
if s:ext =~? '\.pl'
  if &enc =~ 1250
    let s:ext = ".pl.cp1250"
  endif
endif

" The Turkish tutor is available in two encodings, guess which one to use
if s:ext =~? '\.tr'
  if &enc == "iso-8859-9" || &enc == "cp1254"
    let s:ext = ".tr.iso9"
  endif
endif

" The Greek tutor is available in three encodings, guess what to use.
" We used ".gr" (Greece) instead of ".el" (Greek); accept both.
if s:ext =~? '\.gr\|\.el'
  if &enc == "iso-8859-7"
    let s:ext = ".el"
  elseif &enc == "utf-8"
    let s:ext = ".el.utf-8"
  elseif &enc =~ 737
    let s:ext = ".el.cp737"
  endif
endif

" The Slovak tutor is available in three encodings, guess which one to use
if s:ext =~? '\.sk'
  if &enc =~ 1250
    let s:ext = ".sk.cp1250"
  endif
endif

" The Slovak tutor is available in two encodings, guess which one to use
" Note that the utf-8 version is the original, the cp1250 version is created
" from it.
if s:ext =~? '\.sr'
  if &enc =~ 1250
    let s:ext = ".sr.cp1250"
  endif
endif

" The Czech tutor is available in three encodings, guess which one to use
if s:ext =~? '\.cs'
  if &enc =~ 1250
    let s:ext = ".cs.cp1250"
  endif
endif

" The Russian tutor is available in three encodings, guess which one to use.
if s:ext =~? '\.ru'
  if &enc =~ '1251'
    let s:ext = '.ru.cp1251'
  elseif &enc =~ 'koi8'
    let s:ext = '.ru'
  endif
endif

" The Hungarian tutor is available in three encodings, guess which one to use.
if s:ext =~? '\.hu'
  if &enc =~ 1250
    let s:ext = ".hu.cp1250"
  elseif &enc =~ 'iso-8859-2'
    let s:ext = '.hu'
  endif
endif

" The Croatian tutor is available in three encodings, guess which one to use.
if s:ext =~? '\.hr'
  if &enc =~ 1250
    let s:ext = ".hr.cp1250"
  elseif &enc =~ 'iso-8859-2'
    let s:ext = '.hr'
  endif
endif

" If 'encoding' is utf-8 s:ext must end in utf-8.
if &enc == 'utf-8' && s:ext !~ '\.utf-8'
  let s:ext .= '.utf-8'
endif

" 2. Build the name of the file:
let s:tutorfile = "/tutor/tutor"
let s:tutorxx = $VIMRUNTIME . s:tutorfile . s:ext

" 3. Finding the file:
if filereadable(s:tutorxx)
  let $TUTOR = s:tutorxx
elseif s:ext !~ '\.utf-8' && filereadable(s:tutorxx . ".utf-8")
  " Fallback to utf-8 if available.
  let $TUTOR = s:tutorxx . ".utf-8"
else
  let $TUTOR = $VIMRUNTIME . s:tutorfile
  echo "The file " . s:tutorxx . " does not exist.\n"
  echo "Copying English version: " . $TUTOR
  4sleep
endif

" 4. Making the copy and exiting Vim:
e $TUTOR
wq! $TUTORCOPY
