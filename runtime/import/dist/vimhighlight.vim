vim9script

# Maintainer:	github user lacygoill
# Last Change:	2023 Mar 08

# Init {{{1

const LINK: string = '->'

# Interface {{{1
export def HighlightTest() # {{{2
    # Open a new window if the current one isn't empty
    if line('$') != 1 || getline(1) != ''
        new
    endif

    edit Highlight\ test

    # `:help scratch-buffer`
    &l:bufhidden = 'hide'
    &l:buftype = 'nofile'
    &l:swapfile = false

    var report: list<string> =<< trim END
        Highlighting groups for various occasions
        -----------------------------------------
    END

    var various_groups: list<string> = GetVariousGroups()
        ->filter((_, group: string): bool => group->hlexists() && !group->IsCleared())
        ->sort()
        ->uniq()

    report->extend(various_groups->FollowChains())

    var language_section: list<string> =<< trim END

        Highlighting groups for language syntaxes
        -----------------------------------------
    END
    report->extend(language_section)

    var syntax_groups: list<string> = getcompletion('', 'highlight')
        ->filter((_, group: string): bool =>
            various_groups->index(group) == -1
            && !group->IsCleared()
            && group !~ '^HighlightTest')

    # put the report
    report
        ->extend(syntax_groups->FollowChains())
        ->setline(1)

    # highlight the group names
    execute $'silent! global /^\w\+\%(\%(\s*{LINK}\s*\)\w\+\)*$/ Highlight({bufnr('%')})'

    cursor(1, 1)
enddef
# }}}1
# Core {{{1
def Highlight(buf: number) # {{{2
    var lnum: number = line('.')
    for group: string in getline('.')->split($'\s*{LINK}\s*')
        silent! prop_type_add($'highlight-test-{group}', {
            bufnr: buf,
            highlight: group,
            combine: false,
        })
        prop_add(lnum, col('.'), {
            length: group->strlen(),
            type: $'highlight-test-{group}'
        })
        search('\<\w\+\>', '', lnum)
    endfor
enddef
# }}}1
# Util {{{1
def IsCleared(name: string): bool # {{{2
    return name
        ->hlget()
        ->get(0, {})
        ->get('cleared')
enddef

def FollowChains(groups: list<string>): list<string> # {{{2
    # A group might be linked to another, which itself might be linked...
    # We want the whole chain, for every group.
    var chains: list<string>
    for group: string in groups
        var target: string = group->LinksTo()
        var chain: string = group
        while !target->empty()
            chain ..= $' {LINK} {target}'
            target = target->LinksTo()
        endwhile
        var a_link_is_cleared: bool = chain
            ->split($'\s*{LINK}\s*')
            ->indexof((_, g: string): bool => g->IsCleared()) >= 0
        if a_link_is_cleared
            continue
        endif
        chains->add(chain)
    endfor
    return chains
enddef

def LinksTo(group: string): string # {{{2
    return group
        ->hlget()
        ->get(0, {})
        ->get('linksto', '')
enddef

def GetVariousGroups(): list<string> # {{{2
    return getcompletion('hl-', 'help')
        ->filter((_, helptag: string): bool => helptag =~ '^hl-\w\+$')
        ->map((_, helptag: string) => helptag->substitute('^hl-', '', ''))
        ->extend(range(1, 9)->map((_, n: number) => $'User{n}'))
enddef
