# vi:set ts=8 sts=4 sw=4 et fdm=marker:
#
# russian.nsi: Russian language strings for gvim NSIS installer.
#
# Locale ID    : 1049
# Locale name  : ru-RU
# fileencoding : UTF-8
# Author       : Restorer

!insertmacro MUI_LANGUAGE "Russian"


# Overwrite the default translation.
# These strings should be always English.  Otherwise dosinst.c fails.
LangString ^SetupCaption     ${LANG_RUSSIAN} \
        "$(^Name) Setup"
LangString ^UninstallCaption ${LANG_RUSSIAN} \
        "$(^Name) Uninstall"

##############################################################################
# MUI Configuration Strings                                               {{{1
##############################################################################

#LangString str_dest_folder          ${LANG_RUSSIAN} \
#    "Маршрут к каталогу установки (должен оканчиваться каталогом $\"vim$\")"

LangString str_show_readme          ${LANG_RUSSIAN} \
    "После окончания установки ознакомиться с кратким описанием"

# Install types:
LangString str_type_typical         ${LANG_RUSSIAN} \
    "Стандартная"

LangString str_type_minimal         ${LANG_RUSSIAN} \
    "Минимальная"

LangString str_type_full            ${LANG_RUSSIAN} \
    "Полная"


##############################################################################
# Section Titles & Description                                            {{{1
##############################################################################

LangString str_section_old_ver      ${LANG_RUSSIAN} \
    "Удаление предыдущих версий"
LangString str_desc_old_ver         ${LANG_RUSSIAN} \
    "Будут удалены предыдущие установленные версии программы"

LangString str_section_exe          ${LANG_RUSSIAN} \
    "Графический интерфейс и вспомогательные файлы"
LangString str_desc_exe             ${LANG_RUSSIAN} \
    "Графический интерфейс программы Vim и все необходимые для этого файлы. \
    Это обязательный компонент"

LangString str_section_console      ${LANG_RUSSIAN} \
    "Консольная программа Vim"
LangString str_desc_console         ${LANG_RUSSIAN} \
    "Вариант редактора Vim (vim.exe), используемый в командной оболочке"

LangString str_section_batch        ${LANG_RUSSIAN} \
    "Создать командные файлы"
LangString str_desc_batch           ${LANG_RUSSIAN} \
    "Создание командных bat-файлов позволяющих работать с редактором \
     Vim из командной строки Windows"

LangString str_group_icons          ${LANG_RUSSIAN} \
    "Создать ярлыки для редактора Vim"
LangString str_desc_icons           ${LANG_RUSSIAN} \
    "Создание ярлыков редактора Vim для облегчения запуска программы"

LangString str_section_desktop      ${LANG_RUSSIAN} \
    "На Рабочем столе"
LangString str_desc_desktop         ${LANG_RUSSIAN} \
    "Создание ярлыков программы Gvim на Рабочем столе"

LangString str_section_start_menu   ${LANG_RUSSIAN} \
    "В меню кнопки Пуск"
LangString str_desc_start_menu      ${LANG_RUSSIAN} \
    "Создание ярлыков программы Gvim в меню кнопки Пуск"

#LangString str_section_quick_launch ${LANG_RUSSIAN} \
#    "На панели быстрого запуска"
#LangString str_desc_quick_launch    ${LANG_RUSSIAN} \
#    "Создание ярлыков программы GVim на панели быстрого запуска"

LangString str_section_edit_with    ${LANG_RUSSIAN} \
    "В контекстном меню"
LangString str_desc_edit_with       ${LANG_RUSSIAN} \
    "Добавление вызова программы Gvim в пункт $\"Открыть с помощью...$\" контекстного меню"

#LangString str_section_edit_with32  ${LANG_RUSSIAN} \
#    "32-разрядная версия программы"
#LangString str_desc_edit_with32     ${LANG_RUSSIAN} \
#    "Добавление вызова программы Gvim в пункт $\"Открыть с помощью...$\" контекстного меню \
#     для 32-разрядных приложений"

#LangString str_section_edit_with64  ${LANG_RUSSIAN} \
#    "64-разрядная версия программы"
#LangString str_desc_edit_with64     ${LANG_RUSSIAN} \
#    "Добавление вызова программы Gvim в пункт $\"Открыть с помощью...$\" контекстного меню \
#     для 64-разрядных приложений"

LangString str_section_vim_rc       ${LANG_RUSSIAN} \
    "Настройки программы по умолчанию"
LangString str_desc_vim_rc          ${LANG_RUSSIAN} \
    "Создание файла _vimrc с предустановленными настройками, если нет других \
    файлов настроек"

LangString str_group_plugin         ${LANG_RUSSIAN} \
    "Создать каталог для подключаемых модулей"
LangString str_desc_plugin          ${LANG_RUSSIAN} \
    "Создание каталога для размещения подключаемых модулей, которые расширяют \
     возможности редактора Vim"

LangString str_section_plugin_home  ${LANG_RUSSIAN} \
    "Личный каталог"
LangString str_desc_plugin_home     ${LANG_RUSSIAN} \
    "Создание каталога для подключаемых модулей в домашнем каталоге пользователя"

LangString str_section_plugin_vim   ${LANG_RUSSIAN} \
    "Общий каталог"
LangString str_desc_plugin_vim      ${LANG_RUSSIAN} \
    "Создание каталога для подключаемых модулей в каталоге установки редактора Vim. \
     Модули в этом каталоге будут доступны для любого пользователя \
     зарегистрировавшегося в системе"

LangString str_section_nls          ${LANG_RUSSIAN} \
    "Поддержка региональных языков"
LangString str_desc_nls             ${LANG_RUSSIAN} \
    "Установка файлов для поддержки региональных языков операционной системы"

LangString str_unsection_register   ${LANG_RUSSIAN} \
    "Отменить регистрацию компонентов программы Vim"
LangString str_desc_unregister      ${LANG_RUSSIAN} \
    "Отмена регистрации компонентов программы Vim в операционной системе"

LangString str_unsection_exe        ${LANG_RUSSIAN} \
    "Удалить файлы редактора Vim"
LangString str_desc_rm_exe          ${LANG_RUSSIAN} \
    "Удаление всех исполняемых и вспомогательных файлов редактора Vim"

LangString str_ungroup_plugin       ${LANG_RUSSIAN} \
    "Удалить каталог подключаемых модулей"
LangString str_desc_rm_plugin       ${LANG_RUSSIAN} \
    "Удаление каталога подключаемых модулей, если в нём нет файлов"

LangString str_unsection_plugin_home ${LANG_RUSSIAN} \
    "Личный каталог"
LangString str_desc_rm_plugin_home  ${LANG_RUSSIAN} \
    "Удаление каталога подключаемых модулей из домашнего каталога пользователя"

LangString str_unsection_plugin_vim ${LANG_RUSSIAN} \
    "Общий каталог"
LangString str_desc_rm_plugin_vim   ${LANG_RUSSIAN} \
    "Удаление каталога подключаемых модулей из каталога установки редактора Vim"

LangString str_unsection_rootdir    ${LANG_RUSSIAN} \
    "Удалить основной каталог программы Vim"
LangString str_desc_rm_rootdir      ${LANG_RUSSIAN} \
    "Удаление основного каталога программы Vim. В этом каталоге находятся файлы \
    настроек!"


##############################################################################
# Messages                                                                {{{1
##############################################################################

#LangString str_msg_too_many_ver  ${LANG_RUSSIAN} \
#    "Обнаружено предыдущих версий программы Vim: $vim_old_ver_count.$\r$\n\
#     Данная программа установки может удалить не более ${VIM_MAX_OLD_VER}.$\r$\n\
#     Удалить лишние версии программы Vim и повторите установку"

#LangString str_msg_invalid_root  ${LANG_RUSSIAN} \
#    "Недопустимый каталог установки программы Vim $vim_install_root!$\r$\n\
#     Маршрут установки должен оканчиваться каталогом $\"vim$\""

#LangString str_msg_bin_mismatch  ${LANG_RUSSIAN} \
#    "Недопустимый маршрут к каталогу с исполняемыми файлами!$\r$\n$\r$\n\
#     Маршрут к каталогу с исполняемыми файлами должен быть $\"$vim_bin_path$\",$\r$\n\
#     но от операционной системы получен как $\"$INSTDIR$\"."

#LangString str_msg_vim_running   ${LANG_RUSSIAN} \
#    "Программа Vim сейчас работает.$\r$\n\
#     Прежде чем продолжить, закройте все работающие редакторы Vim"

#LangString str_msg_register_ole  ${LANG_RUSSIAN} \
#    "Попытка зарегистрировать компоненты программы Vim в пространстве OLE. \
#     Но не получено уведомление об успешности данной операции"

#LangString str_msg_unreg_ole     ${LANG_RUSSIAN} \
#    "Попытка отменить регистрацию компонентов программы Vim в пространстве OLE. \
#     Но не получено уведомление об успешности данной операции"

#LangString str_msg_rm_start      ${LANG_RUSSIAN} \
#    "Выполняется удаление следующих версий программы:"

#LangString str_msg_rm_fail       ${LANG_RUSSIAN} \
#    "Произошёл сбой при выполнении удаления следующих версий программы:"

#LangString str_msg_no_rm_key     ${LANG_RUSSIAN} \
#    "Не удалось найти раздел реестра, содержащий информацию об удалении программы"

#LangString str_msg_no_rm_reg     ${LANG_RUSSIAN} \
#    "Не удалось найти программу выполняющую удаление, указанную в разделе реестра"

#LangString str_msg_no_rm_exe     ${LANG_RUSSIAN} \
#    "Отсутствуют права на доступ к программе, выполняющей удаление"

#LangString str_msg_rm_copy_fail  ${LANG_RUSSIAN} \
#    "Произошла ошибка при копировании программы удаления во временный каталог"

#LangString str_msg_rm_run_fail   ${LANG_RUSSIAN} \
#    "Произошёл сбой при запуске программы, выполняющей удаление"

#LangString str_msg_abort_install ${LANG_RUSSIAN} \
#    "Установка программы была отменена"

LangString str_msg_install_fail  ${LANG_RUSSIAN} \
    "Произошла ошибка при установке программы. Попробуйте повторить установку \
    немного попозже"
# когда луна будет в другой фазе и ветер должен дуть с юго‐запада

LangString str_msg_rm_exe_fail   ${LANG_RUSSIAN} \
    "Некоторые файлы не были удалены из каталога $0 $\r$\n\
     Необходимо выполнить их удаление самостоятельно"

#LangString str_msg_rm_root_fail  ${LANG_RUSSIAN} \
#    "Внимание! В каталоге $\"$vim_install_root$\" содержатся файлы. Удаление каталога не выполнено"

LangString str_msg_uninstalling  ${LANG_RUSSIAN} \
    "Удаление предыдущих версий программ..."

LangString str_msg_registering   ${LANG_RUSSIAN} \
    "Регистрация компонентов программы в системе..."

LangString str_msg_unregistering ${LANG_RUSSIAN} \
    "Отмена регистрации компонентов программы в системе..."


##############################################################################
# Dialog Box                                                              {{{1
##############################################################################

LangString str_vimrc_page_title    ${LANG_RUSSIAN} \
    "Установка параметров программы"
LangString str_vimrc_page_subtitle ${LANG_RUSSIAN} \
    "Параметры, используемые для клавиатуры, «мыши» и функциональности программы"

LangString str_msg_compat_title    ${LANG_RUSSIAN} \
    " Варианты использования программы "
LangString str_msg_compat_desc     ${LANG_RUSSIAN} \
    "Совместимость и функциональность программы"
LangString str_msg_compat_vi       ${LANG_RUSSIAN} \
    "Работа в варианте совместимости с редактором Vi"
LangString str_msg_compat_vim      ${LANG_RUSSIAN} \
    "Работа в варианте функциональности редактора Vim"
LangString str_msg_compat_defaults ${LANG_RUSSIAN} \
    "Работа редактора Vim с некоторыми улучшениями (файл defaults.vim)"
LangString str_msg_compat_all      ${LANG_RUSSIAN} \
    "Работа редактора Vim со всеми улучшениями (файл vimrc_example.vim). \
    Используется по умолчанию"

LangString str_msg_keymap_title   ${LANG_RUSSIAN} \
    " Клавиатурные команды "
LangString str_msg_keymap_desc    ${LANG_RUSSIAN} \
    "Клавиатурные команды используемые в ОС Windows (CTRL+V, CTRL+C, CTRL+S, CTRL+F и т. п.)"
LangString str_msg_keymap_default ${LANG_RUSSIAN} \
    "Не изменять клавиатурные команды. Использовать принятые в редакторе Vim"
LangString str_msg_keymap_windows ${LANG_RUSSIAN} \
    "Изменить указанные клавиатурные команды"

LangString str_msg_mouse_title   ${LANG_RUSSIAN} \
    " Манипулятор «мышь» "
LangString str_msg_mouse_desc    ${LANG_RUSSIAN} \
    "Действий правой и левой кнопки манипулятора «мышь»"
LangString str_msg_mouse_default ${LANG_RUSSIAN} \
    "Правая кнопка — всплывающее меню, левая кнопка — режим визуальный"
LangString str_msg_mouse_windows ${LANG_RUSSIAN} \
    "Правая кнопка — всплывающее меню, левая кнопка — режим выборки (как в ОС Windows)"
LangString str_msg_mouse_unix    ${LANG_RUSSIAN} \
    "Правая кнопка — расширяемый режим выбора, левая кнопка — режим визуальный (как в UNIX‐подобных ОС)"
