vim9script
# This script tests a color scheme for some errors and lists potential errors.
# Load the scheme and source this script, like this:
#    :edit colors/desert.vim | :ru colors/tools/check_colors.vim

def Test_check_colors()
    const savedview = winsaveview()
    cursor(1, 1)

    # err is
    # {
    #    colors_name: "message",
    #    init: "message",
    #    background: "message",
    #    ....etc
    #    highlight: {
    #       'Normal': "Missing ...",
    #       'Conceal': "Missing ..."
    #       ....etc
    #    }
    # }
    var err: dict<any> = {}

    # 1) Check g:colors_name is existing
    if search('\<\%(g:\)\?colors_name\>', 'cnW') == 0
        err['colors_name'] = 'g:colors_name not set'
    else
        err['colors_name'] = 'OK'
    endif

    # 2) Check for some well-defined highlighting groups
    const hi_groups = [
        'ColorColumn',
        'Comment',
        'Conceal',
        'Constant',
	'CurSearch',
        'Cursor',
        'CursorColumn',
        'CursorLine',
        'CursorLineNr',
        'CursorLineFold',
        'CursorLineSign',
        'DiffAdd',
        'DiffChange',
        'DiffDelete',
        'DiffText',
        'Directory',
        'EndOfBuffer',
        'Error',
        'ErrorMsg',
        'FoldColumn',
        'Folded',
        'Identifier',
        'Ignore',
        'IncSearch',
        'LineNr',
        'LineNrAbove',
        'LineNrBelow',
        'MatchParen',
        'ModeMsg',
        'MoreMsg',
        'NonText',
        'Normal',
        'Pmenu',
        'PmenuSbar',
        'PmenuSel',
        'PmenuThumb',
        'PreProc',
        'Question',
        'QuickFixLine',
        'Search',
        'SignColumn',
        'Special',
        'SpecialKey',
        'SpellBad',
        'SpellCap',
        'SpellLocal',
        'SpellRare',
        'Statement',
        'StatusLine',
        'StatusLineNC',
        'StatusLineTerm',
        'StatusLineTermNC',
        'TabLine',
        'TabLineFill',
        'TabLineSel',
        'Title',
        'Todo',
        'ToolbarButton',
        'ToolbarLine',
        'Type',
        'Underlined',
        'VertSplit',
        'Visual',
        'VisualNOS',
        'WarningMsg',
        'WildMenu',
        'debugPC',
        'debugBreakpoint',
    ]
    var groups = {}
    for group in hi_groups
        if search('\c@suppress\s\+\<' .. group .. '\>', 'cnW') != 0
            # skip check, if the script contains a line like
            # @suppress Visual:
            continue
        endif
        if search('hi\%[ghlight]!\= \+link \+' .. group, 'cnW') != 0 # Linked group
            continue
        endif
        if search('hi\%[ghlight] \+\<' .. group .. '\>', 'cnW') == 0
            groups[group] = 'No highlight definition for ' .. group
            continue
        endif
        if search('hi\%[ghlight] \+\<' .. group .. '\>.*[bf]g=', 'cnW') == 0
            groups[group] = 'Missing foreground or background color for ' .. group
            continue
        endif
        if search('hi\%[ghlight] \+\<' .. group .. '\>.*guibg=', 'cnW') != 0
            && search('hi\%[ghlight] \+\<' .. group .. '\>.*ctermbg=', 'cnW') == 0
            && group != 'Cursor'
            groups[group] = 'Missing bg terminal color for ' .. group
            continue
        endif
        if search('hi\%[ghlight] \+\<' .. group .. '\>.*guifg=', 'cnW') == 0
            && group !~ '^Diff'
            groups[group] = 'Missing guifg definition for ' .. group
            continue
        endif
        if search('hi\%[ghlight] \+\<' .. group .. '\>.*ctermfg=', 'cnW') == 0
            && group !~ '^Diff'
            && group != 'Cursor'
            groups[group] = 'Missing ctermfg definition for ' .. group
            continue
        endif
        # do not check for background colors, they could be intentionally left out
        cursor(1, 1)
    endfor
    err['highlight'] = groups

    # 3) Check, that it does not set background highlighting
    # Doesn't ':hi Normal ctermfg=253 ctermfg=233' also set the background sometimes?
    const bg_set = '\(set\?\|setl\(ocal\)\?\) .*\(background\|bg\)=\(dark\|light\)'
    const bg_let = 'let \%([&]\%([lg]:\)\?\)\%(background\|bg\)\s*=\s*\([''"]\?\)\w\+\1'
    const bg_pat = '\%(' .. bg_set .. '\|' .. bg_let .. '\)'
    const line = search(bg_pat, 'cnW')
    if search(bg_pat, 'cnW') != 0
        exe ":" .. line
        if search('hi \U\w\+\s\+\S', 'cbnW') != 0
            err['background'] = 'Should not set background option after :hi statement'
        endif
    else
        err['background'] = 'OK'
    endif
    cursor(1, 1)

    # 4) Check, that t_Co is checked
    var pat = '[&]t_Co\s*[<>=]=\?\s*\d\+'
    if search(pat, 'ncW') == 0
        err['t_Co'] = 'Does not check terminal for capable colors'
    endif

    # 5) Initializes correctly, e.g. should have at least:
    # hi clear
    pat = '^\s*hi\%[ghlight]\s*clear\s*$'
    if search(pat, 'cnW') == 0
        err['init'] = 'No initialization'
    endif

    # 6) Does not use :syn on
    if search('syn\%[tax]\s\+on', 'cnW') != 0
        err['background'] = 'Should not issue :syn on'
    endif

    # 7) Normal should be defined first, not use reverse, fg or bg
    cursor(1, 1)
    pat = 'hi\%[light] \+\%(link\|clear\)\@!\w\+\>'
    search(pat, 'cW') # Look for the first hi def, skipping `hi link` and `hi clear`
    if getline('.') !~# '\m\<Normal\>'
        err['highlight']['Normal'] = 'Should be defined first'
    elseif getline('.') =~# '\m\%(=\%(fg\|bg\)\)'
        err['highlight']['Normal'] = "Should not use 'fg' or 'bg'"
    elseif getline('.') =~# '\m=\%(inv\|rev\)erse'
        err['highlight']['Normal'] = 'Should not use reverse mode'
    endif

    # 8) TODO: XXX: Check if g:terminal_ansi_colors are defined

    winrestview(savedview)
    g:err = err

    Result(err)
enddef


def Result(err: dict<any>)
    var do_groups: bool = v:false
    echohl Title | echomsg "---------------" | echohl Normal
    for key in sort(keys(err))
        if key == 'highlight'
            do_groups = !empty(err[key])
            continue
        else
            if err[key] !~ 'OK'
                echohl Title
            endif
            echomsg printf("%15s: %s", key, err[key])
            echohl Normal
        endif
    endfor
    echohl Title | echomsg "---------------" | echohl Normal
    if do_groups
        echohl Title | echomsg "Groups" | echohl Normal
        for v1 in sort(keys(err['highlight']))
            echomsg printf("%25s: %s", v1, err['highlight'][v1])
        endfor
    endif
enddef

Test_check_colors()
