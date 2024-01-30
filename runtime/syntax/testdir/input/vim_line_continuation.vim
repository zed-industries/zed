" Vim line continuations with interspersed comments

function Foo(
      "\ param a
      \ a,
      "\ param b
      \ b,
      "\ param c
      \ c
      \)
  echomsg
	"\ start string
	\ $"
	"\ print a
	\ a = {a:a},
	"\ print b
	\ b = {a:b},
	"\ print c
	\ c = {a:c}
	"\ end string
	\"
endfunction

call Foo(
      "\ arg 1
      \ 11,
      "\ arg 2
      \ 22,
      "\ arg 3
      \ 33
      \)

let dict = #{
      "\ pair 1
      \ a: 1,
      "\ pair 2
      \ b: 2,
      "\ pair 3
      \ c: 3
      \}

let array = [
      "\ element 1
      \ 1,
      "\ element 2
      \ 2,
      "\ element 3
      \ 3
      \]
