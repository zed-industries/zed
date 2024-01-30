README_dos.txt for version 9.1 of Vim: Vi IMproved.

This file explains the installation of Vim on MS-Windows systems.
See "README.txt" for general information about Vim.

There are two ways to install Vim:
A. Use the self-installing .exe file.
B. Unpack .zip files and run the install.exe program.


A. Using the self-installing .exe
---------------------------------

This is mostly self-explaining.  Just follow the prompts and make the
selections.  A few things to watch out for:

- When an existing installation is detected, you are offered to first remove
  this.  The uninstall program is then started while the install program waits
  for it to complete.  Sometimes the windows overlap each other, which can be
  confusing.  Be sure the complete the uninstalling before continuing the
  installation.  Watch the taskbar for uninstall windows.

- When selecting a directory to install Vim, use the same place where other
  versions are located.  This makes it easier to find your _vimrc file.  For
  example "C:\Program Files\vim" or "D:\vim".  A name ending in "vim" is
  preferred.

- After selecting the directory where to install Vim, clicking on "Next" will
  start the installation.


B. Using .zip files
-------------------

These are the normal steps to install Vim from the .zip archives:

1. Go to the directory where you want to put the Vim files.  Examples:
	cd C:\
	cd D:\editors
   If you already have a "vim" directory, go to the directory in which it is
   located.  Check the $VIM setting to see where it points to:
	set VIM
   For example, if you have
	C:\vim\vim91
   do
	cd C:\
   Binary and runtime Vim archives are normally unpacked in the same location,
   on top of each other.

2. Unpack the zip archives.  This will create a new directory "vim\vim91",
   in which all the distributed Vim files are placed.  Since the directory
   name includes the version number, it is unlikely that you overwrite
   existing files.
   Examples:
	pkunzip -d gvim91.zip
	unzip vim91w32.zip

   You need to unpack the runtime archive and at least one of the binary
   archives.  When using more than one binary version, be careful not to
   overwrite one version with the other, the names of the executables
   "vim.exe" and "gvim.exe" are the same.

   After you unpacked the files, you can still move the whole directory tree
   to another location.  That is where they will stay, the install program
   won't move or copy the runtime files.

3. Change to the new directory:
	cd vim\vim91
   Run the "install.exe" program.  It will ask you a number of questions about
   how you would like to have your Vim setup.  Among these are:
   - You can tell it to write a "_vimrc" file with your preferences in the
     parent directory.
   - It can also install an "Edit with Vim" entry in the Windows Explorer
     popup menu.
   - You can have it create batch files, so that you can run Vim from the
     console or in a shell.  You can select one of the directories in your
     $PATH.  If you skip this, you can add Vim to the search path manually:
     The simplest is to add a line to your autoexec.bat.  Examples:
	set path=%path%;C:\vim\vim91
	set path=%path%;D:\editors\vim\vim91
   - Create entries for Vim on the desktop and in the Start menu.

That's it!


Remarks:

- If Vim can't find the runtime files, ":help" won't work and the GUI version
  won't show a menubar.  Then you need to set the $VIM environment variable to
  point to the top directory of your Vim files.  Example:
    set VIM=C:\editors\vim
  Vim version 9.1 will look for your vimrc file in $VIM, and for the runtime
  files in $VIM/vim91.  See ":help $VIM" for more information.

- To avoid confusion between distributed files of different versions and your
  own modified vim scripts, it is recommended to use this directory layout:
  ("C:\vim" is used here as the root, replace it with the path you use)
  Your own files:
	C:\vim\_vimrc			Your personal vimrc.
	C:\vim\_viminfo			Dynamic info for 'viminfo'.
	C:\vim\vimfiles\ftplugin\*.vim	Filetype plugins
	C:\vim\...			Other files you made.
  Distributed files:
	C:\vim\vim91\vim.exe		The Vim version 9.1 executable.
	C:\vim\vim91\doc\*.txt		The version 9.1 documentation files.
	C:\vim\vim91\bugreport.vim	A Vim version 9.1 script.
	C:\vim\vim91\...		Other version 9.1 distributed files.
  In this case the $VIM environment variable would be set like this:
	set VIM=C:\vim
  Then $VIMRUNTIME will automatically be set to "$VIM\vim91".  Don't add
  "vim91" to $VIM, that won't work.

- You can put your Vim executable anywhere else.  If the executable is not
  with the other Vim files, you should set $VIM. The simplest is to add a line
  to your autoexec.bat.  Examples:
	set VIM=c:\vim
	set VIM=d:\editors\vim

- If you have told the "install.exe" program to add the "Edit with Vim" menu
  entry, you can remove it by running the "uninstall.exe".  See
  ":help win32-popup-menu".

- In Windows 95/98/NT you can create a shortcut to Vim.  This works for all
  DOS and Win32 console versions.  For the console version this gives you the
  opportunity to set defaults for the Console where Vim runs in.

  1. On the desktop, click right to get a menu.  Select New/Shortcut.
  2. In the dialog, enter Command line: "C:\command.com".  Click "Next".
  3. Enter any name.  Click "Finish".
     The new shortcut will appear on the desktop.
  4. With the mouse pointer on the new shortcut, click right to get a menu.
     Select Properties.
  5. In the Program tab, change the "Cmdline" to add "/c" and the name of the
     Vim executable.  Examples:
	C:\command.com /c C:\vim\vim91\vim.exe
	C:\command.com /c D:\editors\vim\vim91\vim.exe
  6. Select the font, window size, etc. that you like.  If this isn't
     possible, select "Advanced" in the Program tab, and deselect "MS-DOS
     mode".
  7. Click OK.

  For gvim, you can use a normal shortcut on the desktop, and set the size of
  the Window in your $VIM/_gvimrc:
	set lines=30 columns=90


For further information, type one of these inside Vim:
	:help dos
	:help win32
