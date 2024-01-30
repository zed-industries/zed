" Filter that removes the Shell Prompt from the xxd command
" 18,20d
:1s#|\$+0&\#ffffff0| |.@1|/|x@1|d|/|x@1|d|.*\n#|$+0\&\#ffffff0| #e
