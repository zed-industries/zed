@echo off

"%~dp0..\zed.exe" %*
IF %ERRORLEVEL% NEQ 0 EXIT /b %ERRORLEVEL%
