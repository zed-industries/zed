@echo off
rem To be used on MS-Windows for Visual C++ 2017 or later.
rem See INSTALLpc.txt for information.
rem
rem Usage:
rem   For x86 builds run this with "x86" option:
rem     msvc-latest x86
rem   For x64 builds run this with "x86_amd64" option or "x64" option:
rem     msvc-latest x86_amd64
rem     msvc-latest x64
rem
rem Optional environment variables:
rem   VSWHERE:
rem     Full path to vswhere.exe.
rem   VSVEROPT:
rem     Option to search specific version of Visual Studio.
rem     Default: -latest
rem     To search VS2017:
rem       set "VSVEROPT=-version [15.0^,16.0^)"
rem     To search VS2019:
rem       set "VSVEROPT=-version [16.0^,17.0^)"
rem     To search VS2022:
rem       set "VSVEROPT=-version [17.0^,18.0^)"

if "%VSWHERE%"=="" (
	set "VSWHERE=%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe"
	set VSWHERE_SET=yes
)
if not exist "%VSWHERE%" (
	echo Error: vswhere not found.
	set VSWHERE=
	set VSWHERE_SET=
	exit /b 1
)

if "%VSVEROPT%"=="" (
	set VSVEROPT=-latest
	set VSVEROPT_SET=yes
)

rem Search Visual Studio Community, Professional or above.
for /f "usebackq tokens=*" %%i in (`"%VSWHERE%" %VSVEROPT% -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath`) do (
	set InstallDir=%%i
)
if exist "%InstallDir%\VC\Auxiliary\Build\vcvarsall.bat" (
	call "%InstallDir%\VC\Auxiliary\Build\vcvarsall.bat" %*
	goto done
)

rem Search Visual Studio 2017 Express.
rem (Visual Studio 2017 Express uses different component IDs.)
for /f "usebackq tokens=*" %%i in (`"%VSWHERE%" %VSVEROPT% -products Microsoft.VisualStudio.Product.WDExpress -property installationPath`) do (
	set InstallDir=%%i
)
if exist "%InstallDir%\VC\Auxiliary\Build\vcvarsall.bat" (
	call "%InstallDir%\VC\Auxiliary\Build\vcvarsall.bat" %*
) else (
	echo Error: vcvarsall.bat not found.
	rem Set ERRORLEVEL to 1.
	call
)

:done
if "%VSWHERE_SET%"=="yes" (
	set VSWHERE=
	set VSWHERE_SET=
)
if "%VSVEROPT_SET%"=="yes" (
	set VSVEROPT=
	set VSVEROPT_SET=
)
set InstallDir=
