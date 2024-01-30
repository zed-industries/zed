vim9script

# Utility functions for testing vim9 script

# Use a different file name for each run.
var sequence = 1

# Check that "lines" inside a ":def" function has no error when called.
export func CheckDefSuccess(lines)
  let cwd = getcwd()
  let fname = 'XdefSuccess' .. s:sequence
  let s:sequence += 1
  call writefile(['def Func()'] + a:lines + ['enddef', 'defcompile'], fname)
  try
    exe 'so ' .. fname
    call Func()
  finally
    call chdir(cwd)
    call delete(fname)
    delfunc! Func
  endtry
endfunc

# Check that "lines" inside a ":def" function has no error when compiled.
export func CheckDefCompileSuccess(lines)
  let fname = 'XdefSuccess' .. s:sequence
  let s:sequence += 1
  call writefile(['def Func()'] + a:lines + ['enddef', 'defcompile'], fname)
  try
    exe 'so ' .. fname
  finally
    call delete(fname)
    delfunc! Func
  endtry
endfunc

# Check that "lines" inside ":def" results in an "error" message.
# If "lnum" is given check that the error is reported for this line.
# Add a line before and after to make it less likely that the line number is
# accidentally correct.
export func CheckDefFailure(lines, error, lnum = -3)
  let cwd = getcwd()
  let fname = 'XdefFailure' .. s:sequence
  let s:sequence += 1
  call writefile(['def Func()', '# comment'] + a:lines + ['#comment', 'enddef', 'defcompile'], fname)
  try
    call assert_fails('so ' .. fname, a:error, a:lines, a:lnum + 1)
  finally
    call chdir(cwd)
    call delete(fname)
    delfunc! Func
  endtry
endfunc

# Check that "lines" inside ":def" results in an "error" message when executed.
# If "lnum" is given check that the error is reported for this line.
# Add a line before and after to make it less likely that the line number is
# accidentally correct.
export func CheckDefExecFailure(lines, error, lnum = -3)
  let cwd = getcwd()
  let fname = 'XdefExecFailure' .. s:sequence
  let s:sequence += 1
  call writefile(['def Func()', '# comment'] + a:lines + ['#comment', 'enddef'], fname)
  try
    exe 'so ' .. fname
    call assert_fails('call Func()', a:error, a:lines, a:lnum + 1)
  finally
    call chdir(cwd)
    call delete(fname)
    delfunc! Func
  endtry
endfunc

export def CheckScriptFailure(lines: list<string>, error: string, lnum = -3)
  var cwd = getcwd()
  var fname = 'XScriptFailure' .. sequence
  sequence += 1
  writefile(lines, fname)
  try
    assert_fails('so ' .. fname, error, lines, lnum)
  finally
    chdir(cwd)
    delete(fname)
  endtry
enddef

export def CheckScriptFailureList(lines: list<string>, errors: list<string>, lnum = -3)
  var cwd = getcwd()
  var fname = 'XScriptFailure' .. sequence
  sequence += 1
  writefile(lines, fname)
  try
    assert_fails('so ' .. fname, errors, lines, lnum)
  finally
    chdir(cwd)
    delete(fname)
  endtry
enddef

export def CheckScriptSuccess(lines: list<string>)
  var cwd = getcwd()
  var fname = 'XScriptSuccess' .. sequence
  sequence += 1
  writefile(lines, fname)
  try
    exe 'so ' .. fname
  finally
    chdir(cwd)
    delete(fname)
  endtry
enddef

# :source a list of "lines" and check whether it fails with "error"
export def CheckSourceFailure(lines: list<string>, error: string, lnum = -3)
  new
  setline(1, lines)
  try
    assert_fails('source', error, lines, lnum)
  finally
    bw!
  endtry
enddef

# :source a list of "lines" and check whether it fails with the list of
# "errors"
export def CheckSourceFailureList(lines: list<string>, errors: list<string>, lnum = -3)
  new
  setline(1, lines)
  try
    assert_fails('source', errors, lines, lnum)
  finally
    bw!
  endtry
enddef

# :source a list of "lines" and check whether it succeeds
export def CheckSourceSuccess(lines: list<string>)
  new
  setline(1, lines)
  try
    :source
  finally
    bw!
  endtry
enddef

export def CheckDefAndScriptSuccess(lines: list<string>)
  CheckDefSuccess(lines)
  CheckScriptSuccess(['vim9script'] + lines)
enddef

# Check that a command fails when used in a :def function and when used in
# Vim9 script.
# When "error" is a string, both with the same error.
# When "error" is a list, the :def function fails with "error[0]" , the script
# fails with "error[1]".
export def CheckDefAndScriptFailure(lines: list<string>, error: any, lnum = -3)
  var errorDef: string
  var errorScript: string
  if type(error) == v:t_string
    errorDef = error
    errorScript = error
  elseif type(error) == v:t_list && len(error) == 2
    errorDef = error[0]
    errorScript = error[1]
  else
    echoerr 'error argument must be a string or a list with two items'
    return
  endif
  CheckDefFailure(lines, errorDef, lnum)
  CheckScriptFailure(['vim9script'] + lines, errorScript, lnum + 1)
enddef

# Check that a command fails when executed in a :def function and when used in
# Vim9 script.
# When "error" is a string, both with the same error.
# When "error" is a list, the :def function fails with "error[0]" , the script
# fails with "error[1]".
export def CheckDefExecAndScriptFailure(lines: list<string>, error: any, lnum = -3)
  var errorDef: string
  var errorScript: string
  if type(error) == v:t_string
    errorDef = error
    errorScript = error
  elseif type(error) == v:t_list && len(error) == 2
    errorDef = error[0]
    errorScript = error[1]
  else
    echoerr 'error argument must be a string or a list with two items'
    return
  endif
  CheckDefExecFailure(lines, errorDef, lnum)
  CheckScriptFailure(['vim9script'] + lines, errorScript, lnum + 1)
enddef


# Check that "lines" inside a legacy function has no error.
export func CheckLegacySuccess(lines)
  let cwd = getcwd()
  let fname = 'XlegacySuccess' .. s:sequence
  let s:sequence += 1
  call writefile(['func Func()'] + a:lines + ['endfunc'], fname)
  try
    exe 'so ' .. fname
    call Func()
  finally
    delfunc! Func
    call chdir(cwd)
    call delete(fname)
  endtry
endfunc

# Check that "lines" inside a legacy function results in the expected error
export func CheckLegacyFailure(lines, error)
  let cwd = getcwd()
  let fname = 'XlegacyFails' .. s:sequence
  let s:sequence += 1
  call writefile(['func Func()'] + a:lines + ['endfunc', 'call Func()'], fname)
  try
    call assert_fails('so ' .. fname, a:error)
  finally
    delfunc! Func
    call chdir(cwd)
    call delete(fname)
  endtry
endfunc

# Execute "lines" in a legacy function, translated as in
# CheckLegacyAndVim9Success()
export def CheckTransLegacySuccess(lines: list<string>)
  var legacylines = lines->mapnew((_, v) =>
				v->substitute('\<VAR\>', 'let', 'g')
				 ->substitute('\<LET\>', 'let', 'g')
				 ->substitute('\<LSTART\>', '{', 'g')
				 ->substitute('\<LMIDDLE\>', '->', 'g')
				 ->substitute('\<LEND\>', '}', 'g')
				 ->substitute('\<TRUE\>', '1', 'g')
				 ->substitute('\<FALSE\>', '0', 'g')
				 ->substitute('#"', ' "', 'g'))
  CheckLegacySuccess(legacylines)
enddef

export def Vim9Trans(lines: list<string>): list<string>
  return lines->mapnew((_, v) =>
	    v->substitute('\<VAR\>', 'var', 'g')
	    ->substitute('\<LET ', '', 'g')
	    ->substitute('\<LSTART\>', '(', 'g')
	    ->substitute('\<LMIDDLE\>', ') =>', 'g')
	    ->substitute(' *\<LEND\> *', '', 'g')
	    ->substitute('\<TRUE\>', 'true', 'g')
	    ->substitute('\<FALSE\>', 'false', 'g'))
enddef

# Execute "lines" in a :def function, translated as in
# CheckLegacyAndVim9Success()
export def CheckTransDefSuccess(lines: list<string>)
  CheckDefSuccess(Vim9Trans(lines))
enddef

# Execute "lines" in a Vim9 script, translated as in
# CheckLegacyAndVim9Success()
export def CheckTransVim9Success(lines: list<string>)
  CheckScriptSuccess(['vim9script'] + Vim9Trans(lines))
enddef

# Execute "lines" in a legacy function, :def function and Vim9 script.
# Use 'VAR' for a declaration.
# Use 'LET' for an assignment
# Use ' #"' for a comment
# Use LSTART arg LMIDDLE expr LEND for lambda
# Use 'TRUE' for 1 in legacy, true in Vim9
# Use 'FALSE' for 0 in legacy, false in Vim9
export def CheckLegacyAndVim9Success(lines: list<string>)
  CheckTransLegacySuccess(lines)
  CheckTransDefSuccess(lines)
  CheckTransVim9Success(lines)
enddef

# Execute "lines" in a legacy function, :def function and Vim9 script.
# Use 'VAR' for a declaration.
# Use 'LET' for an assignment
# Use ' #"' for a comment
export def CheckLegacyAndVim9Failure(lines: list<string>, error: any)
  var legacyError: string
  var defError: string
  var scriptError: string

  if type(error) == type('string')
    legacyError = error
    defError = error
    scriptError = error
  else
    legacyError = error[0]
    defError = error[1]
    scriptError = error[2]
  endif

  var legacylines = lines->mapnew((_, v) =>
				v->substitute('\<VAR\>', 'let', 'g')
				 ->substitute('\<LET\>', 'let', 'g')
				 ->substitute('#"', ' "', 'g'))
  CheckLegacyFailure(legacylines, legacyError)

  var vim9lines = lines->mapnew((_, v) =>
				v->substitute('\<VAR\>', 'var', 'g')
				 ->substitute('\<LET ', '', 'g'))
  CheckDefExecFailure(vim9lines, defError)
  CheckScriptFailure(['vim9script'] + vim9lines, scriptError)
enddef
