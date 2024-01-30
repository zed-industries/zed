@echo off
rem To be used on MS-Windows for Visual C++ 2015 (either Express or Community)
rem See INSTALLpc.txt for information.
rem
rem Usage:
rem   For x86 builds run this without options:
rem     msvc2015
rem   For x64 builds run this with "x86_amd64" option:
rem     msvc2015 x86_amd64
rem   This works on any editions including Express edition.
rem   If you use Community (or Professional) edition, you can also use "x64"
rem   option:
rem     msvc2015 x64

call "%VS140COMNTOOLS%..\..\VC\vcvarsall.bat" %*
