:: command to build huge Vim 64 bit with OLE, Perl and Python.
:: First run: %VCDIR%\vcvarsall.bat x86_amd64
:: Ruby and Tcl are excluded, doesn't seem to work.
SET VCDIR="C:\Program Files (x86)\Microsoft Visual Studio 9.0\VC\"
SET TOOLDIR=E:\
%VCDIR%\bin\nmake -f Make_mvc.mak CPU=AMD64 GUI=yes OLE=yes PERL=E:\perl524 DYNAMIC_PERL=yes PERL_VER=524 PYTHON=%TOOLDIR%python27 DYNAMIC_PYTHON=yes PYTHON_VER=27 PYTHON3=%TOOLDIR%python35 DYNAMIC_PYTHON3=yes PYTHON3_VER=35  %1 IME=yes CSCOPE=yes

