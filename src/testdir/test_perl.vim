" Tests for Perl interface

source check.vim
source shared.vim
CheckFeature perl

" FIXME: RunTest don't see any error when Perl abort...
perl $SIG{__WARN__} = sub { die "Unexpected warnings from perl: @_" };

func Test_change_buffer()
  call setline(line('$'), ['1 line 1'])
  perl VIM::DoCommand("normal /^1\n")
  perl $curline = VIM::Eval("line('.')")
  perl $curbuf->Set($curline, "1 changed line 1")
  call assert_equal('1 changed line 1', getline('$'))
endfunc

func Test_evaluate_list()
  call setline(line('$'), ['2 line 2'])
  perl VIM::DoCommand("normal /^2\n")
  perl $curline = VIM::Eval("line('.')")
  let l = ["abc", "def"]
  perl << EOF
  $l = VIM::Eval("l");
  $curbuf->Append($curline, $l);
EOF
  normal j
  .perldo s|\n|/|g
  call assert_equal('abc/def/', getline('$'))
endfunc

funct Test_VIM_Blob()
  call assert_equal('0z',         perleval('VIM::Blob("")'))
  call assert_equal('0z31326162', 'VIM::Blob("12ab")'->perleval())
  call assert_equal('0z00010203', perleval('VIM::Blob("\x00\x01\x02\x03")'))
  call assert_equal('0z8081FEFF', perleval('VIM::Blob("\x80\x81\xfe\xff")'))
endfunc

func Test_buffer_Delete()
  new
  call setline(1, ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'])
  perl $curbuf->Delete(7)
  perl $curbuf->Delete(2, 5)
  perl $curbuf->Delete(10)
  call assert_equal(['a', 'f', 'h'],  getline(1, '$'))
  bwipe!
endfunc

func Test_buffer_Append()
  new
  perl $curbuf->Append(1, '1')
  perl $curbuf->Append(2, '2', '3', '4')
  perl @l = ('5' ..'7')
  perl $curbuf->Append(0, @l)
  call assert_equal(['5', '6', '7', '', '1', '2', '3', '4'], getline(1, '$'))

  perl $curbuf->Append(0)
  call assert_match('^Usage: VIBUF::Append(vimbuf, lnum, @lines) at .* line 1\.$',
        \           GetMessages()[-1])

  bwipe!
endfunc

func Test_buffer_Set()
  new
  call setline(1, ['1', '2', '3', '4', '5'])
  perl $curbuf->Set(2, 'a', 'b', 'c')
  perl $curbuf->Set(4, 'A', 'B', 'C')
  call assert_equal(['1', 'a', 'b', 'A', 'B'], getline(1, '$'))

  perl $curbuf->Set(0)
  call assert_match('^Usage: VIBUF::Set(vimbuf, lnum, @lines) at .* line 1\.$',
        \           GetMessages()[-1])

  bwipe!
endfunc

func Test_buffer_Get()
  new
  call setline(1, ['1', '2', '3', '4'])
  call assert_equal('2:3', perleval('join(":", $curbuf->Get(2, 3))'))
  bwipe!
endfunc

func Test_buffer_Count()
  new
  call setline(1, ['a', 'b', 'c'])
  call assert_equal(3, perleval('$curbuf->Count()'))
  bwipe!
endfunc

func Test_buffer_Name()
  new
  call assert_equal('', perleval('$curbuf->Name()'))
  bwipe!
  new Xfoo
  call assert_equal('Xfoo', perleval('$curbuf->Name()'))
  bwipe!
endfunc

func Test_buffer_Number()
  call assert_equal(bufnr('%'), perleval('$curbuf->Number()'))
endfunc

func Test_window_Cursor()
  new
  call setline(1, ['line1', 'line2'])
  perl $curwin->Cursor(2, 3)
  call assert_equal('2:3', perleval('join(":", $curwin->Cursor())'))
  " Col is numbered from 0 in Perl, and from 1 in Vim script.
  call assert_equal([0, 2, 4, 0], getpos('.'))
  bwipe!
endfunc

func Test_window_SetHeight()
  new
  perl $curwin->SetHeight(2)
  call assert_equal(2, winheight(0))
  bwipe!
endfunc

func Test_VIM_Windows()
  new
  " VIM::Windows() without argument in scalar and list context.
  perl $winnr = VIM::Windows()
  perl @winlist = VIM::Windows()
  perl $curbuf->Append(0, $winnr, scalar(@winlist))
  call assert_equal(['2', '2', ''], getline(1, '$'))

  " VIM::Windows() with window number argument.
  perl VIM::Windows(VIM::Eval('winnr()'))->Buffer()->Set(1, 'bar')
  call assert_equal('bar', getline(1))
  bwipe!
endfunc

func Test_VIM_Buffers()
  new Xbar
  " VIM::Buffers() without argument in scalar and list context.
  perl $nbuf = VIM::Buffers()
  perl @buflist = VIM::Buffers()

  " VIM::Buffers() with argument.
  perl $mybuf = (VIM::Buffers('Xbar'))[0]
  perl $mybuf->Append(0, $nbuf, scalar(@buflist))
  call assert_equal(['2', '2', ''], getline(1, '$'))
  bwipe!
endfunc

func <SID>catch_peval(expr)
  try
    call perleval(a:expr)
  catch
    return v:exception
  endtry
  call assert_report('no exception for `perleval("'.a:expr.'")`')
  return ''
endfunc

func Test_perleval()
  call assert_false(perleval('undef'))

  " scalar
  call assert_equal(0, perleval('0'))
  call assert_equal(2, perleval('2'))
  call assert_equal(-2, perleval('-2'))
  call assert_equal(2.5, perleval('2.5'))

  sandbox call assert_equal(2, perleval('2'))

  call assert_equal('abc', perleval('"abc"'))
  call assert_equal("abc\ndef", perleval('"abc\0def"'))

  " ref
  call assert_equal([], perleval('[]'))
  call assert_equal(['word', 42, [42],{}], perleval('["word", 42, [42], {}]'))

  call assert_equal({}, perleval('{}'))
  call assert_equal({'foo': 'bar'}, perleval('{foo => "bar"}'))

  perl our %h; our @a;
  let a = perleval('[\(%h, %h, @a, @a)]')
  call assert_true((a[0] is a[1]))
  call assert_true((a[2] is a[3]))
  perl undef %h; undef @a;

  call assert_true(<SID>catch_peval('{"" , 0}') =~ 'Malformed key Dictionary')
  call assert_true(<SID>catch_peval('{"\0" , 0}') =~ 'Malformed key Dictionary')
  call assert_true(<SID>catch_peval('{"foo\0bar" , 0}') =~ 'Malformed key Dictionary')

  call assert_equal('*VIM', perleval('"*VIM"'))
  call assert_true(perleval('\\0') =~ 'SCALAR(0x\x\+)')

  " typeglob
  call assert_equal('*main::STDOUT', perleval('*STDOUT'))
'
  call perleval("++-$foo")
  let messages = split(execute('message'), "\n")
  call assert_match("Can't modify negation", messages[-1])
endfunc

func Test_perldo()
  new
  " :perldo in empty buffer does nothing.
  perldo ++$counter
  call assert_equal(0, perleval("$counter"))

  sp __TEST__
  exe 'read ' g:testname
  perldo s/perl/vieux_chameau/g
  1
  call assert_false(search('\Cperl'))
  bw!

  new

  " Check deleting lines does not trigger ml_get error.
  call setline(1, ['one', 'two', 'three'])
  perldo VIM::DoCommand("%d_")
  call assert_equal([''], getline(1, '$'))

  call setline(1, ['one', 'two', 'three'])
  perldo VIM::DoCommand("1,2d_")
  call assert_equal(['three'], getline(1, '$'))

  call setline(1, ['one', 'two', 'three'])
  perldo VIM::DoCommand("2,3d_"); $_ = "REPLACED"
  call assert_equal(['REPLACED'], getline(1, '$'))

  call setline(1, ['one', 'two', 'three'])
  2,3perldo VIM::DoCommand("1,2d_"); $_ = "REPLACED"
  call assert_equal(['three'], getline(1, '$'))

  bwipe!

  " Check a Perl expression which gives an error.
  new
  call setline(1, 'one')
  perldo 1/0
  call assert_match('^Illegal division by zero at .* line 1\.$', GetMessages()[-1])
  bwipe!

  " Check switching to another buffer does not trigger ml_get error.
  new
  let wincount = winnr('$')
  call setline(1, ['one', 'two', 'three'])
  perldo VIM::DoCommand("new")
  call assert_equal(wincount + 1, winnr('$'))
  %bwipe!
endfunc

func Test_VIM_package()
  perl VIM::DoCommand('let l:var = "foo"')
  call assert_equal(l:var, 'foo')

  set noet
  perl VIM::SetOption('et')
  call assert_true(&et)
endfunc

func Test_stdio()
  redir =>l:out
  perl << trim EOF
    VIM::Msg("VIM::Msg");
    VIM::Msg("VIM::Msg Error", "Error");
    print "STDOUT";
    print STDERR "STDERR";
  EOF
  redir END
  call assert_equal(['VIM::Msg', 'VIM::Msg Error', 'STDOUT', 'STDERR'], split(l:out, "\n"))
endfunc

" Run first to get a clean namespace
func Test_000_SvREFCNT()
  for i in range(8)
    exec 'new X'.i
  endfor
  new t
  perl <<--perl
#line 5 "Test_000_SvREFCNT()"
  my ($b, $w);

  my $num = 0;
  for ( 0 .. 100 ) {
      if ( ++$num >= 8 ) { $num = 0 }
      VIM::DoCommand("buffer X$num");
      $b = $curbuf;
  }

  VIM::DoCommand("buffer t");

  $b = $curbuf      for 0 .. 100;
  $w = $curwin      for 0 .. 100;
  () = VIM::Buffers for 0 .. 100;
  () = VIM::Windows for 0 .. 100;

  VIM::DoCommand('bw! t');
  if (exists &Internals::SvREFCNT) {
      my $cb = Internals::SvREFCNT($$b);
      my $cw = Internals::SvREFCNT($$w);
      VIM::Eval("assert_equal(2, $cb, 'T1')");
      VIM::Eval("assert_equal(2, $cw, 'T2')");
      my $strongref;
      foreach ( VIM::Buffers, VIM::Windows ) {
	  VIM::DoCommand("%bw!");
	  my $c = Internals::SvREFCNT($_);
	  VIM::Eval("assert_equal(2, $c, 'T3')");
	  $c = Internals::SvREFCNT($$_);
	  next if $c == 2 && !$strongref++;
	  VIM::Eval("assert_equal(1, $c, 'T4')");
      }
      $cb = Internals::SvREFCNT($$curbuf);
      $cw = Internals::SvREFCNT($$curwin);
      VIM::Eval("assert_equal(3, $cb, 'T5')");
      VIM::Eval("assert_equal(3, $cw, 'T6')");
  }
  VIM::Eval("assert_false($$b)");
  VIM::Eval("assert_false($$w)");
--perl
  %bw!
endfunc

" This caused a memory error before issue #10386 was fixed
func Test_stack_usage_fix()
   let script =<< CODE
     " This will grow Perl's stack in first invocation
     eval [0, 0]->map({ -> perleval("push@_,0..4096;0") })
     q!
CODE
   call RunVim([], script, '')
endfunc

func Test_set_cursor()
  " Check that setting the cursor position works.
  new
  call setline(1, ['first line', 'second line'])
  normal gg
  perldo $curwin->Cursor(1, 5)
  call assert_equal([1, 6], [line('.'), col('.')])

  " Check that movement after setting cursor position keeps current column.
  normal j
  call assert_equal([2, 6], [line('.'), col('.')])
endfunc

" Test for various heredoc syntax
func Test_perl_heredoc()
  perl << END
VIM::DoCommand('let s = "A"')
END
  perl <<
VIM::DoCommand('let s ..= "B"')
.
  perl << trim END
    VIM::DoCommand('let s ..= "C"')
  END
  perl << trim
    VIM::DoCommand('let s ..= "D"')
  .
  perl << trim eof
    VIM::DoCommand('let s ..= "E"')
  eof
  call assert_equal('ABCDE', s)
endfunc

func Test_perl_in_sandbox()
  sandbox perl print 'test'
  let messages = split(execute('message'), "\n")
  call assert_match("'print' trapped by operation mask", messages[-1])
endfunc

" vim: shiftwidth=2 sts=2 expandtab
