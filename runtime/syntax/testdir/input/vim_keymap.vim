" Vim Keymap file for syntax testing                                                                                                                                                                                                                                           
                                                                                                                                       
" Maintainer:   Doug Kearns <dougkearns@gmail.com>                                                                                     
" Last Changed: 2023 Nov 21                                                                                                                                                                                                                                                    
                                                                                                                                                                                                                                                                               
scriptencoding utf-8                                                                                                                                                                                                                                                           
                                                                                                                                       
let b:keymap_name = "syntax-test"                                                                                                      
                                                                                                                                                                                                                                                                               
loadkeymap                                                                                                                                                                                                                                                                     
                                                                                                                                                                                                                                                                               
" Line comment                                                                                                                         
                                                                                                                                       
  " Another line comment                
                                                                                                                                       
a A    Basic mapping                                       
'a á   More than one char in first column           
                                          
" Special notation                                                                                                                     
<char-62>      B               Special notation allowed in LHS - decimal                                                                                                                                                                                                       
c              <char-0103>     Special notation allowed in RHS - octal                                                                 
<char-0x0064>  <char-0x0044>   Special notation allowed in LHS and RHS - hexadecimal                                                   
                                         
" Vim-script comment characters                               
# <char-0x00a3>                Line should not match as a Vim9-script comment
\" “                   Line should not match as a legacy-script comment 
