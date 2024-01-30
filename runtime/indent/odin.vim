vim9script

# Vim indent plugin file
# Language: Odin
# Maintainer: Maxim Kim <habamax@gmail.com>
# Website: https://github.com/habamax/vim-odin
# Last Change: 2024-01-15

if exists("b:did_indent")
    finish
endif
b:did_indent = 1

b:undo_indent = 'setlocal cindent< cinoptions< cinkeys< indentexpr<'

setlocal cindent
setlocal cinoptions=L0,m1,(s,j1,J1,l1,+0,:0,#1
setlocal cinkeys=0{,0},0),0],!^F,:,o,O

setlocal indentexpr=GetOdinIndent(v:lnum)

def PrevLine(lnum: number): number
    var plnum = lnum - 1
    var pline: string
    while plnum > 1
        plnum = prevnonblank(plnum)
        pline = getline(plnum)
        # XXX: take into account nested multiline /* /* */ */ comments
        if pline =~ '\*/\s*$'
            while getline(plnum) !~ '/\*' && plnum > 1
                plnum -= 1
            endwhile
            if getline(plnum) =~ '^\s*/\*'
                plnum -= 1
            else
                break
            endif
        elseif pline =~ '^\s*//'
            plnum -= 1
        else
            break
        endif
    endwhile
    return plnum
enddef

def GetOdinIndent(lnum: number): number
    var plnum = PrevLine(lnum)
    var pline = getline(plnum)
    var pindent = indent(plnum)
    # workaround of cindent "hang"
    # if the previous line looks like:
    # : #{}
    # : #whatever{whateverelse}
    # and variations where : # { } are in the string
    # cindent(lnum) hangs
    if pline =~ ':\s\+#.*{.*}'
        return pindent
    endif

    var indent = cindent(lnum)
    var line = getline(lnum)

    if line =~ '^\s*#\k\+'
        if pline =~ '[{:]\s*$'
            indent = pindent + shiftwidth()
        else
            indent = pindent
        endif
    elseif pline =~ 'switch\s.*{\s*$'
        indent = pindent
    elseif pline =~ 'case\s*.*,\s*\(//.*\)\?$' # https://github.com/habamax/vim-odin/issues/8
        indent = pindent + matchstr(pline, 'case\s*')->strcharlen()
    elseif line =~ '^\s*case\s\+.*,\s*$'
        indent = pindent - shiftwidth()
    elseif pline =~ 'case\s*.*:\s*\(//.*\)\?$'
        if line !~ '^\s*}\s*$' && line !~ '^\s*case[[:space:]:]'
            indent = pindent + shiftwidth()
        endif
    elseif pline =~ '^\s*@.*' && line !~ '^\s*}'
        indent = pindent
    elseif pline =~ ':[:=].*}\s*$'
        indent = pindent
    elseif pline =~ '^\s*}\s*$'
        if line !~ '^\s*}' && line !~ 'case\s*.*:\s*$'
            indent = pindent
        else
            indent = pindent - shiftwidth()
        endif
    elseif pline =~ '\S:\s*$'
        # looking up for a case something,
        #                       whatever,
        #                       anything:
        # ... 20 lines before
        for idx in range(plnum - 1, plnum - 21, -1)
            if plnum < 1
                break
            endif
            if getline(idx) =~ '^\s*case\s.*,\s*$'
                indent = indent(idx) + shiftwidth()
                break
            endif
        endfor
    elseif pline =~ '{[^{]*}\s*$' && line !~ '^\s*[})]\s*$' # https://github.com/habamax/vim-odin/issues/2
        indent = pindent
    elseif pline =~ '^\s*}\s*$' # https://github.com/habamax/vim-odin/issues/3
        # Find line with opening { and check if there is a label:
        # If there is, return indent of the closing }
        cursor(plnum, 1)
        silent normal! %
        var brlnum = line('.')
        var brline = getline('.')
        if plnum != brlnum && (brline =~ '^\s*\k\+:\s\+for' || brline =~ '^\s*\k\+\s*:=')
            indent = pindent
        endif
    endif

    return indent
enddef
