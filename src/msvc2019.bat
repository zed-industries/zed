@echo off
rem To be used on MS-Windows for Visual C++ 2019.
rem See INSTALLpc.txt for information.
rem
rem Usage:
rem   For x86 builds run this with "x86" option:
rem     msvc2019 x86
rem   For x64 builds run this with "x64" option:
rem     msvc2019 x64

set "VSVEROPT=-version [16.0^,17.0^)"
call "%~dp0msvc-latest.bat" %*
set VSVEROPT=
