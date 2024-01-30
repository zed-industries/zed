@echo off
rem To be used on MS-Windows for Visual C++ 2017.
rem See INSTALLpc.txt for information.
rem
rem Usage:
rem   For x86 builds run this with "x86" option:
rem     msvc2017 x86
rem   For x64 builds run this with "x86_amd64" option:
rem     msvc2017 x86_amd64

set "VSVEROPT=-version [15.0^,16.0^)"
call "%~dp0msvc-latest.bat" %*
set VSVEROPT=
