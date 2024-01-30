README_ole.txt for version 9.1 of Vim: Vi IMproved.

This archive contains gvim.exe with OLE interface.
This version of gvim.exe can also load a number of interface dynamically (you
can optionally install the .dll files for each interface).
It is only for MS-Windows 95/98/ME/NT/2000/XP.

Also see the README_bindos.txt, README_dos.txt and README.txt files.

Be careful not to overwrite the OLE gvim.exe with the non-OLE gvim.exe when
unpacking another binary archive!  Check the output of ":version":
	Win32s - "MS-Windows 16/32 bit GUI version"
	 Win32 - "MS-Windows 32 bit GUI version"
Win32 with OLE - "MS-Windows 32 bit GUI version with OLE support"

For further information, type this inside Vim:
	:help if_ole
