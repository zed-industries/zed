@echo off
rem Thin wrapper for terminal use: Zed.exe is a windows-subsystem binary, so
rem interactive shells would not wait for it; cmd running a batch script does.
"%~dp0..\Zed.exe" %*
exit /b %ERRORLEVEL%
