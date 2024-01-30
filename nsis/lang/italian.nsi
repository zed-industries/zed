# vi:set ts=8 sts=4 sw=4 et fdm=marker:
#
# italian.nsi : Italian language strings for gvim NSIS installer.
#
# Locale ID    : 1040
# fileencoding : UTF-8
# Author       : Antonio Colombo, bovirus - revision: 12.05.2023

!insertmacro MUI_LANGUAGE "Italian"


# Overwrite the default translation.
# These strings should be always English.  Otherwise dosinst.c fails.
LangString ^SetupCaption     ${LANG_ITALIAN} \
        "$(^Name) Setup"
LangString ^UninstallCaption ${LANG_ITALIAN} \
        "$(^Name) Uninstall"

##############################################################################
# MUI Configuration Strings                                               {{{1
##############################################################################

#LangString str_dest_folder          ${LANG_ITALIAN} \
#    "Cartella installazione (il percorso deve finire con $\"vim$\")"

LangString str_show_readme          ${LANG_ITALIAN} \
    "Visualizza file README a fine installazione"

# Install types:
LangString str_type_typical         ${LANG_ITALIAN} \
    "Tipica"

LangString str_type_minimal         ${LANG_ITALIAN} \
    "Minima"

LangString str_type_full            ${LANG_ITALIAN} \
    "Completa"


##############################################################################
# Section Titles & Description                                            {{{1
##############################################################################

LangString str_section_old_ver      ${LANG_ITALIAN} \
    "Disinstalla versioni esistenti"
LangString str_desc_old_ver         ${LANG_ITALIAN} \
    "Disinstalla versioni esistenti di Vim."

LangString str_section_exe          ${LANG_ITALIAN} \
    "GUI e file supporto Vim"
LangString str_desc_exe             ${LANG_ITALIAN} \
    "GUI programmi e file di supporto Vim.  Questa componente è indispensabile."

LangString str_section_console      ${LANG_ITALIAN} \
    "Console Vim (vim.exe per MS-DOS)"
LangString str_desc_console         ${LANG_ITALIAN} \
    "Versione console di Vim (vim.exe)."

LangString str_section_batch        ${LANG_ITALIAN} \
    "Crea file .bat"
LangString str_desc_batch           ${LANG_ITALIAN} \
    "Crea file .bat per varianti di Vim nella cartella \
     di Windows, per utilizzo da riga di comando."

LangString str_group_icons          ${LANG_ITALIAN} \
    "Crea icone Vim"
LangString str_desc_icons           ${LANG_ITALIAN} \
    "Crea icone Vim per rendere facile l'accesso."

LangString str_section_desktop      ${LANG_ITALIAN} \
    "Icone sul Desktop"
LangString str_desc_desktop         ${LANG_ITALIAN} \
    "Crea icone programma gVim sul desktop."

LangString str_section_start_menu   ${LANG_ITALIAN} \
    "Gruppo programmi menù START"
LangString str_desc_start_menu      ${LANG_ITALIAN} \
    "Aggiunge gruppo programmi al menù START."

#LangString str_section_quick_launch ${LANG_ITALIAN} \
#    "Barra avvio veloce"
#LangString str_desc_quick_launch    ${LANG_ITALIAN} \
#    "Aggiunge un collegamento a Vim nella barra di avvio veloce."

LangString str_section_edit_with    ${LANG_ITALIAN} \
    "Aggiungi Vim al menù contestuale"
LangString str_desc_edit_with       ${LANG_ITALIAN} \
    "Aggiunge Vim al menu contestuale $\"Apri con...$\"."

#LangString str_section_edit_with32  ${LANG_ITALIAN} \
#    "Versione a 32 bit"
#LangString str_desc_edit_with32     ${LANG_ITALIAN} \
#    "Aggiungi Vim al menu contestuale $\"Apri con...$\" \
#     per applicazioni a 32 bit."

#LangString str_section_edit_with64  ${LANG_ITALIAN} \
#    "Versione a 64 bit"
#LangString str_desc_edit_with64     ${LANG_ITALIAN} \
#    "Aggiunge Vim al menu contestuale $\"Apri con...$\" \
#     per applicazioni a 64 bit."

LangString str_section_vim_rc       ${LANG_ITALIAN} \
    "Crea configurazione predefinita"
LangString str_desc_vim_rc          ${LANG_ITALIAN} \
    "Crea, se non ne esiste già uno, un file configurazione predefinito (_vimrc) ."

LangString str_group_plugin         ${LANG_ITALIAN} \
    "Crea cartella plugin"
LangString str_desc_plugin          ${LANG_ITALIAN} \
    "Crea cartella plugin.  I plugin consentono di aggiungere funzionalità \
     a Vim copiando i relativi file in una di queste cartelle."

LangString str_section_plugin_home  ${LANG_ITALIAN} \
    "Privata"
LangString str_desc_plugin_home     ${LANG_ITALIAN} \
    "Crea cartella plugin nella cartella HOME."

LangString str_section_plugin_vim   ${LANG_ITALIAN} \
    "Condivisa"
LangString str_desc_plugin_vim      ${LANG_ITALIAN} \
    "Crea cartella plugin nella cartella di installazione di Vim \
     per uso da parte di tutti gli utenti di questo sistema."

LangString str_section_nls          ${LANG_ITALIAN} \
    "Supporto nativo lingua (NLS)"
LangString str_desc_nls             ${LANG_ITALIAN} \
    "Installa i file per il supporto nativo multilingua."

LangString str_unsection_register   ${LANG_ITALIAN} \
    "Rimuovi Vim dal registro"
LangString str_desc_unregister      ${LANG_ITALIAN} \
    "Rimuove Vim dal registro di configurazione sistema."

LangString str_unsection_exe        ${LANG_ITALIAN} \
    "Elimina programmi/file di supporto Vim"
LangString str_desc_rm_exe          ${LANG_ITALIAN} \
    "Elimina tutti i programmi/file di supporto di Vim."

LangString str_ungroup_plugin       ${LANG_ITALIAN} \
    "Elimina cartelle plugin"
LangString str_desc_rm_plugin       ${LANG_ITALIAN} \
    "Elimina le cartelle plugin se sono vuote."

LangString str_unsection_plugin_home ${LANG_ITALIAN} \
    "Private"
LangString str_desc_rm_plugin_home  ${LANG_ITALIAN} \
    "Elimina cartelle plugin nella cartella HOME."

LangString str_unsection_plugin_vim ${LANG_ITALIAN} \
    "Condivise"
LangString str_desc_rm_plugin_vim   ${LANG_ITALIAN} \
    "Elimina cartelle plugin nella cartella di installazione di Vim."

LangString str_unsection_rootdir    ${LANG_ITALIAN} \
    "Elimina la cartella di installazione di Vim"
LangString str_desc_rm_rootdir      ${LANG_ITALIAN} \
    "Elimina la cartella di installazione di Vim. Contiene i file di configurazione!"


##############################################################################
# Messages                                                                {{{1
##############################################################################

#LangString str_msg_too_many_ver  ${LANG_ITALIAN} \
#    "Rilevate nel sistema $vim_old_ver_count versioni di Vim.$\r$\n\
#     Questo programma di installazione può gestire solo \
#     ${VIM_MAX_OLD_VER} versioni.$\r$\n\
#     Disinstalla qualche versione precedente e ricomincia."

#LangString str_msg_invalid_root  ${LANG_ITALIAN} \
#    "Nome cartella di installazione non valida: $vim_install_root!$\r$\n\
#     Dovrebbe terminare con $\"vim$\"."

#LangString str_msg_bin_mismatch  ${LANG_ITALIAN} \
#    "Conflitto nella cartella di installazione!$\r$\n$\r$\n\
#     La cartella di installazione dev'essere $\"$vim_bin_path$\",$\r$\n\
#     ma il sistema indica che il percorso è $\"$INSTDIR$\"."

#LangString str_msg_vim_running   ${LANG_ITALIAN} \
#    "Vim è ancora in esecuzione nel sistema.$\r$\n\
#     Per continuare chiudi tutte le sessioni attive di Vim."

#LangString str_msg_register_ole  ${LANG_ITALIAN} \
#    "Tentativo di registrazione di Vim con OLE. \
#     Non ci sono messaggi che indicano se l'operazione è riuscita."

#LangString str_msg_unreg_ole     ${LANG_ITALIAN} \
#    "Tentativo di rimozione di VIM dal registro via OLE. \
#     Non ci sono messaggi che indicano se l'operazione è riuscita."

#LangString str_msg_rm_start      ${LANG_ITALIAN} \
#    "Disinstallazione della versione:"

#LangString str_msg_rm_fail       ${LANG_ITALIAN} \
#    "Disinstallazione non riuscita per la versione:"

#LangString str_msg_no_rm_key     ${LANG_ITALIAN} \
#    "Impossibile trovare chiave disinstallazione nel registro."

#LangString str_msg_no_rm_reg     ${LANG_ITALIAN} \
#    "Impossibile trovare programma disinstallazione nel registro."

#LangString str_msg_no_rm_exe     ${LANG_ITALIAN} \
#    "Impossibile trovare programma disinstallazione."

#LangString str_msg_rm_copy_fail  ${LANG_ITALIAN} \
#    "Impossibile copiare il programma disinstallazione in una cartella temporanea."

#LangString str_msg_rm_run_fail   ${LANG_ITALIAN} \
#    "Impossibile eseguire programma disinstallazione."

#LangString str_msg_abort_install ${LANG_ITALIAN} \
#    "Il programma di disinstallazione verrà chiuso senza aver eseguito nessuna modifica."

LangString str_msg_install_fail  ${LANG_ITALIAN} \
    "Installazione non riuscita."

LangString str_msg_rm_exe_fail   ${LANG_ITALIAN} \
    "Alcuni file in $0 non sono stati eliminati!$\r$\n\
     I file vanno rimossi manualmente."

#LangString str_msg_rm_root_fail  ${LANG_ITALIAN} \
#    "AVVISO: impossibile eliminare $\"$vim_install_root$\", non è vuota!"

LangString str_msg_uninstalling  ${LANG_ITALIAN} \
    "Disinstallazione vecchia versione Vim..."

LangString str_msg_registering   ${LANG_ITALIAN} \
    "Aggiunta di Vim al registro..."

LangString str_msg_unregistering ${LANG_ITALIAN} \
    "Rimozione di Vim dal registro..."


##############################################################################
# Dialog Box                                                              {{{1
##############################################################################

LangString str_vimrc_page_title    ${LANG_ITALIAN} \
    "Scelta impostazioni _vimrc"
LangString str_vimrc_page_subtitle ${LANG_ITALIAN} \
    "Scelta impostazioni funzionalità aggiuntive, tastiera e mouse."

LangString str_msg_compat_title    ${LANG_ITALIAN} \
    " Comportamento come Vi / Vim "
LangString str_msg_compat_desc     ${LANG_ITALIAN} \
    "&Compatibilità e funzionalità"
LangString str_msg_compat_vi       ${LANG_ITALIAN} \
    "Compatibile Vi"
LangString str_msg_compat_vim      ${LANG_ITALIAN} \
    "Vim originale"
LangString str_msg_compat_defaults ${LANG_ITALIAN} \
    "Vim con alcune funzionalità aggiuntive (defaults.vim)"
LangString str_msg_compat_all      ${LANG_ITALIAN} \
    "Vim con tutte le funzionalità aggiuntive (vimrc_example.vim) (predefinito)"

LangString str_msg_keymap_title   ${LANG_ITALIAN} \
    " Mappature tastiera "
LangString str_msg_keymap_desc    ${LANG_ITALIAN} \
    "&Rimappa alcuni tasti Windows (Ctrl-V, Ctrl-C, Ctrl-A, Ctrl-S, Ctrl-F, etc.)"
LangString str_msg_keymap_default ${LANG_ITALIAN} \
    "Non rimappare i tasti (predefinito)"
LangString str_msg_keymap_windows ${LANG_ITALIAN} \
    "Rimappa solo alcuni tasti"

LangString str_msg_mouse_title   ${LANG_ITALIAN} \
    " Mouse "
LangString str_msg_mouse_desc    ${LANG_ITALIAN} \
    "&Comportamento pulsanti destro/sinistro"
LangString str_msg_mouse_default ${LANG_ITALIAN} \
    "Destro: menu popup, Sinistro: modalità visuale (predefinito)"
LangString str_msg_mouse_windows ${LANG_ITALIAN} \
    "Destro: menu popup, Sinistro: selezione modalità (Windows)"
LangString str_msg_mouse_unix    ${LANG_ITALIAN} \
    "Destro: estensione selezione, Sinistro: modalità visuale (Unix)"
