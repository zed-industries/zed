" Test for benchmarking the RE engine

source check.vim
CheckFeature reltime

func Measure(file, pattern, arg)
  for re in range(3)
    let sstart = reltime()
    let before = ['set re=' .. re]
    let after = ['call search("' .. escape(a:pattern, '\\') .. '", "", "", 10000)']
    let after += ['quit!']
    let args = empty(a:arg) ? '' : a:arg .. ' ' .. a:file
    call RunVim(before, after, args)
    let s = 'file: ' .. a:file .. ', re: ' .. re ..
          \ ', time: ' .. reltimestr(reltime(sstart))
    call writefile([s], 'benchmark.out', "a")
  endfor
endfunc

func Test_Regex_Benchmark()
  call Measure('samples/re.freeze.txt', '\s\+\%#\@<!$', '+5')
endfunc

" vim: shiftwidth=2 sts=2 expandtab
