README_vms.txt for version 9.1 of Vim: Vi IMproved.

This file explains the installation of Vim on VMS systems.
See "README.txt" in the runtime archive for information about Vim.


Most information can be found in the on-line documentation.  Use ":help vms"
inside Vim.  Or get the runtime files and read runtime/doc/os_vms.txt to find
out how to install and configure Vim with runtime files etc.

To compile Vim yourself you need three archives:
  vim-X.X-rt.tar.gz	runtime files
  vim-X.X-src.tar.gz	source files
  vim-X.X-extra.tar.gz	extra source files

Compilation is recommended, in order to make sure that the correct
libraries are used for your specific system.  Read about compiling in
src/INSTALLvms.txt.

To use the binary version, you need one of these archives:

  vim-XX-exe-x86-gui.zip	X86_64 GUI/Motif executables
  vim-XX-exe-x86-term.zip	X86_64 console executables
  vim-XX-exe-ia64-gui.zip	IA64 GUI/Motif executables
  vim-XX-exe-ia64-gtk.zip	IA64 GUI/GTK executables
  vim-XX-exe-ia64-term.zip	IA64 console executables
  vim-XX-exe-axp-gui.zip	Alpha GUI/Motif executables
  vim-XX-exe-axp-gtk.zip	Alpha GUI/GTK executables
  vim-XX-exe-axp-term.zip	Alpha console executables
  vim-XX-exe-vax-gui.zip	VAX GUI executables
  vim-XX-exe-vax-term.zip	VAX console executables

and of course
  vim-XX-runtime.zip		runtime files

The binary archives contain: vim.exe, ctags.exe, xxd.exe files,
but there are also prepared "deploy ready" archives:

vim-XX-x86.zip			GUI and console executables with runtime and
				help files for X86_64 systems
vim-XX-ia64.zip			GUI and console executables with runtime and
				help files for IA64 systems
vim-XX-axp.zip			GUI and console executables with runtime and
				help files for Alpha systems
vim-XX-vax.zip			GUI and console executables with runtime and
				help files for VAX systems

GTK builds need LIBGTK library installed.

These executables and up to date patches for OpenVMS system are downloadable
from http://www.polarhome.com/vim/ or ftp://ftp.polarhome.com/pub/vim/

