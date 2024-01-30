" Vim variable highlighting

" :let

let foo = expr

let foo[0] = expr

let foo[1:2] = expr
let foo[:2] = expr
let foo[1:] = expr
let foo[:] = expr

let foo += expr
let foo -= expr
let foo *= expr
let foo /= expr
let foo %= expr
let foo .= expr
let foo ..= expr

let $FOO = expr
let $FOO .= expr

let @f = expr
let @f .= expr

let &foo = expr

let &t_k1 = "\<Esc>[234;"

let &foo .= expr

let &foo += expr
let &foo -= expr

let &l:foo = expr

let &l:foo .= expr
let &l:foo += expr
let &l:foo -= expr

let &g:foo = expr

let &g:foo .= expr
let &g:foo += expr
let &g:foo -= expr

let [foo, bar] = expr
let [foo,
      \ bar] = expr

let [foo, bar] .= expr
let [foo, bar] += expr
let [foo, bar] -= expr

let [foo, bar; baz] = expr
let [foo,
      \ bar;
      \ baz] = expr

let [foo, bar; baz] .= expr
let [foo, bar; baz] += expr
let [foo, bar; baz] -= expr

let foo =<< END
...
END
let foo =<< trim END
...
END
let foo =<< eval END
...
END
let foo =<< trim eval END
...
END
let foo =<< eval trim END
...
END

let foo
let foo bar

" :unlet

unlet foo
unlet foo bar
unlet foo
      \ bar

unlet! foo
unlet! foo bar
unlet! foo
      \ bar

unlet $FOO
unlet! $FOO

unlet list[3]
unlet list[3:]
unlet dict['two']
unlet dict.two

" :const

const foo = expr

const [foo, bar] = expr

const [foo, bar; baz] = expr

const foo =<< END
...
END
const foo =<< trim END
...
END
const foo =<< eval END
...
END
const foo =<< trim eval END
...
END
const foo =<< eval trim END
...
END

const foo
const foo bar

" :for

for foo in expr
endfor

for [foo, bar] in expr
endfor
