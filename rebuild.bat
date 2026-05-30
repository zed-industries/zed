@echo off
setlocal

set "RUSTC_WRAPPER=sccache"
set "PATH=C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.44.35207\bin\Hostx64\x64;C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\IDE\VC;C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\Tools;%USERPROFILE%\.cargo\bin;C:\Program Files\CMake\bin;%PATH%"
set "INCLUDE=C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.44.35207\include;C:\Program Files (x86)\Windows Kits\10\include\10.0.26100.0\ucrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.26100.0\shared;C:\Program Files (x86)\Windows Kits\10\include\10.0.26100.0\um"
set "LIB=C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.44.35207\lib\x64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.26100.0\ucrt\x64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.26100.0\um\x64;C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.44.35207\lib\spectre\x64"

cd /d "F:\Lepip\zed" || exit /b 1

echo Building zed...
cargo build -p zed -j 14
if errorlevel 1 exit /b %errorlevel%

echo Build succeeded.
exit /b 0