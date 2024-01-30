" Menu Translations:	Russian

if ('utf-8' ==? &enc) && filereadable(expand('<sfile>:p:h') . '/menu_ru_ru.utf-8.vim')
    source <sfile>:p:h/menu_ru_ru.utf-8.vim
elseif ('cp1251' ==? &enc) && filereadable(expand('<sfile>:p:h') . '/menu_ru_ru.cp1251.vim')
    source <sfile>:p:h/menu_ru_ru.cp1251.vim
" elseif ('cp866' ==? &enc) && filereadable(expand('<sfile>:p:h') . '/menu_ru_ru.cp866.vim')
"    source <sfile>:p:h/menu_ru_ru.cp866.vim
elseif ('koi8-r' ==? &enc) && filereadable(expand('<sfile>:p:h') . '/menu_ru_ru.koi8-r.vim')
    source <sfile>:p:h/menu_ru_ru.koi8-r.vim
else
    echomsg 'Could not find the menu file matching the current encoding'
endif

