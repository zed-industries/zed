vim9script

# Maintainer: Maxim Kim <habamax@gmail.com>
# Last update: 2023-12-10
#
# Set of functions to format/beautify JSON data structures.
#
# Could be used to reformat a minified json in a buffer (put it into ~/.vim/ftplugin/json.vim):
#    import autoload 'dist/json.vim'
#    setl formatexpr=json.FormatExpr()
#
# Or to get a formatted string out of vim's dict/list/string:
#    vim9script
#    import autoload 'dist/json.vim'
#    echo json.Format({
#      "widget": { "debug": "on", "window": { "title": "Sample \"Konfabulator\" Widget",
#          "name": "main_window", "width": 500, "height": 500
#        },
#        "image": { "src": "Images/Sun.png", "name": "sun1", "hOffset": 250,
#          "vOffset": 250, "alignment": "center" },
#        "text": { "data": "Click Here", "size": 36, "style": "bold", "name": "text1",
#          "hOffset": 250, "vOffset": 100, "alignment": "center",
#          "onMouseUp": "sun1.opacity = (sun1.opacity / 100) * 90;" } }
#    })
#
# Should output:
#    {
#      "widget": {
#        "debug": "on",
#        "window": {
#          "title": "Sample \"Konfabulator\" Widget",
#          "name": "main_window",
#          "width": 500,
#          "height": 500
#        },
#        "image": {
#          "src": "Images/Sun.png",
#          "name": "sun1",
#          "hOffset": 250,
#          "vOffset": 250,
#          "alignment": "center"
#        },
#        "text": {
#          "data": "Click Here",
#          "size": 36,
#          "style": "bold",
#          "name": "text1",
#          "hOffset": 250,
#          "vOffset": 100,
#          "alignment": "center",
#          "onMouseUp": "sun1.opacity = (sun1.opacity / 100) * 90;"
#        }
#      }
#    }
#
# NOTE: order of `key: value` pairs is not kept.
#
# You can also use a JSON string instead of vim's dict/list to maintain order:
#    echo json.Format('{"hello": 1, "world": 2}')
#    {
#      "hello": 1,
#      "world": 2
#    }


# To be able to reformat with `gq` add following to `~/.vim/ftplugin/json.vim`:
#    import autoload 'dist/json.vim'
#    setl formatexpr=json.FormatExpr()
export def FormatExpr(): number
    FormatRange(v:lnum, v:lnum + v:count - 1)
    return 0
enddef


# import autoload 'dist/json.vim'
# command -range=% JSONFormat json.FormatRange(<line1>, <line2>)
export def FormatRange(line1: number, line2: number)
    var indent_base = matchstr(getline(line1), '^\s*')
    var indent = &expandtab ? repeat(' ', &shiftwidth) : "\t"

    var [l1, l2] = line1 > line2 ? [line2, line1] : [line1, line2]

    var json_src = getline(l1, l2)->join()
    var json_fmt = Format(json_src, {use_tabs: !&et, indent: &sw, indent_base: indent_base})->split("\n")

    exe $":{l1},{l2}d"

    if line('$') == 1 && getline(1) == ''
        setline(l1, json_fmt[0])
        append(l1, json_fmt[1 : ])
    else
        append(l1 - 1, json_fmt)
    endif
enddef


# Format JSON string or dict/list as JSON
# import autoload 'dist/json.vim'
# echo json.Format('{"hello": "world"}', {use_tabs: false, indent: 2, indent_base: 0})

# {
#   "hello": "world"
# }

# echo json.Format({'hello': 'world'}, {use_tabs: false, indent: 2, indent_base: 0})
# {
#   "hello": "world"
# }
#
# Note, when `obj` is dict, order of the `key: value` pairs might be different:
# echo json.Format({'hello': 1, 'world': 2})
# {
#   "world": 2,
#   "hello": 1
# }
export def Format(obj: any, params: dict<any> = {}): string
    var obj_str = ''
    if type(obj) == v:t_string
        obj_str = obj
    else
        obj_str = json_encode(obj)
    endif

    var indent_lvl = 0
    var indent_base = get(params, "indent_base", "")
    var indent = get(params, "use_tabs", false) ? "\t" : repeat(' ', get(params, "indent", 2))
    var json_line = indent_base
    var json = ""
    var state = ""
    for char in obj_str
        if state == ""
            if char =~ '[{\[]'
                json_line ..= char
                json ..= json_line .. "\n"
                indent_lvl += 1
                json_line = indent_base .. repeat(indent, indent_lvl)
            elseif char =~ '[}\]]'
                if json_line !~ '^\s*$'
                    json ..= json_line .. "\n"
                    indent_lvl -= 1
                    if indent_lvl < 0
                        json_line = strpart(indent_base, -indent_lvl * len(indent))
                    else
                        json_line = indent_base .. repeat(indent, indent_lvl)
                    endif
                elseif json =~ '[{\[]\n$'
                    json = json[ : -2]
                    json_line = substitute(json_line, '^\s*', '', '')
                    indent_lvl -= 1
                endif
                json_line ..= char
            elseif char == ':'
                json_line ..= char .. ' '
            elseif char == '"'
                json_line ..= char
                state = 'QUOTE'
            elseif char == ','
                json_line ..= char
                json ..= json_line .. "\n"
                json_line = indent_base .. repeat(indent, indent_lvl)
            elseif char !~ '\s'
                json_line ..= char
            endif
        elseif state == "QUOTE"
            json_line ..= char
            if char == '\'
                state = "ESCAPE"
            elseif char == '"'
                state = ""
            endif
        elseif state == "ESCAPE"
            state = "QUOTE"
            json_line ..= char
        else
            json_line ..= char
        endif
    endfor
    if json_line !~ '^\s*$'
        json ..= json_line .. "\n"
    endif
    return json
enddef
