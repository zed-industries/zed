" Test for getcwd()

func GetCwdInfo(win, tab)
  let tab_changed = 0
  let mod = ":t"
  if a:tab > 0 && a:tab != tabpagenr()
    let tab_changed = 1
    exec "tabnext " . a:tab
  endif
  let bufname = fnamemodify(bufname(winbufnr(a:win)), mod)
  if tab_changed
    tabprevious
  endif
  if a:win == 0 && a:tab == 0
    let dirname = fnamemodify(getcwd(), mod)
    let lflag = haslocaldir()
  elseif a:tab == 0
    let dirname = fnamemodify(getcwd(a:win), mod)
    let lflag = haslocaldir(a:win)
  else
    let dirname = fnamemodify(getcwd(a:win, a:tab), mod)
    let lflag = a:win->haslocaldir(a:tab)
  endif
  return bufname . ' ' . dirname . ' ' . lflag
endfunc

" Do all test in a separate window to avoid E211 when we recursively
" delete the Xtopdir directory during cleanup
func SetUp()
  set visualbell
  set nocp viminfo+=nviminfo

  " On windows a swapfile in Xtopdir prevents it from being cleaned up.
  set noswapfile

  " On windows a stale "Xtopdir" directory may exist, remove it so that
  " we start from a clean state.
  call delete("Xtopdir", "rf")
  new
  eval 'Xtopdir'->mkdir()
  cd Xtopdir
  let g:topdir = getcwd()
  call mkdir('Xcwdir1')
  call mkdir('Xcwdir2')
  call mkdir('Xcwdir3')
endfunction

let g:cwd=getcwd()
function TearDown()
  q
  call chdir(g:cwd)
  call delete("Xtopdir", "rf")
endfunction

function Test_GetCwd()
  new a
  new b
  new c
  3wincmd w
  lcd Xcwdir1
  call assert_equal("a Xcwdir1 1", GetCwdInfo(0, 0))
  call assert_equal(g:topdir, getcwd(-1))
  wincmd W
  call assert_equal("b Xtopdir 0", GetCwdInfo(0, 0))
  call assert_equal(g:topdir, getcwd(-1))
  wincmd W
  lcd Xcwdir3
  call assert_equal("c Xcwdir3 1", GetCwdInfo(0, 0))
  call assert_equal("a Xcwdir1 1", GetCwdInfo(bufwinnr("a"), 0))
  call assert_equal("b Xtopdir 0", GetCwdInfo(bufwinnr("b"), 0))
  call assert_equal("c Xcwdir3 1", GetCwdInfo(bufwinnr("c"), 0))
  call assert_equal(g:topdir, getcwd(-1))
  wincmd W
  call assert_equal("a Xcwdir1 1", GetCwdInfo(bufwinnr("a"), tabpagenr()))
  call assert_equal("b Xtopdir 0", GetCwdInfo(bufwinnr("b"), tabpagenr()))
  call assert_equal("c Xcwdir3 1", GetCwdInfo(bufwinnr("c"), tabpagenr()))
  call assert_equal(g:topdir, getcwd(-1))

  tabnew x
  new y
  new z
  3wincmd w
  call assert_equal("x Xtopdir 0", GetCwdInfo(0, 0))
  call assert_equal(g:topdir, getcwd(-1))
  wincmd W
  lcd Xcwdir2
  call assert_equal("y Xcwdir2 1", GetCwdInfo(0, 0))
  call assert_equal(g:topdir, getcwd(-1))
  wincmd W
  lcd Xcwdir3
  call assert_equal("z Xcwdir3 1", GetCwdInfo(0, 0))
  call assert_equal("x Xtopdir 0", GetCwdInfo(bufwinnr("x"), 0))
  call assert_equal("y Xcwdir2 1", GetCwdInfo(bufwinnr("y"), 0))
  call assert_equal("z Xcwdir3 1", GetCwdInfo(bufwinnr("z"), 0))
  call assert_equal(g:topdir, getcwd(-1))
  let tp_nr = tabpagenr()
  tabrewind
  call assert_equal("x Xtopdir 0", GetCwdInfo(3, tp_nr))
  call assert_equal("y Xcwdir2 1", GetCwdInfo(2, tp_nr))
  call assert_equal("z Xcwdir3 1", GetCwdInfo(1, tp_nr))
  call assert_equal(g:topdir, getcwd(-1))
  " Non existing windows and tab pages
  call assert_equal('', getcwd(100))
  call assert_equal(0, haslocaldir(100))
  call assert_equal('', getcwd(10, 1))
  call assert_equal(0, haslocaldir(10, 1))
  call assert_equal('', getcwd(1, 5))
  call assert_equal(0, haslocaldir(1, 5))
  call assert_fails('call getcwd([])', 'E745:')
  call assert_fails('call getcwd(1, [])', 'E745:')
  call assert_fails('call haslocaldir([])', 'E745:')
  call assert_fails('call haslocaldir(1, [])', 'E745:')
endfunc

function Test_GetCwd_lcd_shellslash()
  new
  let root = fnamemodify('/', ':p')
  exe 'lcd '.root
  let cwd = getcwd()
  if !exists('+shellslash') || &shellslash
    call assert_equal(cwd[-1:], '/')
  else
    call assert_equal(cwd[-1:], '\')
  endif
endfunc

" Test for :tcd
function Test_Tab_Local_Cwd()
  enew | only | tabonly

  call mkdir('Xtabdir1')
  call mkdir('Xtabdir2')
  call mkdir('Xwindir1')
  call mkdir('Xwindir2')
  call mkdir('Xwindir3')

  " Create three tabpages with three windows each
  edit a
  botright new b
  botright new c
  tabnew m
  botright new n
  botright new o
  tabnew x
  botright new y
  botright new z

  " Setup different directories for the tab pages and windows
  tabrewind
  1wincmd w
  lcd Xwindir1
  tabnext
  tcd Xtabdir1
  2wincmd w
  lcd ../Xwindir2
  tabnext
  tcd Xtabdir2
  3wincmd w
  lcd ../Xwindir3

  " Check the directories of various windows
  call assert_equal("a Xwindir1 1", GetCwdInfo(1, 1))
  call assert_equal("b Xtopdir 0", GetCwdInfo(2, 1))
  call assert_equal("c Xtopdir 0", GetCwdInfo(3, 1))
  call assert_equal("m Xtabdir1 2", GetCwdInfo(1, 2))
  call assert_equal("n Xwindir2 1", GetCwdInfo(2, 2))
  call assert_equal("o Xtabdir1 2", GetCwdInfo(3, 2))
  call assert_equal("x Xtabdir2 2", GetCwdInfo(1, 3))
  call assert_equal("y Xtabdir2 2", GetCwdInfo(2, 3))
  call assert_equal("z Xwindir3 1", GetCwdInfo(3, 3))

  " Check the tabpage directories
  call assert_equal('Xtopdir', fnamemodify(getcwd(-1, 1), ':t'))
  call assert_equal('Xtabdir1', fnamemodify(getcwd(-1, 2), ':t'))
  call assert_equal('Xtabdir2', fnamemodify(getcwd(-1, 3), ':t'))
  call assert_equal('', fnamemodify(getcwd(-1, 4), ':t'))

  " Jump to different windows in the tab pages and check the current directory
  tabrewind | 1wincmd w
  call assert_equal('Xwindir1', fnamemodify(getcwd(), ':t'))
  call assert_equal('Xwindir1', fnamemodify(getcwd(0), ':t'))
  call assert_equal('Xwindir1', fnamemodify(getcwd(0, 0), ':t'))
  call assert_true(haslocaldir(0))
  call assert_equal(0, haslocaldir(-1, 0))
  call assert_equal('Xtopdir', fnamemodify(getcwd(-1, 0), ':t'))
  call assert_equal(g:topdir, getcwd(-1))
  2wincmd w
  call assert_equal('Xtopdir', fnamemodify(getcwd(), ':t'))
  call assert_equal('Xtopdir', fnamemodify(getcwd(0), ':t'))
  call assert_equal('Xtopdir', fnamemodify(getcwd(0, 0), ':t'))
  call assert_false(haslocaldir(0))
  call assert_equal(0, haslocaldir(-1, 0))
  call assert_equal('Xtopdir', fnamemodify(getcwd(-1, 0), ':t'))
  call assert_equal(g:topdir, getcwd(-1))
  tabnext | 1wincmd w
  call assert_equal('Xtabdir1', fnamemodify(getcwd(), ':t'))
  call assert_equal('Xtabdir1', fnamemodify(getcwd(0), ':t'))
  call assert_equal('Xtabdir1', fnamemodify(getcwd(0, 0), ':t'))
  call assert_true(haslocaldir(0))
  call assert_equal(2, haslocaldir(-1, 0))
  call assert_equal('Xtabdir1', fnamemodify(getcwd(-1, 0), ':t'))
  call assert_equal(g:topdir, getcwd(-1))
  2wincmd w
  call assert_equal('Xwindir2', fnamemodify(getcwd(), ':t'))
  call assert_equal('Xwindir2', fnamemodify(getcwd(0), ':t'))
  call assert_equal('Xwindir2', fnamemodify(getcwd(0, 0), ':t'))
  call assert_true(haslocaldir(0))
  call assert_equal(2, haslocaldir(-1, 0))
  call assert_equal('Xtabdir1', fnamemodify(getcwd(-1, 0), ':t'))
  call assert_equal(g:topdir, getcwd(-1))
  tabnext | 1wincmd w
  call assert_equal('Xtabdir2', fnamemodify(getcwd(), ':t'))
  call assert_equal('Xtabdir2', fnamemodify(getcwd(0), ':t'))
  call assert_equal('Xtabdir2', fnamemodify(getcwd(0, 0), ':t'))
  call assert_true(haslocaldir(0))
  call assert_equal(2, haslocaldir(-1, 0))
  call assert_equal('Xtabdir2', fnamemodify(getcwd(-1, 0), ':t'))
  call assert_equal(g:topdir, getcwd(-1))
  3wincmd w
  call assert_equal('Xwindir3', fnamemodify(getcwd(), ':t'))
  call assert_equal('Xwindir3', fnamemodify(getcwd(0), ':t'))
  call assert_equal('Xwindir3', fnamemodify(getcwd(0, 0), ':t'))
  call assert_true(haslocaldir(0))
  call assert_equal(2, haslocaldir(-1, 0))
  call assert_equal('Xtabdir2', fnamemodify(getcwd(-1, 0), ':t'))
  call assert_equal(g:topdir, getcwd(-1))

  " A new tab page should inherit the directory of the current tab page
  tabrewind | 1wincmd w
  tabnew g
  call assert_equal("g Xwindir1 1", GetCwdInfo(0, 0))
  tabclose | tabrewind
  2wincmd w
  tabnew h
  call assert_equal("h Xtopdir 0", GetCwdInfo(0, 0))
  tabclose
  tabnext 2 | 1wincmd w
  tabnew j
  call assert_equal("j Xtabdir1 2", GetCwdInfo(0, 0))
  tabclose

  " Change the global directory for the first tab page
  tabrewind | 1wincmd w
  cd ../Xcwdir1
  call assert_equal("a Xcwdir1 0", GetCwdInfo(1, 1))
  call assert_equal("b Xcwdir1 0", GetCwdInfo(2, 1))
  call assert_equal("m Xtabdir1 2", GetCwdInfo(1, 2))
  call assert_equal("n Xwindir2 1", GetCwdInfo(2, 2))

  " Change the global directory for the second tab page
  tabnext | 1wincmd w
  cd ../Xcwdir3
  call assert_equal("m Xcwdir3 0", GetCwdInfo(1, 2))
  call assert_equal("n Xwindir2 1", GetCwdInfo(2, 2))
  call assert_equal("o Xcwdir3 0", GetCwdInfo(3, 2))

  " Change the tab-local directory for the third tab page
  tabnext | 1wincmd w
  cd ../Xcwdir1
  call assert_equal("x Xcwdir1 0", GetCwdInfo(1, 3))
  call assert_equal("y Xcwdir1 0", GetCwdInfo(2, 3))
  call assert_equal("z Xwindir3 1", GetCwdInfo(3, 3))

  enew | only | tabonly
  new
endfunc

" vim: shiftwidth=2 sts=2 expandtab
