" Vim support file to define the default menus
" You can also use this as a start for your own set of menus.
"
" Maintainer:	The Vim Project <https://github.com/vim/vim>
" Last Change:	2023 Aug 10
" Former Maintainer:	Bram Moolenaar <Bram@vim.org>

" Note that ":an" (short for ":anoremenu") is often used to make a menu work
" in all modes and avoid side effects from mappings defined by the user.

" Make sure the '<' and 'C' flags are not included in 'cpoptions', otherwise
" <CR> would not be recognized.  See ":help 'cpoptions'".
let s:cpo_save = &cpo
set cpo&vim

" Avoid installing the menus twice
if !exists("did_install_default_menus")
let did_install_default_menus = 1


if exists("v:lang") || &langmenu != ""
  " Try to find a menu translation file for the current language.
  if &langmenu != ""
    if &langmenu =~ "none"
      let s:lang = ""
    else
      let s:lang = &langmenu
    endif
  else
    let s:lang = v:lang
  endif
  " A language name must be at least two characters, don't accept "C"
  " Also skip "en_US" to avoid picking up "en_gb" translations.
  if strlen(s:lang) > 1 && s:lang !~? '^en_us'
    " When the language does not include the charset add 'encoding'
    if s:lang =~ '^\a\a$\|^\a\a_\a\a$'
      let s:lang = s:lang .. '.' .. &enc
    endif

    " We always use a lowercase name.
    " Change "iso-8859" to "iso_8859" and "iso8859" to "iso_8859", some
    " systems appear to use this.
    " Change spaces to underscores.
    let s:lang = substitute(tolower(s:lang), '\.iso-', ".iso_", "")
    let s:lang = substitute(s:lang, '\.iso8859', ".iso_8859", "")
    let s:lang = substitute(s:lang, " ", "_", "g")
    " Remove "@euro", otherwise "LC_ALL=de_DE@euro gvim" will show English menus
    let s:lang = substitute(s:lang, "@euro", "", "")
    " Change "iso_8859-1" and "iso_8859-15" to "latin1", we always use the
    " same menu file for them.
    let s:lang = substitute(s:lang, 'iso_8859-15\=$', "latin1", "")
    menutrans clear
    exe "runtime! lang/menu_" .. s:lang .. ".vim"

    if !exists("did_menu_trans")
      " There is no exact match, try matching with a wildcard added
      " (e.g. find menu_de_de.iso_8859-1.vim if s:lang == de_DE).
      let s:lang = substitute(s:lang, '\.[^.]*', "", "")
      exe "runtime! lang/menu_" .. s:lang .. "[^a-z]*vim"

      if !exists("did_menu_trans") && s:lang =~ '_'
	" If the language includes a region try matching without that region.
	" (e.g. find menu_de.vim if s:lang == de_DE).
	let langonly = substitute(s:lang, '_.*', "", "")
	exe "runtime! lang/menu_" .. langonly .. "[^a-z]*vim"
      endif

      if !exists("did_menu_trans") && strlen($LANG) > 1 && s:lang !~ '^en_us'
	" On windows locale names are complicated, try using $LANG, it might
	" have been set by set_init_1().  But don't do this for "en" or "en_us".
	" But don't match "slovak" when $LANG is "sl".
	exe "runtime! lang/menu_" .. tolower($LANG) .. "[^a-z]*vim"
      endif
    endif
  endif
endif


" Help menu
an 9999.10 &Help.&Overview<Tab><F1>	:help<CR>
an 9999.20 &Help.&User\ Manual		:help usr_toc<CR>
an 9999.30 &Help.&How-To\ Links		:help how-to<CR>
an <silent> 9999.40 &Help.&Find\.\.\.	:call <SID>Helpfind()<CR>
an 9999.45 &Help.-sep1-			<Nop>
an 9999.50 &Help.&Credits		:help credits<CR>
an 9999.60 &Help.Co&pying		:help copying<CR>
an 9999.70 &Help.&Sponsor/Register	:help sponsor<CR>
an 9999.70 &Help.O&rphans		:help kcc<CR>
an 9999.75 &Help.-sep2-			<Nop>
an 9999.80 &Help.&Version		:version<CR>
an 9999.90 &Help.&About			:intro<CR>

if exists(':tlmenu')
  tlnoremenu 9999.10 &Help.&Overview<Tab><F1>		<C-W>:help<CR>
  tlnoremenu 9999.20 &Help.&User\ Manual		<C-W>:help usr_toc<CR>
  tlnoremenu 9999.30 &Help.&How-To\ Links		<C-W>:help how-to<CR>
  tlnoremenu <silent> 9999.40 &Help.&Find\.\.\.		<C-W>:call <SID>Helpfind()<CR>
  tlnoremenu 9999.45 &Help.-sep1-			<Nop>
  tlnoremenu 9999.50 &Help.&Credits			<C-W>:help credits<CR>
  tlnoremenu 9999.60 &Help.Co&pying			<C-W>:help copying<CR>
  tlnoremenu 9999.70 &Help.&Sponsor/Register		<C-W>:help sponsor<CR>
  tlnoremenu 9999.70 &Help.O&rphans			<C-W>:help kcc<CR>
  tlnoremenu 9999.75 &Help.-sep2-			<Nop>
  tlnoremenu 9999.80 &Help.&Version			<C-W>:version<CR>
  tlnoremenu 9999.90 &Help.&About			<C-W>:intro<CR>
endif

def s:Helpfind()
  if !exists("g:menutrans_help_dialog")
    g:menutrans_help_dialog = "Enter a command or word to find help on:\n\nPrepend i_ for Input mode commands (e.g.: i_CTRL-X)\nPrepend c_ for command-line editing commands (e.g.: c_<Del>)\nPrepend ' for an option name (e.g.: 'shiftwidth')"
  endif
  var h = inputdialog(g:menutrans_help_dialog)
  if h != ""
    v:errmsg = ""
    silent! exe "help " .. h
    if v:errmsg != ""
      echo v:errmsg
    endif
  endif
enddef

" File menu
an 10.310 &File.&Open\.\.\.<Tab>:e		:browse confirm e<CR>
an 10.320 &File.Sp&lit-Open\.\.\.<Tab>:sp	:browse sp<CR>
an 10.320 &File.Open\ &Tab\.\.\.<Tab>:tabnew	:browse tabnew<CR>
an 10.325 &File.&New<Tab>:enew			:confirm enew<CR>
an <silent> 10.330 &File.&Close<Tab>:close
	\ :if winheight(2) < 0 && tabpagewinnr(2) == 0 <Bar>
	\   confirm enew <Bar>
	\ else <Bar>
	\   confirm close <Bar>
	\ endif<CR>
tln <silent> 10.330 &File.&Close<Tab>:close
	\ <C-W>:if winheight(2) < 0 && tabpagewinnr(2) == 0 <Bar>
	\   confirm enew <Bar>
	\ else <Bar>
	\   confirm close <Bar>
	\ endif<CR>
an 10.335 &File.-SEP1-				<Nop>
an <silent> 10.340 &File.&Save<Tab>:w		:if expand("%") == ""<Bar>browse confirm w<Bar>else<Bar>confirm w<Bar>endif<CR>
an 10.350 &File.Save\ &As\.\.\.<Tab>:sav	:browse confirm saveas<CR>

if has("diff")
  an 10.400 &File.-SEP2-			<Nop>
  an 10.410 &File.Split\ &Diff\ With\.\.\.	:browse vert diffsplit<CR>
  an 10.420 &File.Split\ Patched\ &By\.\.\.	:browse vert diffpatch<CR>
endif

if has("printer")
  an 10.500 &File.-SEP3-			<Nop>
  an 10.510 &File.&Print			:hardcopy<CR>
  vunmenu   &File.&Print
  vnoremenu &File.&Print			:hardcopy<CR>
elseif has("unix")
  an 10.500 &File.-SEP3-			<Nop>
  an 10.510 &File.&Print			:w !lpr<CR>
  vunmenu   &File.&Print
  vnoremenu &File.&Print			:w !lpr<CR>
endif
an 10.600 &File.-SEP4-				<Nop>
an 10.610 &File.Sa&ve-Exit<Tab>:wqa		:confirm wqa<CR>
an 10.620 &File.E&xit<Tab>:qa			:confirm qa<CR>

def s:SelectAll()
  exe "norm! gg" .. (&slm == "" ? "VG" : "gH\<C-O>G")
enddef

" Edit menu
an 20.310 &Edit.&Undo<Tab>u			u
an 20.320 &Edit.&Redo<Tab>^R			<C-R>
an 20.330 &Edit.Rep&eat<Tab>\.			.

an 20.335 &Edit.-SEP1-				<Nop>
vnoremenu 20.340 &Edit.Cu&t<Tab>"+x		"+x
vnoremenu 20.350 &Edit.&Copy<Tab>"+y		"+y
cnoremenu 20.350 &Edit.&Copy<Tab>"+y		<C-Y>
if exists(':tlmenu')
  tlnoremenu 20.350 &Edit.&Copy<Tab>"+y 	<C-W>:<C-Y><CR>
endif
nnoremenu 20.360 &Edit.&Paste<Tab>"+gP		"+gP
cnoremenu	 &Edit.&Paste<Tab>"+gP		<C-R>+
if exists(':tlmenu')
  tlnoremenu	 &Edit.&Paste<Tab>"+gP		<C-W>"+
endif
exe 'vnoremenu <script> &Edit.&Paste<Tab>"+gP	' .. paste#paste_cmd['v']
exe 'inoremenu <script> &Edit.&Paste<Tab>"+gP	' .. paste#paste_cmd['i']
nnoremenu 20.370 &Edit.Put\ &Before<Tab>[p	[p
inoremenu	 &Edit.Put\ &Before<Tab>[p	<C-O>[p
nnoremenu 20.380 &Edit.Put\ &After<Tab>]p	]p
inoremenu	 &Edit.Put\ &After<Tab>]p	<C-O>]p
if has("win32")
  vnoremenu 20.390 &Edit.&Delete<Tab>x		x
endif
noremenu  <script> <silent> 20.400 &Edit.&Select\ All<Tab>ggVG	:<C-U>call <SID>SelectAll()<CR>
inoremenu <script> <silent> 20.400 &Edit.&Select\ All<Tab>ggVG	<C-O>:call <SID>SelectAll()<CR>
cnoremenu <script> <silent> 20.400 &Edit.&Select\ All<Tab>ggVG	<C-U>call <SID>SelectAll()<CR>

an 20.405	 &Edit.-SEP2-				<Nop>
if has("win32") || has("gui_gtk") || has("gui_kde") || has("gui_motif")
  an 20.410	 &Edit.&Find\.\.\.			:promptfind<CR>
  vunmenu	 &Edit.&Find\.\.\.
  vnoremenu <silent>	 &Edit.&Find\.\.\.		y:promptfind <C-R>=<SID>FixFText()<CR><CR>
  an 20.420	 &Edit.Find\ and\ Rep&lace\.\.\.	:promptrepl<CR>
  vunmenu	 &Edit.Find\ and\ Rep&lace\.\.\.
  vnoremenu <silent>	 &Edit.Find\ and\ Rep&lace\.\.\. y:promptrepl <C-R>=<SID>FixFText()<CR><CR>
else
  an 20.410	 &Edit.&Find<Tab>/			/
  an 20.420	 &Edit.Find\ and\ Rep&lace<Tab>:%s	:%s/
  vunmenu	 &Edit.Find\ and\ Rep&lace<Tab>:%s
  vnoremenu	 &Edit.Find\ and\ Rep&lace<Tab>:s	:s/
endif

an 20.425	 &Edit.-SEP3-				<Nop>
an 20.430	 &Edit.Settings\ &Window		:options<CR>
an 20.435	 &Edit.Startup\ &Settings		:call <SID>EditVimrc()<CR>

def s:EditVimrc()
  var fname: string
  if $MYVIMRC != ''
    fname = $MYVIMRC
  elseif has("win32")
    if $HOME != ''
      fname = $HOME .. "/_vimrc"
    else
      fname = $VIM .. "/_vimrc"
    endif
  elseif has("amiga")
    fname = "s:.vimrc"
  else
    fname = $HOME .. "/.vimrc"
  endif
  fname = fnameescape(fname)
  if &mod
    exe "split " .. fname
  else
    exe "edit " .. fname
  endif
enddef

def s:FixFText(): string
  # Fix text in nameless register to be used with :promptfind.
  return substitute(@", "[\r\n]", '\\n', 'g')
enddef

" Edit/Global Settings
an 20.440.100 &Edit.&Global\ Settings.Toggle\ Pattern\ &Highlight<Tab>:set\ hls!	:set hls! hls?<CR>
an 20.440.110 &Edit.&Global\ Settings.Toggle\ &Ignoring\ Case<Tab>:set\ ic!	:set ic! ic?<CR>
an 20.440.110 &Edit.&Global\ Settings.Toggle\ &Showing\ Matched\ Pairs<Tab>:set\ sm!	:set sm! sm?<CR>

an 20.440.120 &Edit.&Global\ Settings.&Context\ Lines.\ 1\  :set so=1<CR>
an 20.440.120 &Edit.&Global\ Settings.&Context\ Lines.\ 2\  :set so=2<CR>
an 20.440.120 &Edit.&Global\ Settings.&Context\ Lines.\ 3\  :set so=3<CR>
an 20.440.120 &Edit.&Global\ Settings.&Context\ Lines.\ 4\  :set so=4<CR>
an 20.440.120 &Edit.&Global\ Settings.&Context\ Lines.\ 5\  :set so=5<CR>
an 20.440.120 &Edit.&Global\ Settings.&Context\ Lines.\ 7\  :set so=7<CR>
an 20.440.120 &Edit.&Global\ Settings.&Context\ Lines.\ 10\  :set so=10<CR>
an 20.440.120 &Edit.&Global\ Settings.&Context\ Lines.\ 100\  :set so=100<CR>

an 20.440.130.40 &Edit.&Global\ Settings.&Virtual\ Edit.Never :set ve=<CR>
an 20.440.130.50 &Edit.&Global\ Settings.&Virtual\ Edit.Block\ Selection :set ve=block<CR>
an 20.440.130.60 &Edit.&Global\ Settings.&Virtual\ Edit.Insert\ Mode :set ve=insert<CR>
an 20.440.130.70 &Edit.&Global\ Settings.&Virtual\ Edit.Block\ and\ Insert :set ve=block,insert<CR>
an 20.440.130.80 &Edit.&Global\ Settings.&Virtual\ Edit.Always :set ve=all<CR>
an 20.440.140 &Edit.&Global\ Settings.Toggle\ Insert\ &Mode<Tab>:set\ im!	:set im!<CR>
an 20.440.145 &Edit.&Global\ Settings.Toggle\ Vi\ C&ompatibility<Tab>:set\ cp!	:set cp!<CR>
an <silent> 20.440.150 &Edit.&Global\ Settings.Search\ &Path\.\.\.  :call <SID>SearchP()<CR>
an <silent> 20.440.160 &Edit.&Global\ Settings.Ta&g\ Files\.\.\.  :call <SID>TagFiles()<CR>
"
" GUI options
an 20.440.300 &Edit.&Global\ Settings.-SEP1-				<Nop>
an <silent> 20.440.310 &Edit.&Global\ Settings.Toggle\ &Toolbar		:call <SID>ToggleGuiOption("T")<CR>
an <silent> 20.440.320 &Edit.&Global\ Settings.Toggle\ &Bottom\ Scrollbar :call <SID>ToggleGuiOption("b")<CR>
an <silent> 20.440.330 &Edit.&Global\ Settings.Toggle\ &Left\ Scrollbar	:call <SID>ToggleGuiOption("l")<CR>
an <silent> 20.440.340 &Edit.&Global\ Settings.Toggle\ &Right\ Scrollbar :call <SID>ToggleGuiOption("r")<CR>

def s:SearchP()
  if !exists("g:menutrans_path_dialog")
    g:menutrans_path_dialog = "Enter search path for files.\nSeparate directory names with a comma."
  endif
  var n = inputdialog(g:menutrans_path_dialog, substitute(&path, '\\ ', ' ', 'g'))
  if n != ""
    &path = substitute(n, ' ', '\\ ', 'g')
  endif
enddef

def s:TagFiles()
  if !exists("g:menutrans_tags_dialog")
    g:menutrans_tags_dialog = "Enter names of tag files.\nSeparate the names with a comma."
  endif
  var n = inputdialog(g:menutrans_tags_dialog, substitute(&tags, '\\ ', ' ', 'g'))
  if n != ""
    &tags = substitute(n, ' ', '\\ ', 'g')
  endif
enddef

def s:ToggleGuiOption(option: string)
  # If a:option is already set in guioptions, then we want to remove it
  if match(&guioptions, "\\C" .. option) > -1
    exec "set go-=" .. option
  else
    exec "set go+=" .. option
  endif
enddef

" Edit/File Settings

" Boolean options
an 20.440.100 &Edit.F&ile\ Settings.Toggle\ Line\ &Numbering<Tab>:set\ nu!	:set nu! nu?<CR>
an 20.440.105 &Edit.F&ile\ Settings.Toggle\ Relati&ve\ Line\ Numbering<Tab>:set\ rnu!	:set rnu! rnu?<CR>
an 20.440.110 &Edit.F&ile\ Settings.Toggle\ &List\ Mode<Tab>:set\ list!	:set list! list?<CR>
an 20.440.120 &Edit.F&ile\ Settings.Toggle\ Line\ &Wrapping<Tab>:set\ wrap!	:set wrap! wrap?<CR>
an 20.440.130 &Edit.F&ile\ Settings.Toggle\ W&rapping\ at\ Word<Tab>:set\ lbr!	:set lbr! lbr?<CR>
an 20.440.160 &Edit.F&ile\ Settings.Toggle\ Tab\ &Expanding<Tab>:set\ et!	:set et! et?<CR>
an 20.440.170 &Edit.F&ile\ Settings.Toggle\ &Auto\ Indenting<Tab>:set\ ai!	:set ai! ai?<CR>
an 20.440.180 &Edit.F&ile\ Settings.Toggle\ &C-Style\ Indenting<Tab>:set\ cin!	:set cin! cin?<CR>

" other options
an 20.440.600 &Edit.F&ile\ Settings.-SEP2-		<Nop>
an 20.440.610.20 &Edit.F&ile\ Settings.&Shiftwidth.2	:set sw=2 sw?<CR>
an 20.440.610.30 &Edit.F&ile\ Settings.&Shiftwidth.3	:set sw=3 sw?<CR>
an 20.440.610.40 &Edit.F&ile\ Settings.&Shiftwidth.4	:set sw=4 sw?<CR>
an 20.440.610.50 &Edit.F&ile\ Settings.&Shiftwidth.5	:set sw=5 sw?<CR>
an 20.440.610.60 &Edit.F&ile\ Settings.&Shiftwidth.6	:set sw=6 sw?<CR>
an 20.440.610.80 &Edit.F&ile\ Settings.&Shiftwidth.8	:set sw=8 sw?<CR>

an 20.440.620.20 &Edit.F&ile\ Settings.Soft\ &Tabstop.2	:set sts=2 sts?<CR>
an 20.440.620.30 &Edit.F&ile\ Settings.Soft\ &Tabstop.3	:set sts=3 sts?<CR>
an 20.440.620.40 &Edit.F&ile\ Settings.Soft\ &Tabstop.4	:set sts=4 sts?<CR>
an 20.440.620.50 &Edit.F&ile\ Settings.Soft\ &Tabstop.5	:set sts=5 sts?<CR>
an 20.440.620.60 &Edit.F&ile\ Settings.Soft\ &Tabstop.6	:set sts=6 sts?<CR>
an 20.440.620.80 &Edit.F&ile\ Settings.Soft\ &Tabstop.8	:set sts=8 sts?<CR>

an <silent> 20.440.630 &Edit.F&ile\ Settings.Te&xt\ Width\.\.\.  :call <SID>TextWidth()<CR>
an <silent> 20.440.640 &Edit.F&ile\ Settings.&File\ Format\.\.\.  :call <SID>FileFormat()<CR>

def s:TextWidth()
  if !exists("g:menutrans_textwidth_dialog")
    g:menutrans_textwidth_dialog = "Enter new text width (0 to disable formatting): "
  endif
  var n = inputdialog(g:menutrans_textwidth_dialog, &tw .. '')
  if n != ""
    # Remove leading zeros to avoid it being used as an octal number.
    # But keep a zero by itself.
    var tw = substitute(n, "^0*", "", "")
    &tw = tw == '' ? 0 : str2nr(tw)
  endif
enddef

def s:FileFormat()
  if !exists("g:menutrans_fileformat_dialog")
    g:menutrans_fileformat_dialog = "Select format for writing the file"
  endif
  if !exists("g:menutrans_fileformat_choices")
    g:menutrans_fileformat_choices = "&Unix\n&Dos\n&Mac\n&Cancel"
  endif
  var def_choice: number
  if &ff == "dos"
    def_choice = 2
  elseif &ff == "mac"
    def_choice = 3
  else
    def_choice = 1
  endif
  var n = confirm(g:menutrans_fileformat_dialog, g:menutrans_fileformat_choices, def_choice, "Question")
  if n == 1
    set ff=unix
  elseif n == 2
    set ff=dos
  elseif n == 3
    set ff=mac
  endif
enddef

let s:did_setup_color_schemes = 0

" Setup the Edit.Color Scheme submenu
def s:SetupColorSchemes()
  if s:did_setup_color_schemes
    return
  endif
  s:did_setup_color_schemes = 1

  var n = globpath(&runtimepath, "colors/*.vim", 1, 1)
  n += globpath(&packpath, "pack/*/start/*/colors/*.vim", 1, 1)
  n += globpath(&packpath, "pack/*/opt/*/colors/*.vim", 1, 1)

  # Ignore case for VMS and windows, sort on name
  var names = sort(map(n, 'substitute(v:val, "\\c.*[/\\\\:\\]]\\([^/\\\\:]*\\)\\.vim", "\\1", "")'), 'i')

  # define all the submenu entries
  var idx = 100
  for name in names
    exe "an 20.450." .. idx .. ' &Edit.C&olor\ Scheme.' .. name .. " :colors " .. name .. "<CR>"
    idx += 10
  endfor
  silent! aunmenu &Edit.Show\ C&olor\ Schemes\ in\ Menu
enddef

if exists("do_no_lazyload_menus")
  call s:SetupColorSchemes()
else
  an <silent> 20.450 &Edit.Show\ C&olor\ Schemes\ in\ Menu :call <SID>SetupColorSchemes()<CR>
endif


" Setup the Edit.Keymap submenu
if has("keymap")
  let s:did_setup_keymaps = 0

  def s:SetupKeymaps()
    if s:did_setup_keymaps
      return
    endif
    s:did_setup_keymaps = 1

    var names = globpath(&runtimepath, "keymap/*.vim", 1, 1)
    if !empty(names)
      var idx = 100
      an 20.460.90 &Edit.&Keymap.None :set keymap=<CR>
      for name in names
	# Ignore case for VMS and windows
	var mapname = substitute(name, '\c.*[/\\:\]]\([^/\\:_]*\)\(_[0-9a-zA-Z-]*\)\=\.vim', '\1', '')
	exe "an 20.460." .. idx .. ' &Edit.&Keymap.' .. mapname .. " :set keymap=" .. mapname .. "<CR>"
	idx += 10
      endfor
    endif
    silent! aunmenu &Edit.Show\ &Keymaps\ in\ Menu
  enddef

  if exists("do_no_lazyload_menus")
    call s:SetupKeymaps()
  else
    an <silent> 20.460 &Edit.Show\ &Keymaps\ in\ Menu :call <SID>SetupKeymaps()<CR>
  endif
endif

if has("win32") || has("gui_motif") || has("gui_gtk") || has("gui_kde") || has("gui_photon") || has("gui_mac")
  an 20.470 &Edit.Select\ Fo&nt\.\.\.	:set guifont=*<CR>
endif

" Programming menu
if !exists("g:ctags_command")
  if has("vms")
    let g:ctags_command = "mc vim:ctags *.*"
  else
    let g:ctags_command = "ctags -R ."
  endif
endif

an 40.300 &Tools.&Jump\ to\ This\ Tag<Tab>g^]	g<C-]>
vunmenu &Tools.&Jump\ to\ This\ Tag<Tab>g^]
vnoremenu &Tools.&Jump\ to\ This\ Tag<Tab>g^]	g<C-]>
an 40.310 &Tools.Jump\ &Back<Tab>^T		<C-T>
an 40.320 &Tools.Build\ &Tags\ File		:exe "!" .. g:ctags_command<CR>

if has("folding") || has("spell")
  an 40.330 &Tools.-SEP1-						<Nop>
endif

" Tools.Spelling Menu
if has("spell")
  an 40.335.110 &Tools.&Spelling.&Spell\ Check\ On		:set spell<CR>
  an 40.335.120 &Tools.&Spelling.Spell\ Check\ &Off		:set nospell<CR>
  an 40.335.130 &Tools.&Spelling.To\ &Next\ Error<Tab>]s	]s
  an 40.335.130 &Tools.&Spelling.To\ &Previous\ Error<Tab>[s	[s
  an 40.335.140 &Tools.&Spelling.Suggest\ &Corrections<Tab>z=	z=
  an 40.335.150 &Tools.&Spelling.&Repeat\ Correction<Tab>:spellrepall	:spellrepall<CR>
  an 40.335.200 &Tools.&Spelling.-SEP1-				<Nop>
  an 40.335.210 &Tools.&Spelling.Set\ Language\ to\ "en"	:set spl=en spell<CR>
  an 40.335.220 &Tools.&Spelling.Set\ Language\ to\ "en_au"	:set spl=en_au spell<CR>
  an 40.335.230 &Tools.&Spelling.Set\ Language\ to\ "en_ca"	:set spl=en_ca spell<CR>
  an 40.335.240 &Tools.&Spelling.Set\ Language\ to\ "en_gb"	:set spl=en_gb spell<CR>
  an 40.335.250 &Tools.&Spelling.Set\ Language\ to\ "en_nz"	:set spl=en_nz spell<CR>
  an 40.335.260 &Tools.&Spelling.Set\ Language\ to\ "en_us"	:set spl=en_us spell<CR>
  an <silent> 40.335.270 &Tools.&Spelling.&Find\ More\ Languages	:call <SID>SpellLang()<CR>

  let s:undo_spelllang = ['aun &Tools.&Spelling.&Find\ More\ Languages']
  def s:SpellLang(encChanged = false)
    for cmd in s:undo_spelllang
      exe "silent! " .. cmd
    endfor
    s:undo_spelllang = []

    var enc = &enc == "iso-8859-15" ? "latin1" : &enc

    # Reset g:menutrans_set_lang_to when called for the EncodingChanged event.
    if !exists("g:menutrans_set_lang_to") || encChanged
      g:menutrans_set_lang_to = 'Set Language to'
    endif

    var found = 0
    var _nm = ''
    var names = globpath(&runtimepath, "spell/*." .. enc .. ".spl", 1, 1)
    if !empty(names)
      var n = 300
      for f in names
	var nm = substitute(f, '.*spell[/\\]\(..\)\.[^/\\]*\.spl', '\1', "")
	if nm != "en" && nm !~ '/'
          _nm = nm
	  found += 1
	  var menuname = '&Tools.&Spelling.' .. escape(g:menutrans_set_lang_to, "\\. \t|") .. '\ "' .. nm .. '"'
	  exe 'an 40.335.' .. n .. ' ' .. menuname .. ' :set spl=' .. nm .. ' spell<CR>'
	  s:undo_spelllang += ['aun ' .. menuname]
	endif
	n += 10
      endfor
    endif
    if found == 0
      echomsg "Could not find other spell files"
    elseif found == 1
      echomsg "Found spell file " .. _nm
    else
      echomsg "Found " .. found .. " more spell files"
    endif

    # Need to redo this when 'encoding' is changed.
    augroup spellmenu
    au! EncodingChanged * call SpellLang(true)
    augroup END
  enddef
endif

" Tools.Fold Menu
if has("folding")
  " open close folds
  an 40.340.110 &Tools.&Folding.&Enable/Disable\ Folds<Tab>zi		zi
  an 40.340.120 &Tools.&Folding.&View\ Cursor\ Line<Tab>zv		zv
  an 40.340.120 &Tools.&Folding.Vie&w\ Cursor\ Line\ Only<Tab>zMzx	zMzx
  inoremenu 40.340.120 &Tools.&Folding.Vie&w\ Cursor\ Line\ Only<Tab>zMzx  <C-O>zM<C-O>zx
  an 40.340.130 &Tools.&Folding.C&lose\ More\ Folds<Tab>zm		zm
  an 40.340.140 &Tools.&Folding.&Close\ All\ Folds<Tab>zM		zM
  an 40.340.150 &Tools.&Folding.O&pen\ More\ Folds<Tab>zr		zr
  an 40.340.160 &Tools.&Folding.&Open\ All\ Folds<Tab>zR		zR
  " fold method
  an 40.340.200 &Tools.&Folding.-SEP1-			<Nop>
  an 40.340.210 &Tools.&Folding.Fold\ Met&hod.M&anual	:set fdm=manual<CR>
  an 40.340.210 &Tools.&Folding.Fold\ Met&hod.I&ndent	:set fdm=indent<CR>
  an 40.340.210 &Tools.&Folding.Fold\ Met&hod.E&xpression :set fdm=expr<CR>
  an 40.340.210 &Tools.&Folding.Fold\ Met&hod.S&yntax	:set fdm=syntax<CR>
  an 40.340.210 &Tools.&Folding.Fold\ Met&hod.&Diff	:set fdm=diff<CR>
  an 40.340.210 &Tools.&Folding.Fold\ Met&hod.Ma&rker	:set fdm=marker<CR>
  " create and delete folds
  vnoremenu 40.340.220 &Tools.&Folding.Create\ &Fold<Tab>zf	zf
  an 40.340.230 &Tools.&Folding.&Delete\ Fold<Tab>zd		zd
  an 40.340.240 &Tools.&Folding.Delete\ &All\ Folds<Tab>zD	zD
  " moving around in folds
  an 40.340.300 &Tools.&Folding.-SEP2-				<Nop>
  an 40.340.310.10 &Tools.&Folding.Fold\ Col&umn\ Width.\ &0\ 	:set fdc=0<CR>
  an 40.340.310.20 &Tools.&Folding.Fold\ Col&umn\ Width.\ &2\ 	:set fdc=2<CR>
  an 40.340.310.30 &Tools.&Folding.Fold\ Col&umn\ Width.\ &3\ 	:set fdc=3<CR>
  an 40.340.310.40 &Tools.&Folding.Fold\ Col&umn\ Width.\ &4\ 	:set fdc=4<CR>
  an 40.340.310.50 &Tools.&Folding.Fold\ Col&umn\ Width.\ &5\ 	:set fdc=5<CR>
  an 40.340.310.60 &Tools.&Folding.Fold\ Col&umn\ Width.\ &6\ 	:set fdc=6<CR>
  an 40.340.310.70 &Tools.&Folding.Fold\ Col&umn\ Width.\ &7\ 	:set fdc=7<CR>
  an 40.340.310.80 &Tools.&Folding.Fold\ Col&umn\ Width.\ &8\ 	:set fdc=8<CR>
endif  " has folding

if has("diff")
  an 40.350.100 &Tools.&Diff.&Update		:diffupdate<CR>
  an 40.350.110 &Tools.&Diff.&Get\ Block	:diffget<CR>
  vunmenu &Tools.&Diff.&Get\ Block
  vnoremenu &Tools.&Diff.&Get\ Block		:diffget<CR>
  an 40.350.120 &Tools.&Diff.&Put\ Block	:diffput<CR>
  vunmenu &Tools.&Diff.&Put\ Block
  vnoremenu &Tools.&Diff.&Put\ Block		:diffput<CR>
endif

an 40.358 &Tools.-SEP2-					<Nop>
an 40.360 &Tools.&Make<Tab>:make			:make<CR>
an 40.370 &Tools.&List\ Errors<Tab>:cl			:cl<CR>
an 40.380 &Tools.L&ist\ Messages<Tab>:cl!		:cl!<CR>
an 40.390 &Tools.&Next\ Error<Tab>:cn			:cn<CR>
an 40.400 &Tools.&Previous\ Error<Tab>:cp		:cp<CR>
an 40.410 &Tools.&Older\ List<Tab>:cold			:colder<CR>
an 40.420 &Tools.N&ewer\ List<Tab>:cnew			:cnewer<CR>
an 40.430.50 &Tools.Error\ &Window.&Update<Tab>:cwin	:cwin<CR>
an 40.430.60 &Tools.Error\ &Window.&Open<Tab>:copen	:copen<CR>
an 40.430.70 &Tools.Error\ &Window.&Close<Tab>:cclose	:cclose<CR>

an 40.520 &Tools.-SEP3-					<Nop>
an <silent> 40.530 &Tools.&Convert\ to\ HEX<Tab>:%!xxd
	\ :call <SID>XxdConv()<CR>
an <silent> 40.540 &Tools.Conve&rt\ Back<Tab>:%!xxd\ -r
	\ :call <SID>XxdBack()<CR>

" Use a function to do the conversion, so that it also works with 'insertmode'
" set.
def s:XxdConv()
  var mod = &mod
  if has("vms")
    :%!mc vim:xxd
  else
    s:XxdFind()
    exe ':%!' .. g:xxdprogram
  endif
  if getline(1) =~ "^00000000:"		# only if it worked
    set ft=xxd
  endif
  &mod = mod
enddef

def s:XxdBack()
  var mod = &mod
  if has("vms")
    :%!mc vim:xxd -r
  else
    s:XxdFind()
    exe ':%!' .. g:xxdprogram .. ' -r'
  endif
  set ft=
  if exists('#filetypedetect') && exists('#BufReadPost')
    doautocmd filetypedetect BufReadPost
  endif
  &mod = mod
enddef

def s:XxdFind()
  if !exists("g:xxdprogram")
    # On the PC xxd may not be in the path but in the install directory
    if has("win32") && !executable("xxd")
      g:xxdprogram = $VIMRUNTIME .. (&shellslash ? '/' : '\') .. "xxd.exe"
      if g:xxdprogram =~ ' '
	g:xxdprogram = '"' .. g:xxdprogram .. '"'
      endif
    else
      g:xxdprogram = "xxd"
    endif
  endif
enddef

let s:did_setup_compilers = 0

" Setup the Tools.Compiler submenu
def s:SetupCompilers()
  if s:did_setup_compilers
    return
  endif
  s:did_setup_compilers = 1

  var names = globpath(&runtimepath, "compiler/*.vim", 1, 1)
  var idx = 100
  for name in names
    # Ignore case for VMS and windows
    var cname = substitute(name, '\c.*[/\\:\]]\([^/\\:]*\)\.vim', '\1', '')
    exe "an 30.440." .. idx .. ' &Tools.Se&t\ Compiler.' .. cname .. " :compiler " .. cname .. "<CR>"
    idx += 10
  endfor
  silent! aunmenu &Tools.Show\ Compiler\ Se&ttings\ in\ Menu
enddef

if exists("do_no_lazyload_menus")
  call s:SetupCompilers()
else
  an <silent> 30.440 &Tools.Show\ Compiler\ Se&ttings\ in\ Menu :call <SID>SetupCompilers()<CR>
endif

" Load ColorScheme, Compiler Setting and Keymap menus when idle.
if !exists("do_no_lazyload_menus")
  def s:SetupLazyloadMenus()
    s:SetupColorSchemes()
    s:SetupCompilers()
    if has("keymap")
      s:SetupKeymaps()
    endif
  enddef
  augroup SetupLazyloadMenus
    au!
    au CursorHold,CursorHoldI * call <SID>SetupLazyloadMenus() | au! SetupLazyloadMenus
  augroup END
endif


if !exists("no_buffers_menu")

" Buffer list menu -- Setup functions & actions

" wait with building the menu until after loading 'session' files. Makes
" startup faster.
let s:bmenu_wait = 1

" Dictionary of buffer number to name. This helps prevent problems where a
" buffer as renamed and we didn't keep track of that.
let s:bmenu_items = {}

if !exists("bmenu_priority")
  let bmenu_priority = 60
endif

" invoked from a BufCreate or BufFilePost autocommand
def s:BMAdd()
  if s:bmenu_wait == 0
    # when adding too many buffers, redraw in short format
    if s:bmenu_count == &menuitems && s:bmenu_short == 0
      s:BMShow()
    else
      var name = expand("<afile>")
      var num = str2nr(expand("<abuf>"))
      if s:BMCanAdd(name, num)
	s:BMFilename(name, num)
	s:bmenu_count += 1
      endif
    endif
  endif
enddef

" invoked from a BufDelete or BufFilePre autocommand
def s:BMRemove()
  if s:bmenu_wait == 0
    var bufnum = expand("<abuf>")
    if s:bmenu_items->has_key(bufnum)
      var menu_name = s:bmenu_items[bufnum]
      exe 'silent! aun &Buffers.' .. menu_name
      s:bmenu_count = s:bmenu_count - 1
      unlet s:bmenu_items[bufnum]
    endif
  endif
enddef

" Return non-zero if buffer with number "name" / "num" is useful to add in the
" buffer menu.
def s:BMCanAdd(name: string, num: number): bool
  # no directory or unlisted buffer
  if isdirectory(name) || !buflisted(num)
    return false
  endif

  # no name with control characters
  if name =~ '[\x01-\x1f]'
    return false
  endif

  # no special buffer, such as terminal or popup
  var buftype = getbufvar(num, '&buftype')
  if buftype != '' && buftype != 'nofile' && buftype != 'nowrite'
    return false
  endif

  # only existing buffers
  return bufexists(num)
enddef

" Create the buffer menu (delete an existing one first).
def s:BMShow()
  s:bmenu_wait = 1
  s:bmenu_short = 1
  s:bmenu_count = 0
  s:bmenu_items = {}

  # Remove old menu, if it exists; keep one entry to avoid a torn off menu to
  # disappear.  Use try/catch to avoid setting v:errmsg
  try 
    unmenu &Buffers 
  catch 
  endtry
  exe 'noremenu ' .. g:bmenu_priority .. ".1 &Buffers.Dummy l"
  try 
    unmenu! &Buffers 
  catch 
  endtry

  # create new menu
  exe 'an <silent> ' .. g:bmenu_priority .. ".2 &Buffers.&Refresh\\ menu :call <SID>BMShow()<CR>"
  exe 'an ' .. g:bmenu_priority .. ".4 &Buffers.&Delete :confirm bd<CR>"
  exe 'an ' .. g:bmenu_priority .. ".6 &Buffers.&Alternate :confirm b #<CR>"
  exe 'an ' .. g:bmenu_priority .. ".7 &Buffers.&Next :confirm bnext<CR>"
  exe 'an ' .. g:bmenu_priority .. ".8 &Buffers.&Previous :confirm bprev<CR>"
  exe 'an ' .. g:bmenu_priority .. ".9 &Buffers.-SEP- :"
  unmenu &Buffers.Dummy

  # figure out how many buffers there are
  var buf = 1
  while buf <= bufnr('$')
    if s:BMCanAdd(bufname(buf), buf)
      s:bmenu_count = s:bmenu_count + 1
    endif
    buf += 1
  endwhile
  if s:bmenu_count <= &menuitems
    s:bmenu_short = 0
  endif

  # iterate through buffer list, adding each buffer to the menu:
  buf = 1
  while buf <= bufnr('$')
    var name = bufname(buf)
    if s:BMCanAdd(name, buf)
      call s:BMFilename(name, buf)
    endif
    buf += 1
  endwhile
  s:bmenu_wait = 0
  aug buffer_list
    au!
    au BufCreate,BufFilePost * call s:BMAdd()
    au BufDelete,BufFilePre * call s:BMRemove()
  aug END
enddef

def s:BMHash(name: string): number
  # Make name all upper case, so that chars are between 32 and 96
  var nm = substitute(name, ".*", '\U\0', "")
  var sp: number
  if has("ebcdic")
    # HACK: Replace all non alphabetics with 'Z'
    #       Just to make it work for now.
    nm = substitute(nm, "[^A-Z]", 'Z', "g")
    sp = char2nr('A') - 1
  else
    sp = char2nr(' ')
  endif
  # convert first six chars into a number for sorting:
  return (char2nr(nm[0]) - sp) * 0x800000 + (char2nr(nm[1]) - sp) * 0x20000 + (char2nr(nm[2]) - sp) * 0x1000 + (char2nr(nm[3]) - sp) * 0x80 + (char2nr(nm[4]) - sp) * 0x20 + (char2nr(nm[5]) - sp)
enddef

def s:BMHash2(name: string): string
  var nm = substitute(name, ".", '\L\0', "")
  if nm[0] < 'a' || nm[0] > 'z'
    return '&others.'
  elseif nm[0] <= 'd'
    return '&abcd.'
  elseif nm[0] <= 'h'
    return '&efgh.'
  elseif nm[0] <= 'l'
    return '&ijkl.'
  elseif nm[0] <= 'p'
    return '&mnop.'
  elseif nm[0] <= 't'
    return '&qrst.'
  else
    return '&u-z.'
  endif
enddef

" Insert a buffer name into the buffer menu.
def s:BMFilename(name: string, num: number)
  var munge = s:BMMunge(name, num)
  var hash = s:BMHash(munge)
  var cmd: string
  if s:bmenu_short == 0
    s:bmenu_items[num] = munge
    cmd = 'an ' .. g:bmenu_priority .. '.' .. hash .. ' &Buffers.' .. munge
  else
    var menu_name = s:BMHash2(munge) .. munge
    s:bmenu_items[num] = menu_name
    cmd = 'an ' .. g:bmenu_priority .. '.' .. hash .. '.' .. hash .. ' &Buffers.' .. menu_name
  endif
  exe cmd .. ' :confirm b' .. num .. '<CR>'
enddef

" Truncate a long path to fit it in a menu item.
if !exists("g:bmenu_max_pathlen")
  let g:bmenu_max_pathlen = 35
endif

def s:BMTruncName(fname: string): string
  var name = fname
  if g:bmenu_max_pathlen < 5
    name = ""
  else
    var len = strlen(name)
    if len > g:bmenu_max_pathlen
      var amountl = (g:bmenu_max_pathlen / 2) - 2
      var amountr = g:bmenu_max_pathlen - amountl - 3
      var pattern = '^\(.\{,' .. amountl .. '}\).\{-}\(.\{,' .. amountr .. '}\)$'
      var left = substitute(name, pattern, '\1', '')
      var right = substitute(name, pattern, '\2', '')
      if strlen(left) + strlen(right) < len
	name = left .. '...' .. right
      endif
    endif
  endif
  return name
enddef

def s:BMMunge(fname: string, bnum: number): string
  var name = fname
  if name == ''
    if !exists("g:menutrans_no_file")
      g:menutrans_no_file = "[No Name]"
    endif
    name = g:menutrans_no_file
  else
    name = fnamemodify(name, ':p:~')
  endif
  # detach file name and separate it out:
  var name2 = fnamemodify(name, ':t')
  if bnum >= 0
    name2 = name2 .. ' (' .. bnum .. ')'
  endif
  name = name2 .. "\t" .. s:BMTruncName(fnamemodify(name, ':h'))
  name = escape(name, "\\. \t|")
  name = substitute(name, "&", "&&", "g")
  name = substitute(name, "\n", "^@", "g")
  return name
enddef

" When just starting Vim, load the buffer menu later
if has("vim_starting")
  augroup LoadBufferMenu
    au! VimEnter * if !exists("no_buffers_menu") | call <SID>BMShow() | endif
    au  VimEnter * au! LoadBufferMenu
  augroup END
else
  call <SID>BMShow()
endif

endif " !exists("no_buffers_menu")

" Window menu
an 70.300 &Window.&New<Tab>^Wn			<C-W>n
an 70.310 &Window.S&plit<Tab>^Ws		<C-W>s
an 70.320 &Window.Sp&lit\ To\ #<Tab>^W^^	<C-W><C-^>
an 70.330 &Window.Split\ &Vertically<Tab>^Wv	<C-W>v
an <silent> 70.332 &Window.Split\ File\ E&xplorer	:call MenuExplOpen()<CR>
if !exists("*MenuExplOpen")
  def MenuExplOpen()
    if @% == ""
      :20vsp .
    else
      exe ":20vsp " .. fnameescape(expand("%:p:h"))
    endif
  enddef
endif
an 70.335 &Window.-SEP1-				<Nop>
an 70.340 &Window.&Close<Tab>^Wc			:confirm close<CR>
an 70.345 &Window.Close\ &Other(s)<Tab>^Wo		:confirm only<CR>
an 70.350 &Window.-SEP2-				<Nop>
an 70.355 &Window.Move\ &To.&Top<Tab>^WK		<C-W>K
an 70.355 &Window.Move\ &To.&Bottom<Tab>^WJ		<C-W>J
an 70.355 &Window.Move\ &To.&Left\ Side<Tab>^WH		<C-W>H
an 70.355 &Window.Move\ &To.&Right\ Side<Tab>^WL	<C-W>L
an 70.360 &Window.Rotate\ &Up<Tab>^WR			<C-W>R
an 70.362 &Window.Rotate\ &Down<Tab>^Wr			<C-W>r
an 70.365 &Window.-SEP3-				<Nop>
an 70.370 &Window.&Equal\ Size<Tab>^W=			<C-W>=
an 70.380 &Window.&Max\ Height<Tab>^W_			<C-W>_
an 70.390 &Window.M&in\ Height<Tab>^W1_			<C-W>1_
an 70.400 &Window.Max\ &Width<Tab>^W\|			<C-W>\|
an 70.410 &Window.Min\ Widt&h<Tab>^W1\|			<C-W>1\|

" The popup menu
an 1.10 PopUp.&Undo			u
an 1.15 PopUp.-SEP1-			<Nop>
vnoremenu 1.20 PopUp.Cu&t		"+x
vnoremenu 1.30 PopUp.&Copy		"+y
cnoremenu 1.30 PopUp.&Copy		<C-Y>
nnoremenu 1.40 PopUp.&Paste		"+gP
cnoremenu 1.40 PopUp.&Paste		<C-R>+
exe 'vnoremenu <script> 1.40 PopUp.&Paste	' .. paste#paste_cmd['v']
exe 'inoremenu <script> 1.40 PopUp.&Paste	' .. paste#paste_cmd['i']
vnoremenu 1.50 PopUp.&Delete		x
an 1.55 PopUp.-SEP2-			<Nop>
vnoremenu 1.60 PopUp.Select\ Blockwise	<C-V>

nnoremenu 1.70 PopUp.Select\ &Word	vaw
onoremenu 1.70 PopUp.Select\ &Word	aw
vnoremenu 1.70 PopUp.Select\ &Word	<C-C>vaw
inoremenu 1.70 PopUp.Select\ &Word	<C-O>vaw
cnoremenu 1.70 PopUp.Select\ &Word	<C-C>vaw

nnoremenu 1.73 PopUp.Select\ &Sentence	vas
onoremenu 1.73 PopUp.Select\ &Sentence	as
vnoremenu 1.73 PopUp.Select\ &Sentence	<C-C>vas
inoremenu 1.73 PopUp.Select\ &Sentence	<C-O>vas
cnoremenu 1.73 PopUp.Select\ &Sentence	<C-C>vas

nnoremenu 1.77 PopUp.Select\ Pa&ragraph	vap
onoremenu 1.77 PopUp.Select\ Pa&ragraph	ap
vnoremenu 1.77 PopUp.Select\ Pa&ragraph	<C-C>vap
inoremenu 1.77 PopUp.Select\ Pa&ragraph	<C-O>vap
cnoremenu 1.77 PopUp.Select\ Pa&ragraph	<C-C>vap

nnoremenu 1.80 PopUp.Select\ &Line	V
onoremenu 1.80 PopUp.Select\ &Line	<C-C>V
vnoremenu 1.80 PopUp.Select\ &Line	<C-C>V
inoremenu 1.80 PopUp.Select\ &Line	<C-O>V
cnoremenu 1.80 PopUp.Select\ &Line	<C-C>V

nnoremenu 1.90 PopUp.Select\ &Block	<C-V>
onoremenu 1.90 PopUp.Select\ &Block	<C-C><C-V>
vnoremenu 1.90 PopUp.Select\ &Block	<C-C><C-V>
inoremenu 1.90 PopUp.Select\ &Block	<C-O><C-V>
cnoremenu 1.90 PopUp.Select\ &Block	<C-C><C-V>

noremenu  <script> <silent> 1.100 PopUp.Select\ &All	:<C-U>call <SID>SelectAll()<CR>
inoremenu <script> <silent> 1.100 PopUp.Select\ &All	<C-O>:call <SID>SelectAll()<CR>
cnoremenu <script> <silent> 1.100 PopUp.Select\ &All	<C-U>call <SID>SelectAll()<CR>

if has("spell")
  " Spell suggestions in the popup menu.  Note that this will slow down the
  " appearance of the menu!
  def s:SpellPopup()
    if exists("s:changeitem") && s:changeitem != ''
      call s:SpellDel()
    endif

    # Return quickly if spell checking is not enabled.
    if !&spell || &spelllang == ''
      return
    endif

    var curcol = col('.')
    var w: string
    var a: string
    [w, a] = spellbadword()
    if col('.') > curcol		# don't use word after the cursor
      w = ''
    endif
    if w != ''
      if a == 'caps'
	s:suglist = [substitute(w, '.*', '\u&', '')]
      else
	s:suglist = spellsuggest(w, 10)
      endif
      if len(s:suglist) > 0
	if !exists("g:menutrans_spell_change_ARG_to")
	  g:menutrans_spell_change_ARG_to = 'Change\ "%s"\ to'
	endif
	s:changeitem = printf(g:menutrans_spell_change_ARG_to, escape(w, ' .'))
	s:fromword = w
	var pri = 1
	for sug in s:suglist
	  exe 'anoremenu 1.5.' .. pri .. ' PopUp.' .. s:changeitem .. '.' .. escape(sug, ' .')
		\ .. ' :call <SID>SpellReplace(' .. pri .. ')<CR>'
	  pri += 1
	endfor

	if !exists("g:menutrans_spell_add_ARG_to_word_list")
	  g:menutrans_spell_add_ARG_to_word_list = 'Add\ "%s"\ to\ Word\ List'
	endif
	s:additem = printf(g:menutrans_spell_add_ARG_to_word_list, escape(w, ' .'))
	exe 'anoremenu 1.6 PopUp.' .. s:additem .. ' :spellgood ' .. w .. '<CR>'

	if !exists("g:menutrans_spell_ignore_ARG")
	  g:menutrans_spell_ignore_ARG = 'Ignore\ "%s"'
	endif
	s:ignoreitem = printf(g:menutrans_spell_ignore_ARG, escape(w, ' .'))
	exe 'anoremenu 1.7 PopUp.' .. s:ignoreitem .. ' :spellgood! ' .. w .. '<CR>'

	anoremenu 1.8 PopUp.-SpellSep- :
      endif
    endif
    call cursor(0, curcol)	# put the cursor back where it was
  enddef

  def s:SpellReplace(n: number)
    var l = getline('.')
    # Move the cursor to the start of the word.
    call spellbadword()
    call setline('.', strpart(l, 0, col('.') - 1) .. s:suglist[n - 1]
	  \ .. strpart(l, col('.') + len(s:fromword) - 1))
  enddef

  def s:SpellDel()
    exe "aunmenu PopUp." .. s:changeitem
    exe "aunmenu PopUp." .. s:additem
    exe "aunmenu PopUp." .. s:ignoreitem
    aunmenu PopUp.-SpellSep-
    s:changeitem = ''
  enddef

  augroup SpellPopupMenu
    au! MenuPopup * call <SID>SpellPopup()
  augroup END
endif

" The GUI toolbar (for MS-Windows and GTK)
if has("toolbar")
  an 1.10 ToolBar.Open			:browse confirm e<CR>
  an <silent> 1.20 ToolBar.Save		:if expand("%") == ""<Bar>browse confirm w<Bar>else<Bar>confirm w<Bar>endif<CR>
  an 1.30 ToolBar.SaveAll		:browse confirm wa<CR>

  if has("printer")
    an 1.40   ToolBar.Print		:hardcopy<CR>
    vunmenu   ToolBar.Print
    vnoremenu ToolBar.Print		:hardcopy<CR>
  elseif has("unix")
    an 1.40   ToolBar.Print		:w !lpr<CR>
    vunmenu   ToolBar.Print
    vnoremenu ToolBar.Print		:w !lpr<CR>
  endif

  an 1.45 ToolBar.-sep1-		<Nop>
  an 1.50 ToolBar.Undo			u
  an 1.60 ToolBar.Redo			<C-R>

  an 1.65 ToolBar.-sep2-		<Nop>
  vnoremenu 1.70 ToolBar.Cut		"+x
  vnoremenu 1.80 ToolBar.Copy		"+y
  cnoremenu 1.80 ToolBar.Copy		<C-Y>
  nnoremenu 1.90 ToolBar.Paste		"+gP
  cnoremenu	 ToolBar.Paste		<C-R>+
  exe 'vnoremenu <script>	 ToolBar.Paste	' .. paste#paste_cmd['v']
  exe 'inoremenu <script>	 ToolBar.Paste	' .. paste#paste_cmd['i']

  if !has("gui_athena")
    an 1.95   ToolBar.-sep3-		<Nop>
    an 1.100  ToolBar.Replace		:promptrepl<CR>
    vunmenu   ToolBar.Replace
    vnoremenu ToolBar.Replace		y:promptrepl <C-R>=<SID>FixFText()<CR><CR>
    an 1.110  ToolBar.FindNext		n
    an 1.120  ToolBar.FindPrev		N
  endif

  an 1.215 ToolBar.-sep5-		<Nop>
  an <silent> 1.220 ToolBar.LoadSesn	:call <SID>LoadVimSesn()<CR>
  an <silent> 1.230 ToolBar.SaveSesn	:call <SID>SaveVimSesn()<CR>
  an 1.240 ToolBar.RunScript		:browse so<CR>

  an 1.245 ToolBar.-sep6-		<Nop>
  an 1.250 ToolBar.Make			:make<CR>
  an 1.270 ToolBar.RunCtags		:exe "!" .. g:ctags_command<CR>
  an 1.280 ToolBar.TagJump		g<C-]>

  an 1.295 ToolBar.-sep7-		<Nop>
  an 1.300 ToolBar.Help			:help<CR>
  an <silent> 1.310 ToolBar.FindHelp	:call <SID>Helpfind()<CR>

" Only set the tooltips here if not done in a language menu file
if exists("*Do_toolbar_tmenu")
  call Do_toolbar_tmenu()
else
  let did_toolbar_tmenu = 1
  tmenu ToolBar.Open		Open file
  tmenu ToolBar.Save		Save current file
  tmenu ToolBar.SaveAll		Save all files
  tmenu ToolBar.Print		Print
  tmenu ToolBar.Undo		Undo
  tmenu ToolBar.Redo		Redo
  tmenu ToolBar.Cut		Cut to clipboard
  tmenu ToolBar.Copy		Copy to clipboard
  tmenu ToolBar.Paste		Paste from Clipboard
  if !has("gui_athena")
    tmenu ToolBar.Replace	Find / Replace...
    tmenu ToolBar.FindNext	Find Next
    tmenu ToolBar.FindPrev	Find Previous
  endif
  tmenu ToolBar.LoadSesn	Choose a session to load
  tmenu ToolBar.SaveSesn	Save current session
  tmenu ToolBar.RunScript	Choose a Vim Script to run
  tmenu ToolBar.Make		Make current project (:make)
  tmenu ToolBar.RunCtags	Build tags in current directory tree (!ctags -R .)
  tmenu ToolBar.TagJump		Jump to tag under cursor
  tmenu ToolBar.Help		Vim Help
  tmenu ToolBar.FindHelp	Search Vim Help
endif

" Select a session to load; default to current session name if present
def s:LoadVimSesn()
  var name: string
  if strlen(v:this_session) > 0
    name = fnameescape(v:this_session)
  else
    name = "Session.vim"
  endif
  execute "browse so " .. name
enddef

" Select a session to save; default to current session name if present
def s:SaveVimSesn()
  if strlen(v:this_session) == 0
    v:this_session = "Session.vim"
  endif
  execute "browse mksession! " .. fnameescape(v:this_session)
enddef

endif

endif " !exists("did_install_default_menus")

" Define these items always, so that syntax can be switched on when it wasn't.
" But skip them when the Syntax menu was disabled by the user.
if !exists("did_install_syntax_menu")
  an 50.212 &Syntax.&Manual		:syn manual<CR>
  an 50.214 &Syntax.A&utomatic		:syn on<CR>
  an <silent> 50.216 &Syntax.On/Off\ for\ &This\ File :call <SID>SynOnOff()<CR>
  if !exists("*s:SynOnOff")
    def s:SynOnOff()
      if has("syntax_items")
	syn clear
      else
	if !exists("g:syntax_on")
	  syn manual
	endif
	set syn=ON
      endif
    enddef
  endif
endif


" Install the Syntax menu only when filetype.vim has been loaded or when
" manual syntax highlighting is enabled.
" Avoid installing the Syntax menu twice.
if (exists("did_load_filetypes") || exists("syntax_on"))
	\ && !exists("did_install_syntax_menu")
  let did_install_syntax_menu = 1

" Skip setting up the individual syntax selection menus unless
" do_syntax_sel_menu is defined (it takes quite a bit of time).
if exists("do_syntax_sel_menu")
  runtime! synmenu.vim
else
  an <silent> 50.10 &Syntax.&Show\ File\ Types\ in\ Menu	:let do_syntax_sel_menu = 1<Bar>runtime! synmenu.vim<Bar>aunmenu &Syntax.&Show\ File\ Types\ in\ Menu<CR>
  an 50.195 &Syntax.-SEP1-		<Nop>
endif

an 50.210 &Syntax.&Off			:syn off<CR>
an 50.700 &Syntax.-SEP3-		<Nop>
an 50.710 &Syntax.Co&lor\ Test		:sp $VIMRUNTIME/syntax/colortest.vim<Bar>so %<CR>
an 50.720 &Syntax.&Highlight\ Test	:runtime syntax/hitest.vim<CR>
an 50.730 &Syntax.&Convert\ to\ HTML	:runtime syntax/2html.vim<CR>

" Uncomment the next line to compile the functions early to find any mistakes
" defcompile

endif " !exists("did_install_syntax_menu")

" Restore the previous value of 'cpoptions'.
let &cpo = s:cpo_save
unlet s:cpo_save

" vim: set sw=2 :
