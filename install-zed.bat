@echo off
:: ============================================
:: Zed Editor - Add "Open with Zed" to
:: right-click context menu
:: Right-click and "Run as administrator"
:: ============================================

title Zed Context Menu Setup
color 0A

echo.
echo  ======================================
echo    ZED - CONTEXT MENU SETUP
echo  ======================================
echo.

:: Check for admin rights
net session >nul 2>&1
if %errorlevel% neq 0 (
    color 0C
    echo  ERROR: Administrator rights required!
    echo.
    echo  Please right-click this file and
    echo  select "Run as administrator"
    echo.
    pause
    exit /b 1
)

set "ZED_PATH=C:\Users\Ale\AppData\Local\Programs\Zed"

:: Check if zed.exe exists at install location
if not exist "%ZED_PATH%\zed.exe" (
    color 0C
    echo  ERROR: zed.exe not found at
    echo  %ZED_PATH%
    echo.
    pause
    exit /b 1
)

echo  [1/2] Adding "Open with Zed" to right-click menu...

:: Right-click on folders
reg add "HKCU\SOFTWARE\Classes\Directory\shell\Zed" /ve /d "Open with &Zed" /f >nul
reg add "HKCU\SOFTWARE\Classes\Directory\shell\Zed" /v "Icon" /d "%ZED_PATH%\zed.exe" /f >nul
reg add "HKCU\SOFTWARE\Classes\Directory\shell\Zed\command" /ve /d "\"%ZED_PATH%\zed.exe\" \"%%V\"" /f >nul

:: Right-click inside folders (background)
reg add "HKCU\SOFTWARE\Classes\Directory\Background\shell\Zed" /ve /d "Open with &Zed" /f >nul
reg add "HKCU\SOFTWARE\Classes\Directory\Background\shell\Zed" /v "Icon" /d "%ZED_PATH%\zed.exe" /f >nul
reg add "HKCU\SOFTWARE\Classes\Directory\Background\shell\Zed\command" /ve /d "\"%ZED_PATH%\zed.exe\" \"%%V\"" /f >nul

:: Right-click on files
reg add "HKCU\SOFTWARE\Classes\*\shell\Zed" /ve /d "Open with &Zed" /f >nul
reg add "HKCU\SOFTWARE\Classes\*\shell\Zed" /v "Icon" /d "%ZED_PATH%\zed.exe" /f >nul
reg add "HKCU\SOFTWARE\Classes\*\shell\Zed\command" /ve /d "\"%ZED_PATH%\zed.exe\" \"%%1\"" /f >nul

echo        Done!
echo.

echo  [2/2] Adding Zed to PATH...
:: Add to user PATH so "zed" works from terminal
for /f "tokens=2*" %%a in ('reg query "HKCU\Environment" /v Path 2^>nul') do set "USER_PATH=%%b"
echo %USER_PATH% | findstr /i /c:"%ZED_PATH%" >nul
if %errorlevel% neq 0 (
    setx PATH "%USER_PATH%;%ZED_PATH%" >nul
    echo        Added to PATH!
) else (
    echo        Already in PATH.
)
echo.

echo  ======================================
echo       SETUP COMPLETE!
echo  ======================================
echo.
echo  Zed location: %ZED_PATH%
echo.
echo  You can now:
echo   - Right-click any folder ^> "Open with Zed"
echo   - Right-click inside a folder ^> "Open with Zed"
echo   - Right-click any file ^> "Open with Zed"
echo   - Run "zed" from terminal
echo.
echo  ======================================
echo.
pause
