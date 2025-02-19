@echo off

"%~dp0..\Zed.exe" %*
IF %ERRORLEVEL% NEQ 0 EXIT /b %ERRORLEVEL%
